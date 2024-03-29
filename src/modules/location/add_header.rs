use crate::LocationStage;

pub struct AddHeaderStage {
    key: String,
    value: String,
}

impl LocationStage for AddHeaderStage {
    type StageInitArgs = (Vec<String>, String, String);

    fn init(args: Self::StageInitArgs) -> Self {
        println!("control is in AddHeaderStage");

        AddHeaderStage {
            key: args.1,
            value: args.2,
        }
    }

    fn process(mut self) -> String {
        // self.args.0.push(format!("{} : {}", args.1, args.2));

        println!("processing in the AddHeaderStage");

        String::new()
    }
}
