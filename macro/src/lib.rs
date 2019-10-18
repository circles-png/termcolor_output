//! Wrapper crate for [`termcolor_output_impl`] procedural macro.
//! 
//! The reason for this code to be split into two crates is simple: we want to make
//! this functionality available on stable. In fact, this dual-crate system is simply
//! the manual implementation of the code generated by [`proc_macro_hack`].
//!
//! ## What is it
//! 
//! The [`termcolor`] crate is a cross-platform implementation for the different console
//! APIs, abstracting away both Linux terminals and Windows consoles. It has, however,
//! a but cumbersome API itself (only a bit though), since for formatting-heavy parts
//! we have to litter our code with explicit styling commands. This crate allows to
//! abstract these things away, providing the interface similar to the standard [`write!`]
//! macro.
//!  
//! [`termcolor_output_impl`]: http://crates.io/crates/termcolor_output_impl
//! [`proc_macro_hack`]: http://github.com/dtolnay/proc-macro-hack
//! [`termcolor`]: http://docs.rs/termcolor
//! [`write!`]: https://doc.rust-lang.org/stable/std/macro.write.html

trait ColoredOutput {}

#[doc(hidden)]
pub fn guard(w: &mut impl termcolor::WriteColor) -> &mut impl termcolor::WriteColor {
    w
}

#[doc(hidden)]
pub trait WriteColorGuard {
    fn guard(&mut self) -> &mut Self {
        self
    }
}
impl<T: termcolor::WriteColor> WriteColorGuard for T {} 

/// The macro writing colored text.
/// 
/// Like the standard [`write!`] macro, it takes the writer, 
/// 
/// ## Examples
/// 
/// Simple formatting is provided in exactly the same way as for standard writes:
/// ```
/// use termcolor_output::colored;
/// fn write_simple(writer: &mut impl termcolor::WriteColor) {
///     colored!(writer, "This text is {} styled", "not").unwrap();
/// }
/// ```
#[macro_export]
macro_rules! colored {
    ($($arg:tt)*) => {{
        use termcolor_output_impl::ColoredOutput;
        use termcolor::WriteColor;
        use std::io::Write;
        use $crate::WriteColorGuard;
        #[derive(ColoredOutput)]
        enum __Writer {
            data = (stringify!($($arg)*), 0).1
        }
        colored_impl!()
    }}
}
