use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_file_to_vec(file: BufReader<&File>) -> Vec<String> {
    let mut res = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        res.push(l);
    }
    res
}

fn part01(values : &Vec<String>) {
    let mut sum2 = 0;
    let mut sum3 = 0;
    for line in values {
        let mut count = HashMap::new();
        let chars = line.chars();
        for c in chars {
            if count.contains_key(&c) {
                let entry = count.entry(c).or_insert(0);
                *entry += 1;
            } else {
                count.insert(c, 1);
            }
        }
        let mut contains2 = 0;
        let mut contains3 = 0;

        for (_, cnt) in &count {
            if *cnt == 2 {
                contains2 = 1;
            } else if *cnt == 3 {
                contains3 = 1;
            }
        }
        sum2 += contains2;
        sum3 += contains3;
    }

    println!("Part01 result = {}", sum2 * sum3);
}

fn part02(values : &Vec<String>) {
    let mut arr: [[i32; 250]; 250] = [[0; 250]; 250];

    let mut max = 0;
    let mut idx = 0;
    let mut prt = 0;

    for line_idx in 0..values.len() {
        let line = &values[line_idx];
        for partner_idx in (line_idx+1)..values.len() {
            let partner = &values[partner_idx];
            for chr in 0..line.len() {
                if &line[chr..chr+1] == &partner[chr..chr+1] {
                    arr[line_idx][partner_idx] += 1;
                    if arr[line_idx][partner_idx] > max {
                        max = arr[line_idx][partner_idx];
                        idx = line_idx;
                        prt = partner_idx;
                    }
                }
            }
        }
    }

    let line = &values[idx];
    let partner = &values[prt];
    print!("Part 02 result = ");
    for chr in 0..line.len() {
        if &line[chr..chr+1] == &partner[chr..chr+1] {
            print!("{}", &line[chr..chr+1]);
        }
    }
    print!("\n");
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let values = read_file_to_vec(file);

    part01(&values);
    part02(&values);
}
