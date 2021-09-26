use std::clone::Clone;
use std::marker::Copy;

pub struct Hotel {
}


impl Copy for Hotel {
}

impl Clone for Hotel {
    fn clone(&self) -> Self {
        Hotel {
        }
    }
}


impl Hotel {
    pub fn new() -> Hotel {
        Hotel {

        }
    }

    pub fn reserve(&self) -> bool {
        // thread::sleep(Duration::from_secs(1));
        return true;
    }
}