//! Tests for the `Signal` trait.

extern crate sample;

use sample::Signal;

#[test]
fn test_equilibrium() {
    let equilibrium: Vec<[i8; 1]> = sample::signal::equilibrium().take(4).collect();
    assert_eq!(equilibrium, vec![[0], [0], [0], [0]]);
}

#[test]
fn test_scale_amp() {
    let foo = [[0.5], [0.8], [-0.4], [-0.2]];
    let amp = 0.5;
    let amp_scaled: Vec<_> = foo.iter().cloned().scale_amp(amp).collect();
    assert_eq!(amp_scaled, vec![[0.25], [0.4], [-0.2], [-0.1]]);
}

#[test]
fn test_offset_amp() {
    let foo = [[0.5], [0.9], [-0.4], [-0.2]];
    let amp = -0.5;
    let amp_offset: Vec<_> = foo.iter().cloned().offset_amp(amp).collect();
    assert_eq!(amp_offset, vec![[0.0], [0.4], [-0.9], [-0.7]]);
}
