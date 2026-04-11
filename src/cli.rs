// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::{Parser, Subcommand};
use jiff::{civil::DateTime, Span};



#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Selfish
{
	#[command(subcommand)]
	pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands
{
	Init,
	Todo { #[command(subcommand)] command: TodoCommands },
	Schedule { #[command(subcommand)] command: ScheduleCommands }
}

#[derive(Subcommand)]
pub enum TodoCommands
{
	Add { name: String },
	List
}

#[derive(Subcommand)]
pub enum ScheduleCommands
{
	Add
	{
		todo: String,
		start: DateTime,
		duration: Span
	}
}
