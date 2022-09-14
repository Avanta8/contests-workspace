//{"name":"C. Magic Grid","group":"Codeforces - Manthan, Codefest 19 (open for everyone, rated, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1208/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n","output":"8 9 1 13\n3 12 7 5\n0 2 4 11\n6 10 15 14\n"},{"input":"8\n","output":"19 55 11 39 32 36 4 52\n51 7 35 31 12 48 28 20\n43 23 59 15 0 8 16 44\n3 47 27 63 24 40 60 56\n34 38 6 54 17 53 9 37\n14 50 30 22 49 5 33 29\n2 10 18 46 41 21 57 13\n26 42 62 58 1 45 25 61\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMagicGrid"}}}

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
    collections::arr2d::Arr2d,
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
    let mut mat = Arr2d::new(n, n, 0);
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            let x = 4 * (j + i * n / 2);
            mat[i][j] = x;
            mat[i][j + n / 2] = x + 1;
            mat[i + n / 2][j] = x + 2;
            mat[i + n / 2][j + n / 2] = x + 3;
        }
    }
    out_line!(mat);
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
