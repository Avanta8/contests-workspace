//{"name":"B. Rule of League","group":"Codeforces - Codeforces Round #821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 2 0\n8 1 2\n3 0 0\n2 0 1\n6 3 0\n","output":"1 1 4 4\n-1\n-1\n2\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRuleOfLeague"}}}

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
    let (n, mut x, mut y) = input.read::<(i64, i64, i64)>();

    if x > y {
        std::mem::swap(&mut x, &mut y);
    }

    if x != 0 || y == 0 || (n - 1) % y != 0 {
        out_line!(-1);
        return;
    }

    let mut ans = vec![];
    let mut c = 2;
    for _ in 0..(n - 1) / y {
        ans.extend(std::iter::repeat(c).take(y as usize));
        c += y;
    }

    assert_eq!(ans.len() as i64, n - 1);
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
