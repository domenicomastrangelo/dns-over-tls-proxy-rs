use std::usize;

#[derive(Debug)]
struct DNSHeader {
    query_id: u16,
    flags: Flags,
    question_count: u16,
    answer_count: u16,
    name_server_count: u16,
    additional_record_count: u16,
}

#[derive(Debug)]
struct Flags {
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
}

#[derive(Debug)]
struct DNSQuestion {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

#[derive(Debug)]
struct DNSRecord {
    name: Vec<u8>,
    rtype: u16,
    rclass: u16,
    ttl: u32,
    data_length: u16,
    rdata: Vec<u8>,
}

#[derive(Debug)]
pub struct DNSMessage {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSRecord>,
    name_servers: Vec<DNSRecord>,
    additional_records: Vec<DNSRecord>,
}

impl DNSMessage {
    pub fn new() -> DNSMessage {
        DNSMessage {
            header: DNSHeader {
                query_id: 0,
                flags: Flags {
                    qr: 0,
                    opcode: 0,
                    aa: 0,
                    tc: 0,
                    rd: 0,
                    ra: 0,
                    z: 0,
                    rcode: 0,
                },
                question_count: 0,
                answer_count: 0,
                name_server_count: 0,
                additional_record_count: 0,
            },
            questions: Vec::new(),
            answers: Vec::new(),
            name_servers: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn pack() -> Vec<u8> {
        todo!()
    }

    pub fn unpack(msg: &[u8], dns_message: &mut DNSMessage) {
        let question_count = bytes_to_u16(&msg[4..6]);
        let answer_count = bytes_to_u16(&msg[6..8]);
        let name_server_count = bytes_to_u16(&msg[8..10]);
        let additional_record_count = bytes_to_u16(&msg[10..12]);

        let mut bit_position = 12;

        *dns_message = DNSMessage {
            header: DNSHeader {
                query_id: bytes_to_u16(&msg[0..2]),
                flags: Flags {
                    qr: (msg[2] >> 7) & 0x01,
                    opcode: (msg[2] >> 3) & 0x0F,
                    aa: (msg[2] >> 2) & 0x01,
                    tc: (msg[2] >> 1) & 0x01,
                    rd: msg[2] & 0x01,
                    ra: (msg[3] >> 7) & 0x01,
                    z: (msg[3] >> 4) & 0x07,
                    rcode: msg[3] & 0x0F,
                },
                question_count,
                answer_count,
                name_server_count,
                additional_record_count,
            },
            questions: get_questions(msg, &mut bit_position, question_count),
            answers: get_answers(msg, &mut bit_position, answer_count),
            name_servers: get_name_servers(msg, &mut bit_position, name_server_count),
            additional_records: get_additional_records(
                msg,
                &mut bit_position,
                additional_record_count,
            ),
        };
    }
}

fn bytes_to_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn get_questions(msg: &[u8], bit_position: &mut usize, question_count: u16) -> Vec<DNSQuestion> {
    let mut questions = Vec::new();
    let qname = get_data(msg, bit_position);
    let qtype = &msg[*bit_position..*bit_position + 2];
    *bit_position += 2;
    let qclass = &msg[*bit_position..*bit_position + 2];
    *bit_position += 2;
    questions.push(DNSQuestion {
        qname,
        qtype: bytes_to_u16(qtype),
        qclass: bytes_to_u16(qclass),
    });
    questions
}

fn get_answers(msg: &[u8], bit_position: &mut usize, question_count: u16) -> Vec<DNSRecord> {
    Vec::<DNSRecord>::new()
}

fn get_name_servers(msg: &[u8], bit_position: &mut usize, question_count: u16) -> Vec<DNSRecord> {
    Vec::<DNSRecord>::new()
}

fn get_additional_records(
    msg: &[u8],
    bit_position: &mut usize,
    question_count: u16,
) -> Vec<DNSRecord> {
    Vec::<DNSRecord>::new()
}

fn get_data(msg: &[u8], bit_position: &mut usize) -> Vec<u8> {
    let mut qname = Vec::<u8>::new();

    loop {
        let len = msg[*bit_position] as usize;
        qname.push(msg[*bit_position]);
        *bit_position += 1;

        if len == 0 {
            qname.push(0x00);
            break;
        }

        qname.extend_from_slice(&msg[*bit_position..*bit_position + len]);
        *bit_position += len;
    }

    qname
}
