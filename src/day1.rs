const DIRECTIONS: &'static str =
    "R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, \
     R3, R1, L1, R1, R2, L1, L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, \
     L2, R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, R3, R2, L1, R5, \
     R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, R2, L5, L5, R2, R3, R5, R4, R2, R1, \
     L1, L5, L2, L3, L4, L5, L4, L5, L1, R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, \
     L4, R3, R1, L2, R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, L1, \
     L4, R2, L3, R5, R3, R1, L3";
// const DIRECTIONS: &'static str = "R8, R4, R4, R8";

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}
use self::Direction::*;

impl Direction {
    fn turn(&mut self, rl: &char) {
        *self = match *rl {
            'R' => {
                match *self {
                    N => E,
                    E => S,
                    S => W,
                    W => N,
                }
            }
            'L' => {
                match *self {
                    N => W,
                    E => N,
                    S => E,
                    W => S,
                }
            }
            _ => panic!("can't turn {}", rl),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<Point> {
        if self.is_parallel(other) {
            return None;
        }

        let horz = if self.is_horizontal() { self } else { other };
        let vert = if self.is_vertical() { self } else { other };

        let &Line { start: Point { x: hx1, y: hy }, end: Point { x: hx2, y: _ } } = horz;
        let &Line { start: Point { x: vx, y: vy1 }, end: Point { x: _, y: vy2 } } = vert;

        // order coords so that 1 < 2
        let (hx1, hx2) = if hx1 < hx2 { (hx1, hx2) } else { (hx2, hx1) };
        let (vy1, vy2) = if vy1 < vy2 { (vy1, vy2) } else { (vy2, vy1) };

        if (vx > hx1 && vx < hx2) && (hy > vy1 && hy < vy2) {
            println!("({},{})", vx, hy);
            Some(Point { x: vx, y: hy })
        } else {
            None
        }
    }

    fn is_parallel(&self, other: &Line) -> bool {
        (self.is_horizontal() && other.is_horizontal()) ||
        (self.is_vertical() && other.is_vertical())
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

pub fn main() {
    let mut path = Vec::new();
    let mut pos = Point { x: 0, y: 0 };
    let mut prev_pos;
    let mut intersect = None;
    let mut facing = N;
    let dirs = DIRECTIONS.split(", ");

    for d in dirs {
        prev_pos = pos;

        let mut x = d.chars();
        let turn = x.next().unwrap();
        let steps = x.collect::<String>().parse::<i32>().unwrap();

        facing.turn(&turn);
        match facing {
            N => pos.y += steps,
            S => pos.y -= steps,
            E => pos.x += steps,
            W => pos.x -= steps,
        }

        println!("{}, {} => {:?} {:?}", turn, steps, facing, pos);

        let line = Line {
            start: prev_pos,
            end: pos,
        };
        if let None = intersect {
            intersect = path.iter()
                .map(|l: &Line| l.intersect(&line))
                .fold(None, |acc, p| {
                    match acc {
                        None => p,
                        Some(_) => acc,
                    }
                });
        }

        path.push(line);
    }

    let dist = pos.x.abs() + pos.y.abs();

    let intersect_dist = match intersect {
        Some(p) => p.x.abs() + p.y.abs(),
        None => 0,
    };

    println!("part1 distance: {}", dist);
    println!("part2 distance: {}", intersect_dist);
}
