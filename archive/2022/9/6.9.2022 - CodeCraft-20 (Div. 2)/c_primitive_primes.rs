//{"name":"C. Primitive Primes","group":"Codeforces - CodeCraft-20 (Div. 2)","url":"https://codeforces.com/contest/1316/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"3 2 2\n1 1 2\n2 1\n","output":"1\n"},{"input":"2 2 999999937\n2 1\n3 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrimitivePrimes"}}}

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

use algo_lib::io::{
    input::Input,
    output::output,
    task_io_settings::{TaskIoSettings, TaskIoType},
    task_runner::run_task,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, p) = input.read::<(usize, usize, i64)>();

    let A = input.read_vec::<i64>(n);
    let B = input.read_vec::<i64>(m);

    let ai = A.into_iter().position(|x| x % p != 0).unwrap();
    let bi = B.into_iter().position(|x| x % p != 0).unwrap();

    out_line!(ai + bi);
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
