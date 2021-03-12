use core::marker::PhantomData;
#[derive(Copy, Clone)]
pub struct RW;
#[derive(Copy, Clone)]
pub struct R;
#[derive(Copy, Clone)]
pub struct W;
mod sealed;
pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}
pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}
pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}
#[derive(Copy, Clone)]
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
impl<T: Copy, A: Access> Reg<T, A> {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }
    pub fn ptr(&self) -> *mut T {
        self.ptr as _
    }
}
impl<T: Copy, A: Read> Reg<T, A> {
    pub unsafe fn read(&self) -> T {
        (self.ptr as *mut T).read_volatile()
    }
}
impl<T: Copy, A: Write> Reg<T, A> {
    pub unsafe fn write_value(&self, val: T) {
        (self.ptr as *mut T).write_volatile(val)
    }
}

impl<T: Default + Copy, A: Write> Reg<T, A> {
    pub unsafe fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}
impl<T: Copy, A: Read + Write> Reg<T, A> {
    pub unsafe fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = self.read();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}
