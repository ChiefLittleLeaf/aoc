use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Hash, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct House {
    point: Point,
    presents: i32,
}

impl House {
    fn new() -> Self {
        Self {
            point: Point::new(0, 0),
            presents: 1,
        }
    }

    fn location(&mut self, direction: char) {
        match direction {
            'v' => self.point.y = &self.point.y - 1,
            '>' => self.point.x = &self.point.x + 1,
            '<' => self.point.x = &self.point.x - 1,
            '^' => self.point.y = &self.point.y + 1,
            _ => println!("unknown char"),
        }
    }

    fn add_present(&mut self, h: &mut HashMap<Point, House>) {
        if h.contains_key(&self.point) {
            self.presents += 1;
            h.insert(self.point.clone(), self.clone());
        }
    }
}

fn main() {
    let mut file = File::open("../resources/day3.input").expect("file does not exist");
    let mut f = String::new();
    file.read_to_string(&mut f).expect("unable to read file");
    let f = f.trim_matches('\n');
    let mut delivery: HashMap<Point, House> = HashMap::new();
    let mut santa = House::new();
    let mut robo_santa = House::new();
    delivery.entry(santa.point.clone()).or_insert(santa.clone());
    delivery
        .entry(robo_santa.point.clone())
        .or_insert(robo_santa.clone());
    robo_santa.add_present(&mut delivery);
    for i in f.chars() {
        santa.location(i);
        santa.add_present(&mut delivery);
        delivery.entry(santa.point.clone()).or_insert(santa.clone());
    }
    println!("len of first year delivery = {:?}", delivery.len());

    let mut h2: HashMap<Point, House> = HashMap::new();
    let mut s2 = House::new();
    let mut robo_santa = House::new();
    h2.entry(s2.point.clone()).or_insert(s2.clone());
    h2.entry(robo_santa.point.clone())
        .or_insert(robo_santa.clone());
    robo_santa.add_present(&mut h2);

    for (i, v) in f.chars().enumerate() {
        if i % 2 == 0 {
            s2.location(v);
            s2.add_present(&mut h2);
            h2.entry(s2.point.clone()).or_insert(s2.clone());
        } else {
            robo_santa.location(v);
            robo_santa.add_present(&mut h2);
            h2.entry(robo_santa.point.clone())
                .or_insert(robo_santa.clone());
        }
    }
    println!("len of second delivery = {:?}", h2.len());
}
