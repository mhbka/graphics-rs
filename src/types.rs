//! geometric types.


/// a 2D integer coordinate (ie, can represent a single pixel)
#[derive(Debug)]
pub struct Vec2Di {
    pub x: i32, pub y: i32
}

impl Vec2Di {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2Di {x, y}
    }
}


/// a 3D float coordinate; also works as a 3D vector etc.
#[derive(Clone, Copy, Debug)]
pub struct Vec3Df {
    pub x: f32, pub y: f32, pub z: f32
}

impl Vec3Df {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3Df {x, y, z}
    }

    pub fn to_coord(&self) -> Vec2Di {
        Vec2Di {x: self.x as i32, y: self.y as i32}
    }

    pub fn normalize(&mut self) {
        let magnitude = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        self.x = self.x / magnitude;
        self.y = self.y / magnitude;
        self.z = self.z / magnitude;
    }

    pub fn cross_product(&self, other_vec: &Self) -> Self {
        Vec3Df {
            x: self.y * other_vec.z - self.z * other_vec.y,
            y: self.z * other_vec.x - self.x * other_vec.z,
            z: self.x * other_vec.y - self.y * other_vec.x
        }
    }   

    pub fn scalar_product(&self, other_vec: &Self) -> f32 {
        self.x * other_vec.x +
        self.y * other_vec.y +
        self.z * other_vec.z
    }
}


// A Face, formed by connecting 3 vertices
// Note that in obj files, indexes start from 1
pub struct Face {
    pub vertices: [Vec3Df; 3]
}
