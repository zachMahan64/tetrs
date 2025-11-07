use include_dir::{Dir, include_dir};
use rodio::{Decoder, Sink};
use rodio::{OutputStreamBuilder, Source};
use std::error::Error;
use std::io::Cursor;

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_waveform(name: &str) -> Option<&'static [u8]> {
    ASSETS.get_file(name).map(|f| f.contents())
}

pub fn play_wav_from_assets(name: &str, loop_forever: bool) -> Result<(), Box<dyn Error>> {
    let bytes = get_waveform(name).ok_or("Waveform not found")?;
    let cursor = Cursor::new(bytes);
    let decoder = Decoder::try_from(cursor)?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    if loop_forever {
        sink.append(decoder.repeat_infinite());
    } else {
        sink.append(decoder);
    }

    sink.sleep_until_end();

    Ok(())
}

pub const THEME_NORMAL: &str = "tetrs_theme_normal.wav";
pub const THEME_FAST: &str = "tetrs_theme_fast.wav";
