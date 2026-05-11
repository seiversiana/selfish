// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;

use clap::Parser;

use cli::{Cli, Commands};



fn main()
{
	let cli = Cli::parse();

	match &cli.command
	{
		Commands::Init => todo!()
	}
}
