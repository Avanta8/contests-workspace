//{"name":"Permutations","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1070","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n","output":"4 2 5 3 1\n"},{"input":"3\n","output":"NO SOLUTION\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Permutations"}}}

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

    if n <= 3 {
        if n == 1 {
            out_line!(1);
        } else {
            out_line!("NO SOLUTION");
            return;
        }
    }

    let mut ans = VecDeque::new();
    ans.push_back(n);

    for x in (1..n).step_by(2) {
        ans.push_front(x);
        if x + 1 < n {
            ans.push_back(x + 1);
        }
    }

    out_line!(ans.into_iter().collect_vec());
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
