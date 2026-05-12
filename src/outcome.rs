// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;


pub enum Exit
{

}

pub enum Fatal
{

}

pub enum Outcome
{
	Fatal(Fatal),
	Exit(Exit)
}

impl From<Exit> for Outcome
{
	fn from(value: Exit) -> Self
	{
		Outcome::Exit(value)
	}
}

impl From<Fatal> for Outcome
{
	fn from(value: Fatal) -> Self
	{
		Outcome::Fatal(value)
	}
}
