use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};

fn main() {
    let styles = [
        ("Rough bar:", "█  ", "red"),
        ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
        ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
        ("Fade in:  ", "█▓▒░  ", "blue"),
        ("Blocky:   ", "█▛▌▖  ", "magenta"),
    ];
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.dim.bold} checksum: {wide_msg}"),
    );

    thread::sleep(Duration::new(5, 0));
    pb.set_message("44427359e8f382550a6165ab701781c2ea9418f5d10b3d847af3072e097add29");
    pb.finish();

    let m = MultiProgress::new();

    for s in styles.iter() {
        let pb = m.add(ProgressBar::new(512));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", s.2))
                .progress_chars(s.1),
        );
        pb.set_prefix(s.0);
        let wait = Duration::from_millis(thread_rng().gen_range(10, 30));
        thread::spawn(move || {
            for i in 0..512 {
                pb.inc(1);
                pb.set_message(&format!("{:3}%", 100 * i / 512));
                thread::sleep(wait);
            }
            pb.finish_with_message("100%");
        });
    }

    m.join().unwrap();
}
