
use mystr::MyStr;

pub fn print( str: MyStr ) {
	let MyStr{ length, data } = str;
	
	let slice = unsafe {
		std::slice::from_raw_parts( data, length )
	};
	println!( "{:?}", slice );
}
