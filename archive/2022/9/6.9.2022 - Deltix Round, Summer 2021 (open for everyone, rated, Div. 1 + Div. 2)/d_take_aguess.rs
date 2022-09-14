//{"name":"D. Take a Guess","group":"Codeforces - Deltix Round, Summer 2021 (open for everyone, rated, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1556/problem/D","interactive":true,"timeLimit":2000,"tests":[{"input":"7 6\n\n2\n\n7\n","output":"and 2 5\n\nor 5 6\n\nfinish 5\n"}],"testType":"multiEof","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTakeAGuess"}}}

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

use std::fmt::Display;

use algo_lib::{
    collections::{bit_set::BitSet, iter_ext::IterExt},
    io::{
        input::Input,
        output::output,
        task_io_settings::{TaskIoSettings, TaskIoType},
        task_runner::run_task,
    },
    numbers::num_traits::bit_ops::BitOps,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

/*
 a   b   c   d   e   f   g
001 110 100 010 011 101 010

a|b = 111
a&b = 000
b|c = 110
b&c = 100
c|d = 110
c&d = 000
d|e = 011
d&e = 010
e|f = 111
e&f = 001
f|g = 111
f&g = 000

 a   b   c   d   e   f   g
001 110 100 010 011 101 010
*/

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Bit {
    On,
    Off,
    #[default]
    Unknown,
}

impl Bit {
    /// Returns `true` if the bit is [`On`].
    ///
    /// [`On`]: Bit::On
    #[must_use]
    fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }

    /// Returns `true` if the bit is [`Off`].
    ///
    /// [`Off`]: Bit::Off
    #[must_use]
    fn is_off(&self) -> bool {
        matches!(self, Self::Off)
    }

    /// Returns `true` if the bit is [`Unknown`].
    ///
    /// [`Unknown`]: Bit::Unknown
    #[must_use]
    fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Number {
    bits: [Bit; 32],
}

impl Number {
    fn as_i32(&self) -> i32 {
        let mut n = 0;
        for b in self.bits {
            n <<= 1;
            if b.is_on() {
                n |= 1;
            }
        }
        n
    }
}

impl From<i32> for Number {
    fn from(mut n: i32) -> Self {
        let mut v = [Bit::Unknown; 32];
        for i in (0..32).rev() {
            v[i] = if n & 1 == 1 { Bit::On } else { Bit::Off };
            n >>= 1;
        }
        Self { bits: v }
    }
}

fn ask(input: &mut Input, q: impl Display, i: usize, j: usize) -> i32 {
    out_line!(format!("{} {} {}", q, i + 1, j + 1));
    let res = input.read::<i32>();
    if res == -1 {
        std::process::exit(69);
    }
    res
}

fn finish(x: i32) {
    out_line!(format!("finish {x}"));
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    let ors = (0..n)
        .zip(1..n + 1)
        .map(|(i, j)| (i % n, j % n))
        .map(|(i, j)| Number::from(ask(input, "or", i, j)))
        .collect_vec();
    let ands = (0..n)
        .zip(1..n + 1)
        .map(|(i, j)| (i % n, j % n))
        .map(|(i, j)| Number::from(ask(input, "and", i, j)))
        .collect_vec();

    let mut nums = vec![Number::default(); n];

    for i in 0..n {
        let j = (i + 1) % n;
        let or = &ors[i];
        let and = &ands[i];
        for (idx, b) in or.bits.into_iter().enumerate() {
            if b.is_off() {
                nums[i].bits[idx] = Bit::Off;
                nums[j].bits[idx] = Bit::Off;
            }
        }
        for (idx, b) in and.bits.into_iter().enumerate() {
            if b.is_on() {
                nums[i].bits[idx] = Bit::On;
                nums[j].bits[idx] = Bit::On;
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for i in 0..n {
            let j = (i + 1) % n;
            for idx in 0..32 {
                if nums[i].bits[idx].is_unknown() {
                    if nums[j].bits[idx].is_off() && ors[i].bits[idx].is_on() {
                        nums[i].bits[idx] = Bit::On;
                        changed = true;
                    } else if nums[j].bits[idx].is_on() && ands[i].bits[idx].is_off() {
                        nums[i].bits[idx] = Bit::Off;
                        changed = true;
                    }
                }
                if nums[j].bits[idx].is_unknown() {
                    if nums[i].bits[idx].is_off() && ors[i].bits[idx].is_on() {
                        nums[j].bits[idx] = Bit::On;
                        changed = true;
                    } else if nums[i].bits[idx].is_on() && ands[i].bits[idx].is_off() {
                        nums[j].bits[idx] = Bit::Off;
                        changed = true;
                    }
                }
            }
        }
    }

    let mut res = nums.into_iter().map(|x| x.as_i32()).collect_vec();
    res.sort_unstable();

    finish(res[k - 1]);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: true,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    submit();
}
//END MAIN
