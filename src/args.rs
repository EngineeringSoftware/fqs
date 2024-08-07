use std::env;

#[derive(Debug)]
pub struct Args {
    query: String,
}

impl Args {
    pub fn new(query: String) -> Args {
        Args { query }
    }

    pub fn query(&self) -> String {
        self.query.to_string()
    }

    pub fn parse() -> Result<Args, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            return Err("Requires exactly one query argument");
        }

        Ok(Args {
            query: String::from(&args[1]),
        })
    }
}
