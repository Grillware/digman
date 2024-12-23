pub mod ticket_repository_impl;
use ddomain::entites::ticket::Ticket;
use serde::{Deserialize, Serialize};

// tomlパース用
#[derive(Serialize, Deserialize, Debug)]
pub struct TicketCollection {
    pub ticket_data: Vec<Ticket>,
}
