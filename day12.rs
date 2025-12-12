use std::fs;

fn main() {

    let lines = fs::read_to_string("input.txt").expect("Cannot open file");

    let mut presents : Vec<usize> = vec![];
    let mut can_fit = 0;
    let mut part = 0;

    for line in lines.lines() {
        if line.is_empty() {
            presents.push( part );
            part = 0;
        } else if line.contains(":") {
            if line.contains("x") {
                let regions: Vec<usize> = line.split(&['x', ':', ' '])
                                              .filter(|&c| c != "")
                                              .map(|s| s.parse::<usize>().unwrap())
                                              .collect();
                let region_dim = regions[0] * regions[1];
                let mut request_dim = 0;

                for (i, quantity) in regions[2..].iter().enumerate() {
                    request_dim += quantity * presents.get(i).expect("Format error");
                }
                if request_dim <= region_dim {
                    can_fit += 1;
                }
            }
        } else {
            part += line.chars().filter(|&c| c == '#').count();
        }
    }

    println!("Result => {}", can_fit);
}
