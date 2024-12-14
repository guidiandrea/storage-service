pub use sea_orm_migration::prelude::*;
mod m20241214_154546_first_migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241214_154546_first_migration::Migration),
        ]
    }
}
