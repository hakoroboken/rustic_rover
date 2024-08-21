use std::cell::RefCell;
use tokio::sync::mpsc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc
};


pub type Publisher<T> = Sender<T>;
pub type Subscriber<T> = Receiver<T>;
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
    pub publisher:Publisher<T>,
    pub subscriber:Subscriber<T>
}

impl<T> ThreadConnector<T> {
    pub fn new()->ThreadConnector<T>
    {
        let (t,r) = channel::<T>();

        ThreadConnector { publisher: t, subscriber: r }
    }
}

pub struct ThreadManager
{
    stop_flag_ : Arc<AtomicBool>,
}

impl ThreadManager {
    pub fn new()->ThreadManager
    {
        let stop_flag = Arc::new(AtomicBool::new(false));

        ThreadManager { stop_flag_: stop_flag}
    }
    pub fn get_clone(&self)->Arc<AtomicBool>
    {
        Arc::clone(&self.stop_flag_)
    }

    pub fn thread_stop(&self)
    {
        self.stop_flag_.store(true, Ordering::Relaxed);
    }
}