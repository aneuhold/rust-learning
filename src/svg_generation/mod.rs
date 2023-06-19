use futures::future::join_all;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;
use uuid::Uuid;

use crate::svg_generation::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::svg_generation::Orientation::{East, North, South, West};

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;

pub struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    pub fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    pub fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    pub fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }

    pub fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    pub fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

    pub fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

/// Parses an input into a set of operations for creating an SVG.
pub async fn parse(input: &str) -> Vec<Operation> {
    let handles = input.as_bytes().iter().map(|&byte| {
        tokio::spawn(async move {
            match byte {
                b'0' => Home,
                b'1'..=b'9' => {
                    // In ASCII, numerals start at 0x30, so this converts the u8 value
                    // of b'2' to 2.
                    let distance = (byte - 0x30) as isize;
                    Forward(distance)
                }
                b'a' | b'b' | b'c' => TurnLeft,
                b'd' | b'e' | b'f' => TurnRight,
                _ => Noop(byte),
            }
        })
    });
    let results = join_all(handles).await;
    results.into_iter().map(|handle| handle.unwrap()).collect()
}

pub fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();

    let mut path_data = Vec::<Command>::with_capacity(operations.len());
    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    path_data.push(start_at_home);

    for op in operations {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("warning: illegal byte encountered: {:?}", byte);
            }
        };

        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);

        turtle.wrap();
    }
    path_data
}

pub fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);

    document
}

/// Runs the svg generation code using a randomly generated UUID as input.
pub async fn run_svg_code() {
    let input = Uuid::new_v4().to_string();
    let filename = format!("{}.svg", input);

    let operations = parse(&input).await;
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(filename, &document).unwrap();
}
