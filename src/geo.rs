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
}

#[derive(Debug, Clone)]
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

    pub fn expand(&mut self, p: Vec3) -> Bbox {
        Bbox {
            lower: self.lower.min(p),
            upper: self.upper.max(p),
        }
    }

    pub fn contains(&self, p: Vec3) -> bool {
        (self.lower.x..=self.upper.x).contains(&p.x)
            && (self.lower.y..=self.upper.y).contains(&p.y)
            && (self.lower.z..=self.upper.z).contains(&p.z)
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
