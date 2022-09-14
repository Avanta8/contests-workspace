//{"name":"D. Not Adding","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 20 1 25 30\n","output":"3\n"},{"input":"3\n6 10 15\n","output":"4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNotAdding"}}}

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
    collections::fxhash::FxHashSet,
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
    let n = input.read();
    let vec = input.read_usize_vec(n);

    let mx = *vec.iter().max().unwrap();

    let mut table = vec![false; mx + 1];
    for x in vec {
        table[x] = true;
    }

    let mut count = 0;
    for x in 1..mx {
        if table[x] {
            continue;
        }
        if (x..=mx)
            .step_by(x)
            .filter(|&p| table[p])
            .reduce(gcd)
            .unwrap_or(0)
            == x
        {
            count += 1;
        }
    }

    out_line!(count);
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
