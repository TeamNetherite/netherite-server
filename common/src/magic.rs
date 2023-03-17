pub auto trait Thing {}

impl !Thing for () {}

pub trait NInto<T> {
    fn n_into(self) -> T;
}

impl<T: Thing> NInto<Option<T>> for T {
    fn n_into(self) -> Option<T> {
        Some(self)
    }
}

impl<T: Thing> NInto<Option<T>> for () {
    fn n_into(self) -> Option<T> {
        None
    }
}