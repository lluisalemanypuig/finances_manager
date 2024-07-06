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

use crate::traits::AsReferences;
use crate::traits::HasConceptType;
use crate::traits::HasSubConceptType;

use crate::date;

#[derive(Debug,PartialEq)]
pub struct Income {
	pub day_of_year: date::Date,
	pub price: f32,
	pub concept: String,
	pub sub_concept: String,
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
 
fn split_string_data(data: &str) -> Vec<String> {
	let all: Vec<_> =
		data
		.split_terminator(' ')
		.map(str::trim)
		.filter(|&s| s != "")
		.collect();
	
	let mut parts: Vec<String> = vec![
		all[0].to_string(),
		all[1].to_string()
	];
	
	let mut next_string = "".to_string();
	
	for s in all.iter().skip(2) {
		if s.starts_with('"') && s.ends_with('"') {
			let trimmed = &s[1..s.len() - 1];
			parts.push( trimmed.to_string() );
		}
		else {
			if s.starts_with('"') {
				next_string += &s[1..];
			}
			else if s.ends_with('"') {
				
				next_string += " ";
				next_string += s;
				let trimmed = &next_string[..next_string.len() - 1];
				
				parts.push( trimmed.to_string() );
				next_string = "".to_string();
			}
			else {
				next_string += " ";
				next_string += s;
			}
		}
	}
	
	parts
}
 
impl std::str::FromStr for Income {
	type Err = ParseIncomeError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<String> = split_string_data(s);
		let [d, pr, c, st, fr, pl, descr] = parts.as_slice() else {
			panic!("Can't segment string '{s}' into 7 parts")
		};
		
		let date_fromstr = d.parse::<date::Date>().map_err(|_| ParseIncomeError)?;
		let price_fromstr = pr.parse::<f32>().map_err(|_| ParseIncomeError)?;
		
		Ok(Income {
			day_of_year: date_fromstr,
			price: price_fromstr,
			concept: c.to_string(),
			sub_concept: st.to_string(),
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

impl HasConceptType for Income {
	fn concept(&self) -> &String { &self.concept }
}
impl HasSubConceptType for Income {
	fn sub_concept(&self) -> &String { &self.sub_concept }
}