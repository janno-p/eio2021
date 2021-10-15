use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum DoorState {
    Closed,
    Opened,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32, i32);

fn parse<R>(mut reader: R) -> Vec<Coord>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let num_doors: i32 = buffer.trim().parse().unwrap();

    let mut doors = HashMap::new();

    for _ in 0..num_doors {
        buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        let values: Vec<&str> = buffer.trim().split(" ").collect();
        if let &[v_x, v_y, v_z, v_d] = &values[..] {
            let addr = Coord(v_x.parse().unwrap(), v_y.parse().unwrap(), v_z.parse().unwrap());
            let (m1, m2) = match v_d {
                "X-" => (Coord(addr.0 - 1, addr.1, addr.2), addr),
                "X+" => (addr, Coord(addr.0 + 1, addr.1, addr.2)),
                "Y-" => (Coord(addr.0, addr.1 - 1, addr.2), addr),
                "Y+" => (addr, Coord(addr.0, addr.1 + 1, addr.2)),
                "Z-" => (Coord(addr.0, addr.1, addr.2 - 1), addr),
                "Z+" => (addr, Coord(addr.0, addr.1, addr.2 + 1)),
                _ => panic!("never"),
            };
            if let Some(state) = doors.get_mut(&(m1, m2)) {
                *state = DoorState::Opened;
            } else {
                doors.insert((m1, m2), DoorState::Closed);
            }
        }
    }

    let mut sections: HashMap<Coord, Vec<Coord>> = HashMap::new();

    for ((m1, m2), _) in doors.iter().filter(|(_, state)| **state == DoorState::Opened) {
        let mut section = vec![];
        if let Some(m1_section) = sections.get_mut(m1) {
            section.append(m1_section);
        } else {
            section.append(&mut vec![*m1])
        }
        if let Some(m2_section) = sections.get_mut(m2) {
            section.append(m2_section);
        } else {
            section.append(&mut vec![*m2]);
        }
        for c in &section {
            if let Some(p) = sections.get_mut(&c) {
                *p = section.clone();
            } else {
                sections.insert(*c, section.clone());
            }
        }
    }

    buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let values: Vec<&str> = buffer.trim().split(" ").collect();
    if let &[v_x, v_y, v_z] = &values[..] {
        let addr = Coord(v_x.parse().unwrap(), v_y.parse().unwrap(), v_z.parse().unwrap());
        if let Some(s) = sections.get(&addr) {
            return s.clone();
        } else {
            return vec![addr];
        }
    }

    panic!("never")
}

fn sort_modules(v: &mut Vec<Coord>) {
    v.sort_by(|a, b| {
        match a.2.cmp(&b.2) {
            std::cmp::Ordering::Equal => {
                match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Equal => {
                        a.0.cmp(&b.0)
                    },
                    o => o,
                }
            },
            o => o,
        }
    });
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut v = parse(input);
    sort_modules(&mut v);
    for c in &v {
        println!("{} {} {}", c.0, c.1, c.2);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, sort_modules};

    static SAMPLE_INPUT: &[u8] = br#"5
2 1 1 X-
1 1 1 X+
1 1 1 Z+
3 1 1 X-
1 1 2 Z-
1 1 1
"#;

    #[test]
    fn test() {
        let mut v = parse(&SAMPLE_INPUT[..]);
        sort_modules(&mut v);
        for c in &v {
            println!("{} {} {}", c.0, c.1, c.2);
        }
    }
}
