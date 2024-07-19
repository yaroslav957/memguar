use crate::advisor::*;
use crate::advisor::Advise::DontNeed;
use crate::locker::*;
use crate::mapper::MappedBuffer;

#[test]
pub fn locker() -> Result<(), LockError> {
    let buf = [420; 16_000];
    let mut locked_buf = Locker::new(buf);

    locked_buf
        .lock()
}

#[test]
pub fn advisor() -> Result<(), AdviseError> {
    let buf = [420; 16_000];
    let mut advised_buf = Adviser::new(buf);

    advised_buf
        .syscall_advise(DontNeed)
}

#[test]
pub fn mapper() -> Result<(), std::io::Error> {
    let buf = [420; 16_000];
    let mapped_buf = MappedBuffer::new(buf)?;
    let _buf = mapped_buf.receive();
    
    Ok(())
}