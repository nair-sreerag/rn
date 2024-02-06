pub mod Utils {
    use std::{fmt::Debug, fs};

    enum WRITEABLE_DATATYPES {
        binary,
        json,
        string,
    }

    impl WRITEABLE_DATATYPES {
        fn default() -> WRITEABLE_DATATYPES {
            WRITEABLE_DATATYPES::string
        }
    }

    struct WriteFileParams<'a> {
        data: &'a str,
        data_type: WRITEABLE_DATATYPES,
    }

    pub fn read_file<T>(path: &str) -> Result<T, serde_json::Error>
    where
        T: serde::de::Deserialize<'static> + Debug,
    {
        let content: String = fs::read_to_string(path).unwrap_or(String::from("Error here"));
    }

    pub fn write_file(params: &WriteFileParams) {}
}
