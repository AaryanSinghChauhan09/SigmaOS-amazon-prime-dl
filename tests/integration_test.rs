//! Integration tests for SigmaOS-amazon-prime-dl.

use sigmaos_amazon_prime_dl_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
