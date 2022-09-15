//{"name":"B. Best Permutation","group":"Codeforces - Educational Codeforces Round 135 (Rated for Div. 2)","url":"https://codeforces.com/contest/1728/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n5\n6\n","output":"2 1 3 4\n1 2 3 4 5\n4 5 1 2 3 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBestPermutation"}}}

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

use std::collections::VecDeque;

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
    let n = input.read::<usize>();

    let mut ans = vec![0; n];
    ans[n - 1] = n;
    ans[n - 2] = n - 1;

    let mut deque = (1..n - 1).collect::<VecDeque<_>>();
    for i in (0..n - 2).rev().step_by(2) {
        ans[i] = deque.pop_front().unwrap();
        if let Some(x) = deque.pop_back() {
            ans[i - 1] = x;
        }
    }

    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
