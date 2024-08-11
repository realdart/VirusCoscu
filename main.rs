use std::process::Command;
use std::thread;
use std::time::Duration;

fn open_link(link: &str) {
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(format!("start {}", link)).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(link).spawn()
    } else {
        Command::new("xdg-open").arg(link).spawn()
    };
}

fn random_number(max: usize) -> usize {
    (unsafe { std::ptr::read_volatile(&0u8) } as usize) % max
}

fn main() {
    let links = vec![
        "https://www.youtube.com/shorts/8cKpv7RnAYo",
        "https://www.youtube.com/watch?v=rekbE2Q8icM",
        "https://www.youtube.com/watch?v=EEoskpxZ37M",
        "https://www.youtube.com/watch?v=tYPHAbQjaew",
        "https://www.youtube.com/watch?v=GOqYM1eR6bg",
        "https://www.youtube.com/watch?v=BUO68XDyWOM",
        "https://www.youtube.com/watch?v=BbzGji4s6KE",
        "https://www.youtube.com/shorts/qaxmz2NJE7c",
        "https://www.youtube.com/shorts/YIzRx1Ek2XA",
        "https://www.youtube.com/shorts/8cKpv7RnAYo"
    ];

    loop {
        let link = &links[random_number(links.len())];
        open_link(link);

        let sleep_time = random_number(10) + 1;
        thread::sleep(Duration::from_secs(sleep_time as u64));
    }
}
