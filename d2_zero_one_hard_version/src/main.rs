//{"name":"D2. Zero-One (Hard Version)","group":"Codeforces - Codeforces Round #821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/D2","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n5 8 9\n01001\n00101\n6 2 11\n000001\n100000\n5 7 2\n01000\n11011\n7 8 3\n0111001\n0100001\n6 3 4\n010001\n101000\n5 10 1\n01100\n01100\n","output":"8\n10\n-1\n6\n7\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2ZeroOneHardVersion"}}}

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

use std::cmp::{max, min};

use algo_lib::{
    collections::{iter_ext::IterExt, vec_ext::ConsecutiveIter},
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
    let (n, x, y) = input.read::<(usize, i64, i64)>();

    let A = input.read_string().bytes().map(|b| b - b'0').collect_vec();
    let B = input.read_string().bytes().map(|b| b - b'0').collect_vec();

    // dbg!(n, A, B);

    let mut idxs = vec![];
    for (i, (&a, &b)) in A.iter().zip(B.iter()).enumerate() {
        if a != b {
            idxs.push(i);
        }
    }
    idxs.sort();

    // dbg!(need);
    dbg!(idxs);
    let need = idxs.len() as i64;
    if need % 2 == 1 {
        out_line!(-1);
        return;
    }

    if y <= x {
        if need == 2 {
            let mut a = idxs[0];
            let mut b = idxs[1];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            // dbg!(a, b);

            if a + 1 == b {
                let diff = max(a, n - 1 - b);
                // dbg!(diff);
                if diff >= 2 {
                    out_line!(min(2 * y, x));
                } else {
                    out_line!(x);
                }
            } else {
                out_line!(y);
            }
        } else {
            out_line!(need / 2 * y);
        }
    } else {
        let mut runs = vec![];
        let mut current = vec![];
        for &x in idxs.iter() {
            if current.is_empty() || x == current.last().unwrap() + 1 {
                current.push(x);
            } else {
                runs.push(current);
                current = vec![x];
            }
        }
        if !current.is_empty() {
            runs.push(current);
        }

        // dbg!(runs);

        let mut total = 0;

        let mut new_runs = vec![];
        for run in runs {
            let len = run.len() as i64;
            if len % 2 == 0 {
                total += len / 2 * x;
            } else {
                new_runs.push(run);
            }
        }
        runs = new_runs;

        let mut prev = None;

        for (i, run) in runs.iter().cloned().enumerate() {
            let len = run.len() as i64;
            total += len / 2 * x;
            let last = *run.last().unwrap();
            let first = *run.first().unwrap();
            if run.len() > 1 {
                if let Some(prev) = prev {
                    let diff = first - prev;

                    // let ava = { max(last, n - 1 - first) >= 2 };

                    // if ava {
                    //     let
                    // } else {

                    // }
                    total += min(diff as i64 * x, y);
                }
                prev = Some(last);
            } else {
                if let Some(prevv) = prev {
                    let diff = first - prevv;
                    if i + 1 < runs.len() {
                        let next_diff = *runs[i + 1].first().unwrap();
                        if next_diff < diff {
                            prev = Some(last);
                            continue;
                        }
                    }
                    total += min(diff as i64 * x, y);
                    prev = None;
                } else {
                    prev = Some(last);
                }
            }
        }

        out_line!(total);

        // let left = run.first().unwrap();
        // let right = run.last().unwrap();

        // let left_diff = n - 1 - left;
        // let right_diff = right;
    }
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
