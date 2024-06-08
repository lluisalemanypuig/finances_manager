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

use crate::expense::Expense;
use crate::date::Month;

#[derive(Debug)]
pub struct MonthlyExpenses {
	pub month: Month,
	pub expenses: Vec<Expense>
}

impl MonthlyExpenses {
	pub fn as_ref(&self) -> &MonthlyExpenses { self }
	pub fn as_mut(&mut self) -> &mut MonthlyExpenses { self }

	pub fn add_expense(&mut self, exp: Expense) {
		let pos = self.expenses.binary_search(&exp);
		match pos {
			Ok(idx) => {
				self.expenses.insert(idx, exp);
			},
			Err(idx) => {
				self.expenses.insert(idx, exp);
			}
		}
	}

	#[duplicate::duplicate_item(
		method             convert   reference(type);
		[get_expense]      [as_ref]  [& type]       ;
		[get_expense_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), i: usize) -> reference([Expense]) {
		self.expenses[i].convert()
	}

	pub fn remove_expense(&mut self, i: usize) {
		self.expenses.remove(i);
	}

	pub fn num_expenses(&self) -> usize {
		self.expenses.len()
	}
}
