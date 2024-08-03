use std::cell::RefCell;
use tokio::sync::mpsc;
use std::sync::mpsc::{channel, Sender, Receiver};
pub struct AsyncThreadConnector<T>
{
    pub publisher:RefCell<Option<mpsc::UnboundedSender<T>>>,
    pub subscriber:RefCell<Option<mpsc::UnboundedReceiver<T>>>
}

impl<T> AsyncThreadConnector<T> {
    pub fn new()->AsyncThreadConnector<T>
    {
        let (t,r) = mpsc::unbounded_channel::<T>();

        AsyncThreadConnector { publisher: RefCell::new(Some(t)), subscriber: RefCell::new(Some(r)) }
    }
}

pub struct ThreadConnector<T>
{
    pub publisher:Sender<T>,
    pub subscriber:Receiver<T>
}

impl<T> ThreadConnector<T> {
    pub fn new()->ThreadConnector<T>
    {
        let (t,r) = channel::<T>();

        ThreadConnector { publisher: t, subscriber: r }
    }
}