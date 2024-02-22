use glam::*;

/// Holds the 3 components of a transform:
///
/// viewport: Maps [-1, 1] into screenspace coordinates
///
/// projection: Performs perspective deformation
///
/// model_view: Shifts the "camera" and moves coordinate to its place in the world
#[derive(Clone, Copy)]
pub struct Transform {
    pub viewport: Affine3A,
    pub projection: Affine3A,
    pub model_view: Affine3A,
}

impl Transform {
    /// Obtain the entire transform.
    pub fn get_whole_transform(&self) -> Affine3A {
        self.viewport * self.projection * self.model_view
    }

    /// Obtain inverse-transpose of the entire transform.
    pub fn get_inverse_transpose(&self) -> Affine3A {
        Affine3A::from_mat4(
            Mat4::from(
                self.get_whole_transform()
            )
            .inverse()
            .transpose()
        )
    }
}

// initialize a transform
pub fn initialize_transform(height: usize, width: usize) -> Transform {
    let eye = Vec3::new(1.0, 1.0, 3.0);
    let centre = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let model_view = lookat(eye, centre, up);
    let projection = Affine3A::IDENTITY;
    let viewport = viewport(width/8, height/8, width*3/4, height*3/4);

    Transform { model_view, projection, viewport }
}

// Return matrix for transforming [0, 1] coordinates into screen cube coordinates
fn viewport(x: usize, y: usize, w: usize, h: usize) -> Affine3A {
    let mut m = Affine3A::IDENTITY;
    let depth = 255.0; // idk the guy said so

    m.translation[0] = x as f32 + w as f32/2.0;
    m.translation[1] = y as f32 + h as f32/2.0;
    m.translation[2] = depth / 2.0;

    m.x_axis[0] = w as f32 / 2.0;
    m.y_axis[1] = h as f32 / 2.0;
    m.z_axis[2] = depth / 2.0;

    m
}

// Calculate matrix to "move" camera
fn lookat(eye: Vec3, centre: Vec3, up: Vec3) -> Affine3A {
    let eye = Vec3::new(-eye.x, -eye.y, eye.z); // WHAT THE FUCK WHY??
    let z = (eye - centre).normalize();
    let x = up.cross(z.clone()).normalize();
    let y = z.cross(x.clone()).normalize();
    let mut model_view = Affine3A::IDENTITY;

    for i in 0..3 {
        model_view.x_axis[i] = x[i];
        model_view.y_axis[i] = y[i];
        model_view.z_axis[i] = z[i];
        model_view.translation[i] = -centre[i];
    };

    model_view
}