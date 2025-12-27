use std::{thread, time};

#[allow(dead_code)]
pub fn miliseconds(time: u64) {
    let wait_time = time::Duration::from_millis(time);
    let now = time::Instant::now();
    thread::sleep(wait_time);
    assert!(now.elapsed() >= wait_time);
}

#[allow(dead_code)]
pub fn seconds(time: u64) {
    let wait_time = time::Duration::from_millis(time*1000);
    let now = time::Instant::now();
    thread::sleep(wait_time);
    assert!(now.elapsed() >= wait_time);
}
