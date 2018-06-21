use cgmath;

pub type Float = f32;
pub type RasterFloat = f32;
pub type TimeSec = f64;
pub type Point3 = cgmath::Point3<Float>;
pub type Vector3 = cgmath::Vector3<Float>;
pub type Matrix4 = cgmath::Matrix4<Float>;

#[derive(Debug, PartialEq)]
pub enum Direction {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
