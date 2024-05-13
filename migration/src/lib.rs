pub use sea_orm_migration::prelude::*;

mod m20240513_023253_create_base_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240513_023253_create_base_tables::Migration),
        ]
    }
}
