use std::num::ParseIntError;

use anyhow::Context;
use nom::{bytes::streaming::take, combinator::map_res, IResult};

use crate::aoc::Aoc;

#[derive(Debug)]
struct Packet {
    version: usize,
    content: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operation {
        type_id: usize,
        subpackets: Vec<Packet>,
    },
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = parse_binary(input, 3)?;
    let (input, packet_type) = parse_binary(input, 3)?;

    if packet_type == 4 {
        let (input, literal) = parse_literal(input)?;

        Ok((
            input,
            Packet {
                version,
                content: PacketType::Literal(literal),
            },
        ))
    } else {
        let (input, length_type_id) = parse_binary(input, 1)?;
        let mut subpackets = Vec::new();
        let input = if length_type_id == 0 {
            let (input, length) = parse_binary(input, 15)?;
            let (input, mut subpacket_bits) = take(length)(input)?;
            while subpacket_bits.len() > 0 {
                let (remaining, subpacket) = parse_packet(subpacket_bits)?;
                subpackets.push(subpacket);
                subpacket_bits = remaining;
            }

            input
        } else {
            let (mut input, num_packets) = parse_binary(input, 11)?;
            for _ in 0..num_packets {
                let (remaining, subpacket) = parse_packet(input)?;
                subpackets.push(subpacket);
                input = remaining;
            }

            input
        };

        Ok((
            input,
            Packet {
                version,
                content: PacketType::Operation {
                    type_id: packet_type,
                    subpackets,
                },
            },
        ))
    }
}

fn parse_literal(input: &str) -> IResult<&str, usize> {
    let mut literal = 0;
    let mut remaining = input;

    loop {
        let (input, continue_bit) = parse_binary(remaining, 1)?;
        let (input, binary) = parse_binary(input, 4)?;
        literal = literal * 16 + binary;
        remaining = input;

        if continue_bit == 0 {
            break;
        }
    }

    Ok((remaining, literal))
}

fn parse_binary(input: &str, number_of_bits: usize) -> IResult<&str, usize> {
    map_res(take(number_of_bits), |binary| {
        usize::from_str_radix(binary, 2)
    })(input)
}

fn hex_to_binary(hex: &str) -> Result<String, ParseIntError> {
    Ok(hex
        .chars()
        .map(|h| {
            let num = usize::from_str_radix(&h.to_string(), 16)?;
            Ok(format!("{:04b}", num))
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .join(""))
}

fn parse_input(input: &str) -> anyhow::Result<Packet> {
    let binary = hex_to_binary(input.trim())?;
    let (_rest, packet) = parse_packet(&binary).map_err(|err| anyhow::anyhow!("{}", err))?;

    Ok(packet)
}

fn total_of_all_version_numbers(packets: &[Packet]) -> usize {
    packets
        .iter()
        .map(|packet| match &packet.content {
            PacketType::Literal(_) => packet.version,
            PacketType::Operation { subpackets, .. } => {
                packet.version + total_of_all_version_numbers(&subpackets[..])
            }
        })
        .sum()
}

fn first_two_values(packet: &[Packet]) -> anyhow::Result<(usize, usize)> {
    let first = packet.iter().nth(0).context("Missing first subvalue")?;
    let second = packet.iter().nth(1).context("Missing second subvalue")?;

    Ok((process_packet(first)?, process_packet(second)?))
}

fn process_packet(packet: &Packet) -> anyhow::Result<usize> {
    match &packet.content {
        PacketType::Literal(value) => Ok(*value),
        PacketType::Operation {
            type_id,
            subpackets,
        } => {
            let values = subpackets
                .iter()
                .map(process_packet)
                .collect::<anyhow::Result<Vec<usize>>>()?;

            match type_id {
                0 => Ok(values.into_iter().sum()),
                1 => Ok(values.into_iter().product()),
                2 => values.into_iter().min().context("No minimum value"),
                3 => values.into_iter().max().context("No maximum value"),
                5 => {
                    let (first, second) = first_two_values(&subpackets[..])?;
                    Ok(if first > second { 1 } else { 0 })
                }
                6 => {
                    let (first, second) = first_two_values(&subpackets[..])?;
                    Ok(if first < second { 1 } else { 0 })
                }
                7 => {
                    let (first, second) = first_two_values(&subpackets[..])?;
                    Ok(if first == second { 1 } else { 0 })
                }
                _other => unreachable!(),
            }
        }
    }
}

pub struct Day16;

impl Aoc for Day16 {
    fn part1(&self, input: &str) -> anyhow::Result<usize> {
        let packet = parse_input(input)?;

        Ok(total_of_all_version_numbers(&[packet]))
    }

    fn part2(&self, input: &str) -> anyhow::Result<usize> {
        let packet = parse_input(input)?;

        process_packet(&packet)
    }
}
