fn color_f32_to_u8(pigment: &f32) -> u32 {
    (pigment * 255.) as u32
}

pub trait Hex {
    fn into_hex(&self) -> String;
}

impl Hex for iced::Color {
    fn into_hex(&self) -> String {
        format!(
            "#{:06x}",
            color_f32_to_u8(&self.r) << 16
                | color_f32_to_u8(&self.g) << 8
                | color_f32_to_u8(&self.b)
        )
    }
}
