use glam::DVec3;

// Struct member `x`, `y`, `z` respectively represent red, green, blue component in a color.
pub type Color = DVec3;

// gamma correct power coefficient
const SRGB_GAMMA: f64 = 2.2;

// type annotation needed for const 
pub const BLACK: Color = Color::ZERO;
pub const GREY: Color = Color::splat(0.5);
pub const WHITE: Color = Color::splat(1.0);
pub const RED: Color = Color::new(0.65, 0.05, 0.05);
pub const GREEN: Color = Color::new(0.12, 0.45, 0.15);
pub const BLUE: Color = Color::new(0.2, 0.4, 0.9);

/// Convert pixel rgb values from [0, 1) to [0, 255] with gamma correct.
pub fn color_bytes(color: Color) -> [u8; 3] {
    [
        (256.0 * color.x.clamp(0.0, 0.999).powf(1.0 / SRGB_GAMMA)) as u8,
        (256.0 * color.y.clamp(0.0, 0.999).powf(1.0 / SRGB_GAMMA)) as u8,
        (256.0 * color.z.clamp(0.0, 0.999).powf(1.0 / SRGB_GAMMA)) as u8,
    ]
}
