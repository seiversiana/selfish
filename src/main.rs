// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod init;
mod outcome;

use clap::Parser;

use crate::cli::{Command, Selfish};
use crate::init::init;
use crate::outcome::Outcome;



fn main()
{
	let selfish = Selfish::parse();

	match match &selfish.command
		{
			Command::Init => init()
		}
	{
		Err(Outcome::Exit(exit))   => println!("{}", exit),
		Err(Outcome::Fatal(fatal)) => println!("{}", fatal),
		_ => ()
	}
}
