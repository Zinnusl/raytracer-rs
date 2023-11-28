use contracts::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Pnt3 = Vec3;

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl Vec3 {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: impl Into<Vec3>) -> f64 {
        let other: Vec3 = other.into();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: impl Into<Vec3>) -> Vec3 {
        let other: Vec3 = other.into();
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Result<UnitVec3, Vec3> {
        let len = self.len();

        if len == 0.0 {
            return Err(*self);
        }

        Ok(UnitVec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        })
    }

    pub fn null() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl UnitVec3 {
    pub fn len(&self) -> f64 {
        1.0
    }

    pub fn dot(&self, other: impl Into<Vec3>) -> f64 {
        let other: Vec3 = other.into();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: impl Into<Vec3>) -> Vec3 {
        let other: Vec3 = other.into();
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> UnitVec3 {
        self.clone()
    }
}

impl From<UnitVec3> for Vec3 {
    fn from(unit_vec: UnitVec3) -> Self {
        Vec3 {
            x: unit_vec.x,
            y: unit_vec.y,
            z: unit_vec.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x + other.x)]
    #[ensures(ret.y == self.y + other.y)]
    #[ensures(ret.z == self.z + other.z)]
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x - other.x)]
    #[ensures(ret.y == self.y - other.y)]
    #[ensures(ret.z == self.z - other.z)]
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x * other)]
    #[ensures(ret.y == self.y * other)]
    #[ensures(ret.z == self.z * other)]
    fn mul(self, other: f64) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for UnitVec3 {
    type Output = Vec3;

    #[ensures(ret.x == self.x + other.x)]
    #[ensures(ret.y == self.y + other.y)]
    #[ensures(ret.z == self.z + other.z)]
    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for UnitVec3 {
    type Output = Vec3;

    #[ensures(ret.x == self.x - other.x)]
    #[ensures(ret.y == self.y - other.y)]
    #[ensures(ret.z == self.z - other.z)]
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for UnitVec3 {
    type Output = Vec3;

    #[ensures(ret.x == self.x * other)]
    #[ensures(ret.y == self.y * other)]
    #[ensures(ret.z == self.z * other)]
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Index<usize> for UnitVec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl std::ops::IndexMut<usize> for UnitVec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl std::fmt::Display for UnitVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_add() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a + b;
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);

        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: -4.0,
            y: -5.0,
            z: -6.0,
        };
        let c = a + b;
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, -3.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a.cross(b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn vec3_mul_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 2.0;
        let c = a * b;
        assert_eq!(c.x, 2.0);
        assert_eq!(c.y, 4.0);
        assert_eq!(c.z, 6.0);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 32.0);

        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 0.0);

        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 1.0);
    }
}
