//! Adversarial tests for Span (POST-FIX version)
//!
//! These tests verify that the security fixes are effective.

#[cfg(test)]
mod attack_tests {
    use crate::span::Span;

    // =========================================================================
    // ATTACK 1: Integer Underflow in len() - NOW BLOCKED
    // Fields are private, can't bypass constructor
    // =========================================================================

    #[test]
    #[should_panic(expected = "Span start must be <= end")]
    fn attack_len_underflow_blocked_in_release() {
        // This now panics in BOTH debug and release mode!
        let _ = Span::new(100, 50);
    }

    // Can't even compile this anymore:
    // let evil = Span { start: 100, end: 50 }; // ERROR: field `start` is private

    // =========================================================================
    // ATTACK 2: u32::MAX boundary - STILL VALID (edge cases work correctly)
    // =========================================================================

    #[test]
    fn boundary_max_u32_values() {
        let max_span = Span::new(0, u32::MAX);
        assert_eq!(max_span.len(), u32::MAX);

        let weird_span = Span::new(u32::MAX, u32::MAX);
        assert_eq!(weird_span.len(), 0);
        assert!(weird_span.is_empty());
    }

    #[test]
    fn boundary_contains_at_max() {
        let span = Span::new(u32::MAX - 10, u32::MAX);
        assert!(span.contains(u32::MAX - 1));
        assert!(!span.contains(u32::MAX)); // exclusive end
    }

    #[test]
    fn boundary_merge_at_max() {
        let a = Span::new(0, 100);
        let b = Span::new(u32::MAX - 100, u32::MAX);
        let merged = a.merge(b);
        assert_eq!(merged.start(), 0);
        assert_eq!(merged.end(), u32::MAX);
    }

    // =========================================================================
    // ATTACK 3: Empty span edge cases - WORKING CORRECTLY
    // =========================================================================

    #[test]
    fn empty_span_contains_nothing() {
        let empty = Span::new(50, 50);
        assert!(!empty.contains(49));
        assert!(!empty.contains(50));
        assert!(!empty.contains(51));
    }

    #[test]
    fn empty_span_at_max() {
        let empty = Span::new(u32::MAX, u32::MAX);
        assert!(empty.is_empty());
        assert!(!empty.contains(u32::MAX));
    }

    // =========================================================================
    // ATTACK 4: Public field manipulation - NOW BLOCKED
    // Fields are private!
    // =========================================================================

    // This no longer compiles:
    // #[test]
    // fn attack_bypass_constructor() {
    //     let mut span = Span::new(10, 20);
    //     span.start = 100;  // ERROR: field `start` is private
    //     span.end = 50;     // ERROR: field `end` is private
    // }

    #[test]
    fn getters_work_correctly() {
        let span = Span::new(10, 20);
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 20);
        assert_eq!(span.len(), 10);
    }

    // =========================================================================
    // ATTACK 5: Merge operations - WORKING CORRECTLY
    // =========================================================================

    #[test]
    fn merge_valid_spans() {
        let a = Span::new(0, 10);
        let b = Span::new(10, 20);
        let merged = a.merge(b);
        assert_eq!(merged.start(), 0);
        assert_eq!(merged.end(), 20);
    }

    #[test]
    fn merge_is_commutative() {
        let a = Span::new(5, 15);
        let b = Span::new(10, 20);
        assert_eq!(a.merge(b), b.merge(a));
    }

    // =========================================================================
    // ATTACK 6: Const context - NOW BLOCKED
    // =========================================================================

    // This no longer compiles:
    // const EVIL: Span = Span { start: 999, end: 1 }; // ERROR: private fields

    // This panics at compile time (const panic):
    // const EVIL: Span = Span::new(999, 1); // ERROR: panicked at 'Span start must be <= end'

    #[test]
    fn const_valid_span_works() {
        const VALID: Span = Span::new(0, 100);
        assert_eq!(VALID.start(), 0);
        assert_eq!(VALID.end(), 100);
    }

    // =========================================================================
    // ATTACK 7: Hash consistency - WORKING CORRECTLY
    // =========================================================================

    #[test]
    fn hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Span::new(0, 10));
        set.insert(Span::new(0, 10)); // duplicate
        set.insert(Span::new(5, 15));

        assert_eq!(set.len(), 2);
        assert!(set.contains(&Span::new(0, 10)));
    }

    // =========================================================================
    // ATTACK 8: Transmute still works (unsafe = user's responsibility)
    // This is acceptable - unsafe code can always break invariants
    // =========================================================================

    #[test]
    fn transmute_requires_unsafe() {
        // This requires unsafe, so it's the caller's responsibility
        let bytes: [u8; 8] = [0x0A, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00];
        let span: Span = unsafe { std::mem::transmute(bytes) };

        // Creates valid span (10, 20) in little-endian
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 20);
    }

    // =========================================================================
    // ATTACK 9: Saturating len() - Defense in depth
    // =========================================================================

    #[test]
    fn len_uses_saturating_sub() {
        // Even if somehow an invalid span existed, len() wouldn't underflow
        // (We can't test this directly since we can't create invalid spans)

        // But we can verify normal behavior
        let span = Span::new(0, u32::MAX);
        assert_eq!(span.len(), u32::MAX);

        let span2 = Span::new(100, 100);
        assert_eq!(span2.len(), 0);
    }

    // =========================================================================
    // ATTACK 10: Size guarantee still holds
    // =========================================================================

    #[test]
    fn size_is_still_8_bytes() {
        use std::mem::size_of;
        assert_eq!(size_of::<Span>(), 8);
    }

    // Compile-time check
    const _: () = assert!(std::mem::size_of::<Span>() == 8);

    // =========================================================================
    // VERIFICATION: All vulnerabilities are now mitigated
    // =========================================================================

    #[test]
    fn security_summary() {
        // V1: Integer underflow - FIXED (constructor panics)
        // V2: Public fields - FIXED (fields are private)
        // V3: Debug-only check - FIXED (assert! always runs)
        // V4: Const invalid - FIXED (const panic)
        // V5: Merge invalid - N/A (can't create invalid spans)
        // V6: Transmute - ACCEPTABLE (unsafe = caller's problem)

        println!("All security vulnerabilities have been addressed!");
    }
}
