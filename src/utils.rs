use std::env;
use std::fs;
use std::path::Path;
use aoc_bud::Aoc;

pub fn save_answer(ans: &str, part: &str) {
    let curr_path = get_current_working_dir();

    let ans_path = Path::new(&curr_path) // /Users/GK47LX/source/AdventOfCode2015/target/aoc/aoc-autobuild/
        .parent() // /Users/GK47LX/source/AdventOfCode2015/target/aoc/
        .unwrap()
        .parent() // /Users/GK47LX/source/AdventOfCode2015/target/
        .unwrap()
        .parent() // /Users/GK47LX/source/AdventOfCode2015/
        .unwrap()
        .join("ans"); // /Users/GK47LX/source/AdventOfCode2015/ans/

    let file_path = ans_path.join(format!("{}.txt", part));

    fs::write(file_path, ans).expect("Unable to write file");
}

pub fn submit_answer(year: i32, day: u8, part: &str, ans: &str) {
    let aoc = Aoc::new(day, year);
    if part == "1" {
        match aoc.solve1(ans) {
            Ok(_v) => print !("Success! Right answer!!\n"),
            Err(_e) => print !("{:?}", _e),
        };
    }
    else{
        match aoc.solve2(ans) {
            Ok(_v) => print !("Success! Right answer!!\n"),
            Err(_e) => print !("{:?}", _e),
        };
    }
    print !("\n")
}

pub fn solve(year: i32, day: u8, part: &str, ans: &str){
    let file_name = format!("day{day}.{part}");
 
    save_answer(&ans, &file_name);

    submit_answer(year, day, part, &ans);    
}

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();

    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}
