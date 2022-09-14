//{"name":"D. Christmas Trees","group":"Codeforces - Codeforces Round #611 (Div. 3)","url":"https://codeforces.com/contest/1283/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2 6\n1 5\n","output":"8\n-1 2 6 4 0 3\n"},{"input":"3 5\n0 3 1\n","output":"7\n5 -2 4 -1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DChristmasTrees"}}}

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

use std::collections::VecDeque;

use algo_lib::{
    collections::{fxhash::FxHashSet, iter_ext::IterExt},
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
    let (n, m) = input.read::<(usize, usize)>();
    let trees = input.read_vec::<i64>(n);

    let mut people = vec![];

    let mut filled = FxHashSet::default();

    let mut bag = VecDeque::default();
    for &p in trees.iter() {
        filled.insert(p);
        bag.push_back((p, 1, 0));
        bag.push_back((p, -1, 0));
    }

    let mut total = 0i64;
    while people.len() < m {
        let (pos, dir, dist) = bag.pop_back().unwrap();
        let new_pos = pos + dir;

        if filled.contains(&new_pos) {
            continue;
        }

        total += dist + 1;
        bag.push_front((new_pos, dir, dist + 1));
        people.push(new_pos);
        filled.insert(new_pos);
    }

    out_line!(total);
    out_line!(people);
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
