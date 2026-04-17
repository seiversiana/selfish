// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;

use clap::Parser;

use crate::cli::Selfish;




fn main()
{
	let selfish = Selfish::parse();
}
