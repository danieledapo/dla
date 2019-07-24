use rand::Rng;

pub mod geo;
pub use geo::Vec3;
pub mod octree;

use crate::geo::Bbox;
use crate::octree::Octree;

#[derive(Debug, Clone)]
pub struct Dla {
    spawn_radius: i64,
    attraction_radius: i64,

    cells: Octree,
    bbox: Bbox,
}

impl Dla {
    pub fn new(
        spawn_radius: u32,
        attraction_radius: u16,
        seeds: impl IntoIterator<Item = Vec3>,
    ) -> Option<Self> {
        let mut seeds = seeds.into_iter();

        let mut cells = Octree::new();

        let first = seeds.next()?;
        cells.add(first);

        let mut bbox = Bbox::new(first);

        for p in seeds {
            cells.add(p);
            bbox = bbox.expand(p);
        }

        Some(Dla {
            cells,
            bbox,
            spawn_radius: i64::from(spawn_radius),
            attraction_radius: i64::from(attraction_radius),
        })
    }

    pub fn cells(&self) -> impl Iterator<Item = &Vec3> {
        self.cells.iter()
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bbox(&self) -> Bbox {
        self.bbox.clone()
    }

    pub fn add<R: Rng>(&mut self, rng: &mut R) -> Vec3 {
        let spawn_bbox = self
            .bbox
            .expand(self.bbox.lower() - self.spawn_radius)
            .expand(self.bbox.upper() + self.spawn_radius);

        let respawn_cell = |rng: &mut R| {
            Vec3::new(
                rng.gen_range(spawn_bbox.lower().x, spawn_bbox.upper().x + 1),
                rng.gen_range(spawn_bbox.lower().y, spawn_bbox.upper().y + 1),
                rng.gen_range(spawn_bbox.lower().z, spawn_bbox.upper().z + 1),
            )
        };

        let mut cell = respawn_cell(rng);

        loop {
            match self.stuck(cell) {
                Some(n) => {
                    let d = cell - n;
                    cell = n + Vec3::new(d.x.signum(), d.y.signum(), d.z.signum());

                    self.cells.add(cell);
                    self.bbox = self.bbox.expand(cell);

                    break;
                }
                None => {
                    let d = match rng.gen_range(0, 6) {
                        0 => Vec3::new(-1, 0, 0),
                        1 => Vec3::new(1, 0, 0),
                        2 => Vec3::new(0, -1, 0),
                        3 => Vec3::new(0, 1, 0),
                        4 => Vec3::new(0, 0, -1),
                        5 => Vec3::new(0, 0, 1),
                        _ => unreachable!(),
                    };
                    cell = cell + d * (self.attraction_radius / 3).max(1);

                    if !spawn_bbox.contains(cell) {
                        cell = respawn_cell(rng);
                    }
                }
            }
        }

        cell
    }

    pub fn stuck(&self, p: Vec3) -> Option<Vec3> {
        let (n, _d2) = self.cells.nearest(p)?;

        let d = n - p;
        if d.x.abs() + d.y.abs() + d.z.abs() <= self.attraction_radius {
            Some(n)
        } else {
            None
        }
    }
}
