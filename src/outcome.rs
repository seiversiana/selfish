// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;



#[derive(Debug, Error)]
pub enum Exit
{

}

#[derive(Debug, Error)]
pub enum Fatal
{

}

pub enum Outcome
{
	Exit(Exit),
	Fatal(Fatal)
}

impl From<Exit> for Outcome
{
	fn from(value: Exit) -> Self {
		Outcome::Exit(value)
	}
}

impl From<Fatal> for Outcome
{
	fn from(value: Fatal) -> Self {
		Outcome::Fatal(value)
	}
}
