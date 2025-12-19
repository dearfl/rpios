use core::cell::UnsafeCell;

pub mod interface {
    pub trait Mutex {
        type Data;
        fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
    }
}

pub struct NullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

// SAFETY: ???
unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
// SAFETY: ???
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        // this is not a real lock, we just only have one core and with disrrupts disabled
        // SAFETY: ???
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }
}
