// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs::create_dir_all;

use directories::ProjectDirs;

use crate::outcome::{Exit, Fatal, Outcome};



pub fn init() -> Result<(), Outcome>
{
	let dirs = ProjectDirs::from("com", "seiversiana", "selfish")
		.ok_or(Fatal::MissingProjectDirs)?;

	let root = dirs.data_local_dir();
	if root.exists()
	{
		return Err(Exit::AlreadyInitialized(root.to_path_buf()).into())
	}

	create_dir_all(root).map_err(|_| Fatal::DirCreateFail(root.to_path_buf()))?;

	println!("Successfully initialized selfish.");
	Ok(())
}
