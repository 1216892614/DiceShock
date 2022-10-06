use std::fmt::Display;

pub(crate) type ResourcesResult<T> = Result<T, ResourcesError>;

#[derive(Debug)]
pub(crate) enum ResourcesError {
    ImageError(image::ImageError),
    LoadModelError(tobj::LoadError),
}

impl Display for ResourcesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ResourcesError::ImageError(e) => e.to_string(),
            ResourcesError::LoadModelError(e) => e.to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for ResourcesError {}

impl From<image::ImageError> for ResourcesError{
    fn from(e: image::ImageError) -> Self {
        Self::ImageError(e)
    }
}

impl From<tobj::LoadError> for ResourcesError{
    fn from(e: tobj::LoadError) -> Self {
        Self::LoadModelError(e)
    }
}
