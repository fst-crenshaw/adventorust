// given two strings, one is big and one is little, take the little one
// and take all of its permutations and try to find it in the big one

struct Permutations {
    idx: usize,
    s: Vec<char>,
}

// A converstion trait: turn type String into type Permutations
impl From<String> for Permutations {
    // Self == Permutations
    fn from(s: String) -> Self {
        Self {
            idx: 0,
            s: s.chars().collect(),
        }
    }
}

impl Iterator for Permutations {
    // "associated types"
    // The thing we're returning
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.s.len() {
            // this where we generate a new perm
            let r = self.s[self.idx].to_owned();
            self.idx += 1;
            return Some(r.to_string());
        } else {
            self.idx += 1;
            return None;
        }
    }
}

fn permute(mut s: Vec<char>) -> Option<Vec<char>> {
    let mut res = None;
    for (idx, c) in s.iter().enumerate().rev().skip(1) {
        if c < &s[idx + 1] {
            // this is the biggest
            res = Some((idx, c));
            break;
        }
    }
    if res.is_none() {
        return None;
    }

    let mut other = None;
    let (res_idx, res_val) = res.unwrap();
    for (idx, c) in s.iter().enumerate().rev() {
        if res_val < c {
            other = Some((idx, c));
            break;
        }
    }
    let (other_idx, other_val) = other.unwrap();
    s.swap(res_idx, other_idx);

    println!("{:?} {} {}", s, res_idx, other_idx);
    Some(s)
}

fn has_permutations(big: &str, lil: &str) -> bool {
    let lil_perms = Permutations::from(lil.to_string());
    for p in lil_perms {
        // println!("{}", p);
    }
    true
}

fn main() {
    println!("Hello, world!");

    permute("abcd".chars().collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm() {
        // find a letter in string
        let big = "abcdefghijklmnopqrstuvwxyz";
        let lil = "p";
        assert!(has_permutations(big, lil))
    }
}
