// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use thiserror::Error;



#[derive(Debug, Error)]
pub enum Exit
{
	#[error("Selfish is already initialized at {0}.")]
	AlreadyInitialized(PathBuf),
}

#[derive(Debug, Error)]
pub enum Fatal
{
	#[error("Could not retrieve project dirs.")]
	MissingProjectDirs,

	#[error("Could not create directory {0}.")]
	DirCreateFail(PathBuf),
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
