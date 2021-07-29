use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	/// Command
	pub action: String,

	/// Arguments
	pub arguments: Vec<String>,
}
