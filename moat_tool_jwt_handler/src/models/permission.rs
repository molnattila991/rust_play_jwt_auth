use std::fmt::{Display, self};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Permission {
    Create,
    Update,
    Delete,
    Read,
}

impl Permission {
    pub fn from_str(permission: &str) -> Permission {
        match permission {
            "Create" => Permission::Create,
            "Update" => Permission::Update,
            "Delete" => Permission::Delete,
            "Read" => Permission::Read,
            _ => Permission::Read,
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Permission::Create => write!(f, "Create"),
            Permission::Update => write!(f, "Update"),
            Permission::Delete => write!(f, "Delete"),
            Permission::Read => write!(f, "Read"),
        }
    }
}