
pub struct MyStr {
	pub length: usize,
	pub data: *const u8
}

pub fn from_slice( slice: &[u8] ) -> MyStr {
	MyStr{ length:slice.len(), data: &slice[0] }
}
