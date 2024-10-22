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

use crate::concepts::tree::Tree;


#[derive(Debug)]
pub struct ConceptTypes {
	m_changes: bool,
	m_concepts: Tree
}

impl ConceptTypes {
	pub fn new() -> ConceptTypes {
		ConceptTypes {
			m_changes: false,
			m_concepts: Tree::new()
		}
	}

	pub fn set_tree(&mut self, t: Tree) {
		self.m_concepts = t;
	}

	pub fn get_tree(&self) -> &Tree { &self.m_concepts }
	pub fn get_tree_mut(&mut self) -> &mut Tree { &mut self.m_concepts }

	pub fn has_changes(&self) -> bool { self.m_changes }
	pub fn set_changes(&mut self, c: bool) {
		self.m_changes = c;
	}

}
