use glam::*;

// State of the camera, used for view matrix
#[derive(Debug)]
pub struct Camera {
    // location/direction
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,

    // rotation
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    // projection
    pub fov: f32
}

impl Camera {
    pub fn new(position: Vec3, front: Vec3, up: Vec3) -> Self {
        Camera { 
            position, 
            front, 
            up,
            yaw: 0.0,
            pitch: 90.0,
            roll: 0.0,
            fov: 45.0
        }
    }

    pub fn look_at(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn update_with_mouse_offset(&mut self, offset: Vec2) {
        self.yaw += offset.x;
        self.pitch += offset.y;
    
        if self.pitch > 89.0 { self.pitch = 89.0; }
        else if self.pitch < -89.0 { self.pitch = -89.0; }
    
        // read here for explanation: https://learnopengl.com/Getting-started/Camera
        let mut direction = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        );

        self.front = direction.normalize();
    }
}