macro_rules! tile {
    ($name:ident { $($key:ident = $char:expr,)+ }) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
        enum $name {
            $($key,)+
        }

        impl $name {
            #[allow(dead_code)]
            pub fn parse_sequence(s: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
                s.chars().map(|c| Self::try_from(c)).collect()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$key => write!(f, "{}", $char),)+
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = Box<dyn std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.chars().next() {
                    $(Some($char) => Ok(Self::$key),)+
                    None => Err(format!("The input string was empty.").into()),
                    _ => Err(format!("'{s}' is not a recognized tile type.").into())
                }
            }
        }

        impl TryFrom<char> for $name {
            type Error = Box<dyn std::error::Error>;

            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    $($char => Ok(Self::$key),)+
                    _ => Err(format!("'{value}' is not a recognized tile type.").into())
                }
            }
        }
    };
}

macro_rules! grid {
    ($name:ident <$tile:ty> $(=> { $($impl:tt)* })?) => {
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        struct $name(crate::helpers::vecs::RectVec<$tile>);

        impl $name {
            #[allow(dead_code)]
            pub fn get(&self, x: usize, y: usize) -> Option<$tile> {
                self.0.get(y, x).copied()
            }

            #[allow(dead_code)]
            pub fn set(&mut self, x: usize, y: usize, tile: $tile) {
                self.0.set(y, x, tile);
            }

            $($($impl)*)?
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = Box<dyn std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let map = s.trim()
                    .lines()
                    .map(|line| line.trim().chars().map(|c| <$tile>::try_from(c)).collect::<Result<Vec<_>, _>>())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self(map.into()))
            }
        }
    };

    ($name:ident <$tile:ident> { $($key:ident = $char:expr,)+ } $(=> { $($impl:tt)* })?) => {
        tile!($tile { $($key = $char,)+ });
        grid!($name <$tile> $(=> { $($impl)* })?);
    };
}