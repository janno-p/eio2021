use std::collections::HashMap;
use std::error;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn hist<R>(mut reader: R) -> Result<HashMap<char, i32>>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let num_rows: i32 = buffer.trim().parse()?;
    
    let mut char_map = HashMap::new();
    for _ in 0..num_rows {
        buffer = String::new();
        reader.read_line(&mut buffer)?;
        let mut chars = buffer.chars();
        while let Some(c) = chars.next() {
            if !c.is_whitespace() {
                if let Some(v) = char_map.get_mut(&c) {
                    *v = *v + 1;
                } else {
                    char_map.insert(c, 1);
                }
            }
        }
    }

    Ok(char_map)
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let map = hist(input)?;
    for (key, val) in &map {
        println!("{} {}", key, val);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hist;

    #[test]
    fn parse() {
        let input = br#"4
A printing press is a mechanical device
for applying pressure to an inked surface
resting upon a print medium (such as paper
or cloth), thereby transferring the ink.
"#;

        let map = hist(&input[..]).unwrap();
        
        assert_eq!(map.len(), 26);
        assert_eq!(map.get(&'('), Some(&1));
        assert_eq!(map.get(&')'), Some(&1));
        assert_eq!(map.get(&','), Some(&1));
        assert_eq!(map.get(&'.'), Some(&1));
        assert_eq!(map.get(&'A'), Some(&1));
        assert_eq!(map.get(&'a'), Some(&10));
        assert_eq!(map.get(&'b'), Some(&1));
        assert_eq!(map.get(&'c'), Some(&6));
        assert_eq!(map.get(&'d'), Some(&3));
        assert_eq!(map.get(&'e'), Some(&15));
        assert_eq!(map.get(&'f'), Some(&3));
        assert_eq!(map.get(&'g'), Some(&4));
        assert_eq!(map.get(&'h'), Some(&5));
        assert_eq!(map.get(&'i'), Some(&12));
        assert_eq!(map.get(&'k'), Some(&2));
        assert_eq!(map.get(&'l'), Some(&3));
        assert_eq!(map.get(&'m'), Some(&3));
        assert_eq!(map.get(&'n'), Some(&12));
        assert_eq!(map.get(&'o'), Some(&5));
        assert_eq!(map.get(&'p'), Some(&9));
        assert_eq!(map.get(&'r'), Some(&14));
        assert_eq!(map.get(&'s'), Some(&10));
        assert_eq!(map.get(&'t'), Some(&8));
        assert_eq!(map.get(&'u'), Some(&5));
        assert_eq!(map.get(&'v'), Some(&1));
        assert_eq!(map.get(&'y'), Some(&2));
    }
}
