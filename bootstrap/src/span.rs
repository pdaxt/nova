//! Source location tracking for Nova
//!
//! The `Span` type represents a range in source code. It is designed to be
//! exactly 8 bytes (two u32 fields) for efficient storage in tokens.
//!
//! # Design Decision (ADR-002)
//!
//! We use `u32` instead of `usize` because:
//! - 4 billion byte limit is sufficient (most source files are <1MB)
//! - Halves memory usage compared to `usize` on 64-bit systems
//! - Enables Token to fit in 12 bytes (critical for cache efficiency)

use std::fmt;

/// A span representing a range in source code.
///
/// Invariant: `start <= end` (enforced at construction)
///
/// # Size Guarantee
///
/// This struct is guaranteed to be exactly 8 bytes:
/// - `start: u32` = 4 bytes
/// - `end: u32` = 4 bytes
///
/// This is enforced by compile-time assertions in tests.
///
/// # Safety
///
/// Fields are private to enforce the `start <= end` invariant.
/// Use `start()` and `end()` getters to access values.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]  // Ensure predictable layout
pub struct Span {
    /// Byte offset of the first character (inclusive)
    start: u32,
    /// Byte offset after the last character (exclusive)
    end: u32,
}

impl Span {
    /// Creates a new span from start (inclusive) to end (exclusive).
    ///
    /// # Panics
    ///
    /// Panics if `start > end`.
    #[inline]
    pub const fn new(start: u32, end: u32) -> Self {
        assert!(start <= end, "Span start must be <= end");
        Self { start, end }
    }

    /// Returns the start offset (inclusive).
    #[inline]
    pub const fn start(&self) -> u32 {
        self.start
    }

    /// Returns the end offset (exclusive).
    #[inline]
    pub const fn end(&self) -> u32 {
        self.end
    }

    /// Returns the length of this span in bytes.
    ///
    /// Uses saturating subtraction as defense-in-depth, though
    /// the invariant guarantees `end >= start`.
    #[inline]
    pub const fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Returns true if the span has zero length.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns true if this span contains the given byte offset.
    ///
    /// The check is `start <= offset < end` (half-open interval).
    #[inline]
    pub const fn contains(&self, offset: u32) -> bool {
        self.start <= offset && offset < self.end
    }

    /// Merges two spans, returning a span that covers both.
    ///
    /// The resulting span starts at the minimum start and ends at the maximum end.
    #[inline]
    pub const fn merge(self, other: Span) -> Span {
        let start = if self.start < other.start { self.start } else { other.start };
        let end = if self.end > other.end { self.end } else { other.end };
        Span { start, end }
    }

    /// Creates a dummy span (0, 0) for synthetic nodes.
    #[inline]
    pub const fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn span_is_8_bytes() {
        // ADR-002: Span MUST be exactly 8 bytes
        assert_eq!(size_of::<Span>(), 8, "Span must be exactly 8 bytes");
    }

    #[test]
    fn span_new() {
        let span = Span::new(10, 20);
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 20);
    }

    #[test]
    fn span_len() {
        assert_eq!(Span::new(0, 0).len(), 0);
        assert_eq!(Span::new(0, 10).len(), 10);
        assert_eq!(Span::new(5, 15).len(), 10);
        assert_eq!(Span::new(100, 200).len(), 100);
    }

    #[test]
    fn span_is_empty() {
        assert!(Span::new(0, 0).is_empty());
        assert!(Span::new(42, 42).is_empty());
        assert!(!Span::new(0, 1).is_empty());
    }

    #[test]
    fn span_contains() {
        let span = Span::new(10, 20);

        // Before span
        assert!(!span.contains(9));

        // At start (inclusive)
        assert!(span.contains(10));

        // Inside span
        assert!(span.contains(15));

        // At end - 1
        assert!(span.contains(19));

        // At end (exclusive)
        assert!(!span.contains(20));

        // After span
        assert!(!span.contains(21));
    }

    #[test]
    fn span_merge() {
        // Adjacent spans
        let a = Span::new(0, 10);
        let b = Span::new(10, 20);
        assert_eq!(a.merge(b), Span::new(0, 20));

        // Overlapping spans
        let c = Span::new(5, 15);
        let d = Span::new(10, 20);
        assert_eq!(c.merge(d), Span::new(5, 20));

        // Nested spans
        let e = Span::new(0, 100);
        let f = Span::new(20, 30);
        assert_eq!(e.merge(f), Span::new(0, 100));

        // Same span
        let g = Span::new(10, 20);
        assert_eq!(g.merge(g), Span::new(10, 20));

        // Order doesn't matter (commutative)
        assert_eq!(a.merge(b), b.merge(a));
    }

    #[test]
    fn span_dummy() {
        let dummy = Span::dummy();
        assert_eq!(dummy.start(), 0);
        assert_eq!(dummy.end(), 0);
        assert!(dummy.is_empty());
    }

    #[test]
    fn span_debug_format() {
        let span = Span::new(10, 20);
        assert_eq!(format!("{:?}", span), "10..20");
    }

    #[test]
    fn span_display_format() {
        let span = Span::new(100, 200);
        assert_eq!(format!("{}", span), "100..200");
    }

    #[test]
    fn span_copy_semantics() {
        let a = Span::new(0, 10);
        let b = a;  // Copy
        assert_eq!(a, b);
        assert_eq!(a.start(), b.start());
    }

    #[test]
    fn span_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Span::new(0, 10));
        set.insert(Span::new(0, 10));  // Duplicate
        set.insert(Span::new(5, 15));

        assert_eq!(set.len(), 2);
        assert!(set.contains(&Span::new(0, 10)));
    }

    // Compile-time size assertion (fails at compile time if wrong)
    const _: () = assert!(size_of::<Span>() == 8);
}
