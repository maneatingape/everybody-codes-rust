use std::ops::{Index, IndexMut};

use crate::util::point::*;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();

        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let bytes = raw.concat();

        Self { width, height, bytes }
    }

    pub fn print(&self) {
        for row in self.bytes.chunks(self.width as usize) {
            println!("{}", str::from_utf8(row).unwrap());
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    #[must_use]
    pub fn find(&self, needle: T) -> Option<Point> {
        self.bytes
            .iter()
            .position(|&h| h == needle)
            .map(|index| Point::new(index as i32 % self.width, index as i32 / self.width))
    }
}

impl<T: Copy> Grid<T> {
    #[must_use]
    pub fn new(width: i32, height: i32, value: T) -> Self {
        Self { width, height, bytes: vec![value; (width * height) as usize] }
    }

    #[must_use]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid::new(self.width, self.height, value)
    }
}

impl<T> Grid<T> {
    #[inline]
    #[must_use]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    #[inline]
    pub fn points(&self) -> impl Iterator<Item = Point> {
        let width = self.width;
        let height = self.height;
        (0..height).flat_map(move |y| (0..width).map(move |x| Point::new(x, y)))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}
