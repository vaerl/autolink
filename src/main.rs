use structopt::StructOpt;

#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let arg = Arg::from_args();
    let file = std::fs::read_to_string(&arg.path).expect("Could not read file!");
    for line in file.lines() {
        if line.contains("##!!") {
            println!("{}", line);
        }
    }
}
