#[derive(Debug)]
pub struct Query<'a> {
    pub sql: &'a str,
    pub args: Vec<String>,
}

impl<'a> Query<'a> {
    pub fn new(sql: &'a str, args: Vec<String>) -> Self {
        Query { sql, args }
    }

    pub fn from(sql: &'a str, args: Vec<&'a str>) -> Self {
        let args: Vec<String> = args.into_iter().map(|x| String::from(x)).collect();
        Query { sql, args }
    }
}
