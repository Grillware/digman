use chrono::{DateTime, Utc};
use ddomain::entites::ticket::Ticket;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TicketDTO {
    pub id: String,
    pub level: String,
    pub title: String,
    pub completion_condition: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl From<TicketDTO> for Ticket {
    fn from(dto: TicketDTO) -> Self {
        Ticket::new(
            dto.id,
            dto.level.into(),
            dto.title,
            dto.status.into(),
            dto.completion_condition,
        )
    }
}

impl From<Ticket> for TicketDTO {
    fn from(mut ticket: Ticket) -> Self {
        let mut dto = TicketDTO::default();
        ticket.substitute(|ticket_mut| {
            dto.id = ticket_mut.id.clone();
            dto.level = ticket_mut.level.to_string();
            dto.title = ticket_mut.title.clone();
            dto.status = ticket_mut.status.to_string();
            dto.completion_condition = ticket_mut.completion_condition.clone();
            dto.created_at = *ticket_mut.created_at;
            dto.resolved_at = *ticket_mut.resolved_at;
        });
        dto
    }
}
