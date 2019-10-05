pub enum Access {
    Public,
    Private
}

use std::fmt;
impl fmt::Display for Access {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Access::Public => write!(f, "public"),
            Access::Private => write!(f, "private")
        }
    }
}
