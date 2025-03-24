mod aoc;

fn usage(exe: &str) -> ! {
    use std::path::Path;
    let exe = Path::new(exe).file_name().unwrap().to_str().unwrap();
    println!("usage: {exe} <day (1-25)>");
    std::process::exit(0)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
    }

    let day: u8 = match args[1].parse() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            usage(&args[0]);
        }
    };

    match day {
        1  => aoc::day_01::solve(),
        2  => aoc::day_02::solve(),
        3  => aoc::day_03::solve(),
        4  => aoc::day_04::solve(),
        5  => aoc::day_05::solve(),
        6  => aoc::day_06::solve(),
        7  => aoc::day_07::solve(),
        8  => aoc::day_08::solve(),
        9  => aoc::day_09::solve(),
        10 => aoc::day_10::solve(),
        11 => aoc::day_11::solve(),
        12 => aoc::day_12::solve(),
        13 => aoc::day_13::solve(),
        14 => aoc::day_14::solve(),
        15 => aoc::day_15::solve(),
        16 => aoc::day_16::solve(),
        17 => aoc::day_17::solve(),
        18 => aoc::day_18::solve(),
        19 => aoc::day_19::solve(),
        20 => aoc::day_20::solve(),
        21 => aoc::day_21::solve(),
        22 => aoc::day_22::solve(),
        23 => aoc::day_23::solve(),
        24 => aoc::day_24::solve(),
        25 => aoc::day_25::solve(),
        _ => println!("day {day} not implemented")
    }
}
