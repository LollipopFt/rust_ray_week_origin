use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    list: [Vec3; 2]
}

impl Ray {
    pub fn new(ori: Vec3, dir: Vec3) -> Self {
        Ray {list: [ori, dir]}
    }

    pub fn ori(&self) -> Vec3 {
        self.list[0]
    }

    pub fn dir(&self) -> Vec3 {
        self.list[1]
    }

    pub fn at(&self, t: f64) -> Vec3 {
            self.ori().add(&self.dir().mul(t))
    }
}