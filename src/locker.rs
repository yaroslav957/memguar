use std::marker::PhantomData;
use std::mem::size_of;

use libc::{c_int, c_void, mlock, munlock};

/// Wrapper-Struct `Locker` that is used to lock the buffer's page.
/// Locking memory pages ensures that those pages are not moved to the page file,
/// which can help avoid I/O delays when accessing memory in OOM state. This is particularly
/// useful when dealing with large buffers or when performance is critical,
/// when working with a small amount of RAM.
#[repr(transparent)]
pub struct Locker<C: AsMut<[T]>, T> {
    pub buf: C,
    pub item_type: PhantomData<T>,
}

impl<C: AsMut<[T]>, T> Locker<C, T> {
    /// `Locker` constructor.
    pub fn new(buf: C) -> Self {
        Self {
            buf,
            item_type: PhantomData,
        }
    }

    /// If `lock` is successful, the buffer's page locked,
    /// preventing it from being swapped out to disk/swap-zone.
    pub fn lock(&mut self) -> Result<(), LockError> {
        let buf = self.buf.as_mut();
        let ptr = buf.as_mut_ptr() as *mut c_void;
        let len = buf.len() * size_of::<T>();
        let result = unsafe {
            mlock(ptr, len)
        };

        match result {
            0 => Ok(()),
            result => Err(LockError::from(result)),
        }
    }

    /// If `unlock` is successful, the buffer's page is unlocked,
    /// allowing the system to perform additional optimizations,
    /// such as moving pages to the swap file or merging adjacent locked memory regions.
    pub fn unlock(&mut self) -> Result<(), LockError> {
        let buf = self.buf.as_mut();
        let ptr = buf.as_mut_ptr() as *mut c_void;
        let len = buf.len() * size_of::<T>();
        let result = unsafe {
            munlock(ptr, len)
        };

        match result {
            0 => Ok(()),
            result => Err(LockError::from(result)),
        }
    }
}

impl<C: AsMut<[T]>, T> Drop for Locker<C, T> {
    fn drop(&mut self) {
        self.unlock().unwrap()
    }
}

/// Parsed types of `mlock` and `munlock` errors
#[derive(Debug)]
pub enum LockError {
    EPERM,
    EINTR,
    EIO,
    EAGAIN,
    ENOMEM,
    EFAULT,
    EBUSY,
    EINVAL,
    ENOSYS,
    EUNIM(c_int),
}

impl From<c_int> for LockError {
    fn from(err: c_int) -> Self {
        match err {
            1 => LockError::EPERM,
            4 => LockError::EINTR,
            5 => LockError::EIO,
            11 => LockError::EAGAIN,
            12 => LockError::ENOMEM,
            14 => LockError::EFAULT,
            16 => LockError::EBUSY,
            22 => LockError::EINVAL,
            38 => LockError::ENOSYS,
            _ => LockError::EUNIM(err),
        }
    }
}
