//{"name":"Book Shop","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1158","interactive":false,"timeLimit":1000,"tests":[{"input":"4 10\n4 8 5 3\n5 12 8 1\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BookShop"}}}

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

use std::cmp::max;

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
    let prices = input.read_vec::<usize>(n);
    let page_counts = input.read_vec::<usize>(n);

    let mut dp = vec![0; x + 1];

    for (&price, &pages) in prices.iter().zip(page_counts.iter()) {
        for i in (0..x).rev() {
            let r = i + price;
            if r > x {
                continue;
            }

            dp[r] = max(dp[r], dp[i] + pages);
        }
    }

    out_line!(dp.into_iter().max().unwrap());
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
