pub mod invoker {
    pub struct Invoker;

    impl Invoker {
        pub fn invoke(method: &str, args: Vec<&str>) -> Result<(), String> {
            match method {
                "message" => {
                    println!("Hello!!");
                    Ok(())
                }
                _ => Err(format!("Unknown method: {}", method)),
            }
        }
    }
}
