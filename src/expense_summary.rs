

pub struct ExpenseSummary {
    pub expense_to_price: std::collections::BTreeMap<String, f32>,
    pub total_spent: f32,
    pub total_income: f32
}

impl ExpenseSummary {
    pub fn new() -> ExpenseSummary {
        ExpenseSummary {
            expense_to_price: std::collections::BTreeMap::new(),
            total_spent: 0.0,
            total_income: 0.0
        }
    }

    pub fn merge(&mut self, other: ExpenseSummary) {
        for (exp, val) in other.expense_to_price.iter() {
            match self.expense_to_price.get_mut(exp) {
                Some(value) => {
                    *value += *val;
                },
                None => {
                    self.expense_to_price.insert(exp.clone(), *val);
                }
            }
        }

        self.total_spent += other.total_spent;
        self.total_income += other.total_income;
    }

    pub fn has_data(&self) -> bool { self.expense_to_price.len() > 0 }
}