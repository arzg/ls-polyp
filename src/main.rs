use polyp::{ProcessletMsg, Value};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    let dir = Path::new(&input);

    let listing: io::Result<Vec<_>> = fs::read_dir(dir)?
        .map(|entry| entry.map(|entry| Value::Path(entry.path())))
        .collect();
    let listing = listing?;

    let serialized = serde_json::to_string(&ProcessletMsg::NewOutput(Value::List(listing)))?;

    stdout.write_all(serialized.as_bytes())?;

    Ok(())
}
