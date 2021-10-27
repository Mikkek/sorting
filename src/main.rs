use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let unsorted = file_format(&filename);
    let sorted = count_sort(unsorted);
    write_file(sorted, &filename);

}

//32767+1
fn count_sort(coords: Vec<Coordinate>) -> Vec<Coordinate>{
    let mut counter: Vec<i32> = vec!(0; 32768);
    let mut sorted_vec = vec!(Coordinate::empty(); coords.len());

    for element in coords.iter(){
        counter[element.y as usize] += 1;
    };
    
    for i in 1..counter.len(){
        counter[i] += counter[i - 1];
    }

    for element in coords{
        let index = counter[element.y as usize];
        sorted_vec[(index - 1) as usize] = element; 
    }
    sorted_vec
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

fn write_file(nums: Vec<Coordinate>, filename: &str) -> Result<(), Error>{
    let path = format!("sorted_{}", filename);

    let mut file = File::create(path)?;
    for i in 0..nums.len(){
        let line = format!("{}\t{}\n", nums[i].x, nums[i].y);
        write!(file, "{}", line)?;
    }
    Ok(())
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