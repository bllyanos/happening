use sqlx::PgPool;

use super::{
    error::WorkspaceServiceError,
    model::{CreateWorkspace, Workspace},
};

#[derive(Clone)]
pub struct WorkspaceService {
    db: PgPool,
}

struct InsertResult {
    id: i64,
    slug: String,
}

impl WorkspaceService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_one_by_slug(&self, slug: &str) -> Result<Workspace, WorkspaceServiceError> {
        sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE slug = $1", slug)
            .fetch_optional(&self.db)
            .await
            .map_err(|err| WorkspaceServiceError::UnknownSqlx(err))?
            .ok_or_else(|| WorkspaceServiceError::NotFound)
    }

    pub async fn create(&self, dto: CreateWorkspace) -> Result<Workspace, WorkspaceServiceError> {
        let result = sqlx::query_as!(
            InsertResult,
            "INSERT INTO workspaces (owner_id, name, slug) values ($1, $2, $3) RETURNING id, slug",
            dto.owner_id,
            dto.name,
            dto.slug,
        )
        .fetch_all(&self.db)
        .await;

        todo!();
    }
}
