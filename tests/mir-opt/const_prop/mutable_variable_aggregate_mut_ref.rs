//@ test-mir-pass: GVN

// EMIT_MIR mutable_variable_aggregate_mut_ref.main.GVN.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: debug x => [[x:_.*]];
    // CHECK: debug z => [[z:_.*]];
    // CHECK: debug y => [[y:_.*]];
    // CHECK: [[x]] = const (285212689_i32, 570425378_i32);
    // CHECK: [[z]] = &mut [[x]];
    // CHECK: ((*[[z]]).1: i32) = const 99_i32;
    // CHECK: [[y]] = [[x]];
    let mut x = (0x11000011, 0x22000022); // Endian-invariant values.
    let z = &mut x;
    z.1 = 99;
    let y = x;
}
