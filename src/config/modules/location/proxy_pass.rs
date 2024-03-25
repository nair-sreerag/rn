use super::LocationStage;

pub struct ProxyPassStage {}

impl LocationStage for ProxyPassStage {
    type StageInitArgs = ();

    fn init(args: Self::StageInitArgs) -> Self {
        ProxyPassStage {}
    }

    fn process(self) -> String {
        String::new()
    }
}
