use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};


pub struct DelayTimer{
    pub time: Arc<Mutex<u8>>,

}


impl DelayTimer {
    pub fn new() -> Self {
        Self {
            time: Arc::new(Mutex::new(0)),
        }
    }


    pub fn run(&self) {
        let time_clone = Arc::clone(&self.time);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_micros(16667));
                let mut time = time_clone.lock().unwrap();
                if *time > 0 {
                    *time -= 1;
                }

            }
        });
    }

    pub fn set(&self, val: u8) {
        *self.time.lock().unwrap() = val;
    }

    pub fn get(&self) -> u8 {
        *self.time.lock().unwrap()
    }


}
