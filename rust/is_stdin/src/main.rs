fn main() {
    if is_readable_stdin() {
        println!("stdin readable")
    } else {
        println!("no stdin")
    }
}

pub fn is_readable_stdin() -> bool {
    fn imp() -> bool {
        use same_file::Handle;
        use std::os::unix::fs::FileTypeExt;

        let ft = match Handle::stdin().and_then(|h| h.as_file().metadata()) {
            Err(_) => return false,
            Ok(md) => md.file_type(),
        };
        ft.is_file() || ft.is_fifo() || ft.is_socket()
    }

    !is_tty_stdin() && imp()
}

/// Returns true if and only if stdin is believed to be connectted to a tty
/// or a console.
pub fn is_tty_stdin() -> bool {
    atty::is(atty::Stream::Stdin)
}
