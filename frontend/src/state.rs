use crate::store::User;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct GlobalState {
    users: Vec<User>,
}
