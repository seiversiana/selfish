// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use jiff::civil::DateTime;



pub struct ScheduleItem
{
	todo: String,
	start: DateTime,
	end: DateTime
}

pub type Schedule = Vec<ScheduleItem>;
