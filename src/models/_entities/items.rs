//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: Option<String>,
    pub role_id: i32,
    pub classification_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::classifications::Entity",
        from = "Column::ClassificationId",
        to = "super::classifications::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Classifications,
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Roles,
}

impl Related<super::classifications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classifications.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}
