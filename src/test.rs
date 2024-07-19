use crate::advisor::Advise::DontNeed;
use crate::advisor::Adviser;
use crate::locker::Locker;
use crate::mapper::MappedBuffer;

#[test]
pub fn locker() {
    let buf = [420; 16_000];
    let mut locked_buf = Locker::new(buf);

    locked_buf
        .lock()
        .unwrap()
}

#[test]
pub fn advisor() {
    let buf = [420; 16_000];
    let mut advised_buf = Adviser::new(buf);

    advised_buf
        .syscall_advise(DontNeed)
        .unwrap();
}

#[test]
pub fn mapper() {
    let buf = [420; 16_000];
    let mapped_buf = MappedBuffer::new(buf);
    let _slice = &mapped_buf[0..=420];
}