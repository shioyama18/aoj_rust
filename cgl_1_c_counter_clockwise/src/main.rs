// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_1_C

#![allow(dead_code)]

macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| scan!($t ;;)).collect::<Vec<_>>()
    };
}

enum Rotation {
    CounterClockwise,
    Clockwise,
    OnlineBack,
    OnlineFront,
    OnSegment,
}

impl Rotation {
    fn translate(&self) -> isize {
        match self {
            Rotation::CounterClockwise => 1,
            Rotation::Clockwise => -1,
            Rotation::OnSegment => 0,
            _ => std::isize::MIN
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    fn dot(p1: Self, p2: Self) -> f64 {
        p1.x * p2.x + p1.y * p2.y
    }

    fn cross(p1: Self, p2: Self) -> f64 {
        p1.x * p2.y - p1.y * p2.x
    }

    fn distance(p1: Self, p2: Self) -> f64 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }

    fn ccw(p1: Self, p2: Self, p3: Point) -> Rotation {
        let a = p2 - p1;
        let b = p3 - p1;
        if Point::cross(a, b) > 0.0 {
            Rotation::CounterClockwise
        } else if Point::cross(a, b) < 0.0 {
            Rotation::Clockwise
        } else if Point::dot(a, b) < 0.0 {
            Rotation::OnlineBack
        } else if a.norm() < b.norm() {
            Rotation::OnlineFront
        } else {
            Rotation::OnSegment
        }
    }

    fn intersect(p1: Self, p2: Self, p3: Self, p4: Self) -> bool {
        let a = Point::ccw(p1, p2, p3).translate();
        let b = Point::ccw(p1, p2, p4).translate();
        let c = Point::ccw(p3, p4, p1).translate();
        let d = Point::ccw(p3, p4, p2).translate();

        a * b <= 0 && c * d <= 0
    }
}

use std::ops::{Add, Sub, Mul};

impl Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, k: f64) -> Self {
        Point {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

#[derive(Debug, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn project(&self, p: Point) -> Point {
        let base = self.p2 - self.p1;
        let r = Point::dot(p - self.p1, base) / base.norm();
        self.p1 + base * r
    }

    fn reflect(&self, p: Point) -> Point {
        p + (self.project(p) - p) * 2.0
    }

    fn intersect(&self, other: &Self) -> bool {
        Point::intersect(self.p1, self.p2, other.p1, other.p2)
    }

    fn distance_segement_point(&self, p: Point) -> f64 {
        if Point::dot(self.p2 - self.p1, p - self.p1) < 0.0 {
            Point::distance(p, self.p1)
        } else if Point::dot(self.p1 - self.p2, p - self.p2) < 0.0 {
            Point::distance(p, self.p2)            
        } else {
            (Point::cross(self.p2 - self.p1, p - self.p1) /
                Point::distance(self.p2, self.p1)).abs()
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        if self.intersect(other) { 
            0.0 
        } else {
            self.distance_segement_point(other.p1)
                .min(self.distance_segement_point(other.p2))
                .min(other.distance_segement_point(self.p1))
                .min(other.distance_segement_point(self.p2))
        }
        
    }
}

fn main() {
    let buffer = scan!(f64;;);
    let points = buffer
        .chunks(2)
        .map(|chunk| Point { x: chunk[0], y: chunk[1] })
        .collect::<Vec<_>>();;

    let q = scan!(usize);

    for _ in 0..q {
        let (x, y) = scan!(f64, f64);
        let p = Point { x, y };
        match Point::ccw(points[0], points[1], p) {
            Rotation::CounterClockwise => println!("COUNTER_CLOCKWISE"),
            Rotation::Clockwise => println!("CLOCKWISE"),
            Rotation::OnlineBack => println!("ONLINE_BACK"),
            Rotation::OnlineFront => println!("ONLINE_FRONT"),
            Rotation::OnSegment => println!("ON_SEGMENT"),
        }
    }
}
