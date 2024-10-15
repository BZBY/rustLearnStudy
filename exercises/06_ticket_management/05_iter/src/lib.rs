use std::path::Iter;
use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    pub(crate) fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.as_slice().into_iter()
    }
    // // `iter` 方法应该返回一个 `slice::Iter`，可以用于遍历 `tickets` 列表中的引用
    // 你这样写会有一些问题，因为你试图调用 into_iter()，这实际上会消耗（move）self.tickets，但你只希望返回对 tickets 的不可变引用迭代器。
    //
    // 主要问题
    // into_iter() 在 Vec 上调用时，会尝试获取 Vec 的所有权，并将其“移动”出去，这意味着它将消耗 Vec 并无法再次访问。这样会导致在调用 iter() 后，你将无法再次访问 self.tickets，这是不可行的，特别是在你只希望提供不可变的访问。
    //
    // 修正建议
    // 你应该使用 self.tickets.iter() 来获取对 tickets 的不可变引用的迭代器。这样可以安全地返回 Ticket 的不可变引用，而不需要消耗 Vec。
    // pub fn iter(&self) -> std::slice::Iter<'_, Ticket> {
    //     self.tickets.iter()  // 返回 `Vec<Ticket>` 的迭代器
    // }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
