use crate::advisor::Advise::{DontNeed, Free};
use crate::advisor::Adviser;
use crate::locker::Locker;

#[test]
pub fn locker() {
    let buf = [1, 2, 3, 4, 5];

    Locker::new(buf)
        .lock()
        .unwrap()
}

#[test]
pub fn advisor() {
    let buf = [1, 2, 3, 4, 5];

    Adviser::new(buf)
        .syscall_advise(DontNeed)
        .unwrap();
}