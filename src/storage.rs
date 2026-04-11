// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use directories::ProjectDirs;

use crate::outcome::{Outcome, FatalError};



pub struct DataTree
{
	dirs: ProjectDirs
}

impl DataTree
{
	pub fn from_default() -> Result<Self, Outcome>
	{
		let dirs = ProjectDirs::from("com", "seiversiana", "selfish")
			.ok_or(Outcome::FatalError(FatalError::ProjectDirsUnavailable))?;

		Ok(DataTree { dirs })
	}

	pub fn initialized(&self) -> Result<bool, Outcome>
	{
		let existences =
		[
			self.root().exists(),
			self.todo_path().exists()
		];

		let first = existences[0];
		if existences.iter().all(|&v| v == first)
		{
			Ok(first)
		}
		else
		{
			Err(Outcome::FatalError(FatalError::BadDataTree(self.root())))
		}
	}

	pub fn root(&self) -> PathBuf
	{
		self.dirs.data_local_dir().to_path_buf()
	}

	pub fn todo_path(&self) -> PathBuf
	{
		self.root().join("todo.json")
	}

	pub fn schedule_path(&self) -> PathBuf
	{
		self.root().join("schedule.json")
	}
}
