//{"name":"C. Adding Powers","group":"Codeforces - Educational Codeforces Round 83 (Rated for Div. 2)","url":"https://codeforces.com/contest/1312/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 100\n0 0 0 0\n1 2\n1\n3 4\n1 4 1\n3 2\n0 1 3\n3 9\n0 59049 810\n","output":"YES\nYES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAddingPowers"}}}

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
    collections::{iter_ext::IterExt, multiset::MultiSet},
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
    let (n, k) = input.read::<(usize, i64)>();

    let tl = input.read_vec::<i64>(n);

    let mut needs = MultiSet::default();

    for &t in tl.iter() {
        needs.insert(t);
    }

    let big = needs.iter().copied().max().unwrap();

    let mut p = 0;
    while k.pow(p) < big {
        p += 1;
    }

    for q in (0..=p).rev() {
        let x = k.pow(q);
        if let Some(s) = needs.remove_last() {
            if x <= s {
                needs.insert(s - x);
            } else {
                needs.insert(s);
            }
        }
    }

    dbg!(needs);

    let ans = needs.iter().all(|&x| x == 0);
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
