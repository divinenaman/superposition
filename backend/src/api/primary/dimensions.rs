use actix_web:: {
    Either::{Left},
    get,
    post,
    web::{Path, Json, Data},
};
use crate::models::db_models::Dimension;
use serde::{Serialize, Deserialize};

use crate::{
    messages::dimensions::{
        FetchDimensions,
        FetchDimension,
        CreateDimension
    }, AppState, DbActor};

use actix::Addr;

use crate::utils::errors::{
    AppError,
    AppErrorType::{
        DBError,
        NotFound,
        DataExists
    }
};

// Get dimension table
#[get("")]
pub async fn get_dimensions(state: Data<AppState>) -> Result<Json<Vec<Dimension>>, AppError> {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchDimensions).await {
        Ok(Ok(result)) => Ok(Json(result)),
        Ok(Err(err)) => Err(AppError {
            message: Some("failed to get dimensions".to_string()),
            cause: Some(Left(err.to_string())),
            status: NotFound
        }),
        Err(err) => Err(AppError {
            message: None,
            cause: Some(Left(err.to_string())),
            status: DBError
        })
    }

}

// Get request to fetch dimension from dimension name
#[derive(Deserialize, Serialize)]
pub struct Key {
    dimension: String,
}

#[get("/{dimension}")]
pub async fn get_dimension_key(state: Data<AppState>, params: Path<Key>) -> Result<Json<Dimension>, AppError> {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let dimension_key = params.into_inner().dimension;

    match db.send(FetchDimension {dimension: dimension_key}).await {
        Ok(Ok(result)) => Ok(Json(result)),
        Ok(Err(err)) => Err(AppError {
            message: Some("failed to get required dimension".to_string()),
            cause: Some(Left(err.to_string())),
            status: NotFound
        }),
        Err(err) => Err(AppError {
            message: None,
            cause: Some(Left(err.to_string())),
            status: DBError
        })
    }
}


// Post request to add key, value to dimension table
#[derive(Deserialize, Serialize, Clone)]
pub struct KeyValue {
    dimension: String,
    priority: i32,
}

#[post("")]
pub async fn post_dimension(state: Data<AppState>, body: Json<KeyValue>) -> Result<Json<Dimension>, AppError> {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateDimension {
        dimension: body.dimension.clone(),
        priority: body.priority
    }).await {

        Ok(Ok(result)) => Ok(Json(result)),
        Ok(Err(err)) => Err(AppError {
            message: Some("failed to add dimension".to_string()),
            cause: Some(Left(err.to_string())),
            status: DataExists
        }),
        Err(err) => Err(AppError {
            message: None,
            cause: Some(Left(err.to_string())),
            status: DBError
        })
    }
}

