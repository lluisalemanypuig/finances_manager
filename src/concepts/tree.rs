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
pub struct KeyTree {
	pub key: String,
	pub tree: Option<Tree>
}

#[derive(Debug)]
pub struct Tree {
	m_children: Vec<KeyTree>
}

impl Tree {
	pub fn new() -> Self {
		Tree {
			m_children: Vec::new()
		}
	}

	/*
	pub fn has_branch(&self, branch: &[String]) -> bool {
		if branch.len() == 0 {
			return false;
		}

		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(&branch[0]) }
		);
		match pos {
			Ok(idx) => {
				match &self.m_children[idx].tree {
					Some(t) => t.has_branch(&branch[1..]),
					None => branch.len() > 1
				}
			},
			Err(_) => false
		}
	}
	*/

	/*
	// Pre: all keys in @e branch exist.
	pub fn get_subtree(&self, branch: &[String]) -> &Self {
		if branch.len() == 0 { return self; }
		self.get_child(&branch[0]).unwrap().get_subtree(&branch[1..])
	}
	*/
	pub fn get_subtree_mut(&mut self, branch: &[String]) -> &mut Self {
		if branch.len() == 0 { return self; }
		self.get_child_mut(&branch[0]).unwrap().get_subtree_mut(&branch[1..])
	}

	pub fn get_child(&self, s: &String) -> Option<&Tree> {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(s) }
		);
		match pos {
			Ok(idx) => {
				match &self.m_children[idx].tree {
					Some(t) => Some(t),
					None => None
				}
			},
			Err(_) => None
		}
	}
	pub fn get_child_mut(&mut self, s: &String) -> Option<&mut Tree> {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(s) }
		);
		match pos {
			Ok(idx) => {
				match &mut self.m_children[idx].tree {
					Some(t) => Some(t),
					None => None
				}
			},
			Err(_) => None
		}
	}
	
	pub fn make_child(&mut self, s: &String) -> &mut Tree {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(s) }
		);
		match pos {
			Ok(idx) => {
				if self.m_children[idx].tree.is_none() {
					self.m_children[idx].tree = Some(Tree::new());
				}
				self.m_children[idx].tree.as_mut().unwrap()
			}
			Err(idx) => {
				self.m_children.insert(
					idx,
					KeyTree {
						key: s.clone(),
						tree: Some(Tree::new())
					}
				);
				self.m_children[idx].tree.as_mut().unwrap()
			}
		}
	}

	pub fn make_subtree(&mut self, branch: &[String]) -> &mut Self {
		if branch.len() == 0 { return self; }
		self.make_child(&branch[0]).make_subtree(&branch[1..])
	}

	pub fn remove_child(&mut self, key: &String) {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(key) }
		);
		match pos {
			Ok(idx) => {
				self.m_children.remove(idx);
			},
			Err(_) => {}
		}
	}

	pub fn rename_key(&mut self, old_key: &String, new_key: String) -> &mut Self {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(old_key) }
		);

		match pos {
			Ok(idx) => {
				self.m_children[idx].key = new_key;
			},
			Err(_) => {
				// 'old_key' does not exist
			}
		}
		self
	}

	/**
	 * @brief Inserts a pair of (key,tree) into this tree.
	 *
	 * If the key already exists, the tree passed in the pair is merged with
	 * the existing tree. If the key does not exist, the tree passed is moved
	 * as a child of the key.
	 */
	pub fn insert_key(&mut self, key: String, tree: Option<Tree>) -> &mut Self {
		let pos = self.m_children.binary_search_by(
			|e| { e.key.cmp(&key) }
		);

		match pos {
			Ok(idx) => {
				if let Some(t2) = tree {
					if let Some(t1) = &mut self.m_children[idx].tree {
						t1.merge(t2);
					}
					else {
						self.m_children[idx].tree = Some(t2);
					}
				}
			},
			Err(idx) => {
				self.m_children.insert(
					idx,
					KeyTree { key, tree }
				);
			}
		}
		self
	}

	pub fn merge(&mut self, t: Tree) -> &mut Self {
		for KeyTree { key, tree } in t.m_children {
			self.insert_key(key, tree);
		}
		self
	}

	pub fn iter(&self) -> std::slice::Iter<'_, KeyTree> {
		self.m_children.iter()
	}
	/*
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, KeyTree> {
		self.m_children.iter_mut()
	}
	*/

	pub fn num_keys(&self) -> usize {
		self.m_children.len()
	}
	pub fn get_keys(&self) -> Vec<&String> {
		let mut res = Vec::new();
		for KeyTree {key: s, tree: _} in self.m_children.iter() {
			res.push(s);
		}
		res
	}
}
