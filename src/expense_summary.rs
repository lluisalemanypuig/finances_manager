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

pub struct ExpenseSummary {
	pub expense_to_price: std::collections::BTreeMap<String, f32>,
	pub total_spent: f32,
	pub total_income: f32
}

impl ExpenseSummary {
	pub fn new() -> ExpenseSummary {
		ExpenseSummary {
			expense_to_price: std::collections::BTreeMap::new(),
			total_spent: 0.0,
			total_income: 0.0
		}
	}

	pub fn merge(&mut self, other: ExpenseSummary) {
		for (exp, val) in other.expense_to_price.iter() {
			match self.expense_to_price.get_mut(exp) {
				Some(value) => {
					*value += *val;
				},
				None => {
					self.expense_to_price.insert(exp.clone(), *val);
				}
			}
		}

		self.total_spent += other.total_spent;
		self.total_income += other.total_income;
	}

	pub fn has_data(&self) -> bool {
		self.expense_to_price.len() > 0 || self.total_income != 0.0
	}
}
