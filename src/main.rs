use std::{
	fs::{File, OpenOptions},
	io::{Read, Write},
};

use alm::{Alm, Rule};
use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "alm")]
#[command(about = "Amy's Lightweight Markup Language", long_about = None)]
struct Cli {
	#[command(subcommand)]
	pub format: OutputFormat,
}

#[derive(Debug, Subcommand)]
enum OutputFormat {
	Godot {
		#[arg(short, long, default_value = "output.txt")]
		output: String,

		input: String,
	},

	Html {
		#[arg(short, long, default_value = "output.html")]
		output: String,

		input: String,
	},
}

#[derive(Debug, Error)]
pub enum AppError {
	#[error("Error processing command line arguments: {0}")]
	CliError(String),

	#[error("Error opening, reading, or writing file `{0}`")]
	IoError(#[from] std::io::Error),

	#[error("Error parsing file `{0}`")]
	ParseError(#[from] pest::error::Error<Rule>),
}

fn main() -> Result<(), AppError> {
	let args = Cli::parse();
	let i;
	let o;
	let f;
	match args.format {
		OutputFormat::Godot { output, input } => {
			f = alm::OutputFormat::Godot;
			i = input;
			o = output;
		}
		OutputFormat::Html { output, input } => {
			f = alm::OutputFormat::Html;
			i = input;
			o = output;
		}
	};
	println!("Processing \"{i}\" -> \"{o}\"...");
	let success_message = format!("Successfully processed \"{i}\" -> \"{o}\".");
	let mut ifile = File::open(i)?;
	let mut buf = String::new();
	let _ = ifile.read_to_string(&mut buf)?;
	let out = Alm::parse(buf.as_str(), f)?;
	let mut ofile = OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(o)?;
	let _ = ofile.write_all(out.as_bytes())?;
	println!("{}", success_message);
	Ok(())
}
