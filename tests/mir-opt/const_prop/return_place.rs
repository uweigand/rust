//@ test-mir-pass: GVN
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ compile-flags: -C overflow-checks=on

// EMIT_MIR return_place.add.GVN.diff
// EMIT_MIR return_place.add.PreCodegen.before.mir
fn add() -> u32 {
    // CHECK-LABEL: fn add(
    // CHECK: _0 = const 570425378_u32;
    0x11000011 + 0x11000011 // Endian-invariant value.
}

fn main() {
    add();
}
