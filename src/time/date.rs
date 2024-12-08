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

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Month {
	January = 0,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December,
}

impl Month {
	pub fn from_u32(value: u32) -> Option<Month> {
		match value {
			0 => Some(Month::January),
			1 => Some(Month::February),
			2 => Some(Month::March),
			3 => Some(Month::April),
			4 => Some(Month::May),
			5 => Some(Month::June),
			6 => Some(Month::July),
			7 => Some(Month::August),
			8 => Some(Month::September),
			9 => Some(Month::October),
			10 => Some(Month::November),
			11 => Some(Month::December),
			_ => None,
		}
	}

	pub fn next(&self) -> Month {
		let m = self.clone() as u32;
		match Month::from_u32(m + 1) {
			Some(m) => m,
			None => Month::January,
		}
	}
}

impl fmt::Display for Month {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Month::January => write!(f, "January"),
			Month::February => write!(f, "February"),
			Month::March => write!(f, "March"),
			Month::April => write!(f, "April"),
			Month::May => write!(f, "May"),
			Month::June => write!(f, "June"),
			Month::July => write!(f, "July"),
			Month::August => write!(f, "August"),
			Month::September => write!(f, "September"),
			Month::October => write!(f, "October"),
			Month::November => write!(f, "November"),
			Month::December => write!(f, "December"),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMonthError;

impl FromStr for Month {
	type Err = ParseMonthError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"January" => Ok(Month::January),
			"February" => Ok(Month::February),
			"March" => Ok(Month::March),
			"April" => Ok(Month::April),
			"May" => Ok(Month::May),
			"June" => Ok(Month::June),
			"July" => Ok(Month::July),
			"August" => Ok(Month::August),
			"September" => Ok(Month::September),
			"October" => Ok(Month::October),
			"November" => Ok(Month::November),
			"December" => Ok(Month::December),
			_ => Err(ParseMonthError),
		}
	}
}

/* ------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
	// It is crucial that these fields are declared in this exact order,
	// otherwise the PartialOrd and Ord traits will not compare dates properly
	pub year: u32,
	pub month: Month,
	pub day: u8,
}

impl From<(u8, Month, u32)> for Date {
	fn from(item: (u8, Month, u32)) -> Self {
		let (d, m, y) = item;
		Date {
			day: d,
			month: m,
			year: y,
		}
	}
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}/{}", self.year, self.month, self.day)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDateError;

impl FromStr for Date {
	type Err = ParseDateError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split("/").collect();
		let [year, month, day] = parts.as_slice() else {
			panic!("Can't segment date '{s}' using '/'")
		};

		let year_fromstr = year.parse::<u32>().map_err(|_| ParseDateError)?;
		let month_fromstr = month
			.parse::<Month>()
			.expect(format!("String '{month}' not valid for a month").as_str());
		let day_fromstr = day.parse::<u8>().map_err(|_| ParseDateError)?;

		Ok(Date {
			year: year_fromstr,
			month: month_fromstr,
			day: day_fromstr,
		})
	}
}

/* ------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
pub struct YearMonth {
	pub year: u32,
	pub month: Month,
}

pub struct YearMonthIter {
	pub current: YearMonth,
	pub end: YearMonth,
}

impl YearMonthIter {
	pub fn new(start: YearMonth, end: YearMonth) -> YearMonthIter {
		YearMonthIter {
			current: start,
			end,
		}
	}
}

impl Iterator for YearMonthIter {
	type Item = YearMonth;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current.year > self.end.year {
			return None;
		}
		if self.current.year == self.end.year {
			if self.current.month > self.end.month {
				return None;
			}
		}

		let result: YearMonth = self.current.clone();

		self.current.month = self.current.month.next();
		if self.current.month == Month::January {
			self.current.year = self.current.year + 1;
		}

		Some(result)
	}
}

pub fn month_range(start: YearMonth, end: YearMonth) -> YearMonthIter {
	YearMonthIter::new(start, end)
}
