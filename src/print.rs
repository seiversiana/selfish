// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0



macro_rules! color_println {
	($color:ident, $($args:tt)*) => {{
		use colored::Colorize;
		println!("{}", format!($($args)*).$color())
	}};
}

pub(crate) use color_println;
