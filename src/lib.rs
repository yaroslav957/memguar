//! A crate that helps to optimize large buffers in near-OOM state or with small amount of RAM

pub use file::*;
pub use wrappers::*;

mod file {
    /// Include `MappedBuffer`
    #[cfg(unix)]
    pub mod mapper;
}

mod wrappers {
    /// Include `Adviser`, `Advise`, `AdviseError`
    #[cfg(unix)]
    pub mod advisor;
    /// Include `Locker`, `LockError`
    #[cfg(unix)]
    pub mod locker;
}

#[cfg(test)]
mod test;