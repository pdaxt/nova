# Span Security Audit Report

**Component:** `bootstrap/src/span.rs`
**Auditor:** Adversarial Testing
**Date:** 2026-01-04
**Severity:** Medium (correctness issues, no memory safety violations)

## Executive Summary

The Span implementation has **6 confirmed vulnerabilities** that can cause incorrect behavior in release builds. The core issue is that the `start <= end` invariant is only enforced by `debug_assert!` and public fields allow bypassing the constructor entirely.

## Vulnerabilities Found

### 🔴 V1: Integer Underflow in `len()` (CONFIRMED)

**Severity:** Medium
**Attack Vector:** Direct field manipulation or release-mode constructor

```rust
// Attack: Create invalid span with public fields
let evil = Span { start: 100, end: 50 };
let len = evil.len();  // Returns 4294967246 (underflow!)
```

**Impact:** `len()` returns garbage value when `start > end`. Any code using `len()` for allocation or iteration will behave incorrectly.

**Root Cause:** No runtime check in `len()`:
```rust
pub const fn len(&self) -> u32 {
    self.end - self.start  // Underflows if start > end!
}
```

---

### 🔴 V2: Public Fields Bypass Constructor Invariant (CONFIRMED)

**Severity:** Medium
**Attack Vector:** Direct struct initialization

```rust
// Attack: Bypass new() entirely
let evil = Span { start: 999, end: 1 };  // Compiles fine!
```

**Impact:** The documented invariant `start <= end` can be trivially violated.

**Root Cause:** Fields are `pub`:
```rust
pub struct Span {
    pub start: u32,  // Should be private!
    pub end: u32,    // Should be private!
}
```

---

### 🔴 V3: Debug-Only Invariant Check (CONFIRMED)

**Severity:** Medium
**Attack Vector:** Release builds

```rust
// In release mode, this creates invalid span WITHOUT panic
let span = Span::new(100, 50);  // No panic in release!
```

**Impact:** Invalid spans can be created through the official constructor in release builds.

**Root Cause:** Using `debug_assert!` instead of runtime check:
```rust
pub const fn new(start: u32, end: u32) -> Self {
    debug_assert!(start <= end);  // Only runs in debug mode!
    Self { start, end }
}
```

---

### 🟡 V4: Const Context Allows Compile-Time Invalid Spans (CONFIRMED)

**Severity:** Low
**Attack Vector:** Const initialization

```rust
// Compiles! Invalid span at compile time.
const EVIL: Span = Span { start: 999, end: 1 };
```

**Impact:** Invalid spans can be embedded in the binary.

---

### 🟡 V5: Merge Propagates Invalid State (CONFIRMED)

**Severity:** Low
**Attack Vector:** Merging two invalid spans

```rust
let a = Span { start: 200, end: 100 }; // invalid
let b = Span { start: 300, end: 150 }; // invalid
let merged = a.merge(b);
// Result: Span { start: 200, end: 150 } - STILL INVALID!
```

**Impact:** Invalid spans remain invalid after merge operations.

---

### 🟡 V6: Transmute/Unsafe Creates Invalid Spans (CONFIRMED)

**Severity:** Low (requires unsafe)
**Attack Vector:** Memory manipulation

```rust
let bytes: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0];
let evil: Span = unsafe { std::mem::transmute(bytes) };
// Span { start: u32::MAX, end: 0 }
```

**Impact:** Any unsafe code can create invalid spans.

---

## Recommended Fixes

### Fix 1: Make Fields Private + Add Getters

```rust
pub struct Span {
    start: u32,  // Private!
    end: u32,    // Private!
}

impl Span {
    pub const fn start(&self) -> u32 { self.start }
    pub const fn end(&self) -> u32 { self.end }
}
```

**Trade-off:** Breaking change for code that accesses fields directly.

### Fix 2: Runtime Check in Constructor

```rust
pub const fn new(start: u32, end: u32) -> Self {
    assert!(start <= end, "Span start must be <= end");
    Self { start, end }
}
```

**Trade-off:** Small runtime cost, but ensures correctness.

### Fix 3: Checked `len()` Method

```rust
pub const fn len(&self) -> u32 {
    // Returns 0 for invalid spans instead of underflowing
    self.end.saturating_sub(self.start)
}
```

**Trade-off:** Masks bugs rather than failing fast.

### Fix 4: Fallible Constructor

```rust
pub const fn try_new(start: u32, end: u32) -> Option<Self> {
    if start <= end {
        Some(Self { start, end })
    } else {
        None
    }
}
```

**Trade-off:** More verbose call sites.

---

## Recommendation

**Minimum fix (preserving public API):**
1. Change `debug_assert!` to `assert!` in `new()`
2. Use `saturating_sub` in `len()` as defense in depth

**Ideal fix (breaking change):**
1. Make fields private
2. Add `start()` and `end()` getters
3. Runtime assert in constructor
4. Document that `Span` values are always valid

---

## Test Coverage

The attack tests are in `bootstrap/src/span_attack.rs` and verify:
- 6 attack vectors work in release mode
- debug_assert catches issues in debug mode (partial protection)
- Memory safety is preserved (Rust prevents UB)

## Conclusion

The current implementation prioritizes **performance** (no runtime checks) over **correctness** (invalid states possible). For a compiler, correctness should take priority. The recommended minimum fix adds negligible overhead while preventing subtle bugs.
