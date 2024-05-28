use crate::config::config::Config;

const MAX_BYTES_PER_UDP_PACKET: usize = 4096;

fn start_upd_server(cnf: &Config) -> Result<(), std::io::Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:53")?;
    loop {
        let mut buf = [0; MAX_BYTES_PER_UDP_PACKET];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let request = &buf[..number_of_bytes];

        let response = handle_udp_connection(cnf, request);
    }
}

fn handle_udp_connection(cnf: &Config, request: &[u8]) {
    todo!()
}

fn forward_dns_query(cnf: &Config, msg: &str) {
    todo!()
}
