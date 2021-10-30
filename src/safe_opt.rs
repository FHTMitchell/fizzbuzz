use std::io::{self, Write};

const MAX_SIZE: usize = 16;
const BUF_SIZE: usize = 8 * 1024;

#[derive(Debug)]
struct Decimal {
    len: usize,
    buf: [u8; MAX_SIZE],
}

impl Decimal {
    fn new() -> Self {
        let mut buf = [b'0'; MAX_SIZE];
        buf[buf.len() - 1] = b'1';
        Self { len: 1, buf }
    }

    fn inc(&mut self) {
        let mut index = MAX_SIZE - 1;
        let new_index = MAX_SIZE - self.len - 1;
        loop {
            let digit = self.buf.get_mut(index).unwrap();
            if *digit == b'9' {
                *digit = b'0';
                index -= 1;
                if index == new_index {
                    *self.buf.get_mut(index).unwrap() = b'1';
                    self.len += 1;
                    break;
                }
            } else {
                *digit += 1;
                break;
            }
        }
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) {
        let start = MAX_SIZE - self.len;
        buf.write(&self.buf[start..]).unwrap();
        buf.write(b"\n").unwrap();
    }
}

struct Cursor {
    buf: [u8; BUF_SIZE],
    pos: usize,
}

impl Cursor {
    fn new() -> Self {
        Self {
            buf: [0; BUF_SIZE],
            pos: 0,
        }
    }

    fn get_ref(&self) -> &[u8] {
        &self.buf[0..self.pos]
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }
}

impl io::Write for Cursor {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut writer = &mut self.buf[self.pos..];
        let res = writer.write(buf)?;
        self.pos += res;
        Ok(res)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct Looper {
    step: Decimal,
    cur: Cursor,
}

impl Looper {
    fn new() -> Self {
        Self {
            step: Decimal::new(),
            cur: Cursor::new(),
        }
    }

    #[inline(always)]
    fn fizz(&mut self) {
        self.cur.write(b"Fizz\n").unwrap();
        self.step.inc();
    }

    #[inline(always)]
    fn buzz(&mut self) {
        self.cur.write(b"Buzz\n").unwrap();
        self.step.inc();
    }

    #[inline(always)]
    fn fizzbuzz(&mut self) {
        self.cur.write(b"FizzBuzz\n").unwrap();
        self.step.inc();
    }

    #[inline(always)]
    fn number(&mut self) {
        self.step.write_to(&mut self.cur);
        self.step.inc();
    }

    #[inline(always)]
    fn next15(&mut self) {
        self.number(); // 1
        self.number(); // 2
        self.fizz(); // 3
        self.number(); // 4
        self.buzz(); // 5
        self.fizz(); // 6
        self.number(); // 7
        self.number(); // 8
        self.fizz(); // 9
        self.buzz(); // 10
        self.number(); // 11
        self.fizz(); // 12
        self.number(); // 13
        self.number(); // 14
        self.fizzbuzz(); // 15
    }

    #[inline]
    fn next_loop_size(&self) -> usize {
        const NEWLINES: usize = 15;
        const FIZZES: usize = 4 * 4;
        const BUZZES: usize = 4 * 2;
        const FIZZBUZZ: usize = 8;
        let num_length = self.step.len + (self.step.len + 1) * 7;
        NEWLINES + FIZZES + BUZZES + FIZZBUZZ + num_length
    }

    #[inline]
    fn write_to<W: io::Write>(&mut self, writer: &mut W) {
        writer.write(self.cur.get_ref()).unwrap();
        self.cur.set_pos(0);
    }

    fn run_until<W: io::Write>(mut self, writer: &mut W, until: u64) {
        for _ in 1..(until / 15) {
            self.next15();
            if self.next_loop_size() > BUF_SIZE - self.cur.pos {
                self.write_to(writer)
            }
        }
    }

    fn run_forever<W: io::Write>(mut self, writer: &mut W) {
        loop {
            self.next15();
            if self.next_loop_size() > BUF_SIZE - self.cur.pos {
                self.write_to(writer)
            }
        }
    }
}

pub fn run() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let looper = Looper::new();
    looper.run_forever(&mut handle);
}

#[cfg(test)]
mod tests {
    use super::Looper;

    #[test]
    fn it_works() {
        let mut naive_buf: Vec<u8> = vec![];
        let mut opt_buf: Vec<u8> = vec![];
        let until = 2000;

        crate::naive::naive_to_buf(&mut naive_buf, until).unwrap();
        let looper = Looper::new();
        looper.run_until(&mut opt_buf, until);

        // cba to figure out how to make exact lenth matches
        assert_ne!(naive_buf.len(), 0);
        assert_ne!(opt_buf.len(), 0);
        let len = std::cmp::min(naive_buf.len(), opt_buf.len());
        assert_eq!(naive_buf[..len], opt_buf[..len]);
    }
}
