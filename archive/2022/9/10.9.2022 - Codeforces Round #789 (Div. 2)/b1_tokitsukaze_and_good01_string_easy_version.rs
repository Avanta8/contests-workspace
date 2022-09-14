//{"name":"B1. Tokitsukaze and Good 01-String (easy version)","group":"Codeforces - Codeforces Round #789 (Div. 2)","url":"https://codeforces.com/contest/1678/problem/B1","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10\n1110011000\n8\n11001111\n2\n00\n2\n11\n6\n100110\n","output":"3\n0\n0\n0\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1TokitsukazeAndGood01StringEasyVersion"}}}

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

fn get_groups(v: &[u8]) -> Vec<usize> {
    let mut groups = vec![];
    let mut c = v[0];
    let mut count = 1;
    for &x in &v[1..] {
        if x == c {
            count += 1;
        } else {
            groups.push(count);
            count = 1;
            c = x;
        }
    }
    groups.push(count);
    groups
}

fn solve(input: &mut Input, _test_case: usize) {
    let _n: usize = input.read();
    let s = input.read_string().bytes().map(|b| b - b'0').collect_vec();

    let groups = get_groups(&s);

    let mut parity = false;
    let mut total = 0;
    for len in groups {
        let even = len % 2 == 0;
        if even && !parity || !even && parity {
            parity = false;
        } else {
            parity = true;
            total += 1;
        }
    }

    out_line!(total);
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
