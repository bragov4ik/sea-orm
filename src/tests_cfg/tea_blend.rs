use super::sea_orm_active_enums::*;
use crate as sea_orm;
use crate::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tea_blend")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tea: Tea,
    #[sea_orm(primary_key)]
    pub blend_part_variety: String,
    pub mass_grams: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::lunch_set::Entity",
    from = "Column::Tea"
    to = "super::lunch_set::Column::Tea")]
    LunchSet,
}

impl Related<super::lunch_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LunchSet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
