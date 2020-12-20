use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    println,
    str::FromStr,
};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

struct OrientationCache {
    inner: HashMap<usize, Vec<Tile>>,
}

impl OrientationCache {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn get_or_insert(&mut self, tile: &Tile) -> Vec<Tile> {
        #[allow(clippy::map_entry)]
        if !self.inner.contains_key(&tile.id) {
            let orientations = tile.orientations();
            self.inner.insert(tile.id, orientations);
        }
        self.inner.get(&tile.id).unwrap().clone()
    }
}

fn part1(input: &str) -> Result<usize, ()> {
    let tiles = parse_tiles(&input);
    let tiles_done = find_tile_orientations(&tiles);

    // Find corners
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y, _) in tiles_done.iter() {
        min_x = min(min_x, *x);
        min_y = min(min_y, *y);
        max_x = max(max_x, *x);
        max_y = max(max_y, *y);
    }

    let result = tiles_done
        .iter()
        .filter(|(x, y, _)| [min_x, max_x].contains(x) && [min_y, max_y].contains(y))
        .map(|(_, _, tile)| tile.id)
        .product();

    Ok(result)
}

fn find_tile_orientations(tiles: &[Tile]) -> Vec<(isize, isize, Tile)> {
    let mut tiles_todo = tiles.iter().cloned().collect::<VecDeque<_>>();
    let mut tiles_done = Vec::<(isize, isize, Tile)>::new();

    let mut orientation_cache = OrientationCache::new();

    while let Some(next_tile) = tiles_todo.pop_front() {
        if tiles_done.is_empty() {
            tiles_done.push((0, 0, next_tile));
            continue;
        }

        let mut changes = Vec::new();
        for (x, y, t) in &tiles_done {
            let orientations_next = orientation_cache.get_or_insert(&next_tile);

            match Tile::matches(&t, &orientations_next) {
                Match::North(oriented_tile) => {
                    changes.push((*x, y + 1, oriented_tile));
                }
                Match::South(oriented_tile) => {
                    changes.push((*x, y - 1, oriented_tile));
                }
                Match::East(oriented_tile) => {
                    changes.push((x + 1, *y, oriented_tile));
                }
                Match::West(oriented_tile) => {
                    changes.push((x - 1, *y, oriented_tile));
                }
                Match::None => {}
            };

            if !changes.is_empty() {
                break;
            }
        }

        if changes.is_empty() {
            tiles_todo.push_back(next_tile);
        } else {
            for c in changes {
                tiles_done.push(c);
            }
        }
    }

    assert_eq!(tiles.len(), tiles_done.len());
    tiles_done.sort_by(|a, b| (a.1, b.0).cmp(&(b.1, a.0)));
    tiles_done
}

enum Match {
    North(Tile),
    South(Tile),
    East(Tile),
    West(Tile),
    None,
}

