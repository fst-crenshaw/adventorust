use std::panic;

// Goal: Use catch_unwind() and extract the panic message from the result.
//
fn main() {
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());

    match result {
        Ok(_) => println!("OK!"),
        Err(r) => match r
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| r.downcast_ref::<&'static str>().cloned())
        {
            Some(as_string) => {
                println!("Some case: ({}): {}", as_string.len(), as_string);
            }
            None => {
                println!("None case: {:?}", r);
            }
        },
    }
}
