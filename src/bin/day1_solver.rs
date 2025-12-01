use advent_of_code_2025::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day1/input.txt", parse_input_line);
    println!("{}", (-1i64).rem_euclid(100));
    println!("Day 1 part 1 ");
    println!("Password is {}", count_zeros(&input) );
    println!("Day 1 part 2 ");
    println!("Password 2 is {}", count_zeros_2(&input) );
      Ok(())
}


fn parse_input_line(line:&str) -> Rotation {
    let dir = line.chars().take(1).collect::<String>();
    let count = line.chars().skip(1).collect::<String>().parse::<u32>().unwrap();
    match dir.as_str() {
        "L" => Rotation::Left(count),
        "R" => Rotation::Right(count),
        _ => panic!("Unknown direction"),
    }
}

enum Rotation {
    Left(u32),
    Right(u32),
}


fn count_zeros(rotation : &Vec<Rotation>) -> u32 {
    let mut zeroes = 0;
    let mut position: i64 = 50;
    for rotation in rotation {
        match rotation {
            Rotation::Left(count) => {
                position = (position - *count as i64)%100;

            }
            Rotation::Right(count) => {
                position = (position+ *count as i64)%100;
            }
        }
        if position == 0 { zeroes += 1; }
    }
    zeroes
}

fn count_zeros_2(rotation : &Vec<Rotation>) -> u32 {
    let mut zeroes = 0;
    let mut position: i64 = 50;
    for rotation in rotation {
        match rotation {
            Rotation::Left(count) => {
                let whole_turns = count / 100;
                let remaining_ticks = (count % 100) as i64;
                zeroes += whole_turns;
                if remaining_ticks > 0 {
                    let new_pos = position - remaining_ticks;
                    // When we get a negative but started on 0, Do not count. Already counted!
                    if new_pos < 0  && position != 0 { zeroes += 1; }
                    position = new_pos.rem_euclid(100) as i64;
                    if position == 0 { zeroes += 1; }
                }
            }
            Rotation::Right(count) => {
                let whole_turns = count / 100;
                let remaining_ticks = (count % 100) as i64;
                zeroes += whole_turns;
                if remaining_ticks > 0 {
                    let new_pos = position + remaining_ticks;
                    if new_pos > 100 { zeroes += 1; }
                    position = new_pos % 100;
                    if position == 0 { zeroes += 1; }
                }
            }
        }
        
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input =vec![Rotation::Left(50), Rotation::Left(101)]; //, Rotation::Right(1)];
        assert_eq!(count_zeros_2(&input), 2);
        // This test made me find the bug! Line 60 missed the check if we already were on 0.
    }
    #[test]
    fn test_part2_right() {
        let input =vec![Rotation::Right(49), Rotation::Right(101)]; //, Rotation::Right(1)];
        assert_eq!(count_zeros_2(&input), 2);
    }
}