use home::home_dir;
use std::env::{current_dir, set_current_dir};
use std::path::Component;

use clap::Parser;

/// Print a neatly formatted path to working directory, for use in custom prompts
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
	/// Max length of path components (except the leaf directory)
	#[arg(short, long)]
	length: Option<usize>,

	/// Placeholder for elided path components
	#[arg(short, long)]
	placeholder: Option<String>,

	/// Placeholder for home directory
	#[arg(short = 'H', long)]
	home_icon: Option<String>,
}

fn main() {
	let args = Args::parse();

	// get current working directory, relative to home if possible
	let (is_in_root, pwd) = {
		let pwd = current_dir().unwrap();
		let home = home_dir().unwrap();
		let home = home.as_path();
		let maybe_pwd = pwd.strip_prefix(home);
		let is_in_root = maybe_pwd.is_ok();
		let pwd = if is_in_root { maybe_pwd.expect("infallible") } else { pwd.as_path() };
		(is_in_root, pwd.to_owned())
	};

	// construct formatted path, shortening long directory names
	let mut components = vec![];
	let num = pwd.components().count();
	for (i, c) in pwd.components().enumerate() {
		let mut s = match c {
			Component::RootDir => "".to_string(),
			Component::Normal(s) => s.to_os_string().into_string().unwrap(),
			Component::CurDir => ".".to_string(),
			Component::ParentDir => "..".to_string(),
			Component::Prefix(_) => unimplemented!("Windows paths are currently not supported.")
		};
		// don't apply placeholder replacement on final element
		if i == num - 1 {
			components.push(s);
			break;
		}
		if s.len() > args.length.unwrap_or(6) {
			s = args.placeholder.clone().unwrap_or("...".to_string());
		}
		components.push(s);
	}

	// replace homedir and root character
	if is_in_root {
		components.insert(0, args.home_icon.unwrap_or("~".to_string()));
	} else if components.len() == 1 {
		components = vec!["/".to_string()];
	}

	print!("{}", components.join("/"));
}
