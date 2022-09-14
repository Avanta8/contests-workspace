//{"name":"C. Border","group":"Codeforces - Codeforces Round #499 (Div. 1)","url":"https://codeforces.com/contest/1010/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2 8\n12 20\n","output":"2\n0 4\n"},{"input":"3 10\n10 20 30\n","output":"1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBorder"}}}

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
    numbers::gcd::gcd,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k) = input.read::<(usize, u32)>();
    let arr = input.read_vec::<u32>(n);

    let arr_gcd = arr.iter().copied().reduce(gcd).unwrap();
    let ovr_gcd = gcd(arr_gcd, k);

    let possible = (0..k).step_by(ovr_gcd as usize).collect_vec();

    out_line!(possible.len());
    out_line!(possible);
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
