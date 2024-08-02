use std::cell::RefCell;
use tokio::sync::mpsc;

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