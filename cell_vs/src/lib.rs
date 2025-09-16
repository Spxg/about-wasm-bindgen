use once_cell::unsync::Lazy;
use std::ops::Deref;

#[cfg(feature = "unsafe_cell")]
use std::cell::UnsafeCell;

#[cfg(feature = "cell")]
use std::cell::Cell;

#[cfg(feature = "ref_cell")]
use std::cell::RefCell;

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
static HEAP_SLAB: LazyCell<UnsafeCell<Vec<i32>>> = LazyCell::new(|| UnsafeCell::new(vec![0]));

#[cfg(feature = "cell")]
static HEAP_SLAB: LazyCell<Cell<Vec<i32>>> = LazyCell::new(|| Cell::new(vec![0]));

#[cfg(feature = "ref_cell")]
static HEAP_SLAB: LazyCell<RefCell<Vec<i32>>> = LazyCell::new(|| RefCell::new(vec![0]));


#[cfg(feature = "ref_cell")]
#[unsafe(no_mangle)]
pub extern "C" fn set(value: i32) {
    HEAP_SLAB
        .try_with(|x| {
            x.borrow_mut()[0] = value;
        })
        .unwrap()
}

#[cfg(feature = "cell")]
#[unsafe(no_mangle)]
pub extern "C" fn set(value: i32) {
    HEAP_SLAB
        .try_with(|x| {
            let mut prev = x.take();
            prev[0] = value;
            x.replace(prev);
        })
        .unwrap()
}

#[cfg(feature = "unsafe_cell")]
#[unsafe(no_mangle)]
pub extern "C" fn set(value: i32) {
    HEAP_SLAB
        .try_with(|x| {
            let v = unsafe { &mut *x.get() };
            v[0] = value;
        })
        .unwrap()
}
