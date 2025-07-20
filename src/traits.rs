pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T: Default + Eq> IsEmpty for T {
    fn is_empty(&self) -> bool {
        self.eq(&Self::default())
    }
}

pub trait Dispose {
    fn dispose(&mut self);
}
