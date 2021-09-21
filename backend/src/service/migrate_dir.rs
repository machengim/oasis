use include_dir::Dir;
use rocket::futures::future::BoxFuture;
use sqlx::{
    error::BoxDynError,
    migrate::{Migration, MigrationSource, MigrationType},
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct MigrationDir<'s> {
    dir: Dir<'s>,
}

impl<'s> MigrationDir<'s> {
    pub fn new(dir: Dir<'s>) -> Self {
        Self { dir }
    }
}

impl<'s> MigrationSource<'s> for MigrationDir<'s> {
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = Vec::new();
            for file in self.dir.files() {
                let file_name = file.path().file_name().unwrap().to_string_lossy();
                let parts = file_name.splitn(2, '_').collect::<Vec<_>>();

                if parts.len() != 2 || !parts[1].ends_with(".sql") {
                    continue;
                }

                let version: i64 = parts[0].parse()?;
                let migration_type = MigrationType::from_filename(parts[1]);
                let description = parts[1]
                    .trim_end_matches(migration_type.suffix())
                    .replace('_', " ")
                    .to_owned();

                let sql = String::from_utf8(file.contents().to_vec())?;

                migrations.push(Migration::new(
                    version,
                    Cow::Owned(description),
                    migration_type,
                    Cow::Owned(sql),
                ));
            }

            Ok(migrations)
        })
    }
}
