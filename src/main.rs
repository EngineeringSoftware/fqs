use fqs::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(msg) = args {
        println!("Error {}", msg);
        return;
    }

    let args = args.unwrap();
    match fqs::query(args) {
        Ok(table) => table.show(),
        Err(err) => panic!("{err}"),
    }
}
