use std::fs;

#[derive(Debug, PartialEq)]
enum Error {
    EmptyRow,
    FileNotFound(String),
}

impl From<std::io::Error> for Error {
    fn from(_val:std::io::Error) -> Self {
        Error::FileNotFound("Not Found".to_owned())
    }
}

fn process_row(row: &str) -> Result<u64, Error> {
    let mut min: u64 = std::u64::MAX;
    let mut max: u64 = std::u64::MIN;

    if row.is_empty() {
        return Err(Error::EmptyRow); // note the semicolon here
    }

    for term in row.split_whitespace() {
        let term = term.parse::<u64>().unwrap();
        if term < min {
            min = term;
        }
        if term > max {
            max = term;
        }
    }
    let diff = max - min;
    println!("{}", diff);

    //diff
    Ok(diff)
}

fn main() -> Result<(), Error> {
    let s = fs::read_to_string("foo.txt")?;
    let s = s.trim();

    let sum = s.split('\n').map(|x| process_row(x).unwrap()).sum::<u64>();

    /*
    let mut sum = 0;
    for term in s.split('\n').map(|row| process_row(row)) {
        sum = sum + term;
    }
    */

    println!("{}", sum);
    return Ok(());


    /*
    if let Ok(s) = fs::read_to_string("foo.txt") {
        let s = s.trim();

        let sum = s.split('\n').map(|x| process_row(x).unwrap()).sum::<u64>();

        /*
        let mut sum = 0;
        for term in s.split('\n').map(|row| process_row(row)) {
            sum = sum + term;
        }
        */

        println!("{}", sum);
        return Ok(());
    }
    Err(Error::FileNotFound)
    */
    Ok(())
}

#[cfg(test)]
// namespace, similar to being in a different file
mod test {
    // use super::process_row;
    // use crate::process_row;
    use super::Error;

    #[test]
     fn test_one() {
         let input = "5 10 15 20";
         assert_eq!(crate::process_row(input), Ok(15));
     }

     #[test]
     fn test_empty() {
        let empty_input = "";
        assert_eq!(crate::process_row(empty_input), Err(Error::EmptyRow));
     }
}
