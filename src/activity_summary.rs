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

pub struct ActivitySummary {
	m_activity_to_money: std::collections::BTreeMap<(String,String), f32>,
	m_total_money: f32
}

impl ActivitySummary {
	pub fn new() -> ActivitySummary {
		ActivitySummary {
			m_activity_to_money: std::collections::BTreeMap::new(),
			m_total_money: 0.0
		}
	}

	pub fn get_width_concept_type(&self) -> usize {
		self.m_activity_to_money
			.iter()
			.map(|(s, _)| -> usize { s.0.len() })
			.max()
			.unwrap_or(0)
	}
	pub fn get_width_concept_subtype(&self) -> usize {
		self.m_activity_to_money
			.iter()
			.map(|(s, _)| -> usize { s.1.len() })
			.max()
			.unwrap_or(0)
	}

	pub fn iter_summary(&self) -> std::collections::btree_map::Iter<'_, (String,String), f32>
	{ self.m_activity_to_money.iter() }

	/*
	pub fn iter_mut_summary(&mut self) -> std::collections::btree_map::IterMut<'_, (String,String), f32>
	{ self.m_activity_to_money.iter_mut() }
	*/

	pub fn merge(&mut self, other: ActivitySummary) {
		for (exp, val) in other.m_activity_to_money.iter() {
			match self.m_activity_to_money.get_mut(exp) {
				Some(value) => {
					*value += *val;
				},
				None => {
					self.m_activity_to_money.insert(exp.clone(), *val);
				}
			}
		}

		self.m_total_money += other.m_total_money;
	}

	pub fn add(&mut self, name: String, subname: String, price: f32) {
		self.m_total_money += price;
		let pair = (name, subname);

		match self.m_activity_to_money.get_mut(&pair) {
			Some(value) => {
				*value += price;
			},
			None => {
				self.m_activity_to_money
					.insert(pair, price);
			}
		}
	}

	pub fn get_total(&self) -> f32 { self.m_total_money }

	pub fn has_data(&self) -> bool {
		self.m_activity_to_money.len() > 0
	}
}
