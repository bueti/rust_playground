use exitfailure::ExitFailure;
use failure::ResultExt;
use std::thread::sleep;
use std::time;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    let ten_millis = time::Duration::from_millis(100);
    for i in 0..100 {
        sleep(ten_millis);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    println!("Value for pattern: {}, path: {:?}", args.pattern, args.path);

    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{:?}`", path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())

}
*/

/*
fn buffered_read() {
    let f = File::open(&args.path)
    .expect("could not read firl");
    let reader = BufReader::new(f);

    for line in reader.lines(){
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
*/
