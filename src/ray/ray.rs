use nalgebra::Vector3;

pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
    primary: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        let primary = b.normalize();
        Ray { a, b, primary }
    }
    pub fn new_with_primary(a: Vector3<f32>, b: Vector3<f32>, primary: Vector3<f32>) -> Self {
        Ray { a, b, primary }
    }
    pub fn origin(&self) -> Vector3<f32> {
        self.a
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.b
    }
    pub fn primary_direction(&self) -> Vector3<f32> {
        self.primary
    }
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.a + t * self.b
    }
}
