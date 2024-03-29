use glam::*;
use super::camera::Camera;

/// Obtains the transform matrix used for coordinate manipulation.
/// Composed of model, view, and projection matrices.
pub fn get_transform(camera: &Camera, object_position: Vec3, scale: Vec3) -> Mat4 {
    let model = get_model(scale);
    let view = get_view(&camera, object_position);
    let projection = get_projection(camera.fov);

    projection * view * model
}

/// Obtain the projection, view and model matrices as a tuple.
pub fn get_transform_matrices(camera: &Camera, object_position: Vec3, scale: Vec3) -> (Mat4, Mat4, Mat4) {
    let model = get_model(scale);
    let view = get_view(&camera, object_position);
    let projection = get_projection(camera.fov);

    (projection, view, model)
}

fn get_model(scale: Vec3) -> Mat4 {
    Mat4::from_scale(scale)
}

fn get_view(camera: &Camera, object_position: Vec3) -> Mat4 {
    camera.look_at() * Mat4::from_translation(object_position) 
}

fn get_projection(fov: f32) -> Mat4 {
    Mat4::perspective_rh_gl(   
        f32::to_radians(fov), 
        800.0/600.0, 
        0.1, 
        100.0
    )
}