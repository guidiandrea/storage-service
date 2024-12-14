use migration::{sea_orm::{Database, DatabaseConnection, DbErr}, MigratorTrait};

#[async_std::main]
async fn main() -> Result<(), DbErr>{
    
    let db = init_db().await.unwrap();
    migration::Migrator::up(&db, None).await?; 
    Ok(())
}


async fn init_db() -> Result<DatabaseConnection,DbErr>{
    let db = Database::connect("postgres://postgres:password@localhost:5432/app").await?;
    Ok(db)
}