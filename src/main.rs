// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod outcome;

use clap::Parser;

use crate::cli::Selfish;
use crate::outcome::Outcome;



fn main()
{
	run();
}

fn run() -> Result<(), Outcome>
{
	let selfish = Selfish::parse();

	Ok(())
}
