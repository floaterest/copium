// 2023-04-30 https://atcoder.jp/contests/abc300/tasks/abc300_d
#![allow(unused_imports, unused_macros, unused_variables, unused_mut, dead_code)]

use std::collections::*;
use std::io::{Read, Write};

use reader::Reader;
use writer::Writer;

// https://github.com/statiolake/proconio-rs/blob/master/proconio/src/source/line.rs
mod reader {
    use std::any::type_name;
    use std::io::{BufRead, BufReader, Read};
    use std::iter::{FromIterator, Peekable};
    use std::mem::transmute;
    use std::str::{FromStr, SplitWhitespace};

    #[derive(Debug)]
    pub struct Reader<R: Read> {
        pub reader: BufReader<R>,
        tokens: Peekable<SplitWhitespace<'static>>,
        line: Box<str>,
    }

    macro_rules! impl_reader {
        ($(($func:ident, $type:ty)),+) => (
            $(pub fn $func(&mut self) -> $type { self.next::<$type>() })+
        )
    }

    impl<R: Read> Reader<R> {
        pub fn new(r: R) -> Reader<R> {
            Reader {
                tokens: "".split_whitespace().peekable(),
                line: "".to_string().into_boxed_str(),
                reader: BufReader::new(r),
            }
        }

        /// read line if needed
        fn prepare(&mut self) {
            while self.tokens.peek().is_none() {
                let mut line = String::new();
                if self.reader.read_line(&mut line).expect("Failed to read line!") == 0 {
                    return; /* EOF */
                }
                self.line = line.into_boxed_str();
                self.tokens = unsafe { transmute::<_, &'static str>(&*self.line) }
                    .split_whitespace()
                    .peekable();
            }
        }

        /// get next token
        pub fn next<T: FromStr>(&mut self) -> T {
            self.prepare();
            match self.tokens.next() {
                Some(token) => match token.parse() {
                    Ok(token) => token,
                    Err(..) => panic!("Cannot parse {} as {}", token, type_name::<T>()),
                },
                None => panic!("Token is empty while trying to read {}", type_name::<T>()),
            }
        }

        impl_reader!((i, i64), (u, usize), (c, char), (s, String), (f, f64));
        pub fn u1(&mut self) -> usize {
            self.u().checked_sub(1).expect("Attempted read 0 as usize1")
        }
        pub fn chars<T: FromIterator<char>>(&mut self) -> T {
            self.s().chars().collect()
        }
    }

    #[macro_export]
    macro_rules! r {
        ($re:expr, ($func:ident, $type:ty)) => ($re.$func::<$type>());
        ($re:expr, $func:ident) => ($re.$func());
        ($re:expr, [$token:tt; $len:expr]) => (std::iter::repeat_with(|| r!($re, $token)).take($len));
        ($re:expr, $($item:tt),+) => (($(r!($re, $item)),+));
    }
    macro_rules! impl_collection {
        ($(($macro:ident, $type:ty)),+) => ($(#[macro_export] macro_rules! $macro {
            ($re:expr, [chars; $len:expr]) => (r!($re, [(chars, $type); $len]).collect::<$type>());
            ($re:expr, [$func:ident; $len:expr]) => (r!($re, [$func; $len]).collect::<$type>());
            ($re:expr, [$item:tt; $len:expr]) => (std::iter::repeat_with(|| $macro!($re, $item)).take($len).collect::<$type>());
        })+)
    }
    impl_collection!((rv, Vec<_>), (rs, HashSet<_>), (rd, VecDeque<_>), (rh, BinaryHeap<_>));
}

mod writer {
    use std::fmt::Display;
    use std::io::{BufWriter, Write};

    pub trait Writable<T> {
        /// write ' ' sep, no end
        fn w<W: Write>(self, wr: &mut Writer<W>);
        /// write ' ' sep, '\n' end
        fn n<W: Write>(self, wr: &mut Writer<W>);
        /// write ' ' sep, ' ' end
        fn s<W: Write>(self, wr: &mut Writer<W>);
    }

