//{"name":"B2. Tokitsukaze and Good 01-String (hard version)","group":"Codeforces - Codeforces Round #789 (Div. 2)","url":"https://codeforces.com/contest/1678/problem/B2","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10\n1110011000\n8\n11001111\n2\n00\n2\n11\n6\n100110\n","output":"3 2\n0 3\n0 1\n0 1\n3 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B2TokitsukazeAndGood01StringHardVersion"}}}

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

use std::iter::repeat;

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

fn get_groups(v: &[u8]) -> Vec<i32> {
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

    let mut ans = vec![];
    let mut ones = true;
    let mut diff = 0;
    let mut total = 0;
    for (i, len) in groups.iter().copied().enumerate() {
        let parity = (i % 2) as u8;
        let new_len = len + diff;

        if len == 1 {
            ones = true;
            diff += 1;
        } else {
            if new_len % 2 == 0 {
                ans.extend(repeat(parity).take(new_len as usize));
                diff = 0;
            } else {
                if new_len == 1 || len == 2 && ones {
                    ones = true;
                    diff += 2;
                } else {
                    ans.extend(repeat(parity).take(new_len as usize + 1));
                    diff = -1;
                }
            }
        }

        if new_len % 2 == 1 {
            total += 1
        }
    }

    if diff > 0 {
        let parity = ans.last().copied().unwrap_or(0);
        ans.extend(vec![parity; diff as usize]);
    }

    let segs = get_groups(&ans).len();
    out_line!(total, segs);
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
