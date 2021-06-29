use std::path::PathBuf;
use libpefile::*;

#[test]
fn last_last() -> Result<(), std::io::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dll_file = PathBuf::from(format!("{}/samples/msaudite.dll", manifest_dir));
    let pefile = PEFile::new(dll_file)?;

    let msg = pefile.messages_iter()?.last().unwrap();
    assert_eq!("Highest System-Defined Audit Message Value.\r\n\u{0}", msg.text);
    Ok(())
}
