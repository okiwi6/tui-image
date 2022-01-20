use image::{self, DynamicImage, GenericImageView};
use tui::widgets::Widget;
use tui::layout::Rect;
use tui::buffer::Buffer;
use viuer::{self, get_kitty_support, is_iterm_supported, KittySupport};

pub struct Image {
    image: DynamicImage,
}

impl Image {
    pub fn new<P>(filepath: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<std::path::Path>
    {
        let image = match image::open(filepath) {
            Ok(image) => image,
            Err(image::error::ImageError::IoError(error)) => return Err(error),
            _ => panic!("Something went horribly wrong reading the image"),
        };
        return Ok(Image{image})
    }
}

impl Widget for Image {
    fn render(self, area: Rect, _: &mut Buffer) {
        let true_image_support = is_iterm_supported() || !matches!(get_kitty_support(), KittySupport::None);

        let x_start = area.left();
        let y_start = area.top();   

        let x_size = area.right() - x_start;
        let y_size = area.bottom() - y_start;

        let aspect_ratio_image = self.image.width() as f32 / self.image.height() as f32;

        let real_x_size = x_size.min((y_size as f32 * aspect_ratio_image) as u16);
        let real_y_size = y_size.min((x_size as f32 / aspect_ratio_image) as u16);

        let conf = 
            viuer::Config {
                transparent: !true_image_support,
                x: x_start,
                y: y_start as i16,
                width: Some(real_x_size as u32),
                height: Some(real_y_size as u32 / 2),
                ..Default::default()
            };
        viuer::print(&self.image, &conf).expect("Could not draw file");
    }
}

