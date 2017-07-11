//! Provides a way to quickly yet unsafely unwrap types whose inner values are
//! known to exist.
//!
//! By calling `unsafe_unwrap()`, the compiler is told, in optimized builds,
//! that the unwrap will never fail. In debug builds it will emit a panic.
//!
//! Sometimes the optimizer can remove checked unwrapping if it can prove that
//! a value exists. However, in times that it may not be able to do so, this
//! works as an alternative.
//!
//! This is akin to the `unsafelyUnwrapped` property of `Optional` in Swift.
//!
//! # Examples
//!
//! ```rust
//! use unsafe_unwrap::UnsafeUnwrap;
//!
//! let x = Some(20);
//! let y = unsafe { x.unsafe_unwrap() };
//! ```

#![no_std]

/// A type whose instances can be unsafely unwrapped without checking.
///
/// Calling `unsafe_unwrap()` over `unwrap()` should remove panicking code
/// related to checked unwrapping in optimized builds.
pub trait UnsafeUnwrap<T> {
    /// Unsafely moves the inner value out of `self` without checking.
    ///
    /// # Safety
    ///
    /// This method trades safety for performance. Only use it when a wrapped
    /// value is known to exist. Otherwise, use `unwrap()` or pattern matching.
    unsafe fn unsafe_unwrap(self) -> T;
}

#[inline(always)]
unsafe fn unreachable() -> ! {
    if cfg!(debug_assertions) {
        unreachable!()
    } else {
        use core::mem::transmute;
        struct ZeroSized;
        enum Impossible {}
        match transmute::<_, Impossible>(ZeroSized) {}
    }
}

impl<T> UnsafeUnwrap<T> for Option<T> {
    #[inline]
    unsafe fn unsafe_unwrap(self) -> T {
        if let Some(x) = self { x } else { unreachable() }
    }
}

impl<T, E> UnsafeUnwrap<T> for Result<T, E> {
    #[inline]
    unsafe fn unsafe_unwrap(self) -> T {
        if let Ok(x) = self { x } else { unreachable() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_unwrap_success() {
        unsafe {
            let x: Option<_> = Some(0);
            x.unsafe_unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn option_unwrap_failure() {
        unsafe {
            let x: Option<()> = None;
            x.unsafe_unwrap();
        }
    }

    #[test]
    fn result_unwrap_success() {
        unsafe {
            let x: Result<_, ()> = Ok(0);
            x.unsafe_unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn result_unwrap_failure() {
        unsafe {
            let x: Result<(), _> = Err(0);
            x.unsafe_unwrap();
        }
    }
}
