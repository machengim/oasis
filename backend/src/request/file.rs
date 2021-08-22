use crate::{
    args,
    entity::file::File,
    service::{state::State, token::Token},
    util::query::Query,
};
use anyhow::Result;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, Sqlite};
use tide::Request;

#[derive(Deserialize)]
pub struct DirListRequest {
    pub dir_id: i64,
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct CreateDirRequest {
    pub parent_id: i64,
    pub dir_name: String,
    // Additional fields
    pub user_id: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct RenameFileRequest {
    pub filename: String,
    pub file_id: Option<i64>,
    pub user_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct DeleteFileRequest {
    pub file_id: i64,
    pub user_id: i64,
}

impl DirListRequest {
    pub fn from(req: &Request<State>) -> tide::Result<Self> {
        let dir_id_str = req.param("dir_id")?;

        let dir_id = match dir_id_str.parse::<i64>() {
            Ok(v) => v,
            _ => -1,
        };

        let token = Token::from_ext(&req)?;

        Ok(Self {
            dir_id,
            user_id: token.uid,
        })
    }

    pub fn validate(&self) -> bool {
        self.dir_id > 0
    }

    pub async fn auth(&self, conn: &mut PoolConnection<Sqlite>) -> Result<bool> {
        if self.user_id <= 0 {
            return Ok(false);
        }

        let request_dir = File::get_file_by_id(self.dir_id, conn).await?;
        if request_dir.owner_id != self.user_id {
            return Ok(false);
        }

        Ok(true)
    }
}

impl CreateDirRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let mut create_dir_req: Self = req.body_json().await?;
        let token = Token::from_ext(req)?;
        create_dir_req.user_id = Some(token.uid);

        Ok(create_dir_req)
    }

    pub fn validate(&self) -> bool {
        self.parent_id > 0 && self.dir_name.len() > 0
    }

    pub async fn auth(&self, conn: &mut PoolConnection<Sqlite>) -> Result<bool> {
        let uid = match self.user_id {
            Some(v) => v,
            None => {
                eprintln!("No user id found");
                return Ok(false);
            }
        };

        let parent_dir = File::get_file_by_id(self.parent_id, conn).await?;
        if parent_dir.owner_id != uid {
            eprintln!("Parent dir owner id not match");
            return Ok(false);
        }

        Ok(true)
    }
}

impl RenameFileRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let mut rename_req: Self = req.body_json().await?;
        let file_id: i64 = req.param("file_id")?.parse::<i64>()?;
        let token = Token::from_ext(req)?;

        rename_req.file_id = Some(file_id);
        rename_req.user_id = Some(token.uid);

        Ok(rename_req)
    }

    pub fn validate(&self) -> bool {
        self.file_id.is_some()
            && self.file_id.unwrap() > 0
            && self.user_id.is_some()
            && self.user_id.unwrap() > 0
    }

    pub async fn auth(&self, conn: &mut PoolConnection<Sqlite>) -> Result<bool> {
        let file_owner = File::find_file_owner(self.file_id.unwrap(), conn).await?;

        Ok(file_owner == self.user_id.unwrap())
    }

    pub fn to_query(&self) -> Query {
        let sql = "update FILE set filename = ?1 where file_id = ?2";
        Query::new(sql, args!(&self.filename, self.file_id.unwrap()))
    }
}

impl DeleteFileRequest {
    pub fn from(req: &Request<State>) -> tide::Result<Self> {
        let file_id: i64 = req.param("file_id")?.parse()?;
        let user_id = Token::from_ext(&req)?.uid;

        Ok(Self { file_id, user_id })
    }

    pub fn validate(&self) -> bool {
        self.file_id > 0 && self.user_id > 0
    }

    pub async fn auth(&self, conn: &mut PoolConnection<Sqlite>) -> Result<bool> {
        let file_owner = File::find_file_owner(self.file_id, conn).await?;

        Ok(file_owner == self.user_id)
    }
}
