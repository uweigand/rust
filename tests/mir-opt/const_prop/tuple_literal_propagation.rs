//@ test-mir-pass: GVN
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR tuple_literal_propagation.main.GVN.diff

fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: = consume(const (285212689_u32, 570425378_u32))
    let x = (0x11000011, 0x22000022); // Endian-invariant values.
    consume(x);
}

#[inline(never)]
fn consume(_: (u32, u32)) {}
