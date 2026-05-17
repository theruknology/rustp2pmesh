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

// Closure and Iterators
// It takes a slice of (address, ping_ms) pairs and returns only the peers with ping under 50ms, sorted by ping ascending, formatted as "192.168.1.1 (32ms)"

fn fast_peers(peers : &[(String, u32)]) -> Vec<String> {
    let mut results: Vec<(u32, &String)> = peers
        .iter()
        .filter(|p| p.1 < 50)
        .map(|p| (p.1, &p.0))
        .collect();

    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
        .iter()
        .map(|(ping, addr)| format!("{} ({})ms", addr, ping))
        .collect()
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

    let peers = vec![
        (String::from("192.168.1.1"), 32u32),
        (String::from("192.168.1.2"), 80u32),
        (String::from("192.168.1.3"), 15u32),
        (String::from("192.168.1.4"), 120u32),
        (String::from("192.168.1.5"), 44u32),
    ];

    let results = fast_peers(&peers);

    for p in &results {
        println!("{}", p);
    }
}
