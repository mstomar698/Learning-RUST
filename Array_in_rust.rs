use std::io;

fn main() {
	 let a = [1, 2, 3, 4, 5];
	
	println!("Please Enter the Array Index");

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("failed to read line");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered is not Valid")
	let element = a[Index];

	println!("The value of the element at Index {index} is : {element}");
}
