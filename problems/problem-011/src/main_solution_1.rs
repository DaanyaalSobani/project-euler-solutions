use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path_str = concat!(env!("CARGO_MANIFEST_DIR"), "/input_grid.txt");
    let path = Path::new(path_str);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("file contents are in s now!\n"),
    }
    
    let mut grid: Vec<Vec<u64>> = Vec::new();
    for line in s.lines() {
        let row: Vec<u64> = line.split(" ")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
        grid.push(row);
    }
    let mut max : u64 = 0;
    let mut cord_of_max: (usize,usize) = (0,0);
    let mut direction = "";
    for i in 0..grid.len() {
        for j in 0..grid[i].len(){            
            let mut prod_right: u64 = 0;
            /*product of element and three elements to the right */
            if j+3 < grid[i].len() {
                 prod_right = grid[i][j] * grid[i][j+1] * grid[i][j+2] * grid[i][j+3];  
            }
            if prod_right > max {
                cord_of_max = (i,j);
                max=prod_right;
                direction = "right";
            }
            let mut prod_down: u64 = 0;
            /*product of element and three elements to the right */
            if i+3 < grid.len() {
                 prod_down = grid[i][j] * grid[i+1][j] * grid[i+2][j] * grid[i+3][j];  
            }
            if prod_down > max {
                cord_of_max = (i,j);
                max=prod_down;
                direction = "down";
            }

            let mut prod_right_diag: u64 = 0;
            /*product of element and three elements to the right */
            if i+3 < grid.len() && j+3 < grid[i].len() {
                 prod_right_diag = grid[i][j] * grid[i+1][j+1] * grid[i+2][j+2] * grid[i+3][j+3];  
            }
            if prod_right_diag > max {
                cord_of_max = (i,j);
                max=prod_right_diag;
                direction = "right_diag";
            }
            let mut prod_left_diag: u64 = 0;
            /*product of element and three elements to the right */
            if i+3 < grid.len() && j >= 3 {
                 prod_left_diag = grid[i][j] * grid[i+1][j-1] * grid[i+2][j-2] * grid[i+3][j-3];  
            }
            if prod_left_diag > max {
                cord_of_max = (i,j);
                max=prod_left_diag;
                direction = "left_diag";
            }
            
        }
    }
    println!("{:?}",max);
    println!("{:?}",cord_of_max);
    println!("{:?}",direction);

}
