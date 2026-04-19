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
	run();
}

fn run() -> Result<(), Outcome>
{
	let selfish = Selfish::parse();

	match &selfish.command
	{
		Command::Init => init(),
		_ => todo!()
	}
}
