use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(ori: Vec3, dir: Vec3) -> Self {
        Ray {
            ori,
            dir
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }
}