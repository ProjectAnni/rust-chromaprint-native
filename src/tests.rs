use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::{Path, PathBuf};
use *;

#[test]
fn get_version() {
    let version = version();
    assert_eq!(version, "1.4.4");
}

#[test]
fn test_fingerprint() {
    let mut context = Context::new();
    context.start(44100, 1);
    let file = load_audio_file(get_data_path("test_stereo_44100.raw"));
    context.feed(&file);
    context.finish();

    let fingerprint = assert_eq!(
        context.fingerprint(),
        "AQAAC0kkZUqYREkUnFAXHk8uuMZl6EfO4zu-4ABKFGESWIIMEQE"
    );
    assert_eq!(context.fingerprint_hash(), 3732003127);
    context.raw_fingerprint();
}

fn get_data_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("chromaprint/tests/data/{}", name))
}

fn load_audio_file<T: AsRef<Path>>(path: T) -> Vec<i16> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
        .chunks(2)
        .map(|chunk| from_ne_bytes([chunk[0], chunk[1]]))
        .collect()
}

pub fn from_ne_bytes(bytes: [u8; 2]) -> i16 {
    unsafe { mem::transmute(bytes) }
}
