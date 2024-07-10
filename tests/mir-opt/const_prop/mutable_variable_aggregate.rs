//@ test-mir-pass: GVN

// EMIT_MIR mutable_variable_aggregate.main.GVN.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: debug y => [[y:_.*]];
    // CHECK: [[x]] = const (285212689_i32, 570425378_i32);
    // CHECK: ([[x]].1: i32) = const 99_i32;
    // CHECK: [[y]] = [[x]];
    let mut x = (0x11000011, 0x22000022); // Endian-invariant values.
    x.1 = 99;
    let y = x;
}
