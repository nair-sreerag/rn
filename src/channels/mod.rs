mod channels;

pub trait Channel {
    // TODO: this should return something
    fn create_mpsc();
}
