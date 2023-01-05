use std::fs;

fn main() {
    let f: Vec<String> = fs::read_to_string("resources/day2.input")
        .unwrap()
        .trim_end_matches("\n")
        .split("\n")
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
    let mut total: u64 = 0;
    let ribbon_total = ribbon_amount(&f);

    for i in f.iter() {
        let a = area(i.to_string());
        total = total + a;
    }
    println!("total wrapping paper: {:?}", total);
    println!("total ribbon needed: {:?}", ribbon_total);
}

fn ribbon_amount(rib: &Vec<String>) -> u64 {
    let mut rt: u64 = 0;
    for i in rib.iter() {
        let mut r: Vec<u64> = i.split("x").filter_map(|x| x.parse::<u64>().ok()).collect();
        r.sort();
        let t = 2 * (r[0]) + 2 * (r[1]) + (r[0] * r[1] * r[2]);
        rt += t;
    }
    rt
}

fn area(deminsions: String) -> u64 {
    let mut i: Vec<u64> = deminsions
        .split("x")
        .filter_map(|d| d.parse::<u64>().ok())
        .collect();
    let area = 2 * (i[0] * i[1]) + 2 * (i[1] * i[2]) + 2 * (i[2] * i[0]);
    i.sort();

    let sa = i[0] * i[1];
    area + sa
}
