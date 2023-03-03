use crate::modules::traits::has_id::HasIdTrait;
use std::collections::HashMap;

trait HasId {
    fn get_id(&self) -> i32;
}

pub trait AsMap<T> {
    fn as_map(self) -> HashMap<i32, T>;
}

impl<T, I> AsMap<T> for I
where
    I: Iterator<Item = T>,
    T: HasIdTrait,
{
    fn as_map(self) -> HashMap<i32, T> {
        self.map(|item| (item.get_id(), item))
            .collect::<HashMap<_, _>>()
    }
}
