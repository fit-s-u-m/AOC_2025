const START:usize = 50;
const LAST:usize = 100;

#[derive(Debug)]

enum Rotation { R, L }

#[derive(Debug)]
struct Input {
    rot: Option<Rotation>,
    num: usize,
}

fn common() -> Vec<Input> {
    let data = aoc::read_one_per_line::<String>("./data/day1.txt");
    let mut output:Vec<Input> = vec![];
    match data {
        Ok (lines) =>{
            for line in &lines {
                if line.is_empty() {
                    continue;
                }
                let rotation = line.get(0..1).unwrap_or("<empty>");
                let number_str = line.get(1..).unwrap_or("<empty>");
                match number_str.parse::<usize>() {
                    Ok(num) =>{
                        if let Some(rot_char) = rotation.chars().next() {
                            let current_val =   match rot_char {
                                'R' => Input {rot:Some(Rotation::R), num},
                                'L' => Input {rot:Some(Rotation::L), num},
                                _   =>  Input {rot:None,num}                             
                            };
                            output.push(current_val);

                        } else {
                            println!("Rotation is empty");
                        }
                    },
                    Err(_) => eprintln!("Failed to parse number from line: {:?}", line),
                }
            }

        }
        Err(e) =>eprintln!("Error Reading file: {:?}", e)
    }
    output
}
fn part_1() {
    let mut dial = START;
    let mut count = 0;
    let inputs = common();

    for i in inputs {
        dial = match i {
            Input {rot:Some(Rotation::R), num} => (dial + num) %LAST,
            Input {rot:Some(Rotation::L), num} => (dial + LAST - (num %LAST)) %LAST,
            Input {rot: _,num:_} => dial 
        };
        if dial == 0{
            count += 1;
        }
    }
    println!("dial: {dial}, count: {count}");
}
fn part_2 (){
    let mut dial = START;
    let mut count = 0;
    let mut count_mid_zeros = 0;
    let inputs = common();

    for i in inputs {
        dial = match i {
            Input {rot:Some(Rotation::R), num} =>{
                 if (dial + num) >100 {
                    count_mid_zeros += 1;
                 }
                 (dial + num) %LAST
             },
            Input {rot:Some(Rotation::L), num} =>{
                if dial <  num {
                    count_mid_zeros += 1;
                 } 
                (dial + LAST - (num %LAST)) %LAST
            },
            Input {rot: _,num:_} => dial 
        };
        if dial == 0{
            count += 1;
        }
    }
    println!("part 2 dial: {dial}, count: {count} , midZero: {count_mid_zeros} , result {:?}",count + count_mid_zeros );

}
fn main(){
    part_1();
    part_2()
}
