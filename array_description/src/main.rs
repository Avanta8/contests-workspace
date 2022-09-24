//{"name":"Array Description","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1746","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5\n2 0 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ArrayDescription"}}}

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

type Cache = FxHashMap<(usize, usize, usize), usize>;

static mut CACHE: Option<Cache> = None;

fn cache() -> &'static mut Cache {
    unsafe {
        if let Some(c) = CACHE.as_mut() {
            c
        } else {
            CACHE = Some(FxHashMap::default());
            cache()
        }
    }
}

const MOD: usize = 1_000_000_007;

fn get(a: usize, b: usize, space: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    if space == 0 {
        return (((a as isize) - (b as isize)).abs() <= 1) as usize;
    }
    if let Some(&ans) = cache().get(&(a, b, space)) {
        return ans;
    }

    let mut total = 0;
    for (na, nb) in [
        (a, b),
        (a, b),
        (a + 1, b),
        (a - 1, b),
        (a, b + 1),
        (a, b - 1),
    ] {
        total += get(na, nb, space - 1);
    }
    let ans = total / 2 % MOD;
    cache().insert((a, b, space), ans);
    ans
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read::<(usize, usize)>();

    let arr = input.read_vec::<usize>(n);

    let mut zeros = vec![];
    let mut last = None::<(Option<usize>, usize)>;
    for (i, &x) in arr.iter().enumerate() {
        if let Some(l) = last {
            if x != 0 {
                zeros.push((l.0, Some(x), i - l.1));
                last = None;
            }
        } else {
            if x == 0 {
                last = Some((if i > 0 { Some(arr[i - 1]) } else { None }, i));
            }
        }
    }
    if let Some(l) = last {
        zeros.push((l.0, None, arr.len() - l.1));
    }

    let mut total = 1;
    for (left, right, space) in zeros {
        let left_range = if let Some(l) = left { l..=l } else { 1..=m };
        let right_range = if let Some(r) = right { r..=r } else { 1..=m };
        for l in left_range {
            for r in right_range.clone() {
                total = (total * get(l, r, space)) % MOD;
            }
        }
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
