use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create folders table
        manager
            .create_table(
                Table::create()
                    .table(Folders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Folders::FolderId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Folders::FolderName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Folders::FolderParentId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Folders::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Folders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Folders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_folder_parent")
                            .from(Folders::Table, Folders::FolderParentId)
                            .to(Folders::Table, Folders::FolderId)
                    )
                    .to_owned(),
            )
            .await?;

        // Create files table
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::FileId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Files::Filename)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Files::Size)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Files::Path)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Files::FolderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Files::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Files::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Files::UserId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_files_folder")
                            .from(Files::Table, Files::FolderId)
                            .to(Folders::Table, Folders::FolderId)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to handle foreign key constraints
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Folders::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
    Username,
    Password,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Folders {
    Table,
    FolderId,
    FolderName,
    FolderParentId,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Files {
    Table,
    FileId,
    Filename,
    Size,
    Path,
    FolderId,
    CreatedAt,
    ModifiedAt,
    UserId,
}
