#![allow(dead_code)]

use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPosition<'a, const WIDTH: usize, const HEIGHT: usize, T>
where
    T: Default + Copy,
{
    grid: &'a Grid<WIDTH, HEIGHT, T>,
    position: Position,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Grid<const WIDTH: usize, const HEIGHT: usize, T>
where
    T: Default + Copy,
{
    tiles: [[T; WIDTH]; HEIGHT],
    width: usize,
    height: usize,
}

pub struct GridSlice<'a, const WIDTH: usize, const HEIGHT: usize, T>
where
    T: Default + Copy,
{
    grid: &'a Grid<WIDTH, HEIGHT, T>,
    axis: Axis,
    index: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> Default for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            tiles: [[Default::default(); WIDTH]; HEIGHT],
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            tiles: [[Default::default(); WIDTH]; HEIGHT],
            width,
            height,
        }
    }

    pub fn rows<'a>(&'a self) -> impl Iterator<Item = GridSlice<'a, WIDTH, HEIGHT, T>> {
        (0..self.height).map(move |y| GridSlice::<'a, WIDTH, HEIGHT, T> {
            grid: self,
            axis: Axis::Horizontal,
            index: y,
        })
    }

    pub fn columns<'a>(&'a self) -> impl Iterator<Item = GridSlice<'a, WIDTH, HEIGHT, T>> {
        (0..self.width).map(move |x| GridSlice::<'a, WIDTH, HEIGHT, T> {
            grid: self,
            axis: Axis::Vertical,
            index: x,
        })
    }

    pub fn tiles(&self) -> impl Iterator<Item = T> + use<'_, WIDTH, HEIGHT, T> {
        self.positions().map(|position| self[position])
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + use<'_, WIDTH, HEIGHT, T> {
        (0..self.width).flat_map(move |y| (0..self.height).map(move |x| (Position { x, y })))
    }

    pub fn tiles_with_positions(
        &self,
    ) -> impl Iterator<Item = (Position, T)> + use<'_, WIDTH, HEIGHT, T> {
        self.positions().map(|position| (position, self[position]))
    }

    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn position<'a>(&'a self, position: Position) -> GridPosition<'a, WIDTH, HEIGHT, T> {
        GridPosition {
            grid: self,
            position,
        }
    }

    pub fn clear(&mut self) {
        self.tiles = [[Default::default(); WIDTH]; HEIGHT];
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> FromIterator<(Position, T)> for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    fn from_iter<I: IntoIterator<Item = (Position, T)>>(iter: I) -> Self {
        let mut grid = Self::default();
        for (position, item) in iter {
            grid[position] = item;
        }
        grid
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> Index<Position> for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    type Output = T;

    fn index(&self, position: Position) -> &Self::Output {
        &self.tiles[position.y][position.x]
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> Index<GridPosition<'a, WIDTH, HEIGHT, T>> for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    type Output = T;

    fn index(&self, grid_position: GridPosition<'a, WIDTH, HEIGHT, T>) -> &Self::Output {
        &self[grid_position.position]
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> IndexMut<Position> for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.tiles[position.y][position.x]
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> IndexMut<GridPosition<'a, WIDTH, HEIGHT, T>> for Grid<WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    fn index_mut(&mut self, grid_position: GridPosition<'a, WIDTH, HEIGHT, T>) -> &mut Self::Output {
        &mut self[grid_position.position]
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> Index<usize>
    for GridSlice<'a, WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.axis {
            Axis::Horizontal => {
                if index >= WIDTH {
                    panic!("Index out of bounds")
                }
                &self.grid.tiles[self.index][index]
            }
            Axis::Vertical => {
                if index >= HEIGHT {
                    panic!("Index out of bounds")
                }
                &self.grid.tiles[index][self.index]
            }
        }
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> GridSlice<'a, WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    pub fn tiles(&self) -> impl Iterator<Item = T> + use<'_, WIDTH, HEIGHT, T> {
        (0..self.len()).map(|index| self[index])
    }

    pub fn tiles_with_positions(
        &self,
    ) -> impl Iterator<Item = (Position, T)> + use<'_, WIDTH, HEIGHT, T> {
        (0..self.len()).map(|index| {
            (
                match self.axis {
                    Axis::Horizontal => Position {
                        x: index,
                        y: self.index,
                    },
                    Axis::Vertical => Position {
                        x: self.index,
                        y: index,
                    },
                },
                self[index],
            )
        })
    }

    pub fn len(&self) -> usize {
        match self.axis {
            Axis::Horizontal => self.grid.width,
            Axis::Vertical => self.grid.height,
        }
    }
}

impl Position {
    pub fn manhatten_distance(&self, rhs: Position) -> usize {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }

    pub fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, T> GridPosition<'a, WIDTH, HEIGHT, T>
where
    T: Default + Copy,
{
    pub fn manhatten_distance(&self, rhs: Self) -> usize {
        self.position.manhatten_distance(rhs.position)
    }

    pub fn left(&self) -> Option<Self> {
        if self.position.x > 0 {
            Some(GridPosition {
                grid: self.grid,
                position: self.position.left(),
            })
        } else {
            None
        }
    }

    pub fn right(&self) -> Option<Self> {
        if self.position.x < self.grid.width - 1 {
            Some(GridPosition {
                grid: self.grid,
                position: self.position.right(),
            })
        } else {
            None
        }
    }

    pub fn up(&self) -> Option<Self> {
        if self.position.y > 0 {
            Some(GridPosition {
                grid: self.grid,
                position: self.position.up(),
            })
        } else {
            None
        }
    }

    pub fn down(&self) -> Option<Self> {
        if self.position.y < self.grid.height - 1 {
            Some(GridPosition {
                grid: self.grid,
                position: self.position.down(),
            })
        } else {
            None
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn index() {
        let mut grid: Grid<10, 10, usize> = Grid::default();
        for y in 0..grid.height {
            for x in 0..grid.width {
                grid.tiles[y][x] = y * 10 + x;
            }
        }

        for (y, row) in grid.rows().enumerate() {
            for (x, t) in row.tiles_with_positions().enumerate() {
                println!("{y}, {x:?}: {t:?}");
            }
        }
        println!();

        for (position, t) in grid.tiles_with_positions() {
            println!("{position:?}: {t}");
        }
        println!();
    }
}
