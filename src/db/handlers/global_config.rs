use crate::db::models::{db_models::GlobalConfig, insertables::global_config::NewGlobalConfigKey};
use crate::db::utils::DbActor;

use crate::db::messages::global_config::{CreateGlobalKey, FetchConfigKey, FetchGlobalConfig};
use crate::db::schema::global_config::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchGlobalConfig> for DbActor {
    type Result = QueryResult<Vec<GlobalConfig>>;

    fn handle(&mut self, _msg: FetchGlobalConfig, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch GlobalConfig: Unable to establish connection");

        global_config.get_results::<GlobalConfig>(&mut conn)
    }
}

impl Handler<FetchConfigKey> for DbActor {
    type Result = QueryResult<GlobalConfig>;

    fn handle(&mut self, msg: FetchConfigKey, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Dimension: Unable to establish connection");

        global_config
            .filter(key.eq(msg.key))
            .get_result::<GlobalConfig>(&mut conn)
    }
}

impl Handler<CreateGlobalKey> for DbActor {
    type Result = QueryResult<GlobalConfig>;

    fn handle(&mut self, msg: CreateGlobalKey, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Dimension: Unable to establish connection");

        let new_key = NewGlobalConfigKey {
            key: msg.key,
            value: msg.value,
        };

        diesel::insert_into(global_config)
            .values(new_key)
            .get_result::<GlobalConfig>(&mut conn)
    }
}