// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::outcome::{Outcome, FatalError};



pub fn create_if_missing(path: &Path, content: &str) -> Result<(), Outcome>
{
	let mut file = match fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(path)
	{
		Ok(file) => file,
		Err(error) if error.kind() == io::ErrorKind::AlreadyExists =>
		{
			return Ok(());
		}
		Err(_) =>
		{
			return Err(Outcome::FatalError(FatalError::FileCreateFail(path.to_path_buf())));
		}
	};

	file.write_all(content.as_bytes())
		.map_err(|_| Outcome::FatalError(FatalError::FileWriteFail(path.to_path_buf())))
}

pub fn read_deserialize<T: DeserializeOwned>(path: &Path) -> Result<T, Outcome>
{
	let data = fs::read_to_string(path)
		.map_err(|_| Outcome::FatalError(FatalError::FileReadFail(path.to_path_buf())))?;

	serde_json::from_str(&data)
		.map_err(|_| Outcome::FatalError(FatalError::DeserializeFail(path.to_path_buf())))
}

pub fn write_serialize<T: Serialize>(path: &Path, data: &T) -> Result<(), Outcome>
{
	let data = serde_json::to_string_pretty(data)
		.map_err(|_| Outcome::FatalError(FatalError::SerializeFail(path.to_path_buf())))?;

	fs::write(&path, data)
		.map_err(|_| Outcome::FatalError(FatalError::FileWriteFail(path.to_path_buf())))
}
