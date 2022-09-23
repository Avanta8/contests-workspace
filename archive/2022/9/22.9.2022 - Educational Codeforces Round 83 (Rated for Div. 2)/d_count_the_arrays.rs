//{"name":"D. Count the Arrays","group":"Codeforces - Educational Codeforces Round 83 (Rated for Div. 2)","url":"https://codeforces.com/contest/1312/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n","output":"6\n"},{"input":"3 5\n","output":"10\n"},{"input":"42 1337\n","output":"806066790\n"},{"input":"100000 200000\n","output":"707899035\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountTheArrays"}}}

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
    numbers::{mod_int::ModIntF, mod_utils::Combinations, number_ext::Power},
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read::<(i32, i32)>();

    if n == 2 {
        out_line!(0);
        return;
    }

    let combs = Combinations::<ModIntF>::new(m as usize + 1);

    let ans =
        combs.c(m as usize, n as usize - 1) * ModIntF::new(2).power(n - 3) * ModIntF::new(n - 2);

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
