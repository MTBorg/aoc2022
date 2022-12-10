use take_until::TakeUntilExt;

struct Map {
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.look_up(x, y) || self.look_down(x, y) || self.look_right(x, y) || self.look_left(x, y)
    }

    fn at(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x]
    }

    fn look_up(&self, x: usize, y: usize) -> bool {
        (0..y)
            .rev()
            .find(|&y2| self.at(x, y2) >= self.at(x, y))
            .is_none()
    }

    fn look_down(&self, x: usize, y: usize) -> bool {
        (y + 1..self.height())
            .find(|&y2| self.at(x, y2) >= self.at(x, y))
            .is_none()
    }

    fn look_right(&self, x: usize, y: usize) -> bool {
        (x + 1..self.width())
            .find(|&x2| self.at(x2, y) >= self.at(x, y))
            .is_none()
    }

    fn look_left(&self, x: usize, y: usize) -> bool {
        (0..x)
            .rev()
            .find(|&x2| self.at(x2, y) >= self.at(x, y))
            .is_none()
    }

    fn score(&self, x: usize, y: usize) -> u32 {
        self.score_up(x, y) * self.score_down(x, y) * self.score_right(x, y) * self.score_left(x, y)
    }

    fn score_up(&self, x: usize, y: usize) -> u32 {
        (0..y)
            .rev()
            .take_until(|&y2| self.at(x, y2) >= self.at(x, y))
            .count() as u32
    }

    fn score_down(&self, x: usize, y: usize) -> u32 {
        (y + 1..self.height())
            .take_until(|&y2| self.at(x, y2) >= self.at(x, y))
            .count() as u32
    }

    fn score_right(&self, x: usize, y: usize) -> u32 {
        (x + 1..self.width())
            .take_until(|&x2| self.at(x2, y) >= self.at(x, y))
            .count() as u32
    }

    fn score_left(&self, x: usize, y: usize) -> u32 {
        (0..x)
            .rev()
            .take_until(|&x2| self.at(x2, y) >= self.at(x, y))
            .count() as u32
    }

    fn iter(&self) -> MapIterator {
        MapIterator {
            map: &self,
            x: 0,
            y: 0,
        }
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> std::iter::Iterator for MapIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.map.height() - 1 {
            return None;
        }
        let element = (self.x, self.y);

        self.x += 1;
        if self.x >= self.map.width() {
            self.x = 0;
            self.y += 1;
        }

        return Some(element);
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let map = Map { grid };

    let count = map.iter().filter(|&(x, y)| map.is_visible(x, y)).count();

    println!("Part1: {count}");

    let mut scores: Vec<u32> = map.iter().map(|(x, y)| map.score(x, y)).collect();
    scores.sort();
    let max = scores.last().unwrap();

    println!("Part2: {max}");
}
