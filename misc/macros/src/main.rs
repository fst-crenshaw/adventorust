/*
 * Types of macros:
 * - declarative macros ("plain" macros)
 * - procedural macros (custom derive, attribute-like, function-like)
 *
 * Useful tools:
 * - cargo expand, to see generated output (you may have to `cargo install` it)
 */

/*
 * A declarative macro
 */
macro_rules! sand {
    ( $( $i:ident )? ) => {
        let mut count = 0;
        $(
            let _ = $i;
            count += 1;
        )*
        //println!("{}", $i);
        println!("# idents={}", count);
        ()
    };

    ( $( $i:expr ),* ) => {
        let mut count = 0;
        $(
            let _ = $i;
            count += 1;
        )?
        println!("# exprs={}", count);
        ()
    }
}

fn main() {
    let s = "tequila";
    let s2 = "car";

    // exprs
    sand!(s, s2, 3);
    sand!(s, s2, s);
    sand!(2, 3);

    // idents
    sand!(s, s2);
    sand!(s);
}
