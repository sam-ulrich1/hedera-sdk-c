use std::{mem, slice};

#[repr(C)]
#[derive(Debug)]
pub struct CArray<T> {
    pub ptr: *mut T,
    pub len: usize,
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

#[macro_export]
macro_rules! vec_to_carray {
    ($rt:ty, $ct:ty) => {
        impl From<Vec<$rt>> for CArray<$ct> {
            fn from(data: Vec<$rt>) -> Self {
                let data = data.into_iter().map(Into::into).collect::<Vec<$ct>>();
                let mut data = data.into_boxed_slice();
                let (ptr, len) = (data.as_mut_ptr(), data.len());

                std::mem::forget(data);

                Self { ptr, len }
            }
        }
    };
}
