//{"name":"Increasing Subsequence","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1145/","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n7 3 5 3 6 2 9 8\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IncreasingSubsequence"}}}

#![allow(
    unused_imports,
    non_snake_case,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::needless_range_loop
)]

use algo_lib::{
    collections::{iter_ext::IterExt, vec_ext::ConsecutiveIter},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let arr = input.read_vec::<u32>(n);

    // let mut map = BTreeMap::new();

    // map.insert(0, 0u32);
    // for &x in arr.iter() {
    //     let (_, &below_count) = map.range(..x).next_back().unwrap();
    //     if let Some((&above, &above_count)) = map.range(x..).next() {
    //         assert_eq!(above_count, below_count + 1);
    //         map.remove(&above);
    //     }
    //     map.insert(x, below_count + 1);
    // }

    // let ans = *map.values().last().unwrap();
    // out_line!(ans);

    let mut dp = vec![(0, None)]; // d[i] = (x, p) where x is the minimum number requried that can make an increasing subsequence of length i and p is the index where x is.

    let mut prev = vec![None; n];

    for (idx, &x) in arr.iter().enumerate() {
        let i = dp.partition_point(|&y| y.0 < x);
        if i == dp.len() {
            dp.push((x, Some(idx)));
        } else {
            dp[i] = (x, Some(idx));
        }
        prev[idx] = dp[i - 1].1;
    }

    let mut seq = vec![];
    let mut idx = dp.last().unwrap().1;
    while let Some(i) = idx {
        let x = arr[i];
        seq.push(x);
        idx = prev[i];
    }
    seq.reverse();

    dbg!(seq);

    let ans = dp.len() - 1;

    assert_eq!(ans, seq.len());
    assert!(seq.consecutive_iter().all(|(&a, &b)| a < b));

    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // submit();
}
//END MAIN
