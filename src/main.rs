//Advent of Code 2023 - Day 2 Part 2
//Patryk Perkiewicz

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
fn main() {
    let input = read_lines("input.txt");
    //let input = read_lines("test_input.txt");
    let mut output = 0;

    for mut input_line in input {
    //    let mut do_break = 0;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
   //     println!("input_line: {}", input_line);
   //     println!("id: {}", input_line.get(5..input_line.find(":").unwrap_or(5)).unwrap_or("0").parse::<u32>().unwrap());
    //    let id = input_line.get(5..input_line.find(":").unwrap_or(5)).unwrap_or("0").parse::<u32>().unwrap();

   //     println!("new input_line: {}", input_line.get(input_line.find(":").unwrap_or(5)+2..input_line.chars().count()).unwrap().to_string());
        input_line = input_line.get(input_line.find(":").unwrap_or(5)+2..input_line.chars().count()).unwrap().to_string();
        
        let sets: Vec<&str> = input_line.split("; ").collect();

        for set in sets {
   //         println!("sets: {}", set);

            let results: Vec<&str> = set.split(", ").collect();
            
            for result in results {
  //              println!("Color: {}", result);
                let color: Vec<&str> = result.split(" ").collect();

  //              println!("color[0]: {}, color[1]: {}", color[0], color[1]);

                if color[1] == "red" {
                    if color[0].parse::<u32>().unwrap() > min_red {
                        min_red = color[0].parse::<u32>().unwrap();
                    }
                }   
                else if color[1] == "green" {
                    if color[0].parse::<u32>().unwrap() > min_green {
                        min_green = color[0].parse::<u32>().unwrap();
                    }
                }
                else if color[1] == "blue" {
                    if color[0].parse::<u32>().unwrap() > min_blue {
                        min_blue = color[0].parse::<u32>().unwrap();
                    }
                }             
            }
        }
        output+=min_blue*min_green*min_red;
    }
    println!("{}", output);
}
    
