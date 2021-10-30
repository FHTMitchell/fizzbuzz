mod naive;
mod safe_opt;
mod unsafe_opt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(flag) = args.get(1) {
        match flag.as_str() {
            "naive" => naive::naive(),
            "cnaive" => naive::clike_naive(),
            "safe" => safe_opt::run(),
            "unsafe" => unsafe_opt::run(),
            _ => panic!("Unrecognized arg: {}", flag),
        }
    } else {
        panic!("Missing argument")
    }
}
