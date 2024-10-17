use std::sync::{mpsc, Arc};
// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SendError, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: Arc<SyncSender<Command>>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ClientError> {
        let (response_tx, response_rx) = sync_channel(1);
        let command = Command::Insert {
            draft,
            response_channel: response_tx,
        };
        self.sender.send(command).map_err(|_| ClientError::SendError)?;
        response_rx.recv().map_err(|_| ClientError::RecvError)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ClientError> {
        let (response_tx, response_rx) = sync_channel(1);
        let command = Command::Get {
            id,
            response_channel: response_tx,
        };
        self.sender.send(command).map_err(|_| ClientError::SendError)?;
        response_rx.recv().map_err(|_| ClientError::RecvError)
    }
}
#[derive(Debug)]
pub enum ClientError {
    SendError,
    RecvError,
}
pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);

    std::thread::spawn(move || server(receiver));

    TicketStoreClient { sender: Arc::new(sender) }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id).cloned();
                let _ = response_channel.send(ticket);
            }
            Err(_) => {
                break;
            }
        }
    }
}
