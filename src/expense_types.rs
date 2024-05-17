
#[derive(Debug)]
pub struct ExpenseTypes {
	changes: bool,
	pub types: Vec<String>
}

impl ExpenseTypes {
	pub fn new() -> ExpenseTypes {
		ExpenseTypes {
			changes: false,
			types: Vec::new()
		}
	}
	pub fn new_vec(ts: Vec<String>) -> ExpenseTypes {
		ExpenseTypes {
			changes: false,
			types: ts
		}
	}

	pub fn has_changes(&self) -> bool { self.changes }
	pub fn set_changes(&mut self, c: bool) {
		self.changes = c;
	}

	pub fn exists_expense_type(&self, expense_type: &String) -> bool {
		self.types.iter().position(|e| e == expense_type).is_some()
	}

	pub fn position_expense_type(&self, expense_type: &String) -> Option<usize> {
		self.types.iter().position(|e| e == expense_type)
	}

	pub fn remove_element(&mut self, idx: usize) {
		self.types.remove(idx);
		self.changes = true;
	}

	pub fn replace_element(&mut self, idx: usize, new_elem: String) {
		self.types[idx] = new_elem;
		self.changes = true;
	}

	pub fn add_element(&mut self, new_elem: String) {
		self.types.push(new_elem);
		self.changes = true;
	}

}
