use crate::store::{Booking, User};
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct GlobalState {
    pub users: Vec<User>,
    pub bookings: Vec<Booking>,
}
