use crate::args;
use crate::cli;
use crate::config;

pub fn run(args: args::Arguments) -> anyhow::Result<()> {
	let config = config::parse().unwrap();
	cli::match_cmds(args, config)?;
	Ok(())
}
