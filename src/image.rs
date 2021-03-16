use image::{io::Reader, ImageFormat, error::ImageError};
use base64::{encode};
use std::io::Cursor;

pub struct Error;
impl From<ImageError> for Error {
    fn from(_: ImageError) -> Self {
        Error
    }
}


pub fn filter_image(data: &[u8]) -> Result<String, Error> {
    let mut reader = Reader::new(Cursor::new(data));
    reader.set_format(ImageFormat::Png);
    let image = reader.decode()?; // ImageResult<DynamicImage>
    let image = image.grayscale().filter3x3(&[-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0]);
    let mut buf = vec![];
    image.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(encode(buf))
}

