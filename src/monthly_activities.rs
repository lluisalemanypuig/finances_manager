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

use crate::traits::AsReferences;

use crate::date::Month;

#[derive(Debug)]
pub struct MonthlyActivities<T> {
	m_month: Month,
	m_activities: Vec<T>
}

impl<T: AsReferences<T> + Ord> MonthlyActivities<T> {
	/* PUBLIC */

	pub fn new() -> Self {
		MonthlyActivities {
			m_month: Month::January,
			m_activities : Vec::new()
		}
	}
	pub fn new_month(m: &Month) -> Self {
		MonthlyActivities {
			m_month: m.clone(),
			m_activities : Vec::new()
		}
	}

	pub fn iter(&self) -> std::slice::Iter<'_, T> { self.m_activities.iter() }
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.m_activities.iter_mut() }

	pub fn as_ref(&self) -> &Self { self }
	pub fn as_mut(&mut self) -> &mut Self { self }

	pub fn get_activities(&self) -> &Vec<T> { &self.m_activities }
	pub fn get_activities_mut(&mut self) -> &mut Vec<T> { &mut self.m_activities }

	pub fn get_month(&self) -> &Month { &self.m_month }

	#[duplicate::duplicate_item(
		method     convert   reference(type);
		[get]      [as_ref]  [& type]       ;
		[get_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), i: usize) -> reference([T]) {
		self.m_activities[i].convert()
	}

	pub fn push(&mut self, exp: T) {
		Self::add_to_vector(&mut self.m_activities, exp);
	}

	pub fn remove(&mut self, i: usize) {
		self.m_activities.remove(i);
	}

	pub fn size(&self) -> usize {
		self.m_activities.len()
	}

	pub fn merge(&mut self, month: MonthlyActivities<T>) {
		for e in month.m_activities.into_iter() {
			Self::add_to_vector(&mut self.m_activities, e);
		}
	}

	/* PRIVATE */

	fn add_to_vector(v: &mut Vec<T>, d: T) {
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

#[derive(Debug)]
pub struct MonthlyActivitiesCollection<T> {
	m_changes: bool,
	m_activities: Vec<MonthlyActivities<T>>,
}

impl<T> MonthlyActivitiesCollection<T>
where
	T: AsReferences<T> + Ord
{
	pub fn new() -> Self {
		MonthlyActivitiesCollection {
			m_changes: false,
			m_activities: Vec::new()
		}
	}
	pub fn new_changes(changes: bool) -> Self {
		MonthlyActivitiesCollection {
			m_changes: changes,
			m_activities: Vec::new()
		}
	}

	pub fn iter(&self) -> std::slice::Iter<'_, MonthlyActivities<T>> { self.m_activities.iter() }
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MonthlyActivities<T>> {
		self.m_changes = true;
		self.m_activities.iter_mut()
	}

	pub fn get_activities(&self) -> &Vec<MonthlyActivities<T>> { &self.m_activities }
	pub fn get_activities_mut(&mut self) -> &mut Vec<MonthlyActivities<T>> { &mut self.m_activities }

	pub fn has_changes(&self) -> bool { self.m_changes }
	pub fn set_changes(&mut self, c: bool) {
		self.m_changes = c;
	}
	
	pub fn has_month(&self, m: &Month) -> bool {
		let res = self.m_activities.binary_search_by(
			|e| e.get_month().cmp(&m)
		);
		match res {
			Ok(_) => true,
			Err(_) => false
		}
	}

	#[duplicate::duplicate_item(
		method           convert   reference(type);
		[get_month]      [as_ref]  [& type]       ;
		[get_month_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), m: &Month) -> Option<reference([MonthlyActivities<T>])> {
		let res = self.m_activities.binary_search_by(
			|e| e.get_month().cmp(&m)
		);
		match res {
			Ok(idx) => Some(self.m_activities[idx].convert()),
			Err(_) => None
		}
	}

	pub fn add(&mut self, m: &Month) -> &mut MonthlyActivities<T> {
		let res = self.m_activities.binary_search_by(
			|e| e.get_month().cmp(&m)
		);
		match res {
			Ok(pos) => {
				// month already exists
				&mut self.m_activities[pos]
			},
			Err(pos) => {
				// month does not exist
				self.m_activities.insert(pos, MonthlyActivities::new());
				&mut self.m_activities[pos]
			}
		}
	}

	pub fn push(&mut self, m: MonthlyActivities<T>) {
		let res = self.m_activities.binary_search_by(
				|e| e.get_month().cmp(&m.get_month())
		);

		match res {
			Ok(_) => { },
			Err(pos) => {
				self.m_activities.insert(pos, m);
			}
		}
	}

	pub fn merge(&mut self, acts: MonthlyActivitiesCollection<T>) {
		for month in acts.m_activities.into_iter() {
			if !self.has_month(month.get_month()) {
				self.push(month);
			}
			else {
				self.get_month_mut(month.get_month()).unwrap().merge(month);
			}
		}
	}

}
