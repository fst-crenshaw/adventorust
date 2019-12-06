use anyhow::Result;
use std::fs;

mod lights;

fn main() -> Result<()> {
    crate::lights::lights::test_lights();

    let s = fs::read_to_string("input.txt")?;
    let s = s.trim();

    let mut decorations = crate::lights::lights::LightGrid {
        grid: [[0; 1000]; 1000],
    };

    for line in s.split('\n') {
        let i = crate::lights::lights::parse(line);
        crate::lights::lights::execute(&i, &mut decorations);
    }

    println!("");
    println!("Number of Lights On: {}", decorations.number_on());

    Ok(())
}
