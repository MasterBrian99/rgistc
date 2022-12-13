pub mod response_logs {
    use colored::Colorize;
    use ureq::Response;

    pub fn log_api_errors(code: u16, response: Response) {
        let shader = r#"
        ___ _ __ _ __ ___  _ __ 
       / _ \ '__| '__/ _ \| '__|
      |  __/ |  | | | (_) | |   
       \___|_|  |_|  \___/|_|   
"#;
        println!("{}", shader.red());
        println!("Error Code: {}", code.to_string().red().bold());
        println!("Error : {}", response.status_text().red().bold())
    }
    pub fn log_common_errors(err: &str) {
        let shader = r#"
        ___ _ __ _ __ ___  _ __ 
       / _ \ '__| '__/ _ \| '__|
      |  __/ |  | | | (_) | |   
       \___|_|  |_|  \___/|_|   
"#;
        println!("{}", shader.red());
        println!("Error : {}", err.red().bold())
    }
    pub fn success_response() {
        let shader = r#"
         ___ _   _  ___ ___ ___  ___ ___ 
        / __| | | |/ __/ __/ _ \/ __/ __|
        \__ \ |_| | (_| (_|  __/\__ \__ \
        |___/\__,_|\___\___\___||___/___/
"#;
        println!("{}", shader.blue().bold());
    }
}
