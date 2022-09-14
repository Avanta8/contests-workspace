//{"name":"D. Vasya And The Matrix","group":"Codeforces - Educational Codeforces Round 48 (Rated for Div. 2)","url":"https://codeforces.com/contest/1016/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\n2 9\n5 3 13\n","output":"YES\n3 4 5\n6 7 8\n"},{"input":"3 3\n1 7 6\n2 15 12\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVasyaAndTheMatrix"}}}

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

use std::ops::BitXor;

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
    let (n, m) = input.read::<(usize, usize)>();
    let row_clues = input.read_vec::<u32>(n);
    let col_clues = input.read_vec::<u32>(m);

    let row_xor = row_clues.iter().copied().reduce(u32::bitxor).unwrap();
    let col_xor = col_clues.iter().copied().reduce(u32::bitxor).unwrap();

    if row_xor != col_xor {
        out_line!(false);
        return;
    }

    let mut mat = Arr2d::new(n, m, 0);

    for (i, c) in row_clues.iter().copied().enumerate() {
        mat[(i, 0)] = c;
    }
    for (j, c) in col_clues.iter().copied().enumerate() {
        mat[(0, j)] = c;
    }

    mat[(0, 0)] = col_clues
        .iter()
        .skip(1)
        .copied()
        .fold(row_clues[0], u32::bitxor);

    out_line!(true);
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
