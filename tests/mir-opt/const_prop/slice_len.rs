//@ test-mir-pass: GVN
//@ compile-flags: -Zmir-enable-passes=+InstSimplify
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR_FOR_EACH_BIT_WIDTH

// EMIT_MIR slice_len.main.GVN.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: debug a => [[a:_.*]];
    // CHECK: [[slice:_.*]] = const {{.*}} as &[u32] (PointerCoercion(Unsize));
    // CHECK: assert(const true,
    // CHECK: [[a]] = const 570425378_u32;
    let a = (&[0x11000011u32, 0x22000022, 0x33000033] as &[u32])[1]; // Endian-invariant values.
}
