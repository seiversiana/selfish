// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs::create_dir_all;

use directories::ProjectDirs;



pub fn init()
{
	let dirs = ProjectDirs::from("com", "seiversiana", "selfish").expect("Could not retrieve project dirs.");
	let root = dirs.data_local_dir();

	if root.exists()
	{
		println!("Selfish is already initialized!");
		return;
	}

	create_dir_all(root).expect("Could not create data root dir.");

	println!("Successfully initialized selfish.");
}
