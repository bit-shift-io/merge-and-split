// modified version of this: https://github.com/johanhelsing/bevy_sparse_grid_2d/tree/main
// also of possible use: https://github.com/kul-sudo/spatial_hashing/tree/main

use std::collections::{HashMap, HashSet};

use smallvec::SmallVec;

use crate::core::math::{aabb2d::Aabb2d, vec2::Vec2};

type Key = (i32, i32);

type Entity = usize;

/// A spatial container that allows querying for entities that share one or more grid cell
#[derive(Default, Debug, Clone)]
pub struct SpatialHash<T: Copy + Eq + std::hash::Hash = Entity, const TILE_SIZE: usize = 1> {
    map: HashMap<Key, SmallVec<[T; 128]>>,
}

impl<T: Copy + Eq + std::hash::Hash, const TILE_SIZE: usize> SpatialHash<T, TILE_SIZE> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert an entity in the given Aabb coordinates
    pub fn insert_aabb(&mut self, aabb: impl Into<Aabb2d>, entity: T) {
        for key in KeyIter::new::<TILE_SIZE>(aabb) {
            self.map.entry(key).or_default().push(entity);
        }
    }

    /// Insert an entity at the given point coordinate
    pub fn insert_point(&mut self, point: Vec2, entity: T) {
        let key = Self::key_from_point(point);
        self.map.entry(key).or_default().push(entity);
    }

    /// Get an iterator with the entities in the grid cells covered by the given [`Aabb2d`]
    ///
    /// may contain duplicates if some entities are in more than one grid cell
    #[inline]
    pub fn aabb_iter(&'_ self, aabb: impl Into<Aabb2d>) -> impl Iterator<Item = T> + '_ {
        KeyIter::new::<TILE_SIZE>(aabb)
            .filter_map(|key| self.map.get(&key))
            .flatten()
            .copied()
    }

    /// Get an iterator with the entities in the grid cells at the given point
    #[inline]
    pub fn point_iter(&'_ self, point: Vec2) -> impl Iterator<Item = T> + '_ {
        let key = Self::key_from_point(point);

        std::iter::once(key)
            .filter_map(|key| self.map.get(&key))
            .flatten()
            .copied()
    }

    /// Creates a hash set with all the entities in the grid cells covered by the given [`Aabb2d`]
    #[inline]
    pub fn query_aabb(&self, aabb: impl Into<Aabb2d>) -> HashSet<T> {
        self.aabb_iter(aabb).collect()
    }

    /// Remove all entities from the map
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Remove all entities from the map, but keep the heap-allocated inner data structures
    pub fn soft_clear(&mut self) {
        for (_, vec) in self.map.iter_mut() {
            vec.clear()
        }
    }

    fn key_from_point(point: Vec2) -> Key {
        (
            (point.x / TILE_SIZE as f32).floor() as i32,
            (point.y / TILE_SIZE as f32).floor() as i32,
        )
    }
}

struct KeyIter {
    width: i32,
    start: Key,
    current: i32,
    count: i32,
}

impl KeyIter {
    fn new<const TILE_SIZE: usize>(aabb: impl Into<Aabb2d>) -> Self {
        let Aabb2d { min, max } = aabb.into();
        // convert to key space
        let s = TILE_SIZE as f32;
        let min = ((min.x / s).floor() as i32, (min.y / s).floor() as i32);
        let max = ((max.x / s).ceil() as i32, (max.y / s).ceil() as i32);
        let width = max.0 - min.0;
        let height = max.1 - min.1;
        let count = width * height;
        Self {
            start: min,
            current: -1,
            width,
            count,
        }
    }
}

impl Iterator for KeyIter {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current < self.count {
            Some((
                self.start.0 + self.current.rem_euclid(self.width),
                self.start.1 + self.current / self.width,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: usize = 1;

    #[test]
    fn keys_single() {
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(Aabb2d {
            min: Vec2::new(0.001, 0.001),
            max: Vec2::new(0.001, 0.001),
        })
        .collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], (0, 0));
    }

    #[test]
    fn keys_four_around_origin() {
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(Aabb2d {
            min: Vec2::new(-0.001, -0.001),
            max: Vec2::new(0.001, 0.001),
        })
        .collect();
        assert!(keys.contains(&(0, 0)));
        assert!(keys.contains(&(0, -1)));
        assert!(keys.contains(&(-1, 0)));
        assert!(keys.contains(&(-1, -1)));
        assert_eq!(keys.len(), 4);
    }

    #[test]
    fn matches() {
        let entity = 123;
        let mut db = SpatialHash::<Entity, TILE_SIZE>::new(); //default();
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(-0.001, -0.001),
                max: Vec2::new(0.001, 0.001),
            },
            entity,
        );

        let matches: Vec<Entity> = db
            .aabb_iter(Aabb2d {
                min: Vec2::new(0.001, 0.001),
                max: Vec2::new(0.001, 0.001),
            })
            .collect();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], entity);
    }

    #[test]
    fn key_negative() {
        let h = TILE_SIZE as f32 / 2.0;
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(Aabb2d {
            min: Vec2::new(-h, -h),
            max: Vec2::new(-h, -h),
        })
        .collect();
        assert!(keys.contains(&(-1, -1)));
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn query_points() {
        let mut db = SpatialHash::<Entity, TILE_SIZE>::new(); //default();
        let e1 = 1;
        let e2 = 2;
        db.insert_point(Vec2::new(0.5, 0.5), e1);
        db.insert_point(Vec2::new(0.499, 0.501), e2);

        let matches: HashSet<_> = db.point_iter(Vec2::new(0.499, 0.501)).collect();
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn query_points_negative() {
        let mut db = SpatialHash::<Entity, TILE_SIZE>::new(); //default();
        let e1 = 1;
        let e2 = 2;
        db.insert_point(Vec2::new(0.5, 0.5), e1);
        db.insert_point(Vec2::new(-0.5, -0.5), e2);

        let matches: HashSet<_> = db.point_iter(Vec2::new(-0.5, -0.5)).collect();
        assert!(!matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn matches_complex() {
        let h = TILE_SIZE as f32 / 2.0;
        let e1 = 1;
        let e2 = 2;
        let e3 = 3;
        let mut db: SpatialHash = SpatialHash::new(); //default();
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(-h, -h),
                max: Vec2::new(h, h),
            },
            e1,
        );
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(h, h),
                max: Vec2::new(h, h),
            },
            e2,
        );
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(-h, -h),
                max: Vec2::new(-h, -h),
            },
            e3,
        );

        let matches: Vec<Entity> = db
            .aabb_iter(Aabb2d {
                min: Vec2::new(-h, -h),
                max: Vec2::new(h, h),
            })
            .collect();
        // assert_eq!(matches.len(), 3);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert!(matches.contains(&e3));

        let matches = db.query_aabb(Aabb2d {
            min: Vec2::new(-0.001, -0.001),
            max: Vec2::new(-0.001, -0.001),
        });
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e3));

        let matches: Vec<Entity> = db
            .aabb_iter(Aabb2d {
                min: Vec2::new(-0.001, -0.001),
                max: Vec2::new(-0.001, -0.001),
            })
            .collect();
        assert_eq!(matches[0], e1);
    }

    #[test]
    fn query_points_tilesize_10() {
        let mut db = SpatialHash::<Entity, 10>::new(); //default();
        let e1 = 1;
        let e2 = 2;
        let e3 = 3;
        db.insert_point(Vec2::new(12f32, 15f32), e1);
        db.insert_point(Vec2::new(15f32, 12f32), e2);
        db.insert_point(Vec2::new(15f32, 20f32), e3);
        let matches: HashSet<_> = db.point_iter(Vec2::new(19.9, 19.9)).collect();
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert!(!matches.contains(&e3));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn non_entity() {
        let h = TILE_SIZE as f32 / 2.0;
        let e1 = 1;
        let e2 = 2;
        let mut db = SpatialHash::<usize>::new(); //default();
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(-h, -h),
                max: Vec2::new(h, h),
            },
            e1,
        );
        db.insert_aabb(
            Aabb2d {
                min: Vec2::new(h, h),
                max: Vec2::new(h, h),
            },
            e2,
        );
        let matches: Vec<usize> = db
            .aabb_iter(Aabb2d {
                min: Vec2::new(-h, -h),
                max: Vec2::new(h, h),
            })
            .collect();
        // assert_eq!(matches.len(), 2);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
    }
}
