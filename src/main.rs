//! SigmaOS-7-days-to-die — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_7_days_to_die_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-7-days-to-die: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-7-days-to-die: ok");
}
