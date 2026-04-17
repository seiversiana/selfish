// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0



macro_rules! color_println {
	($color:ident, $($args:tt)*) => {{
		use colored::Colorize;
		println!("{}", format!($($args)*).$color());
	}};
}

macro_rules! fatal {
	($($args:tt)*) => {{
		crate::print::color_println!(red, $($args)*);
	}}
}

macro_rules! warning {
	($($args:tt)*) => {{
		crate::print::color_println!(yellow, $($args)*);
	}}
}

macro_rules! info {
	($($args:tt)*) => {{
		crate::print::color_println!(blue, $($args)*);
	}}
}

macro_rules! success {
	($($args:tt)*) => {{
		crate::print::color_println!(green, $($args)*);
	}}
}

pub(crate) use {color_println, fatal, warning, info, success};
