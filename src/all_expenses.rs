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

extern crate duplicate;

use crate::date::Month;
use crate::expense_types::ExpenseTypes;
use crate::monthly_expenses::MonthlyExpenses;
use crate::yearly_expenses::YearlyExpenses;

#[derive(Debug)]
pub struct AllExpenses {
	pub min_year: u32,
	pub max_year: u32,
	pub expense_types: ExpenseTypes,
	pub expenses: Vec<YearlyExpenses>
}

impl AllExpenses {

	#[duplicate::duplicate_item(
		method          convert   reference(type);
		[get_year]      [as_ref]  [& type]       ;
		[get_year_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), y: &u32) -> Option<reference([YearlyExpenses])> {
		if !(self.min_year <= *y && *y <= self.max_year) {
			return None;
		}
		
		let res = self.expenses.binary_search_by(|e| e.year.cmp(&y));
		if let Ok(idx) = res {
			Some( self.expenses[idx].convert() )
		}
		else {
			None
		}
	}

	pub fn get_month(&self, y: &u32, m: &Month) -> Option<&MonthlyExpenses> {
		if let Some(year) = self.get_year(y) {
			year.get_month(m)
		}
		else {
			None
		}
	}

	pub fn has_year(&self, y: &u32) -> bool {
		let res = self.expenses.binary_search_by(|e| e.year.cmp(&y));
		match res {
			Ok(_) => true,
			Err(_) => false,
		}
	}

	pub fn add_year(&mut self, y: &u32) -> &mut YearlyExpenses {
		let res = self.expenses.binary_search_by(|e| e.year.cmp(&y));
		match res {
			Ok(pos) => {
				// year already exists
				&mut self.expenses[pos]
			},
			Err(pos) => {
				// year does not exist
				self.expenses.insert(pos,
					YearlyExpenses::new_year(y, true)
				);
				if self.min_year > *y {
					self.min_year = *y;
				}
				if self.max_year < *y {
					self.max_year = *y;
				}
				&mut self.expenses[pos]
			}
		}
	}

}
