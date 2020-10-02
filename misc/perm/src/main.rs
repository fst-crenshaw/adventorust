// given two strings, one is big and one is little, take the little one
// and take all of its permutations and try to find it in the big one

fn get_magic_number() -> u32 {
    3
}

struct Permutations {
    idx: usize,
    s: Vec<char>,
    hard_coded: Vec<String>,
}

// A converstion trait: turn type String into type Permutations
impl From<String> for Permutations {
    // Self == Permutations
    fn from(s: String) -> Self {
        Self {
            idx: 0,
            s: s.chars().collect(),
            hard_coded: vec!["ab".to_string(), "ba".to_string()],
        }
    }
}
impl Iterator for Permutations {
    // "associated types"
    // The thing we're returning
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        //self.idx += 1;
        if self.idx < self.hard_coded.len() {
            // this is where the perm magic happens ?
            let r = self.hard_coded[self.idx].to_owned();
            self.idx += 1;
            return Some(r);
        } else {
            self.idx += 1;
            return None;
        }
    }
}

fn has_permutations(big: &str, lil: &str) -> bool {
    let lil_perms = Permutations::from(lil.to_string());
    for p in lil_perms {
        println!("{}", p);
    }
    true
}

fn main() {
    println!("Hello, world!");
    println!("The magic number is {}", get_magic_number());

    has_permutations("abc", "a");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_magic_number;

    #[test]
    fn test_magic_number() {
        let result = get_magic_number();
        assert_eq!(result, 3);
    }
    #[test]
    fn test_perm() {
        // find a letter in string
        let big = "abcdefghijklmnopqrstuvwxyz";
        let lil = "p";
        assert!(has_permutations(big, lil))
    }
}
