//{"name":"Minimizing Coins","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1634","interactive":false,"timeLimit":1000,"tests":[{"input":"3 11\n1 5 7\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimizingCoins"}}}

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

use std::cmp::min;

use algo_lib::{
    collections::iter_ext::IterExt,
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
    let (n, x) = input.read::<(usize, usize)>();
    let arr = input.read_vec::<usize>(n);

    let mut dp = vec![u64::MAX; x + 1];
    dp[0] = 0;

    for i in 0..x {
        for &c in arr.iter() {
            let r = i + c;
            if dp[i] == u64::MAX || r > x {
                continue;
            }

            dp[r] = min(dp[r], dp[i] + 1);
        }
    }

    if dp[x] < u64::MAX {
        out_line!(dp[x]);
    } else {
        out_line!(-1);
    }
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
