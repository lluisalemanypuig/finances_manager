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

 pub type Key = String;
 pub type Value = Vec<String>;
 pub type Container = std::collections::BTreeMap<String, Vec<String>>;
 pub type Iter<'a> = std::collections::btree_map::Iter<'a,Key,Value>;
 //pub type IterMut<'a> = std::collections::btree_map::IterMut<'a,Key,Value>;

#[derive(Debug)]
pub struct ConceptTypes {
	m_changes: bool,

	m_types: Vec<String>,

	m_sub_types: Container
}

impl ConceptTypes {
	pub fn new() -> ConceptTypes {
		ConceptTypes {
			m_changes: false,
			m_types: Vec::new(),
			m_sub_types: Container::new()
		}
	}

	pub fn iter_types(&self) -> std::slice::Iter<'_, String> { self.m_types.iter() }
	//pub fn iter_mut_types(&mut self) -> std::slice::IterMut<'_, String> { self.m_types.iter_mut() }
	
	pub fn iter_subtypes(&self) -> Iter { self.m_sub_types.iter() }
	//pub fn iter_mut_subtypes(&mut self) -> IterMut { self.m_sub_types.iter_mut() }

	pub fn add_type(&mut self, concept: String) {
		self.m_types.push(concept);
		self.m_changes = true;
	}
	pub fn set_subtypes(&mut self, concept: String, subtypes: Vec<String>) {
		self.m_sub_types.insert(concept, subtypes);
		self.m_changes = true;
	}

	pub fn has_changes(&self) -> bool { self.m_changes }
	pub fn set_changes(&mut self, c: bool) {
		self.m_changes = c;
	}

	pub fn has_type(&self, concept_type: &String) -> bool {
		self.m_types.contains(concept_type)
	}
	/*
	pub fn has_subtype(&self, concept_type: &String, concept_subtype: &String) -> bool {
		if !self.has_type(concept_type) { return false; }
		self.m_sub_types.get(concept_type).unwrap().contains(concept_subtype)
	}
	*/

	pub fn get_types(&self) -> &Vec<String> {
		&self.m_types
	}
	pub fn get_subtypes(&self, concept_type: &String) -> &Vec<String> {
		&self.m_sub_types.get(concept_type).unwrap()
	}

	pub fn position_type(&self, expense_type: &String) -> Option<usize> {
		self.m_types.iter().position(|e| e == expense_type)
	}

	pub fn remove(&mut self, idx: usize) {
		self.m_types.remove(idx);
		self.m_changes = true;
	}

	pub fn replace(&mut self, idx: usize, new_elem: String) {
		self.m_types[idx] = new_elem;
		self.m_changes = true;
	}

	pub fn add(&mut self, new_elem: String) {
		self.m_types.push(new_elem);
		self.m_changes = true;
	}
	
}
