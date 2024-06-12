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

#[derive(Debug)]
pub struct ConceptTypes {
	changes: bool,

	pub types: Vec<String>
}

impl ConceptTypes {
	pub fn new() -> ConceptTypes {
		ConceptTypes {
			changes: false,
			types: Vec::new()
		}
	}
	pub fn new_vec(ts: Vec<String>) -> ConceptTypes {
		ConceptTypes {
			changes: false,
			types: ts
		}
	}

	pub fn has_changes(&self) -> bool { self.changes }
	pub fn set_changes(&mut self, c: bool) {
		self.changes = c;
	}

	pub fn has_type(&self, expense_type: &String) -> bool {
		self.types.iter().position(|e| e == expense_type).is_some()
	}

	pub fn position_type(&self, expense_type: &String) -> Option<usize> {
		self.types.iter().position(|e| e == expense_type)
	}

	pub fn remove(&mut self, idx: usize) {
		self.types.remove(idx);
		self.changes = true;
	}

	pub fn replace(&mut self, idx: usize, new_elem: String) {
		self.types[idx] = new_elem;
		self.changes = true;
	}

	pub fn add(&mut self, new_elem: String) {
		self.types.push(new_elem);
		self.changes = true;
	}

	pub fn is_type_ok(&self, expense_type: &String) -> bool {
		self.has_type(expense_type)
	}

}
