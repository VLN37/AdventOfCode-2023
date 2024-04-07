#[derive(Clone, Copy)]
pub enum Cardinal {
    East,
    West,
    North,
    South,
}

impl Cardinal {
    fn iter() -> [Cardinal; 4] {
        [
            Cardinal::North,
            Cardinal::South,
            Cardinal::East,
            Cardinal::West,
        ]
    }
}

#[derive(Default, Debug)]
pub struct Maze {
    maze:     Vec<Vec<char>>,
    steps:    usize,
    x:        usize,
    y:        usize,
    init_x:   usize,
    init_y:   usize,
    distance: usize,
}

impl Maze {
    pub fn from(input: &str) -> Maze {
        let iter = input.lines().map(|x| Vec::from_iter(x.chars()));
        let maze: Vec<Vec<char>> = iter.collect();
        Maze {
            maze,
            ..Default::default()
        }
    }

    pub fn pipe_jump(&mut self) -> bool {
        let mut is_viable = false;
        // println!("distance {}", self.distance);
        self.distance += 1;
        for direction in Cardinal::iter() {
            if self.valid_move(direction) {
                self.mark_visited();
                self.move_towards(direction);
                is_viable = self.pipe_jump();
            }
        }
        if is_viable {
            self.steps += 1;
        } else if self.back_to_the_future() {
            self.mark_visited();
            is_viable = true;
        }
        is_viable
    }

    fn current_tile(&self) -> char { self.maze[self.x][self.y] }

    fn starting_coordinates(&self) -> (usize, usize) { (self.init_x, self.init_y) }

    fn current_coordinates(&self) -> (usize, usize) { (self.x, self.y) }

    fn move_towards(&mut self, direction: Cardinal) {
        match direction {
            Cardinal::South => self.x += 1,
            Cardinal::North => self.x -= 1,
            Cardinal::East => self.y += 1,
            Cardinal::West => self.y -= 1,
        }
    }

    fn mark_visited(&mut self) {
        if self.starting_coordinates() != self.current_coordinates() {
            self.maze[self.x][self.y] = 'x';
        }
    }

    fn valid_move(&mut self, direction: Cardinal) -> bool {
        let (next_x, next_y) = match direction {
            Cardinal::South => (self.x + 1, self.y),
            Cardinal::North => (self.x - 1, self.y),
            Cardinal::East => (self.x, self.y + 1),
            Cardinal::West => (self.x, self.y - 1),
        };
        if (next_x, next_y) == (self.init_x, self.init_y) {
            return false;
        }
        let src = self.current_tile();
        let dst = self.maze[next_x][next_y];
        match direction {
            Cardinal::North => "|LJS".contains(src) && "|7F".contains(dst),
            Cardinal::West => "-J7S".contains(src) && "L-F".contains(dst),
            Cardinal::East => "L-FS".contains(src) && "-J7".contains(dst),
            Cardinal::South => "|7FS".contains(src) && "|LJ".contains(dst),
        }
    }

    pub fn find_player(&mut self) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut found = false;

        for (i, line) in self.maze.iter().enumerate() {
            if let Some(idx) = line.iter().position(|x| *x == 'S') {
                x = i;
                y = idx;
                found = true;
                break;
            }
        }
        if !found {
            panic!("Player not found");
        }
        self.x = x;
        self.y = y;
        self.init_x = x;
        self.init_y = y;
    }

    fn back_to_the_future(&self) -> bool {
        let x = self.x;
        let y = self.y;
        let sides = [
            self.maze[x + 1][y],
            self.maze[x - 1][y],
            self.maze[x][y + 1],
            self.maze[x][y - 1],
        ];
        for c in sides.iter() {
            if *c == 'S' {
                return true;
            }
        }
        false
    }

    pub fn report(&self) {
        dbg!(self
            .maze
            .iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>());
        println!("Maximum distance: {}", self.steps / 2 + 1);
    }
}