fn part2(input: &str) -> Result<usize, ()> {
    let tiles = parse_tiles(&input);
    let trimmed_tiles = find_tile_orientations(&tiles)
        .iter()
        .map(|(_, _, tile)| tile)
        .map(|tile| {
            let mut t = tile.clone();
            t.flip();
            t.rotate();
            t.rotate();
            t.trim_tile()
        })
        .collect::<Vec<_>>();

    let joined_tile = join_tiles(&trimmed_tiles);
    let orientations = joined_tile.orientations();

    let (tile, n_sea_monsters) = orientations
        .iter()
        .map(|tile| (tile, tile.count_sea_monsters()))
        .max_by(|left, right| left.1.cmp(&right.1))
        .unwrap();

    let score = tile.data.chars().filter(|ch| '#'.eq(ch)).count() - n_sea_monsters * 15;

    Ok(score)
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: String,
    width: usize,
    height: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            let chunk = &self.data[row * self.width..row * self.width + self.width];
            f.write_str(&chunk)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Tile {
    fn orientations(&self) -> Vec<Tile> {
        let mut orientations = vec![];

        let should_flip = [true, false];
        let should_rotate = [0, 1, 2, 3];
        let mut set = HashSet::new();

        for flip in &should_flip {
            for rotation in &should_rotate {
                let mut tile = self.clone();
                if *flip {
                    tile.flip();
                }

                for _ in 0..*rotation {
                    tile.rotate();
                }

                orientations.push(tile.clone());
                // TODO remove
                set.insert(tile.data);
            }
        }

        assert_eq!(set.len(), 8);

        orientations
    }

    fn matches(r: &Tile, other_orientations: &[Tile]) -> Match {
        for o in other_orientations {
            if r.bottom().eq(&o.top()) {
                return Match::South(o.clone());
            } else if r.top().eq(&o.bottom()) {
                return Match::North(o.clone());
            } else if r.left().eq(&o.right()) {
                return Match::East(o.clone());
            } else if r.right().eq(&o.left()) {
                return Match::West(o.clone());
            }
        }
        Match::None
    }

    fn top(&self) -> String {
        self.data[0..self.width].to_string()
    }

    fn bottom(&self) -> String {
        let start = (self.height - 1) * self.width;
        self.data[start..].to_string()
    }

    fn left(&self) -> String {
        self.data
            .chars()
            .enumerate()
            .filter(|(idx, _)| idx % self.width == 0)
            .map(|(_, ch)| ch)
            .collect::<String>()
    }

    fn right(&self) -> String {
        self.data
            .chars()
            .enumerate()
            .filter(|(idx, _)| (idx + 1) % self.width == 0)
            .map(|(_, ch)| ch)
            .collect::<String>()
    }

    /// Rotates left 90°
    fn rotate(&mut self) {
        let mut rotated = self
            .data
            .chars()
            .enumerate()
            .map(|(idx, ch)| {
                // Rotation matrix for 90° boils down to:
                (
                    (idx % self.width) * self.width - ((idx / self.width) - 9),
                    ch,
                )
            })
            .collect::<Vec<_>>();

        rotated.sort_by(|a, b| b.0.cmp(&a.0));

        self.data = rotated.iter().map(|(_, ch)| *ch).collect();
    }

    /// Flips horizontally, around the y axis
    fn flip(&mut self) {
        let mut flipped = self
            .data
            .chars()
            .enumerate()
            .map(|(idx, ch)| {
                (
                    self.width - 1 - (idx % self.width) + (idx / self.width) * self.height,
                    ch,
                )
            })
            .collect::<Vec<_>>();

        flipped.sort_by(|a, b| a.0.cmp(&b.0));

        self.data = flipped.iter().map(|(_, ch)| *ch).collect();
    }

    fn trim_tile(&self) -> Tile {
        let data = self
            .data
            .chars()
            .enumerate()
            .filter(|(idx, _)| {
                let x = idx % self.width;
                let y = idx / self.width;
                !(x == 0 || x == self.width - 1 || y == self.height - 1 || y == 0)
            })
            .map(|(_, ch)| ch)
            .collect::<String>();
        Tile {
            data,
            id: self.id,
            width: self.width - 2,
            height: self.height - 2,
        }
    }

    fn count_sea_monsters(&self) -> usize {
        self.data
            .chars()
            .enumerate()
            .filter(|(idx, _)| {
                if self.is_sea_monster_at(idx) {
                    println!("Found monster at {}", idx);
                }
                self.is_sea_monster_at(idx)
            })
            .count()
    }

    fn is_sea_monster_at(&self, idx: &usize) -> bool {
        let (x, y) = ((idx % self.width) as isize, (idx / self.width) as isize);

        let monster_coords: [(isize, isize); 14] = [
            (18, -1),
            (5, 0),
            (6, 0),
            (11, 0),
            (12, 0),
            (17, 0),
            (18, 0),
            (19, 0),
            (1, 1),
            (4, 1),
            (7, 1),
            (10, 1),
            (13, 1),
            (16, 1),
        ];

        monster_coords
            .iter()
            .all(|(dx, dy)| self.get(x + *dx, y + *dy).eq(&Some('#')))
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        let idx = x + y * self.width as isize;
        if idx < 0 {
            return None;
        }
        self.data.chars().nth(idx as usize)
    }
}

impl FromStr for Tile {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Tile {
            id: lines
                .next()
                .unwrap()
                .chars()
                .filter(|ch| ch.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap(),
            data: lines.map(|x| x.trim()).collect::<Vec<_>>().join(""),
            width: 10,
            height: 10,
        })
    }
}

