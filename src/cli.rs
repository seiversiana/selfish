// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Selfish
{
	#[command(subcommand)]
	command: Command
}

#[derive(Subcommand)]
pub enum Command
{
	Init
}
