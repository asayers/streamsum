use blake3::Hasher;
use std::io::{BufRead, StdoutLock, Write};

fn main() {
    let mut hasher = blake3::Hasher::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut len = 0;
    let mut chunk = 1024;
    loop {
        let target = chunk - hasher.count() as usize;
        let buf = stdin.fill_buf().unwrap();
        if buf.len() == 0 {
            write_char(&mut stdout, &mut hasher);
            writeln!(stdout, " (read {} bytes)", len).unwrap();
            return;
        } else if buf.len() >= target {
            hasher.update(&buf[..target]);
            stdin.consume(target);
            len += target;
            write_char(&mut stdout, &mut hasher);
            chunk = chunk.saturating_add(chunk / 4);
        } else {
            hasher.update(buf);
            let n = buf.len();
            stdin.consume(n);
            len += n;
        }
    }
}

fn write_char(stdout: &mut StdoutLock, hasher: &mut Hasher) {
    let hash = hasher.finalize();
    hasher.reset();
    let table = b"0123456789abcdef";
    let b = hash.as_bytes()[0];
    let c = table[(b >> 4) as usize];
    stdout.write_all(&[c]).unwrap();
    stdout.flush().unwrap();
}
