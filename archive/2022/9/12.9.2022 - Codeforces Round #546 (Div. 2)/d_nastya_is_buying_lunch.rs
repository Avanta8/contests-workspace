//{"name":"D. Nastya Is Buying Lunch","group":"Codeforces - Codeforces Round #546 (Div. 2)","url":"https://codeforces.com/contest/1136/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n1 2\n1 2\n","output":"1\n"},{"input":"3 3\n3 1 2\n1 2\n3 1\n3 2\n","output":"2\n"},{"input":"5 2\n3 1 5 4 2\n5 2\n5 4\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNastyaIsBuyingLunch"}}}

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

use std::collections::BTreeMap;

use algo_lib::{
    collections::{
        fxhash::FxHashSet,
        iter_ext::IterExt,
        vec_ext::{ConsecutiveIter, IncDec},
    },
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
    let arr = input.read_vec::<usize>(n).dec_by_one();
    let pairs = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let set_pairs = pairs.iter().copied().collect::<FxHashSet<_>>();

    let mut rem = arr.iter().copied().enumerate().collect::<BTreeMap<_, _>>();

    let mut total = 0;
    'outer: for (i, &x) in arr.iter().enumerate().rev().skip(1) {
        for (_, &s) in rem.range(i + 1..) {
            if !set_pairs.contains(&(x, s)) {
                continue 'outer;
            }
        }
        total += 1;
        rem.remove(&i);
    }

    out_line!(total);
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
