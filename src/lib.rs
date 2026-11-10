pub mod cli;

pub mod input {
    //! Format parsing and input iteration.
    //!
    //! A [`Formatter`] turns a format string into a sequence of segments, which
    //! are rendered against a [`Provider`] to produce the output names.
    mod formatter;
    mod iterator;
    mod provider;
    mod separator;
    mod source;
    pub use self::formatter::*;
    pub use self::iterator::*;
    pub use self::provider::*;
    pub use self::separator::*;
    pub use self::source::*;
}

pub mod errors {
    //! Error types surfaced by the command-line interface.
    mod format;
    mod source;
    pub use self::format::*;
    pub use self::source::*;
}
