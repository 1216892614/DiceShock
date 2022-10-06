use self::error::{RequestEncodingDismatchError, RequestResult};

pub(super) mod error;

pub(crate) enum Request {
    Model(String),
    Image(String),
    Code(String),
    Icon(String),
    API(String),
}

impl Request {
    /// Request suffix will be matched like this:
    /// - "obj" => **Model**,
    /// - "jpg" | "png" => **Image**,
    /// - "wgsl" | "glsl" => **Code**,
    /// - "svg" => **Icon**,
    /// - _ => **API**,
    pub(crate) fn from_name(name: &str) -> Self {
        let suffix: Vec<_> = name.split(".").collect();
        match suffix[1] {
            "obj" => Self::Model(name.to_owned()),
            "jpg" | "png" => Self::Image(name.to_owned()),
            "wgsl" | "glsl" => Self::Code(name.to_owned()),
            "svg" => Self::Icon(name.to_owned()),
            _ => Self::API(name.to_owned()),
        }
    }

    fn url(&self) -> String {
        match self {
            Request::Model(s) => format!("static/models/{}/", s),
            Request::Image(s) => format!("static/images/{}/", s),
            Request::Icon(s) => format!("static/icons/{}/", s),
            Request::Code(s) => format!("static/code/{}/", s),
            Request::API(s) => format!("api/{}/", s),
        }
    }

    pub(crate) async fn request_utf8(&self) -> RequestResult<String> {
        let url = match self {
            Self::Model(_) | Self::Code(_) | Self::Icon(_) | Self::API(_) => self.url(),
            _ => Err(RequestEncodingDismatchError {
                url: self.url(),
                try_to_encode_as: "utf8",
            })?,
        };

        Ok(gloo::net::http::Request::get(&url)
            .send()
            .await?
            .text()
            .await?)
    }

    async fn request_bytes(&self) -> RequestResult<Vec<u8>> {
        let url = match self {
            Self::Image(_) => self.url(),
            _ => Err(RequestEncodingDismatchError {
                url: self.url(),
                try_to_encode_as: "bytes",
            })?,
        };

        Ok(gloo::net::http::Request::get(&url)
            .send()
            .await?
            .binary()
            .await?)
    }
}

use crate::error::{AppError, AppResult};
use crate::resources::{image::Image, model::Model};
use std::io::{BufReader, Cursor};
impl Request {
    async fn request_image(&self) -> AppResult<Image> {
        Ok(Image::from_binary(&self.request_bytes().await?)?)
    }

    async fn request_model(&self) -> RequestResult<Model> {
        let obj_text = self.request_utf8().await?;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = match Request::from_name(&p).request_utf8().await {
                    Ok(s) => s,
                    Err(e) => {
                        let e: AppError = e.into();
                        e.submit("request material error");
                        panic!()
                    }
                };
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await?;

        Ok(Model::new(models, obj_materials?))
    }
}
