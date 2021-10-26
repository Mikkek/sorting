use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let unsorted = file_format(&filename);
    //let sorted = count_sort(unsorted);
    println!("{:?}", unsorted);

}

//32767+1
fn count_sort(coords: Vec<Coordinate>) -> Vec<Coordinate>{
    let temp_coord = Coordinate{
        x: 0,
        y: 0,
    };
    let mut counter: Vec<i32> = vec!(0; 32768);
    let mut sorted_vec = vec!(Coordinate::empty(); coords.len());

    for element in &coords{
        counter[element.y as usize] += 1;
    };
    
    let mut prev_val = 0;
    for mut index in &counter{
        index += prev_val;
        prev_val = index;
    }

    for element in coords{
        let index = counter[element.y as usize];
        sorted_vec[index as usize] =element; 
    }
    unimplemented!()
}

fn file_format(filename: &str) -> Vec<Coordinate>{
    let content = fs::read_to_string(filename).expect("File not found");

    let coords = content.lines();
    let mut res_vec = Vec::new();
    for index in coords{
        let coord: Vec<&str> = index.split_whitespace().collect();
        let x = coord[0].parse().unwrap();
        let y = coord[1].parse().unwrap();
        res_vec.push(
            Coordinate{
                x,
                y,
            }
        );
    }
    res_vec
}

#[derive(Debug)]
#[derive(Clone)]
struct Coordinate{
    x: i32,
    y: i32
}

impl Coordinate{
    fn empty() -> Coordinate{
        Coordinate{
            x: 0,
            y: 0,
        }
    }
}