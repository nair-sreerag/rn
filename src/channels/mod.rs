mod channels;

pub trait Channel {
    fn create_mpsc(&self, no_of_required_producers: u32) -> Self;
}
