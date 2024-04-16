pub type Job = Box<dyn FnOnce() + Send + 'static>;

// ?!
pub type JobWithoutBox = dyn FnOnce() + Send + 'static;

pub enum ServerType {
    EVENTS,
    STREAM,
    HTTP,
}
