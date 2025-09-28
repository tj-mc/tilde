use super::{OutputAdapter, TimedEvent};
use crate::music::pattern::EventData;
use rodio::{OutputStream, Sink};
use std::io::Cursor;
use std::f32::consts::PI;

/// Real audio output using rodio for cross-platform audio playback
/// Generates sine wave tones for musical notes
pub struct AudioOutput {
    _stream: OutputStream,
    sink: Sink,
    sample_rate: u32,
}

impl AudioOutput {
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {}", e))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        Ok(AudioOutput {
            _stream: stream,
            sink,
            sample_rate: 44100,
        })
    }

    /// Generate sine wave samples for a given frequency and duration
    fn generate_sine_wave(&self, frequency: f32, duration: f32, volume: f32) -> Vec<f32> {
        let sample_count = (self.sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(sample_count);

        for i in 0..sample_count {
            let t = i as f32 / self.sample_rate as f32;
            let sample = (2.0 * PI * frequency * t).sin() * volume;
            samples.push(sample);
        }

        samples
    }

    /// Convert note name (like "c4", "a#3") to frequency in Hz
    fn note_to_frequency(&self, note: &str) -> f32 {
        // Parse note string (e.g., "c4", "a#3", "bb2")
        let note_lower = note.to_lowercase();
        let chars: Vec<char> = note_lower.chars().collect();

        if chars.is_empty() {
            return 440.0; // Default to A4
        }

        // Parse note name (c, d, e, f, g, a, b)
        let base_note = match chars[0] {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => return 440.0, // Invalid note, return A4
        };

        let mut semitone_offset = base_note;
        let mut octave = 4; // Default octave
        let mut i = 1;

        // Parse accidentals (# or b)
        while i < chars.len() {
            match chars[i] {
                '#' => {
                    semitone_offset += 1;
                    i += 1;
                }
                'b' => {
                    semitone_offset -= 1;
                    i += 1;
                }
                '0'..='9' => {
                    // Parse octave number
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

        // Calculate frequency using A4 = 440Hz as reference
        // Formula: f = 440 * 2^((n-69)/12) where n is MIDI note number
        let midi_note = (octave + 1) * 12 + semitone_offset;
        let a4_midi = 69; // A4 is MIDI note 69

        440.0 * 2.0_f32.powf((midi_note - a4_midi) as f32 / 12.0)
    }

    /// Create a WAV file in memory from PCM samples
    fn create_wav_buffer(&self, samples: Vec<f32>) -> Vec<u8> {
        let mut wav_data = Vec::new();

        // WAV header
        wav_data.extend_from_slice(b"RIFF");
        let file_size = 36 + (samples.len() * 2) as u32; // 16-bit samples
        wav_data.extend_from_slice(&file_size.to_le_bytes());

        wav_data.extend_from_slice(b"WAVE");
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes()); // PCM format chunk size
        wav_data.extend_from_slice(&1u16.to_le_bytes());  // PCM format
        wav_data.extend_from_slice(&1u16.to_le_bytes());  // Mono
        wav_data.extend_from_slice(&self.sample_rate.to_le_bytes());
        wav_data.extend_from_slice(&(self.sample_rate * 2).to_le_bytes()); // Byte rate
        wav_data.extend_from_slice(&2u16.to_le_bytes());  // Block align
        wav_data.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

        wav_data.extend_from_slice(b"data");
        let data_size = (samples.len() * 2) as u32;
        wav_data.extend_from_slice(&data_size.to_le_bytes());

        // Convert f32 samples to 16-bit PCM
        for sample in samples {
            let pcm_sample = (sample * 32767.0) as i16;
            wav_data.extend_from_slice(&pcm_sample.to_le_bytes());
        }

        wav_data
    }

    /// Play a single note with the given frequency and duration
    pub fn play_note(&mut self, note: &str, velocity: f64, duration: f64) {
        let frequency = self.note_to_frequency(note);
        let volume = (velocity * 0.7).min(0.8); // Scale velocity to volume
        let actual_duration = (duration * 3.0).max(0.1).min(1.5); // Scale but keep reasonable bounds

        println!("🔊 AudioOutput playing: {} = {:.1}Hz, volume={:.2}, duration={:.1}s",
                 note, frequency, volume, actual_duration);

        let samples = self.generate_sine_wave(frequency, actual_duration as f32, volume as f32);
        let wav_data = self.create_wav_buffer(samples);

        let cursor = Cursor::new(wav_data);
        if let Ok(source) = rodio::Decoder::new(cursor) {
            self.sink.append(source);
            println!("✅ Audio appended to sink successfully");
        } else {
            println!("❌ Failed to decode audio data");
        }
    }
}

impl OutputAdapter for AudioOutput {
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String> {
        match &event.data {
            EventData::Note { pitch, velocity, duration } => {
                self.play_note(pitch, *velocity, *duration);
                Ok(())
            }
            EventData::Rest => {
                // For rests, we don't need to do anything - the timing is handled by the scheduler
                Ok(())
            }
            EventData::Control { param: _, value: _ } => {
                // Control events not supported yet
                Ok(())
            }
        }
    }

    fn start(&mut self) -> Result<(), String> {
        // Audio stream is already started in constructor
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        self.sink.clear();
        Ok(())
    }

    fn name(&self) -> &str {
        "AudioOutput"
    }
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new().expect("Failed to create default AudioOutput")
    }
}