pub mod greetings {
    pub mod english;
    pub mod german {
        pub fn hello() -> String {
            "hallo".to_string()
        }
        pub fn goodbye() -> String {
            "auf wiedersehen".to_string()
        }
    }
}
