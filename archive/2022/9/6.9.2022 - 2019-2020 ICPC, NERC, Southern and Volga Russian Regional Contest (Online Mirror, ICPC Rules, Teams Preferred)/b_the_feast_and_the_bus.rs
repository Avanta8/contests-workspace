//{"name":"B. The Feast and the Bus","group":"Codeforces - 2019-2020 ICPC, NERC, Southern and Volga Russian Regional Contest (Online Mirror, ICPC Rules, Teams Preferred)","url":"https://codeforces.com/contest/1250/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\n3 1 2 3 2 3\n","output":"6\n"},{"input":"10 1\n1 1 1 1 1 1 1 1 1 1\n","output":"10\n"},{"input":"12 4\n1 2 3 1 2 3 4 1 2 1 2 1\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheFeastAndTheBus"}}}

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

use std::{
    cmp::{max, min},
    collections::BTreeMap,
};

use algo_lib::{
    collections::{iter_ext::IterExt, multiset::MultiSet, vec_ext::IncDec},
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
    let (n, k) = input.read::<(usize, usize)>();
    let arr = input.read_vec::<usize>(n).dec_by_one();

    let mut counts = vec![0; k];
    for &x in arr.iter() {
        counts[x] += 1;
    }

    let big = counts.iter().copied().max().unwrap();

    let mut set = MultiSet::new();
    for &c in counts.iter() {
        set.insert(c);
    }

    let calc = |size| {
        let mut count = 0;
        let mut filled = 0;
        let mut set = set.clone();
        while let Some(last) = set.remove_last() {
            let limit = size - last;
            if let Some(pair) = set.remove_range_last(..=limit) {
                filled = max(filled, last + pair);
            } else {
                filled = max(filled, last);
            }
            count += 1;
        }
        (filled, count)
    };

    let mut best = i32::MAX;
    let mut size = big + big;
    while size >= big {
        let (filled, count) = calc(size);
        best = min(best, filled * count);
        size = filled - 1;
    }

    out_line!(best);
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
