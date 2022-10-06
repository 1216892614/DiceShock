use super::error::ResourcesResult;

#[derive(Debug, Clone)]
pub(crate) struct Image {
    diffuse_rgba: image::RgbaImage,
    /// (width, height) of image texture
    size: (u32, u32),
}

impl Image {
    pub(crate) fn new(diffuse_rgba: image::RgbaImage, dimensions: (u32, u32)) -> Self {
        Self {
            diffuse_rgba,
            size: dimensions,
        }
    }

    pub(crate) fn from_binary(b: &[u8]) -> ResourcesResult<Self> {
        let diffuse_image = image::load_from_memory(b)?;
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let size = diffuse_image.dimensions();

        Ok(Self {
            diffuse_rgba,
            size,
        })
    }
    
    /// (width, height) of image texture
    pub(crate) fn size(&self) -> (u32, u32) {
        self.size
    }
}
