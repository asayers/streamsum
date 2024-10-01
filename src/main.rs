use blake3::Hasher;
use bpaf::{Bpaf, Parser};
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, StdoutLock, Write},
    path::PathBuf,
};

#[derive(Bpaf)]
struct Opts {
    /// A file to read data from.  Reads from stdin if not specified
    #[bpaf(positional("PATH"))]
    file: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    match main2() {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e),
    }
}

fn main2() -> std::io::Result<()> {
    let opts = opts().run();
    let mut hasher = blake3::Hasher::new();
    let mut rdr: Box<dyn BufRead> = match opts.file {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(std::io::stdin().lock()),
    };
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut len = 0;
    let mut chunk = 1024;
    loop {
        let target = chunk - hasher.count() as usize;
        let buf = rdr.fill_buf()?;
        if buf.len() == 0 {
            write_char(&mut stdout, &mut hasher)?;
            writeln!(stdout, " (read {} bytes)", len)?;
            return Ok(());
        } else if buf.len() >= target {
            hasher.update(&buf[..target]);
            rdr.consume(target);
            len += target;
            write_char(&mut stdout, &mut hasher)?;
            chunk = chunk.saturating_add(chunk / 4);
        } else {
            hasher.update(buf);
            let n = buf.len();
            rdr.consume(n);
            len += n;
        }
    }
}

fn write_char(stdout: &mut StdoutLock, hasher: &mut Hasher) -> std::io::Result<()> {
    let hash = hasher.finalize();
    hasher.reset();
    let table = b"0123456789abcdef";
    let b = hash.as_bytes()[0];
    let c = table[(b >> 4) as usize];
    stdout.write_all(&[c])?;
    stdout.flush()?;
    Ok(())
}
