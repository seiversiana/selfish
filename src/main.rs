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
use crate::print::color_println;



fn main()
{
	match run()
	{
		Err(Outcome::Exit(exit))   => color_println!(yellow, "{}", exit),
		Err(Outcome::Fatal(fatal)) => color_println!(red, "{}", fatal),
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
