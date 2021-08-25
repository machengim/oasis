#[derive(Debug)]
pub struct Query<'a> {
    pub sql: &'a str,
    pub args: Vec<String>,
}

impl<'a> Query<'a> {
    pub fn new(sql: &'a str, args: Vec<String>) -> Self {
        Query { sql, args }
    }
}

#[macro_export]
macro_rules! args {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                let x_str = $x.to_string();
                temp_vec.push(x_str);
            )*
            temp_vec
        }
    };
}
