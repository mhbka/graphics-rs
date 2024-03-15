use glam::*;

// State of the camera, used for view matrix
pub struct Camera {
    pub location: Vec3,
    pub target: Vec3,
    pub up: Vec3
}

impl Camera {
    pub fn new(location: Vec3, target: Vec3, up: Vec3) -> Self {
        Camera { location, target, up }
    }

    pub fn look_at(&self) -> Mat4 {
        Mat4::look_at_rh(self.location, self.target, self.up)
    }
}

/// Obtains the transform matrix used for coordinate manipulation.
/// Composed of model, view, and projection matrices.
pub fn get_transform(camera: &Camera, fov: f32, object_position: Vec3) -> Mat4 {
    let model = get_model();
    let view = get_view(&camera, object_position);
    let projection = get_projection(fov);

    projection * view * model
}

fn get_model() -> Mat4 {
    Mat4::IDENTITY
}

fn get_view(camera: &Camera, object_position: Vec3) -> Mat4 {
    // assert_eq!(camera.look_at(), Mat4::from_translation(camera.location - camera.target));

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