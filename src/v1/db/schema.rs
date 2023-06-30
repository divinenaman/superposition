// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dimension_type"))]
    pub struct DimensionType;
}

diesel::table! {
    contexts (id) {
        id -> Varchar,
        value -> Json,
        override_id -> Varchar,
        created_at -> Timestamptz,
        created_by -> Varchar,
        priority -> Int4,
        #[sql_name = "override"]
        override_ -> Json,
    }
}

diesel::table! {
    default_configs (key) {
        key -> Varchar,
        value -> Json,
        created_at -> Timestamptz,
        created_by -> Varchar,
        schema -> Json,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DimensionType;

    dimensions (dimension) {
        dimension -> Varchar,
        priority -> Int4,
        #[sql_name = "type"]
        type_ -> DimensionType,
        created_at -> Timestamptz,
        created_by -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(contexts, default_configs, dimensions,);