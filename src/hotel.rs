//! Handle airlines config
extern crate actix;

use actix::{Actor, SyncContext, Handler, Message, Addr, SyncArbiter};
use rand::{thread_rng, Rng};
use actix::clock::sleep;
use std::time::Duration;

pub struct InfoPackage {
    pub route: String,
}

impl Message for InfoPackage {
    type Result = i32;
}
    
pub struct Hotel { 
}

impl Actor for Hotel {
    type Context = SyncContext<Self>; 
}

impl Handler<InfoPackage> for Hotel {
    type Result = i32;

    fn handle(&mut self, msg: InfoPackage, _ctx: &mut <Hotel as Actor>::Context) -> Self::Result {
        sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
        println!("[{}] Hotel reservation: SUCESSFUL", msg.route.to_string());
        0
    }
        
} 

pub fn get_hotel_address(rate_limite: usize) -> Addr<Hotel> {
    SyncArbiter::start(rate_limite, || Hotel {} )
}
