use strum::Display;

#[derive(Debug, Display, Copy, Clone)]
pub enum Attribute {
    Immutable,
    Unique,
    Nullable,
}