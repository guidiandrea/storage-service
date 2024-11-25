use crate::model::users::{ActiveModel as UsersActiveModel, Entity as Users, Model};
use crate::routes::auth::UserLoginData;
use crate::services::errors::UserCreationError;
use bcrypt::DEFAULT_COST;
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use std::sync::Arc;
#[derive(Clone)]
pub struct UserService {
    pub db: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        UserService { db }
    }

    pub async fn get_user_by_id(&self, username: &str) -> Result<Option<Model>, DbErr> {
        let user = Users::find_by_id(username).one(&*self.db).await?;
        Ok(user)
    }

    pub async fn create_user(&self, user_data: &UserLoginData) -> Result<(), UserCreationError> {
        let user = self.get_user_by_id(&user_data.user).await;
        match user {
            Ok(Some(_)) => return Err(UserCreationError::UserAlreadyExists),
            Ok(None) => {
                let hashed_password = bcrypt::hash(user_data.password.clone(), DEFAULT_COST);
                let new_user = UsersActiveModel {
                    username: Set(user_data.user.clone()),
                    password: Set(hashed_password.unwrap()),
                };
                let new_user = new_user.insert(&*self.db).await;
                match new_user {
                    Ok(_) => Ok(()),
                    Err(_) => return Err(UserCreationError::GenericError),
                }
            }
            Err(_) => return Err(UserCreationError::GenericError),
        }
    }
}
