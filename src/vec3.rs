use std::{
    f64,
    io::{self, Write},
    ops
};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    list: [f64; 3]
}

impl Vec3 {
    pub fn new(pos0: f64, pos1: f64, pos2: f64) -> Vec3 {
        Vec3 {
            list: [pos0, pos1, pos2]
        }
    }

    pub fn x(&self) -> f64 {
        self.list[0]
    }

    pub fn y(&self) -> f64 {
        self.list[1]
    }

    pub fn z(&self) -> f64 {
        self.list[2]
    }

    pub fn length_sq(&self) -> f64 {
          self.list[0]*self.list[0]
        + self.list[1]*self.list[1]
        + self.list[2]*self.list[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_sq())
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            list: [
                -self.list[0],
                -self.list[1],
                -self.list[2],
            ]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            list: [
                self.list[0]+rhs.list[0],
                self.list[1]+rhs.list[1],
                self.list[2]+rhs.list[2],
            ]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            list: [
                self.list[0]*rhs,
                self.list[1]*rhs,
                self.list[2]*rhs,
            ]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            list: [
                self.list[0]/rhs,
                self.list[1]/rhs,
                self.list[2]/rhs,
            ]
        }
    }
}

// utility

impl Vec3 {
    pub fn prnt(&self, handle: &mut io::BufWriter<io::Stdout>) {
        writeln!(handle, "{} {} {}", self.list[0], self.list[1], self.list[2]).ok();
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Vec3 {
            list: [
                self.list[0]+rhs.list[0],
                self.list[1]+rhs.list[1],
                self.list[2]+rhs.list[2]
            ]
        }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        Vec3 {
            list: [
                self.list[0]-rhs.list[0],
                self.list[1]-rhs.list[1],
                self.list[2]-rhs.list[2]
            ]
        }
    }

    pub fn mulv(&self, rhs: &Self) -> Self {
        Vec3 {
            list: [
                self.list[0]*rhs.list[0],
                self.list[1]*rhs.list[1],
                self.list[2]*rhs.list[2]
            ]
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
          self.list[0]*rhs.list[0]
        + self.list[1]*rhs.list[1]
        + self.list[2]*rhs.list[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            list: [
                self.list[1]*rhs.list[2]-self.list[2]*rhs.list[1],
                self.list[2]*rhs.list[0]-self.list[0]*rhs.list[2],
                self.list[0]*rhs.list[1]-self.list[1]*rhs.list[0]
            ]
        }
    }

    pub fn mul(&self, rhs: f64) -> Self {
        Vec3 {
            list: [
                self.list[0]*rhs,
                self.list[1]*rhs,
                self.list[2]*rhs
            ]
        }
    }

    pub fn div(&self, rhs: f64) -> Self {
        Vec3 {
            list: [
                self.list[0]/rhs,
                self.list[1]/rhs,
                self.list[2]/rhs
            ]
        }
    }

    pub fn unit_vec(&self) -> Self {
        Vec3 {
            list: [
                self.list[0]/self.length(),
                self.list[1]/self.length(),
                self.list[2]/self.length()
            ]
        }
    }
}

//colour

impl Vec3 {
    pub fn write_colour(&self, handle: &mut io::BufWriter<io::Stdout>) {
        writeln!(handle, "{} {} {}", (255.999*self.x()) as u8, (255.999*self.y()) as u8, (255.999*self.z()) as u8).ok();
    }
}