fn join_tiles(tiles: &[Tile]) -> Tile {
    let mut output = String::new();
    let tileset_width = (tiles.len() as f64).sqrt() as usize;
    let tile_width = tiles[0].width;

    for global_row in 0..tileset_width {
        let start = global_row * tileset_width;
        let finish = start + tileset_width;
        let tiles_in_row = &tiles[start..finish];
        for local_row in 0..tile_width {
            for tile in tiles_in_row {
                let start = local_row * tile_width;
                let finish = start + tile_width;
                let tile_chunk = &tile.data[start..finish];
                output.push_str(tile_chunk);
            }
        }
    }

    Tile {
        id: 0,
        data: output,
        width: tileset_width * tile_width,
        height: tileset_width * tile_width,
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|section| section.parse::<Tile>().unwrap())
        .collect::<_>()
}

#[cfg(test)]
mod day20_test {
    use std::str::FromStr;

    use crate::{join_tiles, part1, part2, Tile};

    const TEST_TILE: &str = "Tile 1471:
            .#...##.##
            #...#.#..#
            ##.......#
            #..#.....#
            #..##....#
            .#.#..#...
            ##.##.#..#
            ...###..#.
            ......##..
            .##..##.#.";

    #[test]
    fn test_tile_flip() {
        let mut t = Tile::from_str(TEST_TILE).unwrap();
        t.flip();
        assert_eq!(
            t.data,
            String::from(
                "##.##...#.
                #..#.#...#
                #.......##
                #.....#..#
                #....##..#
                ...#..#.#.
                #..#.##.##
                .#..###...
                ..##......
                .#.##..##."
            )
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
        );
    }

    #[test]
    fn test_tile_rotation() {
        let mut t = Tile::from_str(TEST_TILE).unwrap();
        t.rotate();
        assert_eq!(
            t.data,
            String::from(
                "#####.#...
                #......#.#
                ........#.
                ##...##.##
                #......#.#
                .#..#.##..
                ...#####..
                .........#
                #.#..##..#
                .####.#..."
            )
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
        );
    }

    #[test]
    fn test_tile_getters() {
        let t = Tile::from_str(TEST_TILE).unwrap();
        assert_eq!(t.top(), String::from(".#...##.##"));
        assert_eq!(t.bottom(), String::from(".##..##.#."));
        assert_eq!(t.right(), String::from("#####.#..."));
        assert_eq!(t.left(), String::from(".####.#..."));
    }

    #[test]
    fn test_trim_tile() {
        let t = Tile::from_str(TEST_TILE).unwrap();
        let trimmed = "Tile 1471:
            ...#.#..
            #.......
            ..#.....
            ..##....
            #.#..#..
            #.##.#..
            ..###..#
            .....##.
        ";
        assert_eq!(t.trim_tile().data, Tile::from_str(trimmed).unwrap().data);
    }

    #[test]
    fn test_join_tile() {
        let t = Tile::from_str(TEST_TILE).unwrap();
        let mut flipped = t.clone();
        flipped.flip();
        let tiles = vec![t.clone(), t, flipped.clone(), flipped];
        let joined = join_tiles(&tiles);

        let expected = Tile::from_str(
            "Tile 0:
            .#...##.##.#...##.##
            #...#.#..##...#.#..#
            ##.......###.......#
            #..#.....##..#.....#
            #..##....##..##....#
            .#.#..#....#.#..#...
            ##.##.#..###.##.#..#
            ...###..#....###..#.
            ......##........##..
            .##..##.#..##..##.#.

            ##.##...#.##.##...#.
            #..#.#...##..#.#...#
            #.......###.......##
            #.....#..##.....#..#
            #....##..##....##..#
            ...#..#.#....#..#.#.
            #..#.##.###..#.##.##
            .#..###....#..###...
            ..##........##......
            .#.##..##..#.##..##.
            ",
        )
        .unwrap();
        assert_eq!(joined.data, expected.data);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part1(include_str!("../test_input.txt")).unwrap(),
            20899048083289
        );
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 17148689442341);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../test_input.txt")).unwrap(), 273);
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 2009);
    }
}
