use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    Command::new("unifdef")
            .arg("-t")
            .arg("-UPROFILE")
            .arg("src/bf.rs.pre")
            .output()
            .and_then(|o| File::create("src/bf.rs")
                      .and_then(|mut f| f.write_all(&o.stdout)))
            .unwrap();
}
