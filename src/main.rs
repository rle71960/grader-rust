use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::i32;

fn main() {

	let path = Path::new("grades.txt");
	let display = path.display();

	let mut f = match File::open(&path) {
		Err(why) => panic!( "couldn't open {}: {}"
						  , display
						  , why.description() )
		, Ok(f) => f,
	};

	let mut raw_grades = String::new();
	match f.read_to_string(&mut raw_grades) {
		Err(why) => panic!( "couldn't read {}: {}"
						  , display
						  , why.description() )	
		,Ok(s) => print!("")
	};

	let mut grades_strings = raw_grades.trim().split(",").collect::<Vec<&str>>();
	let mut grades: Vec<i32> = Vec::new();

	for grade_string in grades_strings {
		grades.push(grade_string.parse().unwrap());
	}
	
	print!("Read grades: ");
	
	println!("{}\n", &raw_grades);

}
