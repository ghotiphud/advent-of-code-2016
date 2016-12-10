extern crate regex;

use self::regex::Regex;

pub fn main() {
    let input = "rect 1x1
rotate row y=0 by 10
rect 1x1
rotate row y=0 by 10
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 4
rect 1x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 5
rotate column x=0 by 1
rect 4x1
rotate row y=1 by 12
rotate row y=0 by 10
rotate column x=0 by 1
rect 9x1
rotate column x=7 by 1
rotate row y=1 by 3
rotate row y=0 by 2
rect 1x2
rotate row y=1 by 3
rotate row y=0 by 1
rect 1x3
rotate column x=35 by 1
rotate column x=5 by 2
rotate row y=2 by 5
rotate row y=1 by 5
rotate row y=0 by 2
rect 1x3
rotate row y=2 by 8
rotate row y=1 by 10
rotate row y=0 by 5
rotate column x=5 by 1
rotate column x=0 by 1
rect 6x1
rotate row y=2 by 7
rotate row y=0 by 5
rotate column x=0 by 1
rect 4x1
rotate column x=40 by 2
rotate row y=2 by 10
rotate row y=0 by 12
rotate column x=5 by 1
rotate column x=0 by 1
rect 9x1
rotate column x=43 by 1
rotate column x=40 by 2
rotate column x=38 by 1
rotate column x=15 by 1
rotate row y=3 by 35
rotate row y=2 by 35
rotate row y=1 by 32
rotate row y=0 by 40
rotate column x=32 by 1
rotate column x=29 by 1
rotate column x=27 by 1
rotate column x=25 by 1
rotate column x=23 by 2
rotate column x=22 by 1
rotate column x=21 by 3
rotate column x=20 by 1
rotate column x=18 by 3
rotate column x=17 by 1
rotate column x=15 by 1
rotate column x=14 by 1
rotate column x=12 by 1
rotate column x=11 by 3
rotate column x=10 by 1
rotate column x=9 by 1
rotate column x=8 by 2
rotate column x=7 by 1
rotate column x=4 by 1
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=0 by 1
rect 34x1
rotate column x=44 by 1
rotate column x=24 by 1
rotate column x=19 by 1
rotate row y=1 by 8
rotate row y=0 by 10
rotate column x=8 by 1
rotate column x=7 by 1
rotate column x=6 by 1
rotate column x=5 by 2
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 9x1
rotate row y=0 by 40
rotate column x=43 by 1
rotate row y=4 by 10
rotate row y=3 by 10
rotate row y=2 by 5
rotate row y=1 by 10
rotate row y=0 by 15
rotate column x=7 by 2
rotate column x=6 by 3
rotate column x=5 by 2
rotate column x=3 by 2
rotate column x=2 by 4
rotate column x=0 by 2
rect 9x2
rotate row y=3 by 47
rotate row y=0 by 10
rotate column x=42 by 3
rotate column x=39 by 4
rotate column x=34 by 3
rotate column x=32 by 3
rotate column x=29 by 3
rotate column x=22 by 3
rotate column x=19 by 3
rotate column x=14 by 4
rotate column x=4 by 3
rotate row y=4 by 3
rotate row y=3 by 8
rotate row y=1 by 5
rotate column x=2 by 3
rotate column x=1 by 3
rotate column x=0 by 2
rect 3x2
rotate row y=4 by 8
rotate column x=45 by 1
rotate column x=40 by 5
rotate column x=26 by 3
rotate column x=25 by 5
rotate column x=15 by 5
rotate column x=10 by 5
rotate column x=7 by 5
rotate row y=5 by 35
rotate row y=4 by 42
rotate row y=2 by 5
rotate row y=1 by 20
rotate row y=0 by 45
rotate column x=48 by 5
rotate column x=47 by 5
rotate column x=46 by 5
rotate column x=43 by 5
rotate column x=41 by 5
rotate column x=38 by 5
rotate column x=37 by 5
rotate column x=36 by 5
rotate column x=33 by 1
rotate column x=32 by 5
rotate column x=31 by 5
rotate column x=30 by 1
rotate column x=28 by 5
rotate column x=27 by 5
rotate column x=26 by 5
rotate column x=23 by 1
rotate column x=22 by 5
rotate column x=21 by 5
rotate column x=20 by 1
rotate column x=17 by 5
rotate column x=16 by 5
rotate column x=13 by 1
rotate column x=12 by 3
rotate column x=7 by 5
rotate column x=6 by 5
rotate column x=3 by 1
rotate column x=2 by 3";

    const W: usize = 50;
    const H: usize = 6;
    let mut display = vec![vec![false;W];H];

    let rect_regex = r"(?P<cmd>rect) (?P<x>\d*)x(?P<y>\d*)";
    let rot_regex = r"(?P<cmd2>rotate .*) .=(?P<coord>\d*) by (?P<dist>\d*)";

    let regex = Regex::new(&format!("{}|{}", rect_regex, rot_regex)).unwrap();

    let operations = input.lines().map(|line| {
        let caps = regex.captures(line).expect(line);
        let cmd = caps.name("cmd").unwrap_or_else(|| &caps["cmd2"]);

        match cmd {
            "rect" => {
                Op::Rect(
                    caps["x"].parse().unwrap(), 
                    caps["y"].parse().unwrap()
                )
            },
            "rotate row" => {
                Op::RotateRow(
                    caps["coord"].parse().unwrap(),
                    caps["dist"].parse().unwrap()
                )
            },
            "rotate column" => {
                Op::RotateCol(
                    caps["coord"].parse().unwrap(),
                    caps["dist"].parse().unwrap()
                )
            },
            _ => unreachable!()
        }
    });

    for op in operations {
        match op {
            Op::Rect(xmax,ymax) => {
                for y in 0..ymax {
                    for x in 0..xmax {
                        display[y][x] = true;
                    }
                }
            },
            Op::RotateRow(y,dist) => {
                let v = display[y].clone();
                for i in 0..W {
                    display[y][(i+dist)%W] = v[i];
                }
            },
            Op::RotateCol(x,dist) => {
                let v = display.iter().map(|row| row[x]).collect::<Vec<_>>();
                for j in 0..H {
                    display[(j+dist)%H][x] = v[j];
                }
            },
        }
    }


    let mut lit = 0;
    for y in 0..H {
        for x in 0..W {
            print!("{}", if display[y][x] { lit +=1; '#' } else { ' ' });
        }
        println!("");
    }
    println!("");
    println!("{:?}", lit);
}


#[derive(Debug)]
enum Op {
    Rect(usize,usize),
    RotateRow(usize,usize),
    RotateCol(usize,usize),
}