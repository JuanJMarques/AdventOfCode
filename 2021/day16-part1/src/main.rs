use std::fs::{File};
use std::io::{Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let risk = calculate_packet_versions_sum(contents);
    println!("{}", risk);
}

fn calculate_packet_versions_sum(content: String) -> u32 {
    let packet = parse_packet(content);
    sum_packets_versions(&packet, 0).0
}

fn sum_packets_versions(packet: &Vec<u8>, offset: usize) -> (u32, usize) {
    let version = get_packet_bits(packet, offset, 3)[0];
    let packet_type = get_packet_bits(packet, offset + 3, 3)[0];
    if packet_type == 4 {
        let mut last_literal = false;
        let mut literal_number: usize = 0;
        while !last_literal {
            let literal = get_packet_bits(packet, offset + 6 + literal_number * 5, 5)[0];
            if literal >> 4 == 0 {
                last_literal = true;
            }
            literal_number += 1;
        }
        (version as u32, offset + 6 + literal_number * 5)
    } else {
        let operator_type = get_packet_bits(packet, offset + 6, 1)[0] == 0;
        if operator_type {
            let sub_packets_length_arr = get_packet_bits(packet, offset + 7, 15);
            let sub_packets_length = ((sub_packets_length_arr[0] as usize) << 8) + sub_packets_length_arr[1] as usize;
            let mut sub_packet_versions = vec![];
            let mut return_offset = offset + 22;
            while return_offset < offset + 22 + sub_packets_length {
                let (sub_packet_version, packet_return_offset) = sum_packets_versions(packet, return_offset);
                return_offset = packet_return_offset;
                sub_packet_versions.push(sub_packet_version);
            }
            (sub_packet_versions.iter().sum::<u32>() + version as u32, return_offset)
        } else {
            let sub_packets_number_arr = get_packet_bits(packet, offset + 7, 11);
            let sub_packets_number = ((sub_packets_number_arr[0] as usize) << 8) + sub_packets_number_arr[1] as usize;
            let mut sub_packet_versions = vec![];
            let mut return_offset = offset + 18;
            for _ in 0..sub_packets_number {
                let (sub_packet_version, packet_return_offset) = sum_packets_versions(packet, return_offset);
                return_offset = packet_return_offset;
                sub_packet_versions.push(sub_packet_version);
            }
            (sub_packet_versions.iter().sum::<u32>() + version as u32, return_offset)
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
fn test_case_day16_1_1() {
    let risk = calculate_packet_versions_sum(
        String::from("D2FE28"));
    assert_eq!(risk, 6)
}

#[test]
fn test_case_day16_1_2() {
    let risk = calculate_packet_versions_sum(
        String::from("8A004A801A8002F478"));
    assert_eq!(risk, 16)
}

#[test]
fn test_case_day16_1_3() {
    let risk = calculate_packet_versions_sum(
        String::from("620080001611562C8802118E34"));
    assert_eq!(risk, 12)
}

#[test]
fn test_case_day16_1_4() {
    let risk = calculate_packet_versions_sum(
        String::from("C0015000016115A2E0802F182340"));
    assert_eq!(risk, 23)
}

#[test]
fn test_case_day16_1_5() {
    let risk = calculate_packet_versions_sum(
        String::from("A0016C880162017C3686B18A3D4780"));
    assert_eq!(risk, 31)
}