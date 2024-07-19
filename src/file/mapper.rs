use std::marker::PhantomData;
use std::ops::Deref;
use std::os::fd::AsRawFd;
use std::ptr;

use libc::{c_void, MAP_SHARED, mmap, munmap, PROT_READ, PROT_WRITE, size_t};
use tempfile::tempfile;

/// A struct that represents a buffer that is mapped to memory.
///
/// The `MappedBuffer` struct provides a safe and convenient way to create a buffer that is mapped to memory,
/// allowing you to read and write data to the buffer using a slice-like interface.
/// # Examples
///
/// ```
/// use memguar::mapper::MappedBuffer;
///
/// pub fn map_example() -> Result<(), std::io::Error> {
///     let buf = [420; 16_000];
///     let mapped_buf = MappedBuffer::new(buf)?;
///
///     Ok(())
/// }
/// ```
pub struct MappedBuffer<T: Copy> {
    size: usize,
    ptr: *mut c_void,
    _phantom: PhantomData<T>,
}

impl<T: Copy> MappedBuffer<T> {
    pub fn new<B: AsRef<[T]>>(buf: B) -> Result<Self, std::io::Error> {
        let buf = buf.as_ref();
        assert!(size_of_val(buf) > 0, "Zero size buffer");
        let size = size_of_val(buf);
        let file = tempfile()?;

        file.set_len(size as u64)?;

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                size as size_t,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };

        match ptr {
            libc::MAP_FAILED => panic!("Map failed"),
            _ => unsafe {
                ptr::copy_nonoverlapping(buf.as_ptr(), ptr as *mut T, buf.len());
            },
        };

        Ok(Self {
            ptr,
            size,
            _phantom: PhantomData,
        })
    }
    /// If `receive` is successful, It returns a slice that represents the mapped buffer.
    /// # Examples
    ///
    /// ```
    /// use memguar::mapper::MappedBuffer;
    ///
    /// pub fn receive_example() -> Result<(), std::io::Error> {
    ///     let buf = [420; 16_000];
    ///     let mapped_buf = MappedBuffer::new(buf)?;
    ///     let _buf = mapped_buf.receive();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn receive(&self) -> &[T] {
        // Make it safer
        unsafe {
            std::slice::from_raw_parts(self.ptr as *const T, self.size / size_of::<T>())
        }
    }
}

impl<T: Copy> Deref for MappedBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.receive()
    }
}

impl<T: Copy> Drop for MappedBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr, self.size);
        }
    }
}