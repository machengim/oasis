#[derive(Debug)]
pub struct Query<'a> {
    pub sql: &'a str,
    pub args: Vec<String>,
}

impl<'a> Query<'a> {
    pub fn new(sql: &'a str, args: Vec<String>) -> Self {
        Query { sql, args }
    }

    pub fn from<T: Into<String>>(sql: &'a str, args: Vec<T>) -> Self {
        let args: Vec<String> = args.into_iter().map(|x| x.into()).collect();
        Query { sql, args }
    }
}
