pub mod solutions {
    use std::collections::{HashMap, HashSet};
    use std::fmt;

    use crate::AocBufReader;
    use crate::utils::conversion::binary_string_slice_to_usize;

    use lazy_static::lazy_static;


    lazy_static! {
        static ref HEX_TO_BITS: HashMap<char, &'static str> = vec![
            ('0', "0000"),
            ('1', "0001"),
            ('2', "0010"),
            ('3', "0011"),
            ('4', "0100"),
            ('5', "0101"),
            ('6', "0110"),
            ('7', "0111"),
            ('8', "1000"),
            ('9', "1001"),
            ('A', "1010"),
            ('B', "1011"),
            ('C', "1100"),
            ('D', "1101"),
            ('E', "1110"),
            ('F', "1111"),
        ].into_iter().collect();
    }

    #[derive(PartialEq, Eq, Debug)]
    enum PacketType {
        Literal,
        Operator,
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Packet {
        packet_version: usize,
        packet_type: PacketType,
        packet_type_id: usize,
        sub_packets: Vec<Packet>,

        // only populated on PacketType::Literal
        literal_val: Option<usize>,
    }

    impl Packet {
        fn new(packet_version: usize, packet_type: PacketType, packet_type_id: usize) -> Packet {
            Packet { packet_version, packet_type, packet_type_id, sub_packets: vec![], literal_val: None }
        }

        fn sum_all_versions(&self) -> usize {
            let sub_packet_sum: usize = self.sub_packets.iter()
                .map(|p| p.sum_all_versions()).sum();
            sub_packet_sum + self.packet_version
        }

        fn perform_operations(&self) -> usize {
            let mut result: usize;
            let mut subpacket_iterator = self.sub_packets.iter().map(|sp| sp.perform_operations());

            match self.packet_type {
                PacketType::Literal => result = self.literal_val.unwrap(),
                PacketType::Operator => {
                    match self.packet_type_id {
                        0 => result = subpacket_iterator.sum(),
                        1 => result = subpacket_iterator.fold(1, |product, xx| product * xx),
                        2 => result = subpacket_iterator.min().unwrap(),
                        3 => result = subpacket_iterator.max().unwrap(),
                        5 => result = {
                            let first = subpacket_iterator.next().unwrap();
                            let second = subpacket_iterator.next().unwrap();
                            match first > second {
                                true => 1,
                                false => 0
                            }
                        },
                        6 => result = {
                            let first = subpacket_iterator.next().unwrap();
                            let second = subpacket_iterator.next().unwrap();
                            match first < second {
                                true => 1,
                                false => 0
                            }
                        },
                        7 => result = {
                            let first = subpacket_iterator.next().unwrap();
                            let second = subpacket_iterator.next().unwrap();
                            match first == second {
                                true => 1,
                                false => 0
                            }
                        },
                        _ => panic!("Unknown operator!")
                    }
                }
            }
            result
        }
    }


    impl fmt::Display for Packet {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let literal_val_str: String = match self.literal_val {
                Some(val) => val.to_string(),
                None => "None".to_string()
            };

            write!(f, "Packet(version: {}, literal_val: {})", self.packet_version, literal_val_str)
        }
    }


    fn stream_from_hex(s: &str) -> String {
        let mut bits: String = String::new();
        for c in s.chars() {
            bits.push_str(HEX_TO_BITS.get(&c).unwrap());
        }
        bits
    }


    fn read_bits<'a>(stream: &'a str, cursor: &mut usize, n_bits: usize) -> &'a str {
        let range_start: usize = *cursor;
        *cursor += n_bits;
        &stream[range_start..range_start + n_bits]
    }


    fn decode_literal_packet<'a>(stream: &'a str, cursor: &mut usize, packet_version: usize, packet_type_id: usize) -> Packet {
        let mut packet = Packet::new(packet_version, PacketType::Literal, packet_type_id);

        let mut group: &str;
        let mut literal_val = String::new();
        loop {
            group = read_bits(stream, cursor, 5);
            literal_val.push_str(&group[1..]);
            if group.as_bytes()[0] as char == '0' { break }
        }
        packet.literal_val = Some(binary_string_slice_to_usize(&literal_val));
        packet
    }


    fn decode_operator_packet<'a>(stream: &'a str, mut cursor: &mut usize, packet_version: usize, packet_type_id: usize) -> Packet {
        let mut packet = Packet::new(packet_version, PacketType::Operator, packet_type_id);

        let length_type_id: &str = read_bits(stream, cursor, 1);
        match length_type_id {
            "0" => {
                let length: usize = binary_string_slice_to_usize(read_bits(stream, &mut cursor, 15));
                packet.sub_packets.extend(
                    decode_packets(&stream[0..*cursor + length], &mut cursor)
                )
            },
            "1" => {
                let n_sub_packets: usize = binary_string_slice_to_usize(read_bits(stream, &mut cursor, 11));
                for _ in 0..n_sub_packets {
                    packet.sub_packets.push(decode_packet(&stream, &mut cursor))
                }
            },
            _ => panic!("I thought I saw a 2!") // https://www.youtube.com/watch?v=MOn_ySghN2Y
        }

        packet
    }


    fn decode_packet<'a>(stream: &'a str, mut cursor: &mut usize) -> Packet {
        let packet_version: usize = binary_string_slice_to_usize(read_bits(stream, &mut cursor, 3));
        let packet_type_id: usize = binary_string_slice_to_usize(read_bits(stream, &mut cursor, 3));
        match packet_type_id {
            4 => return decode_literal_packet(stream, &mut cursor, packet_version, packet_type_id),
            _ => return decode_operator_packet(stream, &mut cursor, packet_version, packet_type_id)
        }
    }


    fn decode_packets<'a>(stream: &'a str, mut cursor: &mut usize) -> Vec<Packet> {
        let mut packets: Vec<Packet> = vec![];

        while *cursor < stream.len() && stream[*cursor..].chars().collect::<HashSet<char>>().contains(&'1') {
            packets.push(decode_packet(stream, &mut cursor));
        }
        *cursor = stream.len(); // fast forward
        packets
    }


    fn _part_1(hex_stream: &str) -> usize {
        let stream= &stream_from_hex(hex_stream)[..];
        let mut cursor: usize = 0;
        let packets = decode_packets(stream, &mut cursor);
        packets.iter().map(|p| p.sum_all_versions()).sum()
    }


    pub fn part_1(mut aoc_reader: AocBufReader) -> usize {
        _part_1(&aoc_reader.next().unwrap())
    }


    fn _part_2(hex_stream: &str) -> usize {
        let stream= &stream_from_hex(hex_stream)[..];
        let mut cursor: usize = 0;
        let packets = decode_packets(stream, &mut cursor);
        packets.iter().map(|p| p.perform_operations()).sum()
    }


    pub fn part_2(mut aoc_reader: AocBufReader) -> usize {
        _part_2(&aoc_reader.next().unwrap())
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_from_hex() {
            assert_eq!(stream_from_hex("D2FE28"), "110100101111111000101000".to_string());
        }


        #[test]
        fn test_read_bits() {
            let test_str: &'static str = "abcdefg";
            let mut cursor: usize = 0;
            assert_eq!(read_bits(test_str, &mut cursor, 0), "");
            assert_eq!(cursor, 0);

            assert_eq!(read_bits(test_str, &mut cursor, 2), "ab");
            assert_eq!(cursor, 2);

            assert_eq!(read_bits(test_str, &mut cursor, 3), "cde");
            assert_eq!(cursor, 5);
        }


        #[test]
        fn test_decode_literal_packet() {
            let stream: &'static str = "110100101111111000101000";
            let mut cursor: usize = 6;
            let obs_packet = decode_literal_packet(stream, &mut cursor, 6, 4);
            let mut exp_packet = Packet::new(6, PacketType::Literal, 4);
            exp_packet.literal_val = Some(2021usize);
            assert_eq!(obs_packet, exp_packet);
        }

        #[test]
        fn test_decode_packet_literal() {
            let stream: &'static str = "110100101111111000101000";
            let mut cursor: usize = 0;
            let obs_packet = decode_packet(stream, &mut cursor);
            let mut exp_packet = Packet::new(6, PacketType::Literal, 4);
            exp_packet.literal_val = Some(2021usize);
            assert_eq!(obs_packet, exp_packet);
        }


        #[test]
        fn test_decode_packets_literal() {
            let stream: &'static str = "110100101111111000101000";
            let mut cursor: usize = 0;
            let obs_packet = decode_packets(stream, &mut cursor);
            let mut exp_packet = Packet::new(6, PacketType::Literal, 4);
            exp_packet.literal_val = Some(2021usize);
            assert_eq!(obs_packet, vec![exp_packet]);
            assert_eq!(cursor, stream.len());
        }


        #[test]
        fn test_decode_operator_packet() {
            let stream: &'static str = "00111000000000000110111101000101001010010001001000000000";
            let mut cursor: usize = 0;

            let obs_packet = decode_packet(stream, &mut cursor);
            let mut exp_packet = Packet::new(1, PacketType::Operator, 6);
            let mut sub_packet_1 = Packet::new(6, PacketType::Literal, 4);
            sub_packet_1.literal_val = Some(10usize);

            let mut sub_packet_2 = Packet::new(2, PacketType::Literal, 4);
            sub_packet_2.literal_val = Some(20usize);

            exp_packet.sub_packets.extend(vec![sub_packet_1, sub_packet_2]);
            assert_eq!(obs_packet, exp_packet);
        }


        #[test]
        fn test_part_1() {
            let hex_stream = "8A004A801A8002F478";
            assert_eq!(_part_1(hex_stream), 16);

            let hex_stream = "620080001611562C8802118E34";
            assert_eq!(_part_1(hex_stream), 12);
        }


        #[test]
        fn test_part_2() {
            let hex_stream = "C200B40A82";
            assert_eq!(_part_2(hex_stream), 3);

            let hex_stream = "880086C3E88112";
            assert_eq!(_part_2(hex_stream), 7);
        }

    }
}