    // procedural macros go brrrr
    macro_rules! impl_writer {
        (Atom, $(($func:ident, $macr:ident, $fmt:literal)),+) => {
            $(fn $func<W: Write>(self, wr: &mut Writer<W>) {
                $macr!(wr.writer, $fmt, self).unwrap();
            })+
        };
        (Iter, $($func:ident),+) => {
            $(fn $func<W: Write>(self, wr: &mut Writer<W>) {
                let mut last: Option<T> = None;
                for item in self {
                    if let Some(last) = last.take() {
                        last.s(wr);
                    }
                    last = Some(item);
                }
                if let Some(last) = last.take() {
                    last.$func(wr);
                }
            })+
        };
        (Writer, $($func:ident),+) => {
            $(pub fn $func<M, T: Writable<M>>(&mut self, item: T) {
                item.$func(self);
            })+
        };
    }

    pub struct Atom;
    impl<T: Display> Writable<Atom> for T {
        impl_writer!(Atom, (w, write, "{}"), (n, writeln, "{}"), (s, write, "{} "));
    }

    pub struct Iter;
    impl<T: Display, I: Iterator<Item = T>> Writable<Iter> for I {
        impl_writer!(Iter, w, n, s);
    }

    pub struct Writer<W: Write> {
        pub writer: BufWriter<W>,
    }

    impl<W: Write> Writer<W> {
        pub fn new(w: W) -> Self {
            Self { writer: BufWriter::new(w) }
        }

        impl_writer!(Writer, w, n, s);

        // write "Yes\n" or "No\n"
        pub fn y(&mut self, b: bool) {
            self.n(if b { "Yes" } else { "No" });
        }

        // writeln then flush
        pub fn f<M, T: Writable<M>>(&mut self, item: T) {
            self.n(item);
            self.writer.flush().expect("Failed to flush");
        }
    }

    /// write ' ' sep, end with '\n'
    #[macro_export]
    macro_rules! w {
        ($wr:expr, $item:expr) => ($wr.n($item));
        ($wr:expr, $first:expr, $($item:expr),+) => {
            $wr.s($first);
            w!(($wr), $($item),+);
        };
    }
}

#[cfg(debug_assertions)]
fn main() {
    use std::fs::File;
    solve(
        Reader::new(File::open("input.txt").unwrap()),
        Writer::new(std::io::stdout()),
        // Writer::new(File::create("output.txt").unwrap()),
    )
}

#[cfg(not(debug_assertions))]
fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    solve(Reader::new(stdin.lock()), Writer::new(stdout.lock()));
}

// const D8: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

fn sieve(n: usize) -> Vec<usize> {
    let mut arr = vec![true; n + 1];
    (0..=1).for_each(|i| arr[i] = false);
    for i in (2..).take_while(|i| i * i <= n) {
        if !arr[i] {
            continue;
        }
        (0..).map(|j| i * (i + j)).take_while(|&j| j <= n).for_each(|j| arr[j] = false);
    }
    arr.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i).collect()
}

fn solve<R: Read, W: Write>(mut re: Reader<R>, mut wr: Writer<W>) {
    let n = re.u();
    let max = (1e12 / (2 * 2 * 3) as f64).sqrt() as usize;
    let prod = |a, b, c| a * a * b * c * c;
    let good = |a, b, c| prod(a, b, c) <= n;
    let arr = sieve(max);
    let mut ans = 0;
    for (i, &a) in arr.iter().enumerate().take_while(|(_, &a)| good(a, a, a)) {
        for (j, &b) in arr.iter().enumerate().skip(i + 1).take_while(|(_, &b)| good(a, b, b)) {
            ans += match &arr[j + 1..]
                .binary_search(&((n as f64 / prod(a, b, 1) as f64).sqrt() as usize))
            {
                Ok(idx) => idx + 1,
                Err(idx) => *idx,
            };
        }
    }
    wr.n(ans);
}
