use crate::service::{state::State, token::Token};
use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;
use tide::Request;

#[derive(Deserialize, Debug)]
pub struct GetDirRequest {
    pub path: PathBuf,
}

impl GetDirRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let token = Token::from_ext(req)?;
        let mut path = Self::get_user_files_dir(req, &token)?;

        if let Ok(dir_path) = req.param("dir_path") {
            let dir_split = split_dir_string(dir_path)?;
            for sub_dir in dir_split.iter() {
                path = path.join(sub_dir);
            }
        }

        println!("joined path as: {:?}", &path);

        Ok(Self { path })
    }

    pub fn validate(&self) -> bool {
        self.path.exists() && self.path.is_dir()
    }

    pub fn auth(&self, req: &Request<State>) -> Result<bool> {
        let token = Token::from_ext(req)?;

        Ok(token.uid > 0 && token.permission > 0)
    }

    fn get_user_files_dir(req: &Request<State>, token: &Token) -> Result<PathBuf> {
        let storage = req.state().get_storage()?;
        let username = &token.username;

        Ok(PathBuf::from(storage).join(username).join("files"))
    }
}

fn split_dir_string(dir_str: &str) -> Result<Vec<String>> {
    let dir_decode = urlencoding::decode(dir_str)?;
    let dir_split: Vec<String> = dir_decode
        .to_string()
        .split("/")
        .map(|s| match urlencoding::decode(s) {
            Ok(v) => v.to_string(),
            _ => "".to_owned(),
        })
        .collect();

    Ok(dir_split)
}
