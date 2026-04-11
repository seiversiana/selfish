// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;

use colored::Colorize;

use crate::files::create_if_missing;
use crate::storage::DataTree;
use crate::outcome::{Exit, FatalError, Outcome};



pub fn init(tree: &DataTree) -> Result<(), Outcome>
{
	let initialized = tree.initialized();

	match initialized
	{
		Ok(true) => {
			return Err(Outcome::Exit(Exit::AlreadyInitialized(tree.root().to_path_buf())));
		}

		Err(_) =>
		{
			println!("{}", "selfish data dir already exists but is incomplete. Creating the missing files.".blue());
		}

		_ =>
		{
			let root = tree.root();
			fs::create_dir_all(&root).map_err(|_| Outcome::FatalError(FatalError::DirCreateFail(root)))?;
		}
	}

	let content = "{}";
	create_if_missing(&tree.todo_path(), &content)?;
	create_if_missing(&tree.schedule_path(), &content)?;

	println!("{}", "Successfully initialized selfish.".green());

	Ok(())
}
