use std::collections::HashSet;

use rand::Rng;

pub mod geo;
pub use geo::Vec3;

use crate::geo::Bbox;

#[derive(Debug, Clone)]
pub struct Dla {
    spawn_radius: i64,
    attraction_radius: i64,

    // TODO: consider using an Octree or some form of spatial index, this would
    // allow probably more efficient code in case the attraction_radius is big.
    // As of now, the attraction_radius is used to populate the neighbors vector
    // with the deltas to cover a cube of size = attraction_radius*2 and then
    // each of these neighbors is checked to find out whether the current
    // particle is stuck.
    cells: HashSet<Vec3>,
    bbox: Bbox,
    neighbors: Vec<Vec3>,
}

impl Dla {
    pub fn new(
        spawn_radius: u32,
        attraction_radius: u16,
        seeds: impl IntoIterator<Item = Vec3>,
    ) -> Option<Self> {
        let mut seeds = seeds.into_iter();

        let first = seeds.next()?;

        let mut cells = HashSet::with_capacity(seeds.size_hint().0 + 1);
        cells.insert(first);

        let mut bbox = Bbox::new(first);

        for p in seeds {
            cells.insert(p);
            bbox = bbox.expand(p);
        }

        let mut neighbors = Vec::with_capacity(1 + 26 * usize::from(attraction_radius));
        neighbors.push(Vec3::new(0, 0, 0));
        for i in 1..=attraction_radius {
            let i = i64::from(i);
            neighbors.extend_from_slice(&[
                Vec3::new(-i, -i, -i),
                Vec3::new(-i, -i, 0),
                Vec3::new(-i, -i, i),
                Vec3::new(-i, 0, -i),
                Vec3::new(-i, 0, 0),
                Vec3::new(-i, 0, i),
                Vec3::new(-i, i, -i),
                Vec3::new(-i, i, 0),
                Vec3::new(-i, i, i),
                Vec3::new(0, -i, -i),
                Vec3::new(0, -i, 0),
                Vec3::new(0, -i, i),
                Vec3::new(0, 0, -i),
                Vec3::new(0, 0, i),
                Vec3::new(0, i, -i),
                Vec3::new(0, i, 0),
                Vec3::new(0, i, i),
                Vec3::new(i, -i, -i),
                Vec3::new(i, -i, 0),
                Vec3::new(i, -i, i),
                Vec3::new(i, 0, -i),
                Vec3::new(i, 0, 0),
                Vec3::new(i, 0, i),
                Vec3::new(i, i, -i),
                Vec3::new(i, i, 0),
                Vec3::new(i, i, i),
            ]);
        }

        Some(Dla {
            cells,
            bbox,
            spawn_radius: i64::from(spawn_radius),
            attraction_radius: i64::from(attraction_radius),
            neighbors,
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
                Some(d) => {
                    let od = Vec3::new(-d.x.signum(), -d.y.signum(), -d.z.signum());
                    cell = cell + *d + od;

                    self.cells.insert(cell);
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

    pub fn stuck(&self, p: Vec3) -> Option<&Vec3> {
        self.neighbors.iter().find(|n| self.cells.contains(&(p + **n)))
    }
}
