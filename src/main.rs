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

    let my_data : [u8; 32] = [0; 32];
    let s = String::from("hehe");
    describe_decision(&RoutingDecision::Deliver);
    describe_decision(&RoutingDecision::Forward{next_hop:my_data});
    describe_decision(&RoutingDecision::Drop(s));

}
