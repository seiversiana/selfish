// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;



pub enum Exit
{
	NotInitialized,
	AlreadyInitialized(PathBuf),
	TodoExists(String)
}

impl Exit
{
	pub fn message(&self) -> String
	{
		match self
		{
			Self::NotInitialized =>
				"selfish is not yet initialized.".to_string(),

			Self::AlreadyInitialized(path) =>
				format!("selfish is already initialized at {}.", path.display()),

			Self::TodoExists(name) =>
				format!("A todo named '{}' already exists.", name)
		}
	}
}

pub enum FatalError
{
	ProjectDirsUnavailable,
	BadDataTree(PathBuf),
	DirCreateFail(PathBuf),
	FileCreateFail(PathBuf),
	FileReadFail(PathBuf),
	FileWriteFail(PathBuf),
	DeserializeFail(PathBuf),
	SerializeFail(PathBuf)
}

impl FatalError
{
	pub fn message(&self) -> String
	{
		match self
		{
			Self::ProjectDirsUnavailable =>
				"Failed to retrieve project dirs.".to_string(),

			Self::BadDataTree(path) =>
				format!("Data directory '{}' is in an invalid state.", path.display()),

			Self::DirCreateFail(path) =>
				format!("Failed to create directory '{}'.", path.display()),

			Self::FileCreateFail(path) =>
				format!("Failed to create file '{}'.", path.display()),

			Self::FileReadFail(path) =>
				format!("Failed to read from '{}'.", path.display()),

			Self::FileWriteFail(path) =>
				format!("Failed to write to '{}'.", path.display()),

			Self::DeserializeFail(path) =>
				format!("Failed to deserialize from '{}'.", path.display()),

			Self::SerializeFail(path) =>
				format!("Failed to serialize to '{}'.", path.display())
		}
	}
}

pub enum Outcome
{
	Exit(Exit),
	FatalError(FatalError)
}
