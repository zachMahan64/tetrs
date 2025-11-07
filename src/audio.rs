use include_dir::{Dir, include_dir};
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::{
    error::Error,
    io::Cursor,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_waveform(name: &str) -> Option<&'static [u8]> {
    ASSETS.get_file(name).map(|f| f.contents())
}

pub const THEME_FAST: &str = "tetrs_theme_fast.wav";
#[allow(dead_code)]
pub const THEME_NORMAL: &str = "tetrs_theme_normal.wav";

// commands sent to the audio thread.
pub enum AudioCommand {
    Play { name: String, loop_forever: bool },
    Pause,
    Resume,
    // Stop,
    TogglePause,
    // Shutdown,
}

// global Sender to call from anywhere after `init()`
// (onceLock only to init the thread once).
static AUDIO_SENDER: OnceLock<mpsc::Sender<AudioCommand>> = OnceLock::new();
static IS_PAUSED: AtomicBool = AtomicBool::new(false);

pub fn get_is_paused() -> bool {
    IS_PAUSED.load(Ordering::Relaxed)
}

// initialize the audio thread, subsequent calls return the same sender
pub fn init_audio_thread() -> mpsc::Sender<AudioCommand> {
    if let Some(s) = AUDIO_SENDER.get() {
        return s.clone();
    }
    // sender / reciever channel (multiple senders)
    let (tx, rx) = mpsc::channel::<AudioCommand>();

    // spawn audio thread that owns OutputStream and Sinks (not Sync, so must be owned here).
    thread::spawn(move || {
        // open stream here and keep it alive for the thread lifetime
        let stream = match OutputStreamBuilder::open_default_stream() {
            Ok(s) => s,
            Err(_e) => {
                //fail
                // log when debugging
                return;
            }
        };

        // single sink for the "music" track; extend to multiple sinks if needed
        let mut current_sink: Option<Sink> = None;

        // loop and handle commands
        for cmd in rx {
            match cmd {
                AudioCommand::Play { name, loop_forever } => {
                    IS_PAUSED.store(false, Ordering::Relaxed);
                    // stop previous sink if present
                    if let Some(s) = current_sink.take() {
                        s.stop();
                    }

                    match get_waveform(&name) {
                        Some(bytes) => {
                            let cursor = Cursor::new(bytes);
                            match Decoder::try_from(cursor) {
                                Ok(decoder) => {
                                    // connect to the stream mixer
                                    let sink = Sink::connect_new(&stream.mixer());

                                    if loop_forever {
                                        sink.append(decoder.repeat_infinite());
                                    } else {
                                        sink.append(decoder);
                                    }

                                    sink.play();
                                    current_sink = Some(sink);
                                }
                                Err(_e) => {}
                            }
                        }
                        None => {}
                    }
                }

                AudioCommand::Pause => {
                    IS_PAUSED.store(true, Ordering::Relaxed);
                    if let Some(s) = current_sink.as_ref() {
                        s.pause();
                    }
                }

                AudioCommand::Resume => {
                    IS_PAUSED.store(false, Ordering::Relaxed);
                    if let Some(s) = current_sink.as_ref() {
                        s.play();
                    }
                }

                AudioCommand::TogglePause => {
                    IS_PAUSED.store(!IS_PAUSED.load(Ordering::Relaxed), Ordering::Relaxed);
                    if let Some(s) = current_sink.as_ref() {
                        if s.is_paused() {
                            s.play();
                        } else {
                            s.pause();
                        }
                    }
                } /*
                                  AudioCommand::Stop => {
                                      if let Some(s) = current_sink.take() {
                                          s.stop();
                                      }
                                  }

                                  AudioCommand::Shutdown => {
                                      // stop sinks then exit thread
                                      if let Some(s) = current_sink.take() {
                                          s.stop();
                                      }
                                      break;
                                  }
                  */
            }

            // small sleep to yield
            thread::sleep(Duration::from_millis(1));
        }

        // channel closed -> cleanup
        if let Some(s) = current_sink.take() {
            s.stop();
        }
    });

    let _ = AUDIO_SENDER.set(tx.clone());
    tx
}

// convenience wrappers (lazy init)
pub fn play(name: &str, loop_forever: bool) -> Result<(), Box<dyn Error>> {
    let tx = init_audio_thread();
    tx.send(AudioCommand::Play {
        name: name.to_string(),
        loop_forever,
    })?;
    Ok(())
}

pub fn toggle() -> Result<(), Box<dyn Error>> {
    let tx = init_audio_thread();
    tx.send(AudioCommand::TogglePause)?;
    Ok(())
}

#[allow(dead_code)]
pub fn pause() -> Result<(), Box<dyn Error>> {
    let tx = init_audio_thread();
    tx.send(AudioCommand::Pause)?;
    Ok(())
}

#[allow(dead_code)]
pub fn resume() -> Result<(), Box<dyn Error>> {
    let tx = init_audio_thread();
    tx.send(AudioCommand::Resume)?;
    Ok(())
}

/*
pub fn stop() -> Result<(), Box<dyn Error>> {
    let tx = init_audio_thread();
    tx.send(AudioCommand::Stop)?;
    Ok(())
}

pub fn shutdown() -> Result<(), Box<dyn Error>> {
    if let Some(tx) = AUDIO_SENDER.get() {
        tx.send(AudioCommand::Shutdown)?;
    }
    Ok(())
}
*/
