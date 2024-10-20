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

use crate::economy::monthly_activities::MonthlyActivities;
use crate::economy::monthly_activities::MonthlyActivitiesCollection;

use crate::economy::expense::Expense;
use crate::economy::income::Income;

#[derive(Debug)]
pub struct YearlyActivities {
	m_year: u32,

	m_expenses: MonthlyActivitiesCollection<Expense>,
	m_incomes: MonthlyActivitiesCollection<Income>,
}

impl YearlyActivities {
	pub fn new() -> YearlyActivities {
		YearlyActivities {
			m_year: 0,
			m_expenses: MonthlyActivitiesCollection::new(),
			m_incomes: MonthlyActivitiesCollection::new(),
		}
	}
	pub fn new_year(y: u32) -> YearlyActivities {
		YearlyActivities {
			m_year: y,
			m_expenses: MonthlyActivitiesCollection::new_changes(true),
			m_incomes: MonthlyActivitiesCollection::new_changes(true),
		}
	}
	pub fn new_changes(c: bool) -> YearlyActivities {
		YearlyActivities {
			m_year: 0,
			m_expenses: MonthlyActivitiesCollection::new_changes(c),
			m_incomes: MonthlyActivitiesCollection::new_changes(c),
		}
	}
	pub fn new_year_changes(y: u32, c: bool) -> YearlyActivities {
		YearlyActivities {
			m_year: y,
			m_expenses: MonthlyActivitiesCollection::new_changes(c),
			m_incomes: MonthlyActivitiesCollection::new_changes(c),
		}
	}

	pub fn iter_expenses(&self) -> std::slice::Iter<'_, MonthlyActivities<Expense>> { self.m_expenses.iter() }
	pub fn iter_mut_expenses(&mut self) -> std::slice::IterMut<'_, MonthlyActivities<Expense>> {
		self.m_expenses.set_changes(true);
		self.m_expenses.iter_mut()
	}

	pub fn get_expenses(&self) -> &MonthlyActivitiesCollection<Expense> { &self.m_expenses }
	pub fn get_expenses_mut(&mut self) -> &mut MonthlyActivitiesCollection<Expense> {
		self.m_expenses.set_changes(true);
		&mut self.m_expenses
	}

	pub fn iter_incomes(&self) -> std::slice::Iter<'_, MonthlyActivities<Income>> { self.m_incomes.iter() }
	pub fn iter_mut_incomes(&mut self) -> std::slice::IterMut<'_, MonthlyActivities<Income>> {
		self.m_incomes.set_changes(true);
		self.m_incomes.iter_mut()
	}

	pub fn get_incomes(&self) -> &MonthlyActivitiesCollection<Income> { &self.m_incomes }
	pub fn get_incomes_mut(&mut self) -> &mut MonthlyActivitiesCollection<Income> {
		self.m_incomes.set_changes(true);
		&mut self.m_incomes
	}

	pub fn get_year(&self) -> &u32 { &self.m_year }
	pub fn set_year(&mut self, y: u32) {
		self.m_year = y;
	}

	pub fn as_ref(&self) -> &YearlyActivities { self }
	pub fn as_mut(&mut self) -> &mut YearlyActivities {
		self.set_changes(true);
		self
	}

	pub fn merge(&mut self, year_acts: YearlyActivities) {
		self.m_expenses.merge(year_acts.m_expenses);
		self.m_incomes.merge(year_acts.m_incomes);
	}

	pub fn set_changes(&mut self, c: bool) {
		self.m_expenses.set_changes(c);
		self.m_incomes.set_changes(c);
	}
}

impl Eq for YearlyActivities {}

impl PartialEq for YearlyActivities {
	fn eq(&self, other: &Self) -> bool {
		self.m_year == other.m_year
	}
}
impl Ord for YearlyActivities {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.m_year.cmp(&other.m_year)
	}
}
impl PartialOrd for YearlyActivities {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq<u32> for YearlyActivities {
	fn eq(&self, y: &u32) -> bool {
		self.m_year == *y
	}
}
impl PartialOrd<u32> for YearlyActivities {
	fn partial_cmp(&self, y: &u32) -> Option<std::cmp::Ordering> {
		Some(self.m_year.cmp(y))
	}
}
impl PartialEq<YearlyActivities> for u32 {
	fn eq(&self, other: &YearlyActivities) -> bool {
		*self == other.m_year
	}
}
impl PartialOrd<YearlyActivities> for u32 {
	fn partial_cmp(&self, other: &YearlyActivities) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other.m_year))
	}
}
