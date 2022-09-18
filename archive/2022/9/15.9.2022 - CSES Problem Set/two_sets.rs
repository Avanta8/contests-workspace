//{"name":"Two Sets","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1092","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n","output":"YES\n4\n1 2 4 7\n3\n3 5 6\n"},{"input":"6\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TwoSets"}}}

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
    let n = input.read::<i64>();

    match n % 4 {
        0 => {
            let mut a = vec![];
            let mut b = vec![];
            let mut nums = (1..=n).collect::<VecDeque<_>>();
            while !nums.is_empty() {
                a.push(nums.pop_front().unwrap());
                b.push(nums.pop_front().unwrap());
                a.push(nums.pop_back().unwrap());
                b.push(nums.pop_back().unwrap());
            }
            out_line!("YES");
            out_line!(a.len());
            out_line!(a);
            out_line!(b.len());
            out_line!(b);
        }
        3 => {
            let mut a = vec![];
            let mut b = vec![];
            let mut nums = (4..=n).collect::<VecDeque<_>>();
            while !nums.is_empty() {
                a.push(nums.pop_front().unwrap());
                b.push(nums.pop_front().unwrap());
                a.push(nums.pop_back().unwrap());
                b.push(nums.pop_back().unwrap());
            }
            a.extend([1, 2]);
            b.push(3);
            out_line!("YES");
            out_line!(a.len());
            out_line!(a);
            out_line!(b.len());
            out_line!(b);
        }
        _ => {
            out_line!("NO");
        }
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
