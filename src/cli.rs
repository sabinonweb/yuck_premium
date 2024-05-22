#[derive(Debug)]
pub struct Config {
    pub uri: String,
    pub file_type: String,
}

impl Config {
    fn parse_config(args: &[String]) -> Config {
        let uri = args[1].clone();
        let file_type = args[2].clone();
        let config = Config { uri, file_type };
        config.parse_uri();

       config 
    }

    fn parse_uri(&self) {
        let uri_segments: Vec<&str> = self.uri.split("/").collect(); 
        println!("{:?}", uri_segments);
    }
}

pub fn command_line() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::parse_config(&args);

    println!("{:?}", config);
}
