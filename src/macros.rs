/// Generate a new id type
#[macro_export]
macro_rules! id {
    ($id:ident) => {
        /// Identifier
        ///
        /// Resources on GitHub have a unique `id` that is used to interact with them through
        /// GitHub's REST API.
        #[derive(
            Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize,
        )]
        pub struct $id(u64);

        impl $id {
            /// Initializes a new identifier.
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            /// Returns the identifier.
            pub fn get(&self) -> u64 {
                self.0
            }
        }

        impl Display for $id {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

/// Generate a new name type
#[macro_export]
macro_rules! name {
    ($name:ident) => {
        /// Name
        ///
        /// Resources on GitHub can have a human-readable name that identifies them.
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
        pub struct $name(String);

        impl $name {
            /// Initializes a new name.
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

            /// Returns a string representation of the name.
            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
