// Definitions to send to C

#[no_mangle]
pub extern "C" fn noop() {}

#[no_mangle]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
	a + b
}
