use std::fs::{File};
use std::io::{Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let risk = process_hex_packets(contents);
    println!("{}", risk);
}

fn process_hex_packets(content: String) -> u64 {
    let packet = parse_packet(content);
    proccess_packets(&packet, 0).0
}

fn get_literal_value(packet: &Vec<u8>, offset: usize) -> (u64, usize) {
    let mut value = 0;
    let mut last_fragment = false;
    let mut fragments_number: usize = 0;
    while !last_fragment {
        value = value << 4;
        let mut fragment = get_packet_bits(packet, offset + fragments_number * 5, 5)[0];
        fragments_number += 1;
        if fragment >> 4 == 0 {
            last_fragment = true;
        }
        fragment = fragment & 0b1111;
        value = value | fragment as u64;
    }
    (value, offset + fragments_number * 5)
}

fn proccess_packets(packet: &Vec<u8>, offset: usize) -> (u64, usize) {
    let version = get_packet_bits(packet, offset, 3)[0];
    let packet_type = get_packet_bits(packet, offset + 3, 3)[0];
    if packet_type == 4 {
        get_literal_value(packet, offset + 6)
    } else {
        let operator_type = get_packet_bits(packet, offset + 6, 1)[0] == 0;
        let mut sub_packet_values = vec![];
        let mut return_offset = 0;
        if operator_type {
            let sub_packets_length_arr = get_packet_bits(packet, offset + 7, 15);
            let sub_packets_length = ((sub_packets_length_arr[0] as usize) << 8) + sub_packets_length_arr[1] as usize;
            return_offset = offset + 22;
            while return_offset < offset + 22 + sub_packets_length {
                let (sub_packet_value, packet_return_offset) = proccess_packets(packet, return_offset);
                return_offset = packet_return_offset;
                sub_packet_values.push(sub_packet_value);
            }
        } else {
            let sub_packets_number_arr = get_packet_bits(packet, offset + 7, 11);
            let sub_packets_number = ((sub_packets_number_arr[0] as usize) << 8) + sub_packets_number_arr[1] as usize;
            return_offset = offset + 18;
            for _ in 0..sub_packets_number {
                let (sub_packet_value, packet_return_offset) = proccess_packets(packet, return_offset);
                return_offset = packet_return_offset;
                sub_packet_values.push(sub_packet_value);
            }
        }
        match packet_type {
            0 => {
                let sum = sub_packet_values.iter().sum::<u64>();
                (sum, return_offset)
            }
            1 => {
                let prod = sub_packet_values.iter().fold(1, |acc, x| acc * x);
                (prod, return_offset)
            }
            2 => {
                let min = sub_packet_values.iter().min().unwrap();
                (*min, return_offset)
            }
            3 => {
                let max = sub_packet_values.iter().max().unwrap();
                (*max, return_offset)
            }
            5 => {
                if sub_packet_values[0] > sub_packet_values[1] {
                    (1, return_offset)
                } else {
                    (0, return_offset)
                }
            }
            6 => {
                if sub_packet_values[0] < sub_packet_values[1] {
                    (1, return_offset)
                } else {
                    (0, return_offset)
                }
            }
            7 => {
                if sub_packet_values[0] == sub_packet_values[1] {
                    (1, return_offset)
                } else {
                    (0, return_offset)
                }
            }
            _ => {
                panic!("unknown packet type {}", packet_type);
            }
        }
    }
}

fn get_packet_bits(packet: &Vec<u8>, offset: usize, length: usize) -> Vec<u8> {
    let packet_index = offset / 8;
    let packet_offset = offset % 8;
    let packet_number = (length / 8) + 1;
    let mut packet_bits = vec![0; packet_number];
    let mut source_bit = packet_offset;
    let mut source_byte = 0;
    let mut des_bit = 0;
    let mut des_byte = 0;
    for _ in 0..length {
        packet_bits[des_byte] = packet_bits[des_byte] << 1;
        let origin_byte = packet[packet_index + source_byte];
        let source_shift = 7 - source_bit;
        let mask = 1 << source_shift;
        packet_bits[des_byte] = packet_bits[des_byte] | ((origin_byte & mask) >> source_shift);
        source_bit += 1;
        if source_bit == 8 {
            source_bit = 0;
            source_byte += 1;
        }
        des_bit += 1;
        if des_bit == 8 {
            des_bit = 0;
            des_byte += 1;
        }
    }
    packet_bits
}

fn parse_packet(hex_packet: String) -> Vec<u8> {
    if hex_packet.len() % 2 != 0 {
        panic!("Invalid hex instruction");
    }
    let mut packet = Vec::new();
    let mut i = 0;
    let chars = hex_packet.chars().collect::<Vec<char>>();
    while i < chars.len() {
        packet.push(u8::from_str_radix(format!("{}{}", chars[i], chars[i + 1]).as_str(), 16).unwrap());
        i += 2;
    }
    packet
}

#[test]
fn test_case_day16_2_1() {
    let risk = process_hex_packets(
        String::from("C200B40A82"));
    assert_eq!(risk, 3)
}

#[test]
fn test_case_day16_2_2() {
    let risk = process_hex_packets(
        String::from("04005AC33890"));
    assert_eq!(risk, 54)
}

#[test]
fn test_case_day16_2_3() {
    let risk = process_hex_packets(
        String::from("880086C3E88112"));
    assert_eq!(risk, 7)
}

#[test]
fn test_case_day16_2_4() {
    let risk = process_hex_packets(
        String::from("CE00C43D881120"));
    assert_eq!(risk, 9)
}

#[test]
fn test_case_day16_2_5() {
    let risk = process_hex_packets(
        String::from("D8005AC2A8F0"));
    assert_eq!(risk, 1)
}

#[test]
fn test_case_day16_2_6() {
    let risk = process_hex_packets(
        String::from("F600BC2D8F"));
    assert_eq!(risk, 0)
}

#[test]
fn test_case_day16_2_7() {
    let risk = process_hex_packets(
        String::from("9C005AC2F8F0"));
    assert_eq!(risk, 0)
}

#[test]
fn test_case_day16_2_8() {
    let risk = process_hex_packets(
        String::from("9C0141080250320F1802104A08"));
    assert_eq!(risk, 1)
}