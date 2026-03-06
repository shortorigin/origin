use exploit_mitigation::{parse_policy_line, ParserLimits};

fn main() {
    let line = parse_policy_line("runtime:rotate_component", ParserLimits::default())
        .expect("bounded policy line");

    println!(
        "parsed hardening policy {} -> {}",
        line.subject, line.action
    );
}
