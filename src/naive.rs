use std::io;
use std::io::Write;

pub fn naive() {
    for i in 1u64.. {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

pub fn clike_naive() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut i: u64 = 1;
    loop {
        if i % 15 == 0 {
            handle.write(b"FizzBuzz\n").unwrap();
        } else if i % 3 == 0 {
            handle.write(b"Fizz\n").unwrap();
        } else if i % 5 == 0 {
            handle.write(b"Buzz\n").unwrap();
        } else {
            write!(handle, "{}\n", i).unwrap();
        }
        i += 1;
    }
}

pub fn naive_to_buf<W: Write>(w: &mut W, until: u64) -> io::Result<()> {
    for i in 1u64..until {
        if i % 15 == 0 {
            write!(w, "FizzBuzz\n")?;
        } else if i % 3 == 0 {
            write!(w, "Fizz\n")?;
        } else if i % 5 == 0 {
            write!(w, "Buzz\n")?;
        } else {
            write!(w, "{}\n", i)?;
        }
    }
    Ok(())
}
