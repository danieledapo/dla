#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn min(&self, o: Vec3) -> Self {
        Vec3::new(self.x.min(o.x), self.y.min(o.y), self.z.min(o.z))
    }

    pub fn max(&self, o: Vec3) -> Self {
        Vec3::new(self.x.max(o.x), self.y.max(o.y), self.z.max(o.z))
    }

    pub fn dist2(&self, o: Vec3) -> i64 {
        (o - *self).norm2()
    }

    pub fn norm2(&self) -> i64 {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    /// WARNING: not exact, rounding issues
    pub fn norm(&self) -> i64 {
        (self.norm2() as f64).sqrt() as i64
    }

    /// WARNING: not exact, rounding issues
    pub fn normalized(&self) -> Self {
        let l = self.norm();
        Vec3::new(self.x / l, self.y / l, self.z / l)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bbox {
    lower: Vec3,
    upper: Vec3,
}

impl Bbox {
    pub const fn new(p: Vec3) -> Self {
        Bbox { lower: p, upper: p }
    }

    pub fn lower(&self) -> Vec3 {
        self.lower
    }

    pub fn upper(&self) -> Vec3 {
        self.upper
    }

    pub fn center(&self) -> Vec3 {
        (self.lower + self.upper) / 2
    }

    pub fn expand(&self, p: Vec3) -> Bbox {
        Bbox { lower: self.lower.min(p), upper: self.upper.max(p) }
    }

    pub fn contains(&self, p: Vec3) -> bool {
        (self.lower.x..=self.upper.x).contains(&p.x)
            && (self.lower.y..=self.upper.y).contains(&p.y)
            && (self.lower.z..=self.upper.z).contains(&p.z)
    }

    pub fn volume(&self) -> i64 {
        let d = self.upper - self.lower;
        d.x * d.y * d.z
    }

    pub fn dimensions(&self) -> Vec3 {
        self.upper - self.lower
    }

    pub fn dist2(&self, p: Vec3) -> i64 {
        p.dist2(self.clamp(p))
    }

    pub fn clamp(&self, p: Vec3) -> Vec3 {
        p.max(self.lower).min(self.upper)
    }

    pub fn scale(&self, f: i64) -> Self {
        Bbox { lower: self.lower * f, upper: self.upper * f }
    }

    pub fn union(&self, b: &Bbox) -> Self {
        Bbox { lower: self.lower.min(b.lower), upper: self.upper.max(b.upper) }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, d: Vec3) -> Self::Output {
        Vec3::new(self.x + d.x, self.y + d.y, self.z + d.z)
    }
}

impl std::ops::Add<i64> for Vec3 {
    type Output = Self;

    fn add(self, d: i64) -> Self::Output {
        Vec3::new(self.x + d, self.y + d, self.z + d)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, d: Vec3) -> Self::Output {
        Vec3::new(self.x - d.x, self.y - d.y, self.z - d.z)
    }
}

impl std::ops::Sub<i64> for Vec3 {
    type Output = Self;

    fn sub(self, d: i64) -> Self::Output {
        Vec3::new(self.x - d, self.y - d, self.z - d)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, d: Vec3) -> Self::Output {
        Vec3::new(self.x * d.x, self.y * d.y, self.z * d.z)
    }
}

impl std::ops::Mul<i64> for Vec3 {
    type Output = Self;

    fn mul(self, d: i64) -> Self::Output {
        Vec3::new(self.x * d, self.y * d, self.z * d)
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, d: Vec3) -> Vec3 {
        Vec3::new(self.x / d.x, self.y / d.y, self.z / d.z)
    }
}

impl std::ops::Div<i64> for Vec3 {
    type Output = Self;

    fn div(self, d: i64) -> Vec3 {
        Vec3::new(self.x / d, self.y / d, self.z / d)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
