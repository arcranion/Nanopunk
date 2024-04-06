use bevy::math::Vec3;

pub trait VecExt {
    fn acos(&self) -> Self;
    fn asin(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: f32) -> Self;
    fn atan2_vec(&self, other: Self) -> Self;

    fn angles(&self) -> Self;
    fn angles_to(self, other: Self) -> Self;

    fn map<F>(&self, f: fn(f32) -> f32) -> Self;
}

impl VecExt for Vec3 {
    fn acos(&self) -> Self {
        return Vec3 {
            x: self.x.acos(),
            y: self.y.acos(),
            z: self.z.acos()
        };
    }

    fn asin(&self) -> Self {
        return Vec3 {
            x: self.x.asin(),
            y: self.y.asin(),
            z: self.z.asin()
        };
    }

    fn atan(&self) -> Self {
        return Vec3 {
            x: self.x.atan(),
            y: self.y.atan(),
            z: self.z.atan()
        }
    }

    fn atan2(&self, other: f32) -> Self {
        return Vec3 {
            x: self.x.atan2(other),
            y: self.y.atan2(other),
            z: self.z.atan2(other)
        }
    }

    fn atan2_vec(&self, other: Vec3) -> Self {
        return Vec3 {
            x: self.x.atan2(other.x),
            y: self.y.atan2(other.y),
            z: self.z.atan2(other.z)
        }
    }

    fn angles(&self) -> Self {
        return Vec3 {
            x: self.y.atan2(self.z),
            y: self.x.atan2(self.z),
            z: self.y.atan2(self.x)
        }
    }

    fn angles_to(self, other: Self) -> Self {
        let diff = other - self;

        return diff.angles();
    }

    fn map<F>(&self, f: fn(f32) -> f32) -> Self {
        return Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z)
        }
    }
}