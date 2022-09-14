//{"name":"E2. Three Blocks Palindrome (hard version)","group":"Codeforces - Codeforces Round #634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/E2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n8\n1 1 2 2 3 2 1 1\n3\n1 3 3\n4\n1 10 10 1\n1\n26\n2\n2 1\n3\n1 1 1\n","output":"7\n2\n4\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2ThreeBlocksPalindromeHardVersion"}}}

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

use std::cmp::max;

use algo_lib::{
    collections::{prefix::Prefix, vec_ext::IncDec},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const N: usize = 200;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let arr = input.read_vec::<u32>(n).dec_by_one();

    let mut prefixes = vec![];

    for c in 0..N {
        prefixes.push(Prefix::new(
            arr.iter().map(|&x| if x == c as u32 { 1 } else { 0 }),
            n,
        ));
    }

    let mut positions = vec![vec![]; N];
    for (i, &c) in arr.iter().enumerate() {
        positions[c as usize].push(i);
    }

    let get = |c: usize, start: usize, end: usize| -> u32 { prefixes[c].get(start..end) };

    let mut best = 0;
    for pos in positions.into_iter() {
        if pos.len() == 1 {
            best = max(best, 1);
            continue;
        }

        for (i, (&front, &back)) in pos
            .iter()
            .zip(pos.iter().rev())
            .take(pos.len() / 2)
            .enumerate()
        {
            for x in 0..N {
                best = max(best, 2 + i as u32 * 2 + get(x, front + 1, back));
            }
        }
    }

    out_line!(best);
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
