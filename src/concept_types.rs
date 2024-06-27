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

	m_concepts: Vec<String>,

	m_subconcepts: Container
}

impl ConceptTypes {
	pub fn new() -> ConceptTypes {
		ConceptTypes {
			m_changes: false,
			m_concepts: Vec::new(),
			m_subconcepts: Container::new()
		}
	}

	pub fn iter_concepts(&self) -> std::slice::Iter<'_, String> { self.m_concepts.iter() }
	//pub fn iter_mut_types(&mut self) -> std::slice::IterMut<'_, String> { self.m_types.iter_mut() }
	
	pub fn iter_subconcepts(&self) -> Iter { self.m_subconcepts.iter() }
	//pub fn iter_mut_subtypes(&mut self) -> IterMut { self.m_sub_types.iter_mut() }

	pub fn add_concept(&mut self, concept: String) {
		self.m_concepts.push(concept.clone());

		self.m_subconcepts.insert(concept, Vec::new());

		self.set_changes(true);
	}
	pub fn set_subconcept(&mut self, concept: String, subconcept: Vec<String>) {
		self.m_subconcepts.insert(concept, subconcept);
		
		self.set_changes(true);
	}

	pub fn has_changes(&self) -> bool { self.m_changes }
	pub fn set_changes(&mut self, c: bool) {
		self.m_changes = c;
	}

	pub fn has_concept(&self, concept: &String) -> bool {
		self.m_concepts.contains(concept)
	}
	/*
	pub fn has_subtype(&self, concept_type: &String, concept_subtype: &String) -> bool {
		if !self.has_type(concept_type) { return false; }
		self.m_sub_types.get(concept_type).unwrap().contains(concept_subtype)
	}
	*/

	pub fn get_concepts(&self) -> &Vec<String> {
		&self.m_concepts
	}
	pub fn get_subconcept(&self, concept: &String) -> &Vec<String> {
		&self.m_subconcepts.get(concept).unwrap()
	}


	fn position_in_vector(v: &Vec<String>, elem: &String) -> Option<usize> {
		v.iter().position(|e| e == elem)
	}

	pub fn remove_concept(&mut self, concept: String) {
		if let Some(idx) = Self::position_in_vector(&self.m_concepts, &concept) {
			self.m_subconcepts.remove(&concept);
	
			self.m_concepts.remove(idx);
	
			self.set_changes(true);
		}
	}
	pub fn remove_subconcept(&mut self, concept: String, subconcept: String) {
		if self.has_concept(&concept) {

			let subconcepts = self.m_subconcepts.get_mut(&concept).unwrap();
			if let Some(idx) = Self::position_in_vector(&subconcepts, &subconcept) {
				subconcepts.remove(idx);

				self.set_changes(true);
			}
		}
	}

	pub fn rename_concept(&mut self, old_concept: &String, new_concept: String) {
		if let Some(idx) = Self::position_in_vector(&self.m_concepts, old_concept) {
			self.m_concepts[idx] = new_concept.clone();

			let res = self.m_subconcepts.remove(old_concept).unwrap();
			self.m_subconcepts.insert( new_concept, res );

			self.set_changes(true);
		}
	}
	
}
