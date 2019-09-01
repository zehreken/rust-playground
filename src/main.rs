fn main()
{
	let logical: bool = true;
	let a_float: f64 = 1.0;
	let mut an_integer = 5i32; // Suffix annotation

	println!("{}", logical);
	println!("{}", a_float);
	println!("{}", an_integer);

	an_integer = 6;
	println!("{}", an_integer);
	an_integer += 1;
	println!("{}", an_integer);

	let array: [i32; 5] = [1, 2, 3, 4, 5];
	println!("{}", array.len());

	let slice = &array[1 .. 3];
	println!("{}, {}", slice[0], slice.len());
}
