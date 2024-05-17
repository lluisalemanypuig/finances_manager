use std::str::FromStr;
use std::fmt;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone)]
pub enum Month {
	January = 0,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December
}

impl fmt::Display for Month {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Month::January => write!(f, "January"),
			Month::February => write!(f, "February"),
			Month::March => write!(f, "March"),
			Month::April => write!(f, "April"),
			Month::May => write!(f, "May"),
			Month::June => write!(f, "June"),
			Month::July => write!(f, "July"),
			Month::August => write!(f, "August"),
			Month::September => write!(f, "September"),
			Month::October => write!(f, "October"),
			Month::November => write!(f, "November"),
			Month::December => write!(f, "December"),
		}
	}
}

#[derive(Debug,PartialEq,Eq)]
pub struct ParseMonthError;

impl FromStr for Month {
	type Err = ParseMonthError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"January" => Ok(Month::January),
			"February" => Ok(Month::February),
			"March" => Ok(Month::March),
			"April" => Ok(Month::April),
			"May" => Ok(Month::May),
			"June" => Ok(Month::June),
			"July" => Ok(Month::July),
			"August" => Ok(Month::August),
			"September" => Ok(Month::September),
			"October" => Ok(Month::October),
			"November" => Ok(Month::November),
			"December" => Ok(Month::December),
			_ => Err(ParseMonthError)
		}
	}
}

/* ------------------------------------------------------------------------- */

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Date {
	// It is crucial that these fields are declared in this exact order,
	// otherwise the PartialOrd and Ord traits will not compare dates properly
	pub year: u32,
	pub month: Month,
	pub day: u8
}

impl From<(u8,Month,u32)> for Date {
	fn from(item: (u8,Month,u32)) -> Self {
		let (d, m, y) = item;
		Date { day: d, month: m, year: y }
	}
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}/{}", self.year, self.month, self.day)
	}
}

#[derive(Debug,PartialEq,Eq)]
pub struct ParseDateError;

impl FromStr for Date {
	type Err = ParseDateError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split("/").collect();
		let [year, month, day] = parts.as_slice() else {
			panic!("Can't segment date '{s}' using '/'")
		};
		
		let year_fromstr = year.parse::<u32>().map_err(|_| ParseDateError)?;
		let month_fromstr = month.parse::<Month>().expect(
			format!("String '{month}' not valid for a month").as_str()
		);
		let day_fromstr = day.parse::<u8>().map_err(|_| ParseDateError)?;
		
		Ok(Date {
			year: year_fromstr,
			month: month_fromstr,
			day: day_fromstr
		})
	}
}
