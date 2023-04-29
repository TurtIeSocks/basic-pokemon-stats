pub use sea_orm_migration::prelude::*;

mod m20230315_001452_initial;
mod m20230429_043512_timestamp;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230315_001452_initial::Migration),
            Box::new(m20230429_043512_timestamp::Migration),
        ]
    }
}
