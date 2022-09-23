//{"name":"A. Select Three Sticks","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"https://codeforces.com/contest/1734/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n4\n7 3 7 3\n5\n3 4 2 1 1\n8\n3 1 4 1 5 9 2 6\n","output":"2\n4\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASelectThreeSticks"}}}

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
    let n = input.read();
    let arr = input.read_vec::<i64>(n);

    let mut best = u64::MAX;
    for &target in arr.iter() {
        let mut diffs = vec![];
        for &x in arr.iter() {
            diffs.push(x.abs_diff(target));
        }
        diffs.sort();
        best = std::cmp::min(best, diffs[0] + diffs[1] + diffs[2]);
    }
    out_line!(best);
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
