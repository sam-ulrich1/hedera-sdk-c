use std::{mem, slice};

#[repr(C)]
#[derive(Debug)]
pub struct CArray<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for CArray<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len));
        }
    }
}

impl<T> From<Vec<T>> for CArray<T> {
    fn from(data: Vec<T>) -> Self {
        let mut data = data.into_boxed_slice();
        let (ptr, len) = (data.as_mut_ptr(), data.len());

        mem::forget(data);

        Self { ptr, len }
    }
}
