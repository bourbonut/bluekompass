use eframe::egui::{TextureId, TextureOptions, ColorImage, TextureHandle, Ui};
use std::path::{Path, PathBuf};
use image::ImageError;

fn load_image_from_path(path: &Path) -> Result<ColorImage, ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

pub struct BlueKompassImage {
    image_path: Box<Path>,
    texture: Option<TextureHandle>,
}

impl BlueKompassImage {
    pub fn new(image_path: PathBuf) -> Self {
        Self { texture: None, image_path: image_path.into() }
    }
}

impl BlueKompassImage {
    pub fn load(&mut self, ui: &mut Ui) -> (TextureId, [usize; 2]) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                load_image_from_path(&self.image_path).unwrap(),
                TextureOptions::NEAREST
            )
        });
        (texture.id(), texture.size())
    }
}
