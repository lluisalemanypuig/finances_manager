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

use crate::economy::traits::AsReferences;
use crate::economy::traits::HasConcepts;

use crate::time::date;

#[derive(Debug,PartialEq)]
pub struct Income {
	pub day_of_year: date::Date,
	pub price: f32,
	pub concepts: Vec<String>,
	pub from: String,
	pub place: String,
	pub description: String
}

impl Eq for Income { }
 
impl Ord for Income {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.day_of_year.cmp(&other.day_of_year)
	}
}
impl PartialOrd for Income {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
 
#[derive(Debug,PartialEq,Eq)]
pub struct ParseIncomeError;
 
fn split_string_data(data: &str) -> Vec<&str> {
	let all: Vec<&str> =
		data
		.split_terminator('\t')
		.map(str::trim)
		.filter(|&s| s != "")
		.collect();

	let parts: Vec<&str> = vec![
		&all[0][1..all[0].len() - 1],
		&all[1][1..all[1].len() - 1],
		&all[2][1..all[2].len() - 1],
		&all[3][1..all[3].len() - 1],
		&all[4][1..all[4].len() - 1],
		&all[5][1..all[5].len() - 1],
	];
	
	parts
}
 
impl std::str::FromStr for Income {
	type Err = ParseIncomeError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = split_string_data(s);
		let [d, pr, concept_list, fr, pl, descr] = parts.as_slice() else {
			panic!("Can't segment string '{s}' into 7 parts")
		};
		
		let concepts: Vec<String> =
			concept_list
			.split_terminator(';')
			.map(str::trim)
			.filter(|&s| s != "")
			.map(|s| s.to_string())
			.collect();

		let date_fromstr = d.parse::<date::Date>().map_err(|_| ParseIncomeError)?;
		let price_fromstr = pr.parse::<f32>().map_err(|_| ParseIncomeError)?;
		
		Ok(Income {
			day_of_year: date_fromstr,
			price: price_fromstr,
			concepts: concepts,
			from: fr.to_string(),
			place: pl.to_string(),
			description: descr.to_string()
		})
	}
}

impl AsReferences<Income> for Income {
	fn as_ref(&self) -> &Income { self }
	fn as_mut(&mut self) -> &mut Income { self }
}

impl HasConcepts for Income {
	fn get_concepts(&self) -> &Vec<String> { &self.concepts }
}
