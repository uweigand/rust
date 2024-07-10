// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ test-mir-pass: GVN

// EMIT_MIR mutable_variable_unprop_assign.main.GVN.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: debug a => [[a:_.*]];
    // CHECK: debug x => [[x:_.*]];
    // CHECK: debug y => [[y:_.*]];
    // CHECK: debug z => [[z:_.*]];
    // CHECK: [[a]] = foo()
    // CHECK: [[x]] = const (285212689_i32, 570425378_i32);
    // CHECK: ([[x]].1: i32) = [[a]];
    // CHECK: [[y]] = ([[x]].1: i32);
    // CHECK: [[z]] = ([[x]].0: i32);
    let a = foo();
    let mut x: (i32, i32) = (0x11000011, 0x22000022); // Endian-invariant values.
    x.1 = a;
    let y = x.1;
    let z = x.0;
}

#[inline(never)]
fn foo() -> i32 {
    unimplemented!()
}
