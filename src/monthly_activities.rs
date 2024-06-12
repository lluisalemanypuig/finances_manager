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

#[derive(Debug)]
pub struct MonthlyActivities {
	pub month: Month,
	pub expenses: Vec<Expense>,
	pub incomes: Vec<Income>,
}

impl MonthlyActivities {
	/* PUBLIC */

	pub fn new() -> MonthlyActivities {
		MonthlyActivities {
			month: Month::January,
			expenses : Vec::new(),
			incomes: Vec::new()
		}
	}

	pub fn as_ref(&self) -> &MonthlyActivities { self }
	pub fn as_mut(&mut self) -> &mut MonthlyActivities { self }

	pub fn add_expense(&mut self, exp: Expense) {
		Self::add_to_vector(&mut self.expenses, exp);
	}
	pub fn add_income(&mut self, inc: Income) {
		Self::add_to_vector(&mut self.incomes, inc);
	}

	#[duplicate::duplicate_item(
		method             convert   reference(type);
		[get_expense]      [as_ref]  [& type]       ;
		[get_expense_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), i: usize) -> reference([Expense]) {
		self.expenses[i].convert()
	}
	#[duplicate::duplicate_item(
		method             convert   reference(type);
		[get_income]      [as_ref]  [& type]       ;
		[get_income_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), i: usize) -> reference([Income]) {
		self.incomes[i].convert()
	}

	pub fn remove_expense(&mut self, i: usize) {
		self.expenses.remove(i);
	}
	pub fn remove_income(&mut self, i: usize) {
		self.incomes.remove(i);
	}

	pub fn num_expenses(&self) -> usize {
		self.expenses.len()
	}
	pub fn num_incomes(&self) -> usize {
		self.incomes.len()
	}

	fn merge_expenses(&mut self, v: Vec<Expense>) {
		for e in v.into_iter() {
			Self::add_to_vector(&mut self.expenses, e);
		}
	}
	fn merge_incomes(&mut self, v: Vec<Income>) {
		for i in v.into_iter() {
			Self::add_to_vector(&mut self.incomes, i);
		}
	}

	pub fn merge(&mut self, month: MonthlyActivities) {
		self.merge_expenses(month.expenses);
		self.merge_incomes(month.incomes);
	}

	/* PRIVATE */

	fn add_to_vector<T: Ord>(v: &mut Vec<T>, d: T) {
		let pos = v.binary_search(&d);
		match pos {
			Ok(idx) => {
				v.insert(idx, d);
			},
			Err(idx) => {
				v.insert(idx, d);
			}
		}
	}

}
