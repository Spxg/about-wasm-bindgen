use once_cell::unsync::Lazy;
use std::ops::Deref;

#[cfg(feature = "unsafe_cell")]
use std::cell::UnsafeCell;

#[cfg(not(feature = "unsafe_cell"))]
use std::cell::Cell;

/// Wrapper around [`Lazy`] adding `Send + Sync` when `atomics` is not enabled.
pub struct LazyCell<T, F = fn() -> T>(Wrapper<Lazy<T, F>>);

struct Wrapper<T>(T);

unsafe impl<T> Sync for Wrapper<T> {}

unsafe impl<T> Send for Wrapper<T> {}

impl<T, F> LazyCell<T, F> {
    pub const fn new(init: F) -> LazyCell<T, F> {
        Self(Wrapper(Lazy::new(init)))
    }
}

impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub(crate) fn try_with<R>(
        &self,
        f: impl FnOnce(&T) -> R,
    ) -> Result<R, core::convert::Infallible> {
        Ok(f(&self.0.0))
    }

    pub fn force(this: &Self) -> &T {
        &this.0.0
    }
}

impl<T> Deref for LazyCell<T> {
    type Target = T;

    fn deref(&self) -> &T {
        ::once_cell::unsync::Lazy::force(&self.0.0)
    }
}

#[cfg(feature = "unsafe_cell")]
static HEAP_SLAB: LazyCell<UnsafeCell<Vec<i32>>> = LazyCell::new(Default::default);

#[cfg(not(feature = "unsafe_cell"))]
static HEAP_SLAB: LazyCell<Cell<Vec<i32>>> = LazyCell::new(Default::default);

#[cfg(not(feature = "unsafe_cell"))]
#[unsafe(no_mangle)]
pub extern "C" fn inc(value: i32) -> usize {
    HEAP_SLAB
        .try_with(|x| {
            let mut prev = x.take();
            let size = prev.len();
            prev.try_reserve(128.min(size * 2)).unwrap();
            prev.push(value);
            x.replace(prev);
            size + 1
        })
        .unwrap()
}

#[cfg(feature = "unsafe_cell")]
#[unsafe(no_mangle)]
pub extern "C" fn inc(value: i32) -> usize {
    HEAP_SLAB
        .try_with(|x| {
            let v = unsafe { &mut *x.get() };
            let size = v.len();
            v.try_reserve(128.min(size * 2)).unwrap();
            v.push(value);
            size + 1
        })
        .unwrap()
}
