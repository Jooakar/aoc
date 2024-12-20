use crate::util::point::*;

use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let data: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = data[0].len();
        let height = data.len();
        let mut bytes = Vec::with_capacity(width * height);
        data.iter().for_each(|line| bytes.extend_from_slice(line));
        Grid {
            width: width as i32,
            height: height as i32,
            bytes,
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(index.x + index.y * self.width) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(index.x + index.y * self.width) as usize]
    }
}
