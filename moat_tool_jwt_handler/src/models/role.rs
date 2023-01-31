use core::fmt;
use std::fmt::Display;

pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(item: &str) -> Role {
        match item {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}