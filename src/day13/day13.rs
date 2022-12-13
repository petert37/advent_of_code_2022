use std::{cmp::Ordering, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug, Clone, Eq)]
enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::List(l0), Self::List(r0)) => {
                let mut i = 0;
                let ordering = loop {
                    match (l0.get(i), r0.get(i)) {
                        (None, None) => break Ordering::Equal,
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (Some(left_pd), Some(right_pd)) => match left_pd.partial_cmp(right_pd) {
                            Some(Ordering::Less) => break Ordering::Less,
                            Some(Ordering::Greater) => break Ordering::Greater,
                            _ => i += 1,
                        },
                    }
                };
                Some(ordering)
            }
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Integer(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_packet_data(input: &str) -> IResult<&str, PacketData> {
    delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((
                complete::u32.map(|num| PacketData::Integer(num)),
                parse_packet_data,
            )),
        ),
        tag("]"),
    )
    .map(|result| PacketData::List(result))
    .parse(input)
}

fn parse_packets(input: &str) -> IResult<&str, Vec<PacketData>> {
    separated_list1(many1(line_ending), parse_packet_data)(input)
}

fn main() {
    let input = fs::read_to_string("src/day13/input.txt").unwrap();
    let (_, packets) = parse_packets(&input).unwrap();

    let sum = packets
        .chunks(2)
        .enumerate()
        .map(|(i, two_packets)| {
            if two_packets[0] < two_packets[1] {
                i as i32 + 1
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("Sum packet indices: {sum}");

    let start_divider = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let end_divider = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

    let mut packets = packets.clone();
    packets.append(&mut vec![start_divider.clone(), end_divider.clone()]);
    packets.sort();

    let start_index = packets.iter().enumerate().find_map(|(i, p)| {
        if p == &start_divider {
            Some(i + 1)
        } else {
            None
        }
    });

    let end_index = packets.iter().enumerate().find_map(|(i, p)| {
        if p == &end_divider {
            Some(i + 1)
        } else {
            None
        }
    });

    if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
        println!("Decoder key: {}", start_index * end_index);
    }
}
