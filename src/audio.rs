use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::thread;

use rodio::{Decoder, OutputStream, Sink};

/// Non-blocking audio player backed by a dedicated `rodio` thread.
pub struct AudioPlayer {
    tx: Sender<PathBuf>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<PathBuf>();

        thread::spawn(move || {
            let Ok((_stream, handle)) = OutputStream::try_default() else {
                eprintln!("[audio] Failed to open default audio output stream");
                return;
            };
            let mut active_sink: Option<Sink> = None;

            while let Ok(path) = rx.recv() {
                // Drain queued clicks so rapid clicking plays only the latest sound.
                let mut latest = path;
                while let Ok(newer) = rx.try_recv() {
                    latest = newer;
                }

                if let Some(sink) = active_sink.take() {
                    sink.stop();
                }

                match File::open(&latest) {
                    Ok(file) => match Decoder::new(BufReader::new(file)) {
                        Ok(source) => match Sink::try_new(&handle) {
                            Ok(sink) => {
                                sink.append(source);
                                active_sink = Some(sink);
                            }
                            Err(e) => eprintln!("[audio] Sink::try_new error: {e}"),
                        },
                        Err(e) => eprintln!("[audio] Decoder::new error for {}: {e}", latest.display()),
                    },
                    Err(e) => eprintln!("[audio] File::open error for {}: {e}", latest.display()),
                }
            }
        });

        Self { tx }
    }

    pub fn play(&self, relative_path: &str) {
        let path = resolve_sound_path(relative_path);
        if path.exists() {
            let _ = self.tx.send(path);
        } else {
            eprintln!("[audio] Sound file not found: {}", path.display());
        }
    }
}

fn resolve_sound_path(relative_path: &str) -> PathBuf {
    let direct = Path::new(relative_path);
    if direct.exists() {
        return direct.to_path_buf();
    }
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe.parent();
        while let Some(d) = dir {
            let candidate = d.join(relative_path);
            if candidate.exists() {
                return candidate;
            }
            dir = d.parent();
        }
    }
    let manifest_candidate = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    if manifest_candidate.exists() {
        return manifest_candidate;
    }
    direct.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_phoneme_and_word_wav() {
        for p in [
            "phonetic_sounds/phonemes/v_y.wav",
            "phonetic_sounds/words/en_see.wav",
        ] {
            let path = resolve_sound_path(p);
            assert!(path.exists(), "missing {}", path.display());
            let file = File::open(&path).unwrap();
            let decoder = Decoder::new(BufReader::new(file));
            assert!(decoder.is_ok(), "failed to decode {}: {:?}", path.display(), decoder.err());
        }
    }
}
