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

use std::io::{BufRead, Write, Result};

use crate::economy::all_activities::AllActivities;

use crate::concepts::types;
use crate::concepts::types::ConceptTypes;

fn read_types(data_dir: &String, filename: String, concept_types: &mut ConceptTypes) {
	let path = data_dir.to_owned() + &filename;
	let file = std::fs::File::open( path.clone() ).expect("Failed to open file");

	println!("        Reading '{path}'...");

	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let values: Vec<&str> = l
			.split_terminator(":")
			.map(|s| s.trim())
			.collect();

		let concept_type = values[0].to_string();
		concept_types.add_concept(concept_type.clone());
		let concept_subtypes: Vec<String> = values[1..].iter().map(|s| s.to_string()).collect();
		concept_types.set_subconcept(concept_type, concept_subtypes);
	}
}

pub fn read_expense_types(data_dir: &String, all_data: &mut AllActivities) {
	read_types(
		data_dir, 
		"expense_types.txt".to_string(),
		all_data.get_expense_concepts_mut()
	);
}

pub fn read_income_types(data_dir: &String, all_data: &mut AllActivities) {
	read_types(
		data_dir, 
		"income_types.txt".to_string(),
		all_data.get_income_concepts_mut()
	);
}

/* ------------------------------------------------------------------------- */

fn write_concept_types(data_dir: &String, filename: String, iter: types::Iter)
-> Result<()>
{
	let filename = data_dir.to_owned() + &filename;
	println!("Writing into '{filename}'...");
	let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
	for (et, subtypes) in iter {
		write!(file, "{et}")?;
		for s in subtypes.iter() {
			write!(file, " : {s}")?;
		}
		writeln!(file, "")?;
	}
	Ok(())
}

pub fn write_all_data(data_dir: &String, all_data: &AllActivities) -> Result<()> {
	
	if all_data.get_expense_concepts().has_changes() {
		let filename = "expense_types.txt".to_string();
		write_concept_types(data_dir, filename, all_data.iter_expense_subconcepts())?;
	}
	if all_data.get_income_concepts().has_changes() {
		let filename = "income_types.txt".to_string();
		write_concept_types(data_dir, filename, all_data.iter_income_subconcepts())?;
	}

	Ok(())
}
