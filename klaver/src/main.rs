use std::collections::{HashMap, HashSet};
use std::error;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct Input {
    broken_keys: HashSet<i32>,
    notes: HashMap<i32, i32>,
}

impl Input {
    fn parse<R>(mut reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;

        buffer = String::new();
        reader.read_line(&mut buffer)?;
        let broken_keys = buffer.trim().split(" ")
            .map(|v| v.parse::<i32>())
            .collect::<std::result::Result<HashSet<i32>, _>>()?;

        buffer = String::new();
        reader.read_line(&mut buffer)?;

        buffer = String::new();
        reader.read_line(&mut buffer)?;
        let notes = buffer.trim().split(" ")
            .fold(HashMap::new(), |mut m, v| {
                let note = v.parse::<i32>().unwrap();
                if let Some(count) = m.get_mut(&note) {
                    *count = *count + 1;
                } else {
                    m.insert(note, 1);
                }
                m
            });


        Ok(Input { broken_keys, notes })
    }

    fn num_broken_notes(&self) -> i32 {
        self.broken_keys.iter().fold(0, |count, note| {
            if let Some(v) = self.notes.get(&note) {
                count + v
            } else {
                count
            }
        })
    }

    fn transpose(&self) -> Option<i32> {
        let unique_keys: HashSet<_> = self.notes.keys().map(|v| i32::from(*v)).collect();
        let (min, max) = unique_keys.iter()
            .fold((1024, 0), |(agg_min, agg_max), v| (agg_min.min(*v), agg_max.max(*v)));

        for n in 0..1024 {
            if min - n >= 0 {
                if unique_keys.iter().map(|v| *v - n).all(|v| !self.broken_keys.contains(&v)) {
                    return Some(-n)
                }
            }
            if max + n < 1024 {
                if unique_keys.iter().map(|v| *v + n).all(|v| !self.broken_keys.contains(&v)) {
                    return Some(n)
                }
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let input_args = Input::parse(input)?;
    println!("{}", input_args.num_broken_notes());

    if let Some(n) = input_args.transpose() {
        println!("{}", n);
    } else {
        println!("X");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Input;

    static SAMPLE1_INPUT: &[u8] = br#"2
7 8
6
4 5 6 7 8 7
"#;

    static SAMPLE2_INPUT: &[u8] = br#"2
0 1023
4
0 1023 1023 1023
"#;

    #[test]
    fn parse_input_from_sample_1() {
        let input = Input::parse(&SAMPLE1_INPUT[..]).unwrap();
        assert_eq!(input.broken_keys.len(), 2);
        assert!(input.broken_keys.contains(&7));
        assert!(input.broken_keys.contains(&8));
        assert_eq!(input.num_broken_notes(), 3);
        assert_eq!(input.transpose(), Some(-2));
    }

    #[test]
    fn parse_input_from_sample_2() {
        let input = Input::parse(&SAMPLE2_INPUT[..]).unwrap();
        assert_eq!(input.broken_keys.len(), 2);
        assert!(input.broken_keys.contains(&0));
        assert!(input.broken_keys.contains(&1023));
        assert_eq!(input.num_broken_notes(), 4);
        assert_eq!(input.transpose(), None);
    }
}
