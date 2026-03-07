// ==========================================================
// ZERO-SIZED TYPES (ZST)
// ==========================================================

// Zero-sized types occupy no memory. They can be used to encode
// type information or state without runtime cost.
//
// Primary purpose:
// - Encode type info or represent a "unit of behavior" without storage.
// Use cases:
// - Marker types, compile-time state machines
struct Empty;

fn zst_example() {
    let _e = Empty; // Zero runtime footprint
}
