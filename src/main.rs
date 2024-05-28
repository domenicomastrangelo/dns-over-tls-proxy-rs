use dns::dns::DNSMessage;

mod cache;
mod config;
mod dns;
mod proxy;

fn main() {
    tracing_subscriber::fmt::try_init().expect("Failed to initialize the logger");

    let conf = config::config::new(
        "localhost".to_string(),
        "853".to_string(),
        "/etc/ssl/certs/ca-certificates.crt".to_string(),
        "localhost".to_string(),
        "53".to_string(),
    );

    conf.check();

    let mut dns_message = DNSMessage::new();

    let msg: [u8; 29] = [
        // Header (12 bytes)
        0x12, 0x34, // ID
        0x01, 0x00, // Flags
        0x00, 0x01, // QDCOUNT
        0x00, 0x00, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT
        // Question section (21 bytes)
        0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // QNAME: "example"
        0x03, b'c', b'o', b'm', // QNAME: "com"
        0x00, // End of QNAME
        0x00, 0x01, // QTYPE: A
        0x00, 0x01, // QCLASS: IN
    ];

    DNSMessage::unpack(&msg, &mut dns_message);

    println!("{:?}", dns_message);
}
