//{"name":"D. Cyclic Rotation","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 3 2\n1 3 3 2 2\n5\n1 2 4 2 1\n4 2 2 1 1\n5\n2 4 5 5 2\n2 2 4 5 5\n3\n1 2 3\n1 2 3\n3\n1 1 2\n2 1 1\n","output":"YES\nYES\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCyclicRotation"}}}

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
    collections::{fxhash::FxHashMap, iter_ext::IterExt},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn groups(arr: &[i64]) -> Vec<(i64, usize)> {
    let mut count = 1;
    let mut last = arr[0];
    let mut ans = vec![];
    for &x in arr[1..].iter() {
        if x == last {
            count += 1;
        } else {
            ans.push((last, count));
            count = 1;
        }
        last = x;
    }
    ans.push((last, count));
    ans
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let a_arr = input.read_vec::<i64>(n);
    let b_arr = input.read_vec::<i64>(n);

    // let mut a_it = a_arr.iter().copied();
    let a_groups = groups(&a_arr);
    let mut a_it = a_groups.iter().copied().peekable();

    let mut sent = FxHashMap::default();
    'outer: for &b in b_arr.iter() {
        'inner: while let Some((a, count)) = a_it.peek_mut() {
            if let Some(add) = sent.get_mut(a) {
                *count += *add;
                *add = 0;
            }

            if *count == 0 {
                a_it.next();
                continue 'inner;
            }

            if *a == b {
                *count -= 1;
                continue 'outer;
            } else {
                *sent.entry(*a).or_insert(0) += *count;
                a_it.next();
            }
        }
    }

    dbg!(sent);

    let ans = sent.values().all(|&v| v == 0);
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
