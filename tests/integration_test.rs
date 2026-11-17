// Integration test for SigmaOS-7-days-to-die
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_7_days_to_die as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
