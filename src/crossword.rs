use std::fmt;
use std::ops::{Index, IndexMut};
use rand::Rng;

use crate::matcher::Matcher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Black,
    Letter(char),
}

#[derive(Debug, Clone, Copy)]
pub struct Crossword<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[Cell; WIDTH]; HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize); 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Across,
    Down,
}

impl<const WIDTH: usize, const HEIGHT: usize> Crossword<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        let mut grid = [[Cell::Empty; WIDTH]; HEIGHT];
        let mut x = 0;
        let mut y = 0;
        for c in s.chars() {
            match c {
                '□' => {
                    grid[y][x] = Cell::Empty;
                    x += 1;
                },
                '■' => {
                    grid[y][x] = Cell::Black;
                    x += 1;
                },
                '\n' => {
                    x = 0;
                    y += 1;
                },
                _ => {
                    if c >= 'A' && c <= 'Z' {
                        grid[y][x] = Cell::Letter(c);
                        x += 1;
                    } else if c >= 'a' && c <= 'z' {
                        grid[y][x] = Cell::Letter(c.to_ascii_uppercase());
                        x += 1;
                    } else if !c.is_whitespace() {
                        panic!("Invalid character: {}", c)
                    }
                },
            }
        }
        Self {
            grid,
        }
    }

    pub fn set_word(&mut self, word: &str, pos: Pos, direction: Direction) {
        let (x, y) = (pos.0, pos.1);
        let mut x = x;
        let mut y = y;
        for c in word.chars() {
            self[Pos(x, y)] = Cell::Letter(c);
            match direction {
                Direction::Across => x += 1,
                Direction::Down => y += 1,
            }
        }
    }

    pub fn across_positions(&self) -> Vec<(Pos, usize)> {
        let mut positions = Vec::new();
        for y in 0..HEIGHT {
            let mut start: Pos = Pos(0, y);
            for x in 0..WIDTH {
                match self[Pos(x, y)] {
                    Cell::Empty | Cell::Letter(_) => {
                        if x == 0 || self[Pos(x - 1, y)] == Cell::Black {
                            start = Pos(x, y);
                        }
                    },
                    Cell::Black => {
                        if x != 0 && self[Pos(x - 1, y)] != Cell::Black {
                            positions.push((start, x - start.0));
                        }
                    },
                }
            }
            if self[Pos(WIDTH-1, y)] != Cell::Black && start.0 < WIDTH {
                positions.push((start, WIDTH - start.0));
            }
        }
        positions
    }

    pub fn down_positions(&self) -> Vec<(Pos, usize)> {
        let mut positions = Vec::new();
        for x in 0..WIDTH {
            let mut start: Pos = Pos(x, 0);
            for y in 0..HEIGHT {
                match self[Pos(x, y)] {
                    Cell::Empty | Cell::Letter(_) => {
                        if y == 0 || self[Pos(x, y - 1)] == Cell::Black {
                            start = Pos(x, y);
                        }
                    },
                    Cell::Black => {
                        if y != 0 && self[Pos(x, y - 1)] != Cell::Black {
                            positions.push((start, y - start.1));
                        }
                    },
                }
            }
            if self[Pos(x, HEIGHT-1)] != Cell::Black && start.1 < HEIGHT {
                positions.push((start, HEIGHT - start.1));
            }
        }
        positions
    }

    pub fn get_across(&self, pos: Pos, length: usize) -> Vec<Cell> {
        let mut result = Vec::with_capacity(length);
        let mut x = pos.0;
        let y = pos.1;
        for _ in 0..length {
            result.push(self[Pos(x, y)]);
            x += 1;
        }
        result
    }

    pub fn get_down(&self, pos: Pos, length: usize) -> Vec<Cell> {
        let mut result = Vec::with_capacity(length);
        let x = pos.0;
        let mut y = pos.1;
        for _ in 0..length {
            result.push(self[Pos(x, y)]);
            y += 1;
        }
        result
    }
    
    pub fn get_pattern_across(&self, pos: Pos, length: usize) -> Vec<Option<char>> {
        let mut result = Vec::with_capacity(length);
        let mut x = pos.0;
        let y = pos.1;
        for _ in 0..length {
            result.push(match self[Pos(x, y)] {
                Cell::Empty => None,
                Cell::Letter(c) => Some(c),
                Cell::Black => panic!("Black cell in pattern"),
            });
            x += 1;
        }
        result
    }

    pub fn get_pattern_down(&self, pos: Pos, length: usize) -> Vec<Option<char>> {
        let mut result = Vec::with_capacity(length);
        let x = pos.0;
        let mut y = pos.1;
        for _ in 0..length {
            result.push(match self[Pos(x, y)] {
                Cell::Empty => None,
                Cell::Letter(c) => Some(c),
                Cell::Black => panic!("Black cell in pattern"),
            });
            y += 1;
        }
        result
    }

    pub fn get_pattern(&self, pos: Pos, length: usize, direction: Direction) -> Vec<Option<char>> {
        match direction {
            Direction::Across => self.get_pattern_across(pos, length),
            Direction::Down => self.get_pattern_down(pos, length),
        }
    }

    pub fn set_pattern_across(&mut self, pattern: &[Option<char>], pos: Pos) {
        let mut x = pos.0;
        let y = pos.1;
        for c in pattern {
            self[Pos(x, y)] = match c {
                Some(c) => Cell::Letter(*c),
                None => Cell::Empty,
            };
            x += 1;
        }
    }

    pub fn set_pattern_down(&mut self, pattern: &[Option<char>], pos: Pos) {
        let x = pos.0;
        let mut y = pos.1;
        for c in pattern {
            self[Pos(x, y)] = match c {
                Some(c) => Cell::Letter(*c),
                None => Cell::Empty,
            };
            y += 1;
        }
    }

    pub fn set_pattern(&mut self, pattern: &[Option<char>], pos: Pos, direction: Direction) {
        match direction {
            Direction::Across => self.set_pattern_across(pattern, pos),
            Direction::Down => self.set_pattern_down(pattern, pos),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<Pos> for Crossword<WIDTH, HEIGHT> {
    type Output = Cell;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.grid[pos.1][pos.0]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<Pos> for Crossword<WIDTH, HEIGHT> {    
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.grid[pos.1][pos.0]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Crossword<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.grid.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => write!(f, "□")?,
                    Cell::Black => write!(f, "■")?,
                    Cell::Letter(c) => write!(f, "{}", c)?,
                }
                if i < WIDTH - 1 {
                    write!(f, " ")?;
                }
            }
            if i < HEIGHT - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Crossword<WIDTH, HEIGHT> {
    pub fn fill<R: Rng + ?Sized>(&mut self, matcher: &mut Matcher, rng: &mut R) -> bool {
        self.fill_recursive(matcher, &self.across_positions(), &self.down_positions(), rng)
    }

    fn fill_recursive<R: Rng + ?Sized>(&mut self, matcher: &mut Matcher, across: &Vec<(Pos, usize)>, down: &Vec<(Pos, usize)>, rng: &mut R) -> bool {
        let pos = self.choice_pos(across, down);
        if pos.is_none() {
            return true;
        }
        let (pos, length, index, direction) = pos.unwrap();
        let pattern = self.get_pattern(pos, length, direction);
        let matches = matcher.find_vec_random_cached(&pattern, rng);
        if matches.is_empty() {
            return false;
        }
        let mut new_across = across.clone();
        let mut new_down = down.clone();
        if direction == Direction::Across {
            new_across.remove(index);
        } else {
            new_down.remove(index);
        }
        for word in matches.iter() {
            self.set_word(word, pos, direction);
            if self.fill_recursive(matcher, &new_across, &new_down, rng) {
                return true;
            }
            self.set_pattern(pattern.as_slice(), pos, direction);
        }
        false
    }

    fn choice_pos(&self, across: &Vec<(Pos, usize)>, down: &Vec<(Pos, usize)>) -> Option<(Pos, usize, usize, Direction)> {
        let mut across_best_start: Option<(Pos, usize, usize)> = None;
        let mut across_best_score: usize = 0;
        for (i, word) in across.iter().enumerate() {
            let (pos, length) = *word;
            let cells = self.get_across(pos, length);
            let score = cells.iter().filter(|c| matches!(**c, Cell::Letter(_))).count();
            if across_best_start.is_none() || score > across_best_score {
                across_best_start = Some((word.0, word.1, i));
                across_best_score = score;
            }
        }

        let mut down_best_start: Option<(Pos, usize, usize)> = None;
        let mut down_best_score: usize = 0;
        for (i, word) in down.iter().enumerate() {
            let (pos, length) = *word;
            let cells = self.get_down(pos, length);
            let score = cells.iter().filter(|c| matches!(**c, Cell::Letter(_))).count();
            if down_best_start.is_none() || score > down_best_score {
                down_best_start = Some((word.0, word.1, i));
                down_best_score = score;
            }
        }

        if across_best_score >= down_best_score {
            across_best_start.map(|pos| (pos.0, pos.1, pos.2, Direction::Across))
        } else {
            down_best_start.map(|pos| (pos.0, pos.1, pos.2, Direction::Down))
        }
    }
}