use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct ConfigStruct {
    host: String,
}
