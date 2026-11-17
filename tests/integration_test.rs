// Integration test for SigmaOS-amazon-prime-dl
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use amazon_prime_dl as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
