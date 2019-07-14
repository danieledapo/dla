use std::collections::HashSet;

use rand::Rng;

pub mod geo;
pub use geo::Vec3;

use crate::geo::Bbox;

#[derive(Debug, Clone)]
pub struct Dla {
    cells: HashSet<Vec3>,
    bbox: Bbox,
}

impl Dla {
    pub fn new(seeds: impl IntoIterator<Item = Vec3>) -> Option<Self> {
        let mut seeds = seeds.into_iter();

        let first = seeds.next()?;

        let mut cells = HashSet::with_capacity(seeds.size_hint().0 + 1);
        cells.insert(first);

        let mut bbox = Bbox::new(first);

        for p in seeds {
            cells.insert(p);
            bbox = bbox.expand(p);
        }

        Some(Dla { cells, bbox })
    }

    pub fn cells(&self) -> impl Iterator<Item = &Vec3> {
        self.cells.iter()
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn bbox(&self) -> Bbox {
        self.bbox.clone()
    }

    pub fn add(&mut self, rng: &mut impl Rng) {
        let spawn_offset = 10;

        let spawn_bbox = self
            .bbox
            .expand(self.bbox.lower() - spawn_offset)
            .expand(self.bbox.upper() + spawn_offset);

        let mut cell = Vec3::new(
            rng.gen_range(spawn_bbox.lower().x, spawn_bbox.upper().x + 1),
            rng.gen_range(spawn_bbox.lower().y, spawn_bbox.upper().y + 1),
            rng.gen_range(spawn_bbox.lower().z, spawn_bbox.upper().z + 1),
        );

        while !self.stuck(cell) {
            cell = cell
                + Vec3::new(
                    rng.gen_range(-3, 4),
                    rng.gen_range(-3, 4),
                    rng.gen_range(-3, 4),
                );

            if !spawn_bbox.contains(cell) {
                cell = Vec3::new(
                    rng.gen_range(spawn_bbox.lower().x, spawn_bbox.upper().x + 1),
                    rng.gen_range(spawn_bbox.lower().y, spawn_bbox.upper().y + 1),
                    rng.gen_range(spawn_bbox.lower().z, spawn_bbox.upper().z + 1),
                );
            }
        }

        self.cells.insert(cell);
        self.bbox = self.bbox.expand(cell);
    }

    pub fn stuck(&self, p: Vec3) -> bool {
        const NEIGHBORS: [Vec3; 27] = [
            Vec3::new(-1, -1, -1),
            Vec3::new(-1, -1, 0),
            Vec3::new(-1, -1, 1),
            Vec3::new(-1, 0, -1),
            Vec3::new(-1, 0, 0),
            Vec3::new(-1, 0, 1),
            Vec3::new(-1, 1, -1),
            Vec3::new(-1, 1, 0),
            Vec3::new(-1, 1, 1),
            Vec3::new(0, -1, -1),
            Vec3::new(0, -1, 0),
            Vec3::new(0, -1, 1),
            Vec3::new(0, 0, -1),
            Vec3::new(0, 0, 0),
            Vec3::new(0, 0, 1),
            Vec3::new(0, 1, -1),
            Vec3::new(0, 1, 0),
            Vec3::new(0, 1, 1),
            Vec3::new(1, -1, -1),
            Vec3::new(1, -1, 0),
            Vec3::new(1, -1, 1),
            Vec3::new(1, 0, -1),
            Vec3::new(1, 0, 0),
            Vec3::new(1, 0, 1),
            Vec3::new(1, 1, -1),
            Vec3::new(1, 1, 0),
            Vec3::new(1, 1, 1),
        ];

        NEIGHBORS.iter().any(|n| self.cells.contains(&(p + *n)))
    }
}
