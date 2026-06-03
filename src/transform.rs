use std::sync::Arc;

use crate::{aabb::AABB, ray::ray::Ray, surface::surface::Surface};

#[derive(Clone, Copy)]
pub struct Transform3 {
    m: [[f32; 3]; 3],
    t: [f32; 3],
}

impl Transform3 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
            t: [0.0, 0.0, 0.0]
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut s = Self::identity();
        s.t = [x, y, z];
        s
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            m: [
                [sx, 0.0, 0.0],
                [0.0, sy, 0.0],
                [0.0, 0.0, sz],
            ],
            t: [0.0, 0.0, 0.0],
        }
    }

    pub fn rotate_x(theta: f32) -> Self {
        let (s, c) = theta.to_radians().sin_cos();

        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, c, -s],
                [0.0, s, c],
            ],
            t: [0.0, 0.0, 0.0],
        }
    }

    pub fn rotate_y(theta: f32) -> Self {
        let (s, c) = theta.to_radians().sin_cos();

        Self {
            m: [
                [c, 0.0, s],
                [0.0, 1.0, 0.0],
                [-s, 0.0, c],
            ],
            t: [0.0, 0.0, 0.0],
        }
    }

    pub fn rotate_z(theta: f32) -> Self {
        let (s, c) = theta.to_radians().sin_cos();

        Self {
            m: [
                [c, -s, 0.0],
                [s,  c, 0.0],
                [0.0, 0.0, 1.0],
            ],
            t: [0.0, 0.0, 0.0],
        }
    }
}

impl Transform3 {
    fn apply(&self, v: [f32; 3]) -> [f32; 3] {
        let x = self.m[0][0]*v[0] + self.m[0][1]*v[1] + self.m[0][2]*v[2] + self.t[0];
        let y = self.m[1][0]*v[0] + self.m[1][1]*v[1] + self.m[1][2]*v[2] + self.t[1];
        let z = self.m[2][0]*v[0] + self.m[2][1]*v[1] + self.m[2][2]*v[2] + self.t[2];
        [x, y, z]
    }

    fn apply_dir(&self, v: [f32; 3]) -> [f32; 3] {
        let x = self.m[0][0]*v[0] + self.m[0][1]*v[1] + self.m[0][2]*v[2];
        let y = self.m[1][0]*v[0] + self.m[1][1]*v[1] + self.m[1][2]*v[2];
        let z = self.m[2][0]*v[0] + self.m[2][1]*v[1] + self.m[2][2]*v[2];
        [x, y, z]
    }
}

pub struct Transform {
    object: Arc<dyn Surface + Send + Sync>,
    transform: Transform3,
    inv_transform: Transform3,
    bbox: AABB,
}

impl Surface for Transform {
    fn hit(&self, r: &crate::ray::ray::Ray, ray_t: crate::interval::Interval, rec: &mut crate::surface::surface::HitRecord) -> bool {
        let o = self.inv_transform.apply([r.origin().x, r.origin().y, r.origin().z]);
        let d = self.inv_transform.apply_dir([r.direction().x, r.direction().y, r.direction().z]);

        let local_ray = Ray::new(
            o.into(),
            d.into()
        );

        if !self.object.hit(&local_ray, ray_t, rec) {
            return false;
        }

        let p = self.transform.apply([rec.p.x, rec.p.y, rec.p.z]);
        let n = self.transform.apply_dir([rec.normal.x, rec.normal.y, rec.normal.z]);

        rec.p = p.into();
        rec.normal = n.into();

        true
    }

    fn bounding_box(&self) -> &crate::aabb::AABB {
        &self.bbox
    }
}

impl Transform {
    pub fn translate(object: Arc<dyn Surface + Send + Sync>, x: f32, y: f32, z: f32) -> Self {
        let m = Transform3::translate(x, y, z);
        let inv = Transform3::translate(-x, -y, -z);
        let bbox = object.bounding_box().offset(x, y, z);
        Self { object, transform: m, inv_transform: inv, bbox }
    }

    pub fn rotate_x(object: Arc<dyn Surface + Send + Sync>, angle: f32) -> Self {
        let m = Transform3::rotate_x(angle);
        let inv = Transform3::rotate_x(-angle);
        let bbox = compute_rotated_bbox(object.bounding_box(), &m);
        Self { object, transform: m, inv_transform: inv, bbox }
    }

    pub fn rotate_y(object: Arc<dyn Surface + Send + Sync>, angle: f32) -> Self {
        let m = Transform3::rotate_y(angle);
        let inv = Transform3::rotate_y(-angle);
        let bbox = compute_rotated_bbox(object.bounding_box(), &m);
        Self { object, transform: m, inv_transform: inv, bbox }
    }

    pub fn rotate_z(object: Arc<dyn Surface + Send + Sync>, angle: f32) -> Self {
        let m = Transform3::rotate_z(angle);
        let inv = Transform3::rotate_z(-angle);
        let bbox = compute_rotated_bbox(object.bounding_box(), &m);
        Self { object, transform: m, inv_transform: inv, bbox }
    }

    pub fn scale(object: Arc<dyn Surface + Send + Sync>, sx: f32, sy: f32, sz: f32) -> Self {
        let m = Transform3::scale(sx, sy, sz);
        let inv = Transform3::scale(1.0/sx, 1.0/sy, 1.0/sz);
        let bbox = compute_rotated_bbox(object.bounding_box(), &m); // works for scale too
        Self { object, transform: m, inv_transform: inv, bbox }
    }
}

fn compute_rotated_bbox(bbox: &AABB, m: &Transform3) -> AABB {
    let min = [bbox.x.min, bbox.y.min, bbox.z.min];
    let max = [bbox.x.max, bbox.y.max, bbox.z.max];

    let mut new_min = [f32::INFINITY; 3];
    let mut new_max = [f32::NEG_INFINITY; 3];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let corner = [
                    if i == 0 { min[0] } else { max[0] },
                    if j == 0 { min[1] } else { max[1] },
                    if k == 0 { min[2] } else { max[2] },
                ];
                let rotated = m.apply_dir(corner);
                for c in 0..3 {
                    new_min[c] = new_min[c].min(rotated[c]);
                    new_max[c] = new_max[c].max(rotated[c]);
                }
            }
        }
    }

    AABB::new_from_vec(new_min.into(), new_max.into())
}