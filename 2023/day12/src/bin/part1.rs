use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::option::Option;
use std::vec;
use core::cmp::{max, min};
use scanf::sscanf;

fn solution(row: Vec<char>, cond: Vec<usize>) -> u32 {
    let m = cond.len();
    let n = row.len();

    // dp[i][j]: the total number of arrangements of row[0..=j]
    // that satisfy cond[0..=i] (j must be the last #).
    let mut dp = vec![vec![0; n]; m];

    // Initialize the dp[0][j].
    let first_cond = cond[0];
    if first_cond > n {
        return 0;
    }
    for j in first_cond-1..n {
        if j + 1 == first_cond {
            if row[j+1-first_cond..=j].iter().all(|&c| c == '?' || c == '#') {
                dp[0][j] = 1;
            }
        }
        else {
            if row[j+1-first_cond..=j].iter().all(|&c| c == '?' || c == '#') &&
                row[0..=j-first_cond].iter().all(|&c| c == '?' || c == '.') {
                dp[0][j] = 1;
            }
        }
    }
    println!("dp[0]:\n{:?}", dp[0]);

    // Search for other conditions.
    let mut cur_cond_tot = cond[0];
    for i in 1..m {
        cur_cond_tot += cond[i];
        //  cond[i-1]     all . or ?       all . or #
        // |----------| |------------|   |------------|
        //                           ^j-cond[i]       ^j
        for j in cur_cond_tot..n {
            let middle_end = j - cond[i];
            if row[middle_end+1..=j].iter().all(|&c| c == '?' || c == '#') {
                for len_middle in 1..=j-cur_cond_tot+1 {
                    let middle_start = middle_end + 1 - len_middle;
                    if row[middle_start..=middle_end].iter().all(|&c| c == '.' || c == '?') {
                        dp[i][j] += dp[i-1][middle_start-1];
                    }
                }
            }
        }
        println!("dp[{}]:\n{:?}", i, dp[i]);
    }

    let mut tot = 0;
    for j in (0..n).rev() {
        tot += dp[m-1][j];
        if row[j] == '#' {
            break;
        }
    }
    tot
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    for line in reader.lines() {
        let mut row_str: String = String::new();
        let mut cond_str: String = String::new();
        sscanf!(&line.unwrap(), "{} {}", row_str, cond_str);
        let row: Vec<char> = row_str.chars().collect();
        let cond: Vec<usize> = cond_str.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let ans = solution(row, cond);
        println!("Result: {}", ans);
        tot += ans;
    }

    println!("{}", tot);
}
