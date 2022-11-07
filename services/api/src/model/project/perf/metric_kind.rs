use std::str::FromStr;

use bencher_json::{
    project::report::{metric_kind::JsonNewMetricKind, JsonMetricKind},
    ResourceId,
};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use dropshot::HttpError;
use uuid::Uuid;

use crate::{
    error::api_error,
    model::project::QueryProject,
    schema,
    schema::metric_kind as metric_kind_table,
    util::{resource_id::fn_resource_id, slug::unwrap_slug},
    ApiError,
};

fn_resource_id!(metric_kind);

#[derive(Queryable)]
pub struct QueryMetricKind {
    pub id: i32,
    pub uuid: String,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub units: Option<String>,
}

impl QueryMetricKind {
    pub fn get_id(conn: &mut SqliteConnection, uuid: impl ToString) -> Result<i32, ApiError> {
        schema::metric_kind::table
            .filter(schema::metric_kind::uuid.eq(uuid.to_string()))
            .select(schema::metric_kind::id)
            .first(conn)
            .map_err(api_error!())
    }

    pub fn get_uuid(conn: &mut SqliteConnection, id: i32) -> Result<Uuid, ApiError> {
        let uuid: String = schema::metric_kind::table
            .filter(schema::metric_kind::id.eq(id))
            .select(schema::metric_kind::uuid)
            .first(conn)
            .map_err(api_error!())?;
        Uuid::from_str(&uuid).map_err(api_error!())
    }

    pub fn from_resource_id(
        conn: &mut SqliteConnection,
        metric_kind: &ResourceId,
    ) -> Result<Self, ApiError> {
        schema::metric_kind::table
            .filter(resource_id(metric_kind)?)
            .first::<QueryMetricKind>(conn)
            .map_err(api_error!())
    }

    pub fn into_json(self, conn: &mut SqliteConnection) -> Result<JsonMetricKind, ApiError> {
        let Self {
            id: _,
            uuid,
            project_id,
            name,
            slug,
            units,
        } = self;
        Ok(JsonMetricKind {
            uuid: Uuid::from_str(&uuid).map_err(api_error!())?,
            project: QueryProject::get_uuid(conn, project_id)?,
            name,
            slug,
            units,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = metric_kind_table)]
pub struct InsertMetricKind {
    pub uuid: String,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub units: Option<String>,
}

impl InsertMetricKind {
    pub fn from_json(
        conn: &mut SqliteConnection,
        project: &ResourceId,
        metric_kind: JsonNewMetricKind,
    ) -> Result<Self, HttpError> {
        Self::from_json_inner(
            conn,
            QueryProject::from_resource_id(conn, project)?.id,
            metric_kind,
        )
    }

    pub fn from_json_inner(
        conn: &mut SqliteConnection,
        project_id: i32,
        metric_kind: JsonNewMetricKind,
    ) -> Result<Self, HttpError> {
        let JsonNewMetricKind { name, slug, units } = metric_kind;
        let slug = unwrap_slug!(conn, &name, slug, metric_kind, QueryMetricKind);
        Ok(Self {
            uuid: Uuid::new_v4().to_string(),
            project_id,
            name,
            slug,
            units,
        })
    }
}