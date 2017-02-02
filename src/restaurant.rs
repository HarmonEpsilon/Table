use std::string::String;
use std::clone::Clone;

pub struct Party {
    num_diners: u32,
    time_req: u32,
    reservation_name: String,
}

impl Party {
    pub fn create(rsvp: String, diners: u32, t_req: u32) -> Party {
        Party { num_diners: diners, time_req: t_req, reservation_name: rsvp }
    }

    pub fn get_reservation(&self) -> String {
        self.reservation_name.clone()
    }

    pub fn get_diners(&self) -> u32 {
        self.num_diners
    }

    pub fn get_time(&self) -> u32 {
        self.time_req
    }
}

pub struct Table {
    table_id: String,
    num_seats: u32,
    server_name: String,
    timer: u32, 
    party: Box<Party>,
}