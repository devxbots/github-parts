#[macro_export]
macro_rules! id {
    ($id:ident) => {
        #[derive(
            Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize,
        )]
        pub struct $id(u64);

        impl $id {
            pub fn new(id: u64) -> Self {
                Self(id)
            }

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

#[macro_export]
macro_rules! name {
    ($name:ident) => {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
        pub struct $name(String);

        impl $name {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

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
