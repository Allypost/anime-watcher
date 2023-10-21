use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Series table
        {
            manager
                .create_table(
                    Table::create()
                        .table(Series::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Series::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(
                            ColumnDef::new(Series::Name)
                                .string()
                                .not_null()
                                .extra("COLLATE NOCASE"),
                        )
                        .col(ColumnDef::new(Series::Description).string())
                        .col(
                            ColumnDef::new(Series::CreatedAt)
                                .date_time()
                                .default(Expr::current_timestamp()),
                        )
                        .col(
                            ColumnDef::new(Series::UpdatedAt)
                                .date_time()
                                .default(Expr::current_timestamp()),
                        )
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx__series__name")
                        .table(Series::Table)
                        .col(Series::Name)
                        .unique()
                        .full_text()
                        .to_owned(),
                )
                .await?;
        }

        // SeriesSources table
        {
            manager
                .create_table(
                    Table::create()
                        .table(SeriesSources::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(SeriesSources::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(
                            ColumnDef::new(SeriesSources::ForSeriesId)
                                .integer()
                                .not_null(),
                        )
                        .col(
                            ColumnDef::new(SeriesSources::SeriesSite)
                                .string()
                                .not_null(),
                        )
                        .col(
                            ColumnDef::new(SeriesSources::SeriesSiteId)
                                .string()
                                .not_null(),
                        )
                        .col(
                            ColumnDef::new(SeriesSources::CreatedAt)
                                .date_time()
                                .default(Expr::current_timestamp()),
                        )
                        .col(
                            ColumnDef::new(SeriesSources::UpdatedAt)
                                .date_time()
                                .default(Expr::current_timestamp()),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk__series_sources__for_series_id")
                                .from(SeriesSources::Table, SeriesSources::ForSeriesId)
                                .to(Series::Table, Series::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("idx__series_sources__series_site")
                        .table(SeriesSources::Table)
                        .col(SeriesSources::SeriesSite)
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx__series_sources__unique_on_site")
                        .table(SeriesSources::Table)
                        .col(SeriesSources::SeriesSiteId)
                        .col(SeriesSources::SeriesSite)
                        .unique()
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Series::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SeriesSources::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Series {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum SeriesSources {
    Table,
    Id,
    ForSeriesId,
    SeriesSite,
    SeriesSiteId,
    CreatedAt,
    UpdatedAt,
}
