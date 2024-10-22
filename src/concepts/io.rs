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

use std::mem;
use std::fmt;

use std::io::{BufRead, Write, Result};

use crate::economy::all_activities::AllActivities;

use crate::concepts::types::ConceptTypes;

use crate::concepts::tree::Tree;
use crate::concepts::tree::KeyTree;

fn read_types(data_dir: &String, filename: String, concept_types: &mut ConceptTypes) {
	let path = data_dir.to_owned() + &filename;
	let file = std::fs::File::open( path.clone() ).expect("Failed to open file");

	let reader = std::io::BufReader::new(file);
	let mut entire_file_str = String::new();
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}

	let mut tree = build_tree(entire_file_str);
	tree.normalize_tree();
	concept_types.set_tree(tree);
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

fn write_concept_types(data_dir: &String, filename: String, concept_types: &ConceptTypes)
-> Result<()>
{
	let filename = data_dir.to_owned() + &filename;
	println!("Writing into '{filename}'...");
	
	let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
	write!(file, "{}", concept_types.get_tree())
}

pub fn write_all_data(data_dir: &String, all_data: &AllActivities) -> Result<()> {
	
	if all_data.get_expense_concepts().has_changes() {
		let filename = "expense_types.txt".to_string();
		write_concept_types(data_dir, filename, all_data.get_expense_concepts())?;
	}
	if all_data.get_income_concepts().has_changes() {
		let filename = "income_types.txt".to_string();
		write_concept_types(data_dir, filename, all_data.get_income_concepts())?;
	}

	Ok(())
}

/* ------------------------------------------------------------------------- */

fn format_tree_rec(f: &mut fmt::Formatter, t: &Tree, tab: String) -> fmt::Result {
	for KeyTree {key, tree} in t.iter() {
		write!(f, "{tab}{key} (")?;
		if let Some(st) = tree {
			writeln!(f, "")?;
			format_tree_rec(f, st, tab.clone() + "\t")?;
			writeln!(f, "{tab})")?;
		}
		else {
			writeln!(f, ")")?;
		}
	}
	Ok(())
}

impl fmt::Display for Tree {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for KeyTree {key, tree} in self.iter() {
			write!(f, "{key} (")?;
			if let Some(st) = tree {
				writeln!(f, "")?;
				format_tree_rec(f, st, "\t".to_string())?;
				writeln!(f, ")")?;
			}
			else {
				writeln!(f, ")")?;
			}
		}
		Ok(())
	}
}

enum CharacterType {
	Content,
	Separator { open: bool },
	None
}

fn classify_char(c: char) -> CharacterType {
	match c {
		'(' => CharacterType::Separator { open: true },
		')' => CharacterType::Separator { open: false },
		'\t' => CharacterType::None,
		_ => CharacterType::Content
	}
}

#[derive(PartialEq,Debug)]
enum TokenType {
	Content,
	Separator { open: bool },
}

fn tokenize(s: String) -> (Vec<String>, Vec<TokenType>) {
	let mut tokens: Vec<String> = Vec::new();
	let mut token_types: Vec<TokenType> = Vec::new();

	let mut next_string: String = String::new();

	for c in s.chars() {
		
		match classify_char(c) {
			CharacterType::Content => {
				next_string.push(c)
			},
			CharacterType::Separator { open } => {
				if next_string.len() > 0 {
					tokens.push(next_string.trim().to_string());
					token_types.push(TokenType::Content);
				}

				tokens.push("".to_string());
				token_types.push(TokenType::Separator { open });

				next_string = String::new();
			},
			CharacterType::None => { }
		}
	}

	assert_eq!(tokens.len(), token_types.len());
	(tokens, token_types)
}

fn build_tree_rec(
	tokens: &mut Vec<String>,
	token_types: &Vec<TokenType>,
	mut idx: usize
)
-> (Tree, usize)
{
	let mut t = Tree::new();
	
	let mut num_open_paren = 0;
	let mut previous_key: String = String::new();

	while idx < tokens.len() && num_open_paren >= 0 {

		match &token_types[idx] {
			TokenType::Content => {
				previous_key = String::new();
				mem::swap(&mut tokens[idx], &mut previous_key);
				idx += 1;
			},
			TokenType::Separator { open: true } => {
				num_open_paren += 1;

				let (st, next) =
					build_tree_rec(
						tokens,
						token_types,
						idx + 1
					);
				
				if st.num_keys() > 0 {
					t.insert_key(previous_key, Some(st));
				}
				else {
					t.insert_key(previous_key, None);
				}
				previous_key = String::new();

				idx = next - 1;
			},
			TokenType::Separator { open: false } => {
				num_open_paren -= 1;
				idx += 1;
			}
		}
	}
	
	(t, idx)
}

pub fn build_tree(s: String) -> Tree {
	let (mut tokens, token_types) = tokenize(s);

	let (tree, idx) =
		build_tree_rec(
			&mut tokens,
			&token_types,
			0
		);
	
	assert_eq!(idx, tokens.len());

	tree
}