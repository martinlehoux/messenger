pub mod send_message;

use super::error::Error;

pub trait Mutation<T> {
    fn execute(&self) -> Result<T, Error>;
}
