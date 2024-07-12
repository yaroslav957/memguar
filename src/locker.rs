use std::marker::PhantomData;
use std::mem::size_of;

use libc::{c_int, c_void, mlock, munlock};

/// Wrapper-Struct `Locker` that is used to lock the buffer's page in the process's virtual address space.
/// Locking memory pages ensures that those pages are not moved to the page file, avoiding I/O delays when accessing memory
#[repr(transparent)]
pub(crate) struct Locker<C: AsMut<[T]>, T> {
    buf: C,
    item_type: PhantomData<T>,
}

impl<C: AsMut<[T]>, T> Locker<C, T> {
    pub(crate) fn new(buf: C) -> Self {
        Self {
            buf,
            item_type: PhantomData,
        }
    }

    /// If `lock` is successful, we can then use the buffer without worrying about it page being swapped out to disk/swap-zone.
    pub(crate) fn lock(&mut self) -> Result<(), LockError> {
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

    /// If `unlock` is successful, this allows the system to further perform various optimizations,
    /// such as moving pages to the page file or merging adjacent locked memory areas
    pub(crate) fn unlock(&mut self) -> Result<(), LockError> {
        let buf = self.buf.as_mut();
        let ptr = buf.as_mut_ptr() as *mut c_void;
        let len = buf.len() * size_of::<T>();
        let result = unsafe {
            munlock(ptr, len)
        };
        let _ = LockError::from(result);

        Ok(())
    }
}

impl<C: AsMut<[T]>, T> Drop for Locker<C, T> {
    fn drop(&mut self) {
        self.unlock().unwrap()
    }
}

/// Parsed types of `mlock` and `munlock` errors
#[derive(Debug)]
pub(crate) enum LockError {
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
