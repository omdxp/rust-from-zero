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

#[test]
#[should_panic]
#[ignore = "still in dev"]
fn english_greeting_correct() {
    assert_eq!("hellos", greetings::english::hello())
}
