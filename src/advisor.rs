use std::convert::AsMut;
use std::marker::PhantomData;
use std::mem::size_of;

use libc::{c_int, c_void, posix_madvise};

use crate::advisor::Advise::DontNeed;

/// A wrapper-struct Adviser that is used to advise the system
/// about the expected behavior of memory access patterns of the buffer's page.
/// It can help the system to optimize memory usage and performance
#[repr(transparent)]
pub(crate) struct Adviser<C: AsMut<[T]>, T> {
    buf: C,
    item_type: PhantomData<T>,
}

impl<C: AsMut<[T]>, T> Adviser<C, T> {
    pub(crate) fn new(buf: C) -> Self {
        Self {
            buf,
            item_type: PhantomData,
        }
    }

    /// If `syscall_advise` is successful, it allows the system to apply specific optimizations to the page,
    /// based on the specified flag, such as moving it to the swap file
    /// or merging it with adjacent pages.
    pub(crate) fn syscall_advise(&mut self, advise: Advise) -> Result<(), AdviseError> {
        let buf = self.buf.as_mut();
        let ptr = buf.as_mut_ptr() as *mut c_void;
        let len = buf.len() * size_of::<T>();
        let result = unsafe {
            posix_madvise(ptr, len, advise as c_int)
        };

        match result {
            0 => Ok(()),
            result => Err(AdviseError::from(result)),
        }
    }
}

impl<C: AsMut<[T]>, T> Drop for Adviser<C, T> {
    fn drop(&mut self) {
        self.syscall_advise(DontNeed).unwrap()
    }
}
/// Advises for page
#[repr(i32)]
pub(crate) enum Advise {
    WillNeed = 3,
    DontNeed = 4,
}
// Parsed types of `syscall_advise` errors
#[derive(Debug)]
pub(crate) enum AdviseError {
    EFAULT,
    EINVAL,
    ENOMEM,
    ENOSYS,
    EUNIM(c_int),
}

impl From<c_int> for AdviseError {
    fn from(err: c_int) -> Self {
        match err {
            12 => AdviseError::ENOMEM,
            14 => AdviseError::EFAULT,
            22 => AdviseError::EINVAL,
            38 => AdviseError::ENOSYS,
            _ => AdviseError::EUNIM(err),
        }
    }
}