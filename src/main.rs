use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::Write;
fn main() {
    kenshiro();
}

fn kenshiro(){
    for i in 0..10 {
        let attack = "た".repeat(i);
        print!("\rあ{attack}");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    print!("\rお前はもう死んでいる....");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(1000));

}