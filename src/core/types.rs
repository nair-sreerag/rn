pub type Job = Box<dyn FnOnce() + Send + 'static>;

// ?!
pub type JobWithoutBox = dyn FnOnce() + Send + 'static;

pub enum SERVER_TYPE {
    EVENTS,
    STREAM,
    HTTP,
}
