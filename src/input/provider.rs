use regex::Captures;

#[derive(Debug, PartialEq)]
pub enum Capture {
    Index(usize),
    Name(String),
}

/// A source of values for the placeholders appearing in a format string.
///
/// An implementation resolves each placeholder to a string; a placeholder it
/// cannot resolve is omitted from the generated output.
pub trait Provider {
    /// Resolve an indexed placeholder such as `{}` or `{1}`.
    fn provide_index(&self, index: usize) -> Option<&str>;

    /// Resolve a named placeholder such as `{name}`.
    ///
    /// The default implementation leaves named placeholders unresolved.
    fn provide_name(&self, _name: &str) -> Option<&str> {
        None
    }

    /// Resolve `cap`, dispatching on the placeholder kind.
    fn provide(&self, cap: &Capture) -> Option<&str> {
        match cap {
            Capture::Index(index) => self.provide_index(*index),
            Capture::Name(name) => self.provide_name(name.as_str()),
        }
    }
}

impl Provider for Captures<'_> {
    fn provide_index(&self, index: usize) -> Option<&str> {
        self.get(index).map(|m| m.as_str())
    }

    fn provide_name(&self, name: &str) -> Option<&str> {
        self.name(name).map(|m| m.as_str())
    }
}

impl Provider for Vec<&'_ str> {
    fn provide_index(&self, index: usize) -> Option<&str> {
        self.get(index).copied()
    }

    fn provide_name(&self, name: &str) -> Option<&str> {
        self.provide_index(name.parse().unwrap())
    }
}

impl From<&str> for Capture {
    fn from(value: &str) -> Self {
        if let Ok(index) = value.parse() {
            Capture::Index(index)
        } else {
            Capture::Name(value.into())
        }
    }
}
