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

extern crate duplicate;

use crate::expense::Expense;
use crate::income::Income;

use crate::date::Month;
use crate::concept_types::ConceptTypes;
use crate::monthly_activities::MonthlyActivities;
use crate::yearly_activities::YearlyActivities;

#[derive(Debug)]
pub struct AllActivities {
	m_min_year: u32,
	m_max_year: u32,

	pub expense_types: ConceptTypes,
	pub income_types: ConceptTypes,
	pub activities: Vec<YearlyActivities>
}

impl AllActivities {
	pub fn new() -> AllActivities {
		AllActivities {
			m_min_year: 9999,
			m_max_year: 0,
			expense_types: ConceptTypes::new(),
			income_types: ConceptTypes::new(),
			activities: Vec::new()
		}
	}

	#[duplicate::duplicate_item(
		method          convert   reference(type);
		[get_year]      [as_ref]  [& type]       ;
		[get_year_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), y: &u32) -> Option<reference([YearlyActivities])> {
		if !(self.m_min_year <= *y && *y <= self.m_max_year) {
			return None;
		}
		
		let res = self.activities.binary_search_by(|e| e.get_year().cmp(&y));
		if let Ok(idx) = res {
			Some( self.activities[idx].convert() )
		}
		else {
			None
		}
	}

	#[duplicate::duplicate_item(
		method               retrieve       result;
		[get_month_expenses] [get_expenses] [Expense];
		[get_month_incomes]  [get_incomes]  [Income] ;
	)]
	pub fn method(&self, y: &u32, m: &Month) -> Option<&MonthlyActivities<result>> {
		if let Some(year) = self.get_year(y) {
			year.retrieve().get_month(m)
		}
		else {
			None
		}
	}

	pub fn has_year(&self, y: &u32) -> bool {
		self.activities.binary_search_by(|e| e.get_year().cmp(&y)).is_ok()
	}

	pub fn add_year(&mut self, y: u32) -> &mut YearlyActivities {
		let res = self.activities.binary_search_by(|e| e.get_year().cmp(&y));
		match res {
			Ok(pos) => {
				// year already exists
				&mut self.activities[pos]
			},
			Err(pos) => {
				// year does not exist
				self.activities.insert(pos,
					YearlyActivities::new_year(y)
				);
				if self.m_min_year > y {
					self.m_min_year = y;
				}
				if self.m_max_year < y {
					self.m_max_year = y;
				}
				&mut self.activities[pos]
			}
		}
	}

	pub fn push_year(&mut self, y: YearlyActivities) {
		let res = self.activities.binary_search_by(|e| e.get_year().cmp(&y.get_year()));
		match res {
			Ok(_) => { },
			Err(pos) => {
				if self.m_min_year > *y.get_year() {
					self.m_min_year = *y.get_year();
				}
				if self.m_max_year < *y.get_year() {
					self.m_max_year = *y.get_year();
				}
				self.activities.insert(pos, y);
			}
		}
	}

	pub fn merge(&mut self, year_acts: YearlyActivities) {
		if !self.has_year(&year_acts.get_year()) {
			self.push_year(year_acts);
		}
		else {
			self.get_year_mut(&year_acts.get_year()).unwrap().merge(year_acts);
		}
	}

	pub fn set_changes(&mut self, c: bool) {
		for ye in self.activities.iter_mut() {
			ye.set_changes(c);
		}
	}

}
