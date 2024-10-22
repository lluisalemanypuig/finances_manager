use std::mem;
use std::fmt;

use crate::tree::Tree;
use crate::tree::KeyTree;

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