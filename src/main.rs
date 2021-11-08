fn main() {
    let test = World::new().set_atom((1, 6), 1).set_atom((1, 7), 1);
    println!("{}\n", test.to_string_grid());
    for tl in test.split_timeline(test.posssibilities()) {
        println!("{}\n", tl.to_string_grid());
        for t in tl.step() {
            println!("{}\n", t.to_string_grid());
        }
    }
}

const WORLD_WIDTH: usize = 8;
const WORLD_HEIGHT: usize = 8;

type World = [[usize; WORLD_WIDTH]; WORLD_HEIGHT];
//            X      Y
type Point = (usize, usize);
//                    From        To
type Possibilities = (Vec<Point>, Vec<Vec<Point>>);

trait Sim {
    fn new() -> Self;

    fn to_string_grid(&self) -> String;

    fn move_atom(&self, from: Point, to: Point) -> Self;
    fn set_atom(&self, at: Point, element: usize) -> Self;
    fn is_empty(&self, at: Point) -> bool;

    fn posssibilities(&self) -> Possibilities;
    fn split_timeline(&self, possible: Possibilities) -> Vec<Self>
    where
        Self: Sized;
    fn step(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl Sim for World {
    fn new() -> Self {
        [[0; WORLD_WIDTH]; WORLD_HEIGHT]
    }

    fn to_string_grid(&self) -> String {
        self.map(|r| r.map(|n| n.to_string()).join("")).join("\n")
    }

    fn move_atom(&self, from: Point, to: Point) -> Self {
        let mut new = *self;
        new[to.1][to.0] = new[from.1][from.0];
        new[from.1][from.0] = 0;
        new
    }
    fn set_atom(&self, at: Point, element: usize) -> Self {
        let mut new = *self;
        new[at.1][at.0] = element;
        new
    }
    fn is_empty(&self, at: Point) -> bool {
        at.0 < WORLD_WIDTH && at.1 < WORLD_HEIGHT && self[at.1][at.0] == 0
    }

    fn posssibilities(&self) -> Possibilities {
        let mut from = Vec::new();
        let mut to = Vec::new();

        self.iter()
            .map(|r| r.iter().enumerate().collect::<Vec<(usize, &usize)>>())
            .enumerate()
            .for_each(|(y, row)| {
                for (x, _element) in row.iter().filter(|(_x, element)| element > &&0) {
                    let mut can_move_to = Vec::new();
                    let pos = [(x.wrapping_sub(1), y + 1), (*x, y + 1), (x + 1, y + 1)];

                    if match (
                        self.is_empty(pos[0]),
                        self.is_empty(pos[1]),
                        self.is_empty(pos[2]),
                    ) {
                        (false, false, false) => false,
                        (true, false, true) => {
                            can_move_to.push(pos[0]);
                            can_move_to.push(pos[2]);
                            true
                        }
                        (true, false, false) => {
                            can_move_to.push(pos[0]);
                            true
                        }
                        (false, false, true) => {
                            can_move_to.push(pos[2]);
                            true
                        }
                        _ => {
                            can_move_to.push(pos[1]);
                            true
                        }
                    } {
                        from.push((*x, y));
                        to.push(can_move_to);
                    }
                }
            });

        (from, to)
    }
    fn split_timeline(&self, possible: Possibilities) -> Vec<Self> {
        let mut taken: Vec<Point> = Vec::new();
        let mut timelines: Vec<Self> = Vec::new();
        let (from, to) = possible;

        from.iter().enumerate().for_each(|(i, f)| {
            to[i].iter().for_each(|t| {
                if !taken.iter().any(|p| p == t) {
                    taken.push(*t);
                    timelines.push(self.move_atom(*f, *t));
                }
            })
        });

        timelines
    }
    fn step(&self) -> Vec<Self> {
        self.split_timeline(self.posssibilities())
    }
}
