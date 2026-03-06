use secure_patterns::CapabilityGrant;

fn main() {
    let grant = CapabilityGrant::new("packet-sensor", ["inspect_packet", "emit_alert"]);
    grant
        .require("inspect_packet")
        .expect("least-authority permit");

    println!(
        "{} can inspect packets without broad mutation rights",
        grant.service()
    );
}
