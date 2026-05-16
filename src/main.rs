fn parse_peer_address (input: &str) -> Result<(String, u16), String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid Address: {}", input));
    }
    let port: u16 = parts[1]
        .parse()
        .map_err(|_| format!("invalid port: {}",parts[1]))?;
    Ok((parts[0].to_string(), port))
}

fn connect_to_peer(address: &str) -> Result<String, String> {
    let (host, port) = parse_peer_address(address)?;
    Ok(format!("Connected on {}:{}", host, port))
}

trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
}

struct NetworkPacket {
    sender_id: [u8; 32],
    payload: Vec<u8>,
}

struct PeerId([u8; 32]);

impl Serializable for NetworkPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.sender_id);
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}

impl Serializable for PeerId {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.0);
        bytes
    }
}

enum RoutingDecision {
    Forward { next_hop: [u8; 32]},
    Deliver,
    Drop(String),
    Redirect(String),
}

fn describe_decision (d: &RoutingDecision) {

    match d {
        RoutingDecision::Forward {next_hop}=> println!("Forwarding to {:02x?}", &next_hop[..4]),
        RoutingDecision::Deliver => println!("Delivered to local node"),
        RoutingDecision::Drop(reason) => println!("Dropping reason: {}", reason),
        RoutingDecision::Redirect(reason) => return,
    }

}

fn main() {

    let packet = NetworkPacket { sender_id: [1u8; 32], payload: vec![10, 20, 30] };
    let peer = PeerId([2u8; 32]);
    println!("NetworkPacket bytes: {}", packet.to_bytes().len());
    println!("PeerId bytes: {}", peer.to_bytes().len());

    connect_to_peer("127.0.0.1:9001");
    connect_to_peer("not an address");

    match connect_to_peer("127.0.0.1:9001") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }

    match connect_to_peer("hehe") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}
