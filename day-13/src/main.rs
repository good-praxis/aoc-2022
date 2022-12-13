use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let raw_packets = input
        .split("\n\n")
        .filter(|packets| !packets.is_empty())
        .collect::<Vec<&str>>();

    let mut results = vec![];

    for pair in raw_packets {
        let split = pair.split('\n').collect::<Vec<&str>>();
        let first = PacketData::Packet(Packet::from(split[0]));
        let second = PacketData::Packet(Packet::from(split[1]));
        results.push(first.cmp(&second));
    }

    let mut res = 0;
    for i in 1..=results.len() {
        if results[i - 1] == Ordering::Less {
            res += i;
        }
    }
    res
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Packet(Vec<PacketData>);
impl Packet {
    // Input assumed to be a single line
    fn from(input: &str) -> Self {
        Self::from_bytes(input.as_bytes())
    }
    fn from_bytes(input: &[u8]) -> Self {
        let mut vec = vec![];
        let mut queue = vec![];
        let mut stack_count = 0;

        let mut iter = input.iter().peekable();
        let mut oneshot = true;

        loop {
            if let Some(&byte) = iter.next() {
                match byte {
                    b'[' if oneshot => {
                        oneshot = false;
                    }
                    b'[' => {
                        stack_count += 1;
                        queue.push(b'[');
                    }
                    b']' if stack_count == 0 => {
                        break;
                    }
                    b']' => {
                        stack_count -= 1;
                        queue.push(b']');
                        if stack_count == 0 {
                            vec.push(PacketData::Packet(Packet::from_bytes(&queue)));
                            queue.clear();
                        }
                    }
                    b'1' if iter.peek() == Some(&&b'0') => {
                        iter.next();
                        vec.push(PacketData::Value((b'9' + 1).into()));
                    }
                    b' ' | b',' => (),
                    byte if stack_count != 0 => queue.push(byte),
                    byte => vec.push(PacketData::Value(byte.into())),
                }
            } else {
                break;
            }
        }

        Self(vec)
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PacketData {
    Value(usize),
    Packet(Packet),
}
impl PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(f), Self::Value(s)) => f.cmp(&s),
            (Self::Packet(f), Self::Packet(s)) => {
                let mut iter_s = s.0.iter();
                for first in f.0.iter() {
                    if let Some(second) = iter_s.next() {
                        if first.cmp(second) == Ordering::Equal {
                            continue;
                        } else {
                            return first.cmp(second);
                        };
                    } else {
                        return Ordering::Greater;
                    }
                }
                if iter_s.next().is_some() {
                    return Ordering::Less;
                }

                Ordering::Equal
            }
            (Self::Value(val), Self::Packet(pack)) => value_to_packet(*val).cmp(pack),
            (Self::Packet(pack), Self::Value(val)) => pack.cmp(&value_to_packet(*val)),
        }
    }
}

fn value_to_packet(val: usize) -> Packet {
    let byte = [val as u8];
    Packet::from_bytes(&byte)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::{Packet, PacketData};

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(TEST_INPUT), 13);
    }
    #[test]
    fn val_cmp_val() {
        let first = PacketData::Value(1);
        let second = PacketData::Value(2);
        assert_eq!(first.cmp(&second), Ordering::Less);
    }

    #[test]
    fn val_cmp_packet() {
        let first = PacketData::Value(b'1'.into());
        let second = PacketData::Packet(Packet::from_bytes(b"[1, 2]"));
        assert_eq!(first.cmp(&second), Ordering::Less);
    }

    #[test]
    fn ten_test() {
        let packet = Packet::from_bytes(b"10");
        dbg!(packet);
    }

    #[test]
    fn example_pair_2() {
        let first = PacketData::Packet(Packet::from_bytes(b"[[1],[2,3,4]]"));
        let second = PacketData::Packet(Packet::from_bytes(b"[[1],4]"));
        assert_eq!(first.cmp(&second), Ordering::Less);

        let input = "[[1],[2,3,4]]\n[[1],4]\n\n[[1],[2,3,4]]\n[[1],4]\n\n";

        let raw_packets = input
            .split("\n\n")
            .filter(|packets| !packets.is_empty())
            .collect::<Vec<&str>>();
        let split = raw_packets[0].split('\n').collect::<Vec<&str>>();

        let first = PacketData::Packet(Packet::from(split[0]));
        let second = PacketData::Packet(Packet::from(split[1]));
        assert_eq!(first.cmp(&second), Ordering::Less);
    }
    #[test]
    fn example_pair_3() {
        let first = PacketData::Packet(Packet::from_bytes(b"[9]"));
        let second = PacketData::Packet(Packet::from_bytes(b"[[8,7,6]]"));
        assert_eq!(first.cmp(&second), Ordering::Greater);
    }
    #[test]
    fn example_pair_6() {
        let first = PacketData::Packet(Packet::from_bytes(b"[]"));
        let second = PacketData::Packet(Packet::from_bytes(b"[3]"));
        assert_eq!(first.cmp(&second), Ordering::Less);
    }
}
