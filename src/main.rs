// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod init;
mod outcome;
mod print;

use clap::Parser;

use crate::cli::{Command, Selfish};
use crate::init::init;
use crate::outcome::Outcome;
use crate::print::{fatal, warning};



fn main()
{
	match run()
	{
		Err(Outcome::Exit(exit))   => warning!("{}", exit),
		Err(Outcome::Fatal(fatal)) => fatal!("{}", fatal),
		_ => ()
	}
}

fn run() -> Result<(), Outcome>
{
	let selfish = Selfish::parse();

	match &selfish.command
	{
		Command::Init => init()
	}
}
