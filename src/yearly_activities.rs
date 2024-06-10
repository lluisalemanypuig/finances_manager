/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage financial domestic
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

use crate::date::Month;
use crate::monthly_activities::MonthlyActivities;

#[derive(Debug)]
pub struct YearlyActivities {
	changes: bool,

	pub year: u32,
	pub activities: Vec<MonthlyActivities>
}

impl Eq for YearlyActivities {}

impl PartialEq for YearlyActivities {
	fn eq(&self, other: &Self) -> bool {
		self.year == other.year
	}
}
impl Ord for YearlyActivities {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.year.cmp(&other.year)
	}
}
impl PartialOrd for YearlyActivities {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq<u32> for YearlyActivities {
	fn eq(&self, y: &u32) -> bool {
		self.year == *y
	}
}
impl PartialOrd<u32> for YearlyActivities {
	fn partial_cmp(&self, y: &u32) -> Option<std::cmp::Ordering> {
		Some(self.year.cmp(y))
	}
}
impl PartialEq<YearlyActivities> for u32 {
	fn eq(&self, other: &YearlyActivities) -> bool {
		*self == other.year
	}
}
impl PartialOrd<YearlyActivities> for u32 {
	fn partial_cmp(&self, other: &YearlyActivities) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other.year))
	}
}

impl YearlyActivities {
	pub fn new() -> YearlyActivities {
		YearlyActivities {
			changes: false,
			year: 0,
			activities: Vec::new()
		}
	}
	pub fn new_year(y: &u32, changes: bool) -> YearlyActivities {
		YearlyActivities {
			year: *y,
			activities: Vec::new(),
			changes
		}
	}

	pub fn as_ref(&self) -> &YearlyActivities { self }
	pub fn as_mut(&mut self) -> &mut YearlyActivities { self }

	pub fn has_changes(&self) -> bool { self.changes }
	pub fn set_changes(&mut self, c: bool) {
		self.changes = c;
	}

	pub fn has_month(&self, m: &Month) -> bool {
		let res = self.activities.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(_) => true,
			Err(_) => false
		}
	}

	#[duplicate::duplicate_item(
		method           convert   reference(type);
		[get_month]      [as_ref]  [& type]       ;
		[get_month_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), m: &Month) -> Option<reference([MonthlyActivities])> {
		let res = self.activities.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(idx) => Some(self.activities[idx].convert()),
			Err(_) => None
		}
	}

	pub fn add_month(&mut self, m: &Month) -> &mut MonthlyActivities {
		let res = self.activities.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(pos) => {
				// month already exists
				&mut self.activities[pos]
			},
			Err(pos) => {
				// month does not exist
				self.activities.insert(pos, MonthlyActivities::new());
				&mut self.activities[pos]
			}
		}
	}

	pub fn push_month(&mut self, m: MonthlyActivities) {
		self.activities.push(m);
	}

	pub fn merge(&mut self, year_acts: YearlyActivities) {
		for month in year_acts.activities.into_iter() {
			if !self.has_month(&month.month) {
				self.push_month(month);
			}
			else {
				self.get_month_mut(&month.month).unwrap().merge(month);
			}
		}
	}

}
