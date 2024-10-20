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

use crate::economy::expense::Expense;
use crate::economy::income::Income;
use crate::economy::monthly_activities::MonthlyActivities;
use crate::economy::yearly_activities::YearlyActivities;

use crate::time::date::Month;

use crate::concepts::types;
use crate::concepts::types::ConceptTypes;

#[derive(Debug)]
pub struct AllActivities {
	m_min_year: u32,
	m_max_year: u32,

	m_expense_types: ConceptTypes,
	m_income_types: ConceptTypes,
	m_activities: Vec<YearlyActivities>
}

impl AllActivities {
	pub fn new() -> AllActivities {
		AllActivities {
			m_min_year: 9999,
			m_max_year: 0,
			m_expense_types: ConceptTypes::new(),
			m_income_types: ConceptTypes::new(),
			m_activities: Vec::new()
		}
	}

	pub fn iter_activities(&self) -> std::slice::Iter<'_, YearlyActivities> { self.m_activities.iter() }
	pub fn iter_mut_activities(&mut self) -> std::slice::IterMut<'_, YearlyActivities> {
		self.set_changes_activities(true);
		self.m_activities.iter_mut()
	}

	//pub fn get_activities(&self) -> &Vec<YearlyActivities> { &self.m_activities }
	pub fn get_activities_mut(&mut self) -> &mut Vec<YearlyActivities> {
		self.set_changes_activities(true);
		&mut self.m_activities
	}

	// Expenses

	/*
	pub fn iter_expense_concept(&self) -> std::slice::Iter<'_, String> { self.m_expense_types.iter_concepts() }
	pub fn iter_mut_expense_concepts(&mut self) -> std::slice::IterMut<'_, String> {
		self.m_expense_types.set_changes(true);
		self.m_expense_types.iter_mut_types()
	}
	*/
	pub fn iter_expense_subconcepts(&self) -> types::Iter { self.m_expense_types.iter_subconcepts() }
	/*
	pub fn iter_mut_expense_concept_subtypes(&mut self) -> concept_types::IterMut {
		self.m_expense_types.set_changes(true);
		self.m_expense_types.iter_mut_subtypes()
	}
	*/

	pub fn get_expense_concepts(&self) -> &ConceptTypes { &self.m_expense_types }
	pub fn get_expense_concepts_mut(&mut self) -> &mut ConceptTypes {
		self.m_expense_types.set_changes(true);
		&mut self.m_expense_types
	}

	// Incomes

	/*
	pub fn iter_income_concepts(&self) -> std::slice::Iter<'_, String> { self.m_income_types.iter_concepts() }
	pub fn iter_mut_income_concepts(&mut self) -> std::slice::IterMut<'_, String> {
		self.m_income_types.set_changes(true);
		self.m_income_types.iter_mut_types()
	}
	*/

	pub fn get_income_concepts(&self) -> &ConceptTypes { &self.m_income_types }
	pub fn get_income_concepts_mut(&mut self) -> &mut ConceptTypes {
		self.m_income_types.set_changes(true);
		&mut self.m_income_types
	}

	pub fn iter_income_subconcepts(&self) -> types::Iter { self.m_income_types.iter_subconcepts() }
	/*
	pub fn iter_mut_income_subconcepts(&mut self) -> concept_types::IterMut {
		self.m_income_types.set_changes(true);
		self.m_income_types.iter_mut_subtypes()
	}
	*/

	// -----

	#[duplicate::duplicate_item(
		method          convert   reference(type);
		[get_year]      [as_ref]  [& type]       ;
		[get_year_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), y: &u32) -> Option<reference([YearlyActivities])> {
		if !(self.m_min_year <= *y && *y <= self.m_max_year) {
			return None;
		}
		
		let res = self.m_activities.binary_search_by(|e| e.get_year().cmp(&y));
		if let Ok(idx) = res {
			Some( self.m_activities[idx].convert() )
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
		self.m_activities.binary_search_by(|e| e.get_year().cmp(&y)).is_ok()
	}

	pub fn add_year(&mut self, y: u32) -> &mut YearlyActivities {
		let res = self.m_activities.binary_search_by(|e| e.get_year().cmp(&y));
		match res {
			Ok(pos) => {
				// year already exists
				&mut self.m_activities[pos]
			},
			Err(pos) => {
				// year does not exist
				self.m_activities.insert(pos,
					YearlyActivities::new_year(y)
				);
				if self.m_min_year > y {
					self.m_min_year = y;
				}
				if self.m_max_year < y {
					self.m_max_year = y;
				}
				&mut self.m_activities[pos]
			}
		}
	}

	pub fn push_year(&mut self, y: YearlyActivities) {
		let res = self.m_activities.binary_search_by(|e| e.get_year().cmp(&y.get_year()));
		match res {
			Ok(_) => { },
			Err(pos) => {
				if self.m_min_year > *y.get_year() {
					self.m_min_year = *y.get_year();
				}
				if self.m_max_year < *y.get_year() {
					self.m_max_year = *y.get_year();
				}
				self.m_activities.insert(pos, y);
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

	pub fn set_changes_activities(&mut self, c: bool) {
		for ye in self.m_activities.iter_mut() {
			ye.set_changes(c);
		}
	}

	pub fn set_changes(&mut self, c: bool) {
		self.m_expense_types.set_changes(c);
		self.m_income_types.set_changes(c);
		self.set_changes_activities(c);
	}

}
