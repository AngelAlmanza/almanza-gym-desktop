use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    Auth(String),

    #[error("{0}")]
    Validation(String),

    #[error("No encontrado: {0}")]
    NotFound(String),

    #[error("Permiso denegado")]
    PermissionDenied,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
