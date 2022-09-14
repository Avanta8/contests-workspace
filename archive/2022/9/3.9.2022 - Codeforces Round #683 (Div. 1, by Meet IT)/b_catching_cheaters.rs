//{"name":"B. Catching Cheaters","group":"Codeforces - Codeforces Round #683 (Div. 1, by Meet IT)","url":"https://codeforces.com/contest/1446/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4 5\nabba\nbabab\n","output":"5\n"},{"input":"8 10\nbbbbabab\nbbbabaaaaa\n","output":"12\n"},{"input":"7 7\nuiibwws\nqhtkxcn\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCatchingCheaters"}}}

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
    let (n, m) = input.read::<(usize, usize)>();
    let A = input.read_string().chars().collect_vec();
    let B = input.read_string().chars().collect_vec();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for (i, a) in A.iter().copied().enumerate() {
        for (j, b) in B.iter().copied().enumerate() {
            if a == b {
                dp[i + 1][j + 1] = dp[i][j] + 2;
            } else {
                let best = max(dp[i][j + 1], dp[i + 1][j]) - 1;
                dp[i + 1][j + 1] = max(best, 0);
            }
        }
    }

    let best = dp.into_iter().flatten().max().unwrap();
    out_line!(best);
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
}
//END MAIN
