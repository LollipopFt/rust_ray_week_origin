use std::{f64,ops};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(pos0: f64, pos1: f64, pos2: f64) -> Self {
        Vec3 {
            e: [pos0, pos1, pos2]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_sq())
    }

    pub fn length_sq(&self) -> f64 {
         self.e[0]*self.e[0]
        +self.e[1]*self.e[1]
        +self.e[2]*self.e[2]
    }

    pub fn fmt(&self) -> String {
        format!("{} {} {}",
            (255.999*self.x()) as u16,
            (255.999*self.y()) as u16,
            (255.999*self.z()) as u16,
        )
    }

    pub fn prt(&self) {
        println!("{}", self.fmt());
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
         self.e[0]*rhs.e[0]
        +self.e[1]*rhs.e[1]
        +self.e[2]*rhs.e[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            e: [
                self.e[1]*rhs.e[2]-self.e[2]*rhs.e[1],
                self.e[2]*rhs.e[0]-self.e[0]*rhs.e[2],
                self.e[0]*rhs.e[1]-self.e[1]*rhs.e[0]
            ]
        }
    }

    pub fn unit_vec(self) -> Self {
        self/self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            e: [ -self.e[0], -self.e[1], -self.e[2] ]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0]+=rhs.e[0];
        self.e[1]+=rhs.e[1];
        self.e[2]+=rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0]*=rhs;
        self.e[1]*=rhs;
        self.e[2]*=rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0]/=rhs;
        self.e[1]/=rhs;
        self.e[2]/=rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0]+other.e[0],
                self.e[1]+other.e[1],
                self.e[2]+other.e[2]
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0]-other.e[0],
                self.e[1]-other.e[1],
                self.e[2]-other.e[2]
            ]
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            e: [
                self.e[0]*rhs.e[0],
                self.e[1]*rhs.e[1],
                self.e[2]*rhs.e[2]
            ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            e: [ self.e[0]*rhs, self.e[1]*rhs, self.e[2]*rhs ]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3 {
            e: [
                self.e[0]/rhs,
                self.e[1]/rhs,
                self.e[2]/rhs
            ]
        }
    }
}