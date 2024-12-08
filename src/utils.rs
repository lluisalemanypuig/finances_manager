/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage domestic financial
 * activities.
 *
 * Copyright (C) 2024
 *
 * This file is part of Finances manager. The full code is available at:
 *      https://github.com/lluisalemanypuig/finances_manager.git
 *
 * Finances Manager is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Finances Manager is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Finances Manager.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Contact:
 *
 *     Lluís Alemany Puig
 *         email: lluis.alemany.puig@gmail.com
 *         https://github.com/lluisalemanypuig
 *         lluisalemanypuig.github.io
 *
 ********************************************************************/

use unicode_normalization::UnicodeNormalization;

pub fn vector_includes(v: &[String], w: &[String]) -> bool {
	let limit = std::cmp::min(v.len(), w.len());
	for i in 0..limit {
		if v[i].to_lowercase() != w[i].to_lowercase() {
			return false;
		}
	}
	true
}

pub fn compare_strings(s: &String, t: &String, case_sensitive: bool, utf8_sensitive: bool) -> bool {
	let ss = if utf8_sensitive {
		s.to_string()
	} else {
		s.nfd().filter(|c| c.is_ascii()).collect::<String>()
	};
	let tt = if utf8_sensitive {
		t.to_string()
	} else {
		t.nfd().filter(|c| c.is_ascii()).collect::<String>()
	};

	if case_sensitive {
		ss == tt
	} else {
		ss.to_lowercase() == tt.to_lowercase()
	}
}

pub fn string_contains(
	_contained: &String,
	_containee: &String,
	case_sensitive: bool,
	utf8_sensitive: bool,
) -> bool {
	let contained = if utf8_sensitive {
		_contained.to_string()
	} else {
		_contained
			.nfd()
			.filter(|c| c.is_ascii())
			.collect::<String>()
	};
	let containee = if utf8_sensitive {
		_containee.to_string()
	} else {
		_containee
			.nfd()
			.filter(|c| c.is_ascii())
			.collect::<String>()
	};

	if case_sensitive {
		containee.contains(&contained)
	} else {
		containee.to_lowercase().contains(&contained.to_lowercase())
	}
}
