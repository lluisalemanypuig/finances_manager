use crate::date;

#[derive(Debug,PartialEq)]
pub struct Expense {
	pub day_of_year: date::Date,
	pub price: f32,
	pub expense_type: String,
	pub place: String,
	pub description: String
}

impl Eq for Expense { }

impl Ord for Expense {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.day_of_year.cmp(&other.day_of_year)
	}
}

impl PartialOrd for Expense {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug,PartialEq,Eq)]
pub struct ParseExpenseError;

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

impl std::str::FromStr for Expense {
	type Err = ParseExpenseError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<String> = split_string_data(s);
		let [d, pr, et, pl, descr] = parts.as_slice() else {
			panic!("Can't segment string '{s}' into five parts")
		};
		
		let date_fromstr = d.parse::<date::Date>().map_err(|_| ParseExpenseError)?;
		let price_fromstr = pr.parse::<f32>().map_err(|_| ParseExpenseError)?;
		
		Ok(Expense {
			day_of_year: date_fromstr,
			price: price_fromstr,
			expense_type: et.to_string(),
			place: pl.to_string(),
			description: descr.to_string()
		})
	}
}

impl Expense {
	pub fn as_ref(&self) -> &Expense { self }
	pub fn as_mut(&mut self) -> &mut Expense { self }
}