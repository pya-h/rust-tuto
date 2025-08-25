
pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a[String]) -> Result<Config<'a>, String>{
        if args.len() < 3 {
            return Err(format!("[Usage] {} <query> <filename>", args[0]));
        }
        Ok(Config{query: &args[1], filename: &args[2]})
    }

    pub fn to_string(&self) -> String {
        format!("{{query: {}, filename: {}}}", self.query, self.filename)
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }


    pub fn filename(&self) -> String {
        self.filename.clone()
    }
}