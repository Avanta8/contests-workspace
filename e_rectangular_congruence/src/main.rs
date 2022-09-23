//{"name":"E. Rectangular Congruence","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"https://codeforces.com/contest/1734/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n0 0\n","output":"0 1\n0 0\n"},{"input":"3\n1 1 1\n","output":"1 2 2\n1 1 0\n1 0 1\n"},{"input":"5\n1 4 1 2 4\n","output":"1 0 1 3 4\n1 4 3 1 0\n2 4 1 0 2\n1 2 2 2 2\n2 2 0 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERectangularCongruence"}}}

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
    collections::{arr2d::Arr2d, iter_ext::IterExt},
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

    let arr = input.read_vec::<i64>(n);

    let mut mat = Arr2d::new(n, n, 0);
    for (i, &x) in arr.iter().enumerate() {
        mat[(i, i)] = x;
    }

    for row in 0..n {
        dbg!(mat.row(row).collect_vec());
    }

    for y in 0..n {
        for x in 0..n {
            if y != x {
                mat[(y, x)] = (arr[y] + arr[x]) % n as i64;
            }
        }
    }

    for row in 0..n {
        dbg!(mat.row(row).collect_vec());
    }

    for y in 0..n {
        for x in 0..n {
            mat[(y, x)] = (mat[(y, x)] - y as i64 + x as i64).rem_euclid(n as i64);
        }
    }

    for row in 0..n {
        dbg!(mat.row(row).collect_vec());
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
    // submit();
}
//END MAIN
