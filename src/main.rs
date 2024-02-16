use std::thread;
use std::time::Duration;
use std::io::Write;
use std::io::stdout;


fn main() {

    for i in 1..10 {
        thread::spawn(move || {
            for j in 1..10 {
                println!("hi number {} from the {} spawned thread!", j, i);
                let _ = stdout().flush();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    thread::sleep(Duration::from_secs(15));
}