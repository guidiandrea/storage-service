use crate::model::files;
use crate::model::folders;
use crate::model::{files::Entity as Files, folders::Entity as Folders};
use axum::body::Bytes;
use std::io::Write;
use axum::extract::Multipart;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{Condition, DatabaseConnection};
use std::sync::Arc;

use super::errors::FileServiceError;

#[derive(Clone)]
pub struct FileService {
    pub db: Arc<DatabaseConnection>
}

impl FileService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        FileService { db }
    }
    pub async fn get_files_by_folder(self, user: &str, folder_name: &str) {
        let files = Files::find()
            .find_with_related(Folders)
            .filter(
                Condition::all()
                    .add(files::Column::UserId.eq(user))
                    .add(folders::Column::FolderName.eq(folder_name)),
            );
            //.all(&*self.db)
            //.await;
        println!("{:?}", &files)
    }
pub async fn upload(self, mut multipart: Multipart) -> Result<(), FileServiceError> {

    let upload_folder = std::env::var("UPLOADS_FOLDER").unwrap();

    while let Some(data) = multipart.next_field().await.unwrap() {
        let name = data.name().unwrap().to_string();
        let data = data.bytes().await.unwrap();
        
        let file = std::fs::File::create(format!("{}/{}",upload_folder,name));
        match file  {
            Ok(mut f) => {
                f.write_all(&data);
            },
            Err(_) => {
                return Err(FileServiceError::GenericError)
            }
        }

        println!("Length of `{}` is {} bytes", name, data.len());


    }
    Ok(())
}
}
