use super::{OutputAdapter, TimedEvent};
use crate::music::pattern::EventData;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

/// Direct audio output using rodio for real-time audio synthesis
/// Maximum efficiency for live coding - no files, no lag
pub struct DirectAudioOutput {
    _stream: OutputStream,
    sink: Sink,
    sample_rate: u32,
}

impl DirectAudioOutput {
    pub fn new() -> Result<Self, String> {
        println!("🎵 Creating DirectAudioOutput - using rodio for real-time audio");

        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {}", e))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        Ok(DirectAudioOutput {
            _stream: stream,
            sink,
            sample_rate: 44100,
        })
    }


    /// Convert note to frequency
    fn note_to_frequency(&self, note: &str) -> f32 {
        let note_lower = note.to_lowercase();
        let chars: Vec<char> = note_lower.chars().collect();

        if chars.is_empty() {
            return 440.0;
        }

        let base_note = match chars[0] {
            'c' => 0, 'd' => 2, 'e' => 4, 'f' => 5, 'g' => 7, 'a' => 9, 'b' => 11,
            _ => return 440.0,
        };

        let mut semitone_offset = base_note;
        let mut octave = 4;
        let mut i = 1;

        while i < chars.len() {
            match chars[i] {
                '#' => { semitone_offset += 1; i += 1; }
                'b' => { semitone_offset -= 1; i += 1; }
                '0'..='9' => {
                    let mut octave_str = String::new();
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        octave_str.push(chars[i]);
                        i += 1;
                    }
                    if let Ok(parsed_octave) = octave_str.parse::<i32>() {
                        octave = parsed_octave;
                    }
                    break;
                }
                _ => break,
            }
        }

        let midi_note = (octave + 1) * 12 + semitone_offset;
        let a4_midi = 69;
        440.0 * 2.0_f32.powf((midi_note - a4_midi) as f32 / 12.0)
    }

    /// Play note using rodio's built-in sine wave source - no WAV encoding overhead
    pub fn play_note(&mut self, note: &str, velocity: f64, duration: f64) {
        let frequency = self.note_to_frequency(note);
        let volume = (velocity * 0.3) as f32; // Lower volume to prevent distortion

        // Ensure minimum audible duration for very fast patterns
        let min_duration = 0.05; // 50ms minimum for audibility
        let actual_duration = duration.max(min_duration);
        let play_duration = Duration::from_secs_f64(actual_duration);

        println!("🎵 Playing: {} = {:.1}Hz ({}ms)", note, frequency, actual_duration * 1000.0);

        // Use rodio's efficient sine wave source with fade to prevent popping
        let fade_ms = 5; // 5ms fade in/out to prevent popping
        let source = rodio::source::SineWave::new(frequency)
            .take_duration(play_duration)
            .fade_in(Duration::from_millis(fade_ms))
            .amplify(volume);

        // Don't clear sink - let notes overlap naturally with fades
        self.sink.append(source);
    }


}

impl OutputAdapter for DirectAudioOutput {
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String> {
        match &event.data {
            EventData::Note { pitch, velocity, duration } => {
                self.play_note(pitch, *velocity, *duration);
                Ok(())
            }
            EventData::Rest => Ok(()),
            EventData::Control { .. } => Ok(()),
        }
    }

    fn start(&mut self) -> Result<(), String> {
        println!("🎵 DirectAudioOutput ready");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        self.sink.clear();
        println!("🛑 DirectAudioOutput stopped");
        Ok(())
    }

    fn name(&self) -> &str {
        "DirectAudio"
    }
}