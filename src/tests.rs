use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::{Path, PathBuf};

use wrapper::{hash_fingerprint, version, Context};

#[test]
fn get_version() {
    let version = version();
    assert_eq!(version, "1.4.4");
}

#[test]
fn test_fingerprint() -> Result<(), Box<dyn Error>> {
    let mut context = Context::new();

    let file = load_audio_file(get_data_path("test_stereo_44100.raw"))?;
    context.start(44100, 1)?;
    context.feed(&file)?;
    context.finish()?;

    assert_eq!(
        context.fingerprint()?,
        "AQAAC0kkZUqYREkUnFAXHk8uuMZl6EfO4zu-4ABKFGESWIIMEQE"
    );
    assert_eq!(context.fingerprint_hash()?, 3732003127);
    context.raw_fingerprint()?;

    Ok(())
}

#[test]
fn test_hash() -> Result<(), Box<dyn Error>> {
    let raw = [
        3740390231, 3739276119, 3730871573, 3743460629, 3743525173, 3744594229, 3727948087,
        1584920886, 1593302326, 1593295926, 1584907318,
    ]
        .to_vec();

    assert_eq!(hash_fingerprint(&raw)?, 3732003127);

    Ok(())
}

#[test]
fn test_clear_fingerprint() -> Result<(), Box<dyn Error>> {
    let mut context = Context::new();
    context.start(44100, 1)?;
    let first_file = load_audio_file(get_data_path("test_mono_44100.raw"))?;

    context.feed(&first_file)?;
    context.finish()?;
    context.clear_fingerprint()?;

    let second_file = load_audio_file(get_data_path("test_stereo_44100.raw"))?;
    context.feed(&second_file)?;
    context.finish()?;

    assert_eq!(
        context.fingerprint()?,
        "AQAAC0kkZUqYREkUnFAXHk8uuMZl6EfO4zu-4ABKFGESWIIMEQE"
    );

    Ok(())
}

fn get_data_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("chromaprint/tests/data/{}", name))
}

fn load_audio_file<T: AsRef<Path>>(path: T) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer
        .chunks(2)
        .map(|chunk| from_ne_bytes([chunk[0], chunk[1]]))
        .collect())
}

pub fn from_ne_bytes(bytes: [u8; 2]) -> i16 {
    unsafe { mem::transmute(bytes) }
}
