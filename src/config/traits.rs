pub trait IConfig {
    fn load_config(path: String, contents: String);
}
