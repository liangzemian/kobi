use crate::database::download::{download_comic_chapter, DOWNLOAD_DATABASE};
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, IntoActiveModel, QuerySelect};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "download_comic_page")]
pub struct Model {
    pub comic_path_word: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub chapter_uuid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub image_index: i32,
    pub url: String,
    pub cache_key: String,
    pub download_status: i64,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_comic_path_word",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["comic_path_word"],
            "download_comic_page_idx_comic_path_word",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_chapter_uuid",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["chapter_uuid"],
            "download_comic_page_idx_chapter_uuid",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_cache_key",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["cache_key"],
            "download_comic_page_idx_cache_key",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_url",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["url"],
            "download_comic_page_idx_url",
        )
        .await;
    }
}

pub(crate) async fn save(db: &impl ConnectionTrait, model: Model) -> Result<ActiveModel, DbErr> {
    model.into_active_model().save(db).await
}

pub(crate) async fn has_download_pic(cache_key: String) -> anyhow::Result<Option<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find()
        .filter(Expr::col(Column::CacheKey).eq(cache_key))
        .limit(1)
        .one(db.deref())
        .await?)
}
