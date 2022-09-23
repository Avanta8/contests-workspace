//{"name":"C. Removing Smallest Multiples","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"https://codeforces.com/contest/1734/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n111111\n7\n1101001\n4\n0000\n4\n0010\n8\n10010101\n15\n110011100101100\n","output":"0\n11\n4\n4\n17\n60\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRemovingSmallestMultiples"}}}

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

use std::collections::{BTreeSet, VecDeque};

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

    let mut req = vec![];

    let t = input.read_string();
    for (i, x) in (1..).zip(t.chars()) {
        if x == '0' {
            req.push(i);
        }
    }

    let mut pointers = vec![VecDeque::new(); n + 2];
    for x in 0..n + 2 {
        pointers[x].push_back(x);
    }

    let mut total = 0;
    for &r in req.iter() {
        total += pointers[r][0];

        for p in std::mem::take(&mut pointers[r]) {
            let place = r + p;
            if place >= pointers.len() {
                continue;
            }
            pointers[place].push_front(p);
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
