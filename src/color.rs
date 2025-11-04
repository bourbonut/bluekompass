fn color_f32_to_u8(pigment: &f32) -> u8 {
    (pigment * 255.) as u8
}

pub trait Hex {
    fn into_hex(&self) -> String;
}

impl Hex for iced::Color {
    fn into_hex(&self) -> String {
        format!(
            "#{:x}{:x}{:x}",
            color_f32_to_u8(&self.r),
            color_f32_to_u8(&self.g),
            color_f32_to_u8(&self.b)
        )
    }
}
