use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(
    pixel_color: Color,
    pixel_data: &mut Vec<u8>,
    img_width: usize,
    y: usize,
    x: usize,
) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);

    let r_byte = (256.0 * intensity.clamp(r)) as u8;
    let g_byte = (256.0 * intensity.clamp(g)) as u8;
    let b_byte = (256.0 * intensity.clamp(b)) as u8;

    let index = (y * img_width + x) * 4;

    if index + 3 < pixel_data.len() {
        pixel_data[index] = r_byte;
        pixel_data[index + 1] = g_byte;
        pixel_data[index + 2] = b_byte;
        pixel_data[index + 3] = 255;
    }
}
