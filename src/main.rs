// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod init;

use clap::Parser;

use crate::cli::{Command, Selfish};
use crate::init::init;



fn main()
{
	let selfish = Selfish::parse();

	match &selfish.command
	{
		Command::Init => init()
	}
}
