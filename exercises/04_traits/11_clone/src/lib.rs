// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.


// pub trait Clone {
//     fn clone(&self) -> Self;
// }
// impl Clone for Ticket{
//     fn clone(&self) -> Self {
//         Ticket{
//             title: self.title.to_string(),
//             description: self.description.to_string(),
//             status: self.status.to_string(),
//         }
//     }
// }
pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    
    (ticket.clone(), ticket.summary())
}

#[derive(Clone)] //直接使用
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
