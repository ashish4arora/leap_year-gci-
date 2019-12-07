use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

// Made by ashish_arora

fn leap(year: i32) -> bool {

    let rem = |x| year % x == 0;
    rem(4) && (!rem(100) || rem(400))
}


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn main() {
	let year = get_input().trim().parse::<i32>().unwrap();	
	let result = leap(year);
	if result == true {
		println!("It's a leap year")
	} else {
		println!("It's not a leap year")
	}

}