pub mod list_messages;

pub trait Query<T> {
    fn execute(&self) -> T;
}
