fn main() {
    let input = "020D74FCE27E600A78020200DC298F1070401C8EF1F21A4D6394F9F48F4C1C00E3003500C74602F0080B1720298C400B7002540095003DC00F601B98806351003D004F66011148039450025C00B2007024717AFB5FBC11A7E73AF60F660094E5793A4E811C0123CECED79104ECED791380069D2522B96A53A81286B18263F75A300526246F60094A6651429ADB3B0068937BCF31A009ADB4C289C9C66526014CB33CB81CB3649B849911803B2EB1327F3CFC60094B01CBB4B80351E66E26B2DD0530070401C82D182080803D1C627C330004320C43789C40192D002F93566A9AFE5967372B378001F525DDDCF0C010A00D440010E84D10A2D0803D1761045C9EA9D9802FE00ACF1448844E9C30078723101912594FEE9C9A548D57A5B8B04012F6002092845284D3301A8951C8C008973D30046136001B705A79BD400B9ECCFD30E3004E62BD56B004E465D911C8CBB2258B06009D802C00087C628C71C4001088C113E27C6B10064C01E86F042181002131EE26C5D20043E34C798246009E80293F9E530052A4910A7E87240195CC7C6340129A967EF9352CFDF0802059210972C977094281007664E206CD57292201349AA4943554D91C9CCBADB80232C6927DE5E92D7A10463005A4657D4597002BC9AF51A24A54B7B33A73E2CE005CBFB3B4A30052801F69DB4B08F3B6961024AD4B43E6B319AA020020F15E4B46E40282CCDBF8CA56802600084C788CB088401A8911C20ECC436C2401CED0048325CC7A7F8CAA912AC72B7024007F24B1F789C0F9EC8810090D801AB8803D11E34C3B00043E27C6989B2C52A01348E24B53531291C4FF4884C9C2C10401B8C9D2D875A0072E6FB75E92AC205CA0154CE7398FB0053DAC3F43295519C9AE080250E657410600BC9EAD9CA56001BF3CEF07A5194C013E00542462332DA4295680";

    let mut packet = String::from("");

    for c in input.chars() {
        let bits = convert_hexa_to_bits(c);
        packet.push_str(&bits);
    }

    let final_packet = calculate_packet_sum(&mut packet);

    println!("Result: {}", final_packet.versions);
}

struct Packet {
    sum: u64,
    versions: u64,
    length: u64,
}

fn calculate_packet_sum(packet: &mut String) -> Packet {
    let mut sum: u64 = 0;
    let mut versions: u64 = 0;

    let mut packet_skip = 0;

    let next_three_bits = packet.chars().skip(packet_skip).take(3).collect::<String>();
    let packet_version = bits_to_u64(&next_three_bits);

    versions += packet_version;

    packet_skip += 3;

    let next_three_bits = packet.chars().skip(packet_skip).take(3).collect::<String>();
    let packet_type = bits_to_u64(&next_three_bits);

    let is_literal_value_packet = packet_type == 4;

    packet_skip += 3;

    if is_literal_value_packet {
        let mut binary = String::from("");

        const GROUP_SIZE: usize = 5;

        loop {
            let group = packet
                .chars()
                .skip(packet_skip)
                .take(GROUP_SIZE)
                .collect::<String>();
            packet_skip += GROUP_SIZE;

            let is_last_group = group.chars().next().unwrap() == '0';
            let bits = group.chars().skip(1).collect::<String>();

            binary.push_str(&bits);

            if is_last_group {
                break;
            }
        }

        sum += bits_to_u64(&binary);

        return Packet {
            sum,
            versions,
            length: packet_skip as u64,
        };
    }

    let length_type = packet.chars().skip(packet_skip).take(1).collect::<String>();

    packet_skip += 1;

    if length_type == "0" {
        let subpacket_length = 15;

        let length_bits = packet
            .chars()
            .skip(packet_skip)
            .take(subpacket_length)
            .collect::<String>();

        packet_skip += subpacket_length;

        let max_subpackets_length = bits_to_u64(&length_bits);

        let mut used_subpackets_length: u64 = 0;

        loop {
            let next_subpacket =
                calculate_packet_sum(&mut packet.chars().skip(packet_skip).collect::<String>());

            packet_skip += next_subpacket.length as usize;
            sum += next_subpacket.sum;
            used_subpackets_length += next_subpacket.length;
            versions += next_subpacket.versions;

            if used_subpackets_length >= max_subpackets_length {
                break;
            }
        }
    }
    if length_type == "1" {
        let subpacket_length = 11;

        let next_subpacket = packet
            .chars()
            .skip(packet_skip)
            .take(subpacket_length)
            .collect::<String>();

        packet_skip += subpacket_length;

        let number_of_subpackets = bits_to_u64(&next_subpacket);

        for _ in 0..number_of_subpackets {
            let mut subpacket = packet.chars().skip(packet_skip).collect::<String>();

            let packet_result = calculate_packet_sum(&mut subpacket);

            sum += packet_result.sum;
            packet_skip += packet_result.length as usize;
            versions += packet_result.versions;
        }
    }

    Packet {
        sum,
        versions,
        length: packet_skip as u64,
    }
}

fn convert_hexa_to_bits(hexa: char) -> String {
    let output = match hexa {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    return output.to_string();
}

fn bits_to_u64(bits: &str) -> u64 {
    let mut sum = 0;

    for (index, bit) in bits.chars().rev().enumerate() {
        if bit == '1' {
            sum += 2_u64.pow(index as u32);
        }
    }

    sum
}
