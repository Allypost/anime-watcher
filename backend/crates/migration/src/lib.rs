pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20231011_082054_add_mal_id_to_series;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20231011_082054_add_mal_id_to_series::Migration),
        ]
    }
}
