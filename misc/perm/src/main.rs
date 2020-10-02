fn get_magic_number() -> u32 {
    3
}

fn main() {
    println!("Hello, world!");
    println!("The magic number is {}", get_magic_number());
}


#[cfg(test)]
mod tests {
    use crate::get_magic_number;
    
    #[test]
    fn test_magic_number() {
	let result = get_magic_number();
	assert_eq!(result, 3);
    }
}
