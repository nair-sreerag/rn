// this is the file that runs after the config is
// parsed without errors
//

mod composer;

pub trait CoreComposer {
    fn init() -> Self;
}
