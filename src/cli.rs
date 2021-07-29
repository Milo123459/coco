use crate::args;
use crate::config;
use crate::match_patterns;
use fancy_regex::Regex;
use std::io::Error;
use std::process::Command;

pub fn action(input: Vec<&str>) -> anyhow::Result<()> {
	// this will sanitize the vec in a sense
	// the input has \" \" around the value we want so we remove it
	// we also filter out _ from the vec
	let actions = input
		.into_iter()
		.filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
		.collect::<Vec<_>>();
	// log a nice message displaying all the actions
	println!("Commands available:\n{}", actions.join(", "));
	Ok(())
}

fn template(config: String, args: &args::Arguments) -> anyhow::Result<String> {
	let splitted = config.split('+').skip(1);

	let mut result = String::from(&config);

	for val in splitted {
		if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == *"+" {
			let idx = val.chars().next().unwrap().to_digit(10).unwrap() - 1;
			let rest = &args.arguments[idx as usize..];

			if rest.is_empty() {
				return Err(anyhow::Error::new(Error::new(
					std::io::ErrorKind::InvalidInput,
					format!(
						"Argument {0} was not provided. Argument {0} is a rest argument.",
						String::from(val).split("").collect::<Vec<_>>()[1]
					),
				)));
			}

			result = result.replace(
				&format!("+{}+", String::from(val).split("").collect::<Vec<_>>()[1]),
				&rest.join(" "),
			);
		} else {
			let idx = val.split("").nth(1).unwrap().parse::<usize>().unwrap() - 1;

			if args.arguments.len() <= idx {
				return Err(anyhow::Error::new(Error::new(
					std::io::ErrorKind::InvalidInput,
					format!(
						"Argument {} was not provided.",
						String::from(val).split("").collect::<Vec<_>>()[1]
					),
				)));
			}
			let val_ = (&args.arguments[idx]).clone();
			let captures = Regex::new(&format!(
				"\\+{}(?!\\+)",
				String::from(val).split("").collect::<Vec<_>>()[1]
			))?;

			let res = result.clone();

			// poor mans replace
			for _ in 0..captures.captures_iter(&res).count() {
				let res = result.clone();

				let capture = captures
					// when we replace, the value changes, so we rerun the logic
					.captures_iter(&res)
					.collect::<Vec<_>>()
					// we dont use the loop index as since the new value excludes the previous match we dont need to
					.get(0)
					.unwrap()
					.as_ref()
					.unwrap()
					.get(0)
					.unwrap();
				result.replace_range(capture.range(), &val_);
			}
		}
	}
	Ok(result)
}

pub fn run(script: &config::Script, args: args::Arguments) -> anyhow::Result<()> {
	Command::new("curl")
		.arg(template(script.url.clone(), &args).unwrap())
		.status()?;
	Ok(())
}

pub fn run_cmd(args: args::Arguments, config: config::ConfigFile, cmd: &str) -> anyhow::Result<()> {
	let scripts = config.scripts;
	if scripts
		.iter()
		.any(|v| v.name.to_lowercase() == cmd.to_lowercase())
	{
		run(
			scripts
				.iter()
				.find(|v| v.name.to_lowercase() == cmd.to_lowercase())
				.unwrap(),
			args,
		)
		.unwrap();
	} else {
		println!("No script {} found.", cmd);
	}
	Ok(())
}

pub fn list(config: config::ConfigFile) -> anyhow::Result<()> {
	for script in config.scripts {
		println!("{}", script.name);
	}
	Ok(())
}

pub fn add(config: config::ConfigFile, args: args::Arguments) -> anyhow::Result<()> {
	if args.arguments.first().is_some() && args.arguments.get(1).is_some() {
		if !config
			.scripts
			.iter()
			.any(|i| i.name.to_lowercase() == args.arguments.first().unwrap().to_lowercase())
		{
			let mut new_config = config;
			let cloned = args.clone();
			let name = cloned.arguments.first().unwrap();
			let mut url = args;
			url.arguments.remove(0);
			new_config.scripts.push(config::Script {
				name: name.clone(),
				url: url.arguments.join(" "),
			});
			config::write(new_config)?;
			println!("Added script {}", name);
		} else {
			println!("You cannot create a script which has already been created! (Attempted to create a script with a name that was already taken)")
		}
	} else {
		println!("Please provide 2 arguments, the name and the URL with format strings.");
	}
	Ok(())
}

pub fn match_cmds(args: args::Arguments, config: config::ConfigFile) -> anyhow::Result<()> {
	let cmd = &args.action;
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"action" => action(patterns)?,
		"run" => run_cmd(args.clone(), config, cmd)?,
		"list" => list(config)?,
		"add" => add(config, args)?,
		_ => {
			let scripts = config.scripts;
			if scripts.iter().any(|v| v.name.to_lowercase() == cmd.to_lowercase()) {
				return run(scripts.iter().find(|v| v.name.to_lowercase() == cmd.to_lowercase()).unwrap(), args);
			} else {
				return Err(anyhow::Error::new(Error::new(
				std::io::ErrorKind::InvalidInput,
				"Invalid action. Try the command `action` or `list` to list scripts.",
			)))
		}
	}
	};
	Ok(())
}
