# Tilde Music Engine Architecture Plan

## Vision

Create a modular, decoupled music system for Tilde that enables live coding music with multiple output targets (audio, MIDI, OSC). The system will be inspired by Strudel's mini-notation and live coding capabilities but implemented as a native CLI tool with Tilde's functional programming paradigm.

## Core Architecture Overview

```
Pattern Engine    →  Scheduler    →  Output Adapters
     ↓                   ↓              ↓
Event Stream     →  Timed Events  →  Audio/MIDI/OSC
```

## Module Structure

### 1. Pattern Engine (`src/music/pattern/`)

**Responsibility**: Parse and evaluate musical patterns

```rust
// src/music/pattern/mod.rs
pub struct Pattern {
    events: Vec<Event>,
    duration: Duration,
}

pub struct Event {
    time: f64,           // 0.0 to 1.0 within pattern cycle
    data: EventData,
}

pub enum EventData {
    Note { pitch: f64, velocity: f64, duration: f64 },
    Control { param: String, value: f64 },
    Rest,
}

// Mini-notation parser
pub fn parse_mini_notation(input: &str) -> Result<Pattern, String>;

// Pattern transformations
pub trait PatternTransform: Send + Sync {
    fn apply(&self, pattern: &Pattern) -> Pattern;
}

pub struct SpeedTransform(f64);
pub struct TransposeTransform(f64);
pub struct ReverseTransform;
```

### 2. Scheduler (`src/music/scheduler/`)

**Responsibility**: Handle timing, tempo, and event dispatch

```rust
// src/music/scheduler/mod.rs
pub struct Scheduler {
    tempo: f64,
    current_time: f64,
    patterns: HashMap<String, ActivePattern>,
    outputs: Vec<Box<dyn OutputAdapter>>,
}

pub struct ActivePattern {
    pattern: Pattern,
    cycle_position: f64,
    transformations: Vec<Box<dyn PatternTransform>>,
}

impl Scheduler {
    pub fn add_pattern(&mut self, name: String, pattern: Pattern);
    pub fn remove_pattern(&mut self, name: &str);
    pub fn update_pattern(&mut self, name: &str, pattern: Pattern);
    pub fn set_tempo(&mut self, bpm: f64);
    pub fn tick(&mut self) -> Vec<TimedEvent>; // Called from audio thread
}
```

### 3. Output Adapters (`src/music/output/`)

**Responsibility**: Send events to various destinations

```rust
// src/music/output/mod.rs
pub trait OutputAdapter: Send + Sync {
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String>;
    fn start(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn name(&self) -> &str;
}

pub struct TimedEvent {
    pub timestamp: f64,
    pub data: EventData,
    pub channel: Option<String>, // pattern name or channel identifier
}
```

#### Audio Output Adapter
```rust
pub struct AudioOutput {
    stream: OutputStream,
    synths: HashMap<String, Box<dyn Synth>>, // Simple built-in synths
    effects: Vec<Box<dyn AudioEffect>>,
    sample_rate: u32,
}

// Simple synths for local audio
pub trait Synth: Send + Sync {
    fn process_note(&mut self, note: &Note) -> Vec<f32>;
}

pub struct SineSynth;
pub struct SawSynth;
pub struct SampleSynth { samples: HashMap<String, Vec<f32>> };
```

#### MIDI Output Adapter
```rust
pub struct MidiOutput {
    connection: MidiOutputConnection,
    channel_map: HashMap<String, u8>, // pattern name -> MIDI channel
    device_name: String,
}

impl OutputAdapter for MidiOutput {
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String> {
        match &event.data {
            EventData::Note { pitch, velocity, duration } => {
                let channel = self.channel_map.get(event.channel.as_ref().unwrap_or(&"default".to_string())).unwrap_or(&0);
                let note_on = [0x90 | channel, *pitch as u8, (*velocity * 127.0) as u8];
                self.connection.send(&note_on)?;
                // Schedule note off based on duration...
            }
            EventData::Control { param, value } => {
                // Send CC messages
            }
            _ => {}
        }
        Ok(())
    }
}
```

#### OSC Output Adapter
```rust
pub struct OscOutput {
    socket: UdpSocket,
    target_addr: SocketAddr,
}

impl OutputAdapter for OscOutput {
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String> {
        let osc_msg = match &event.data {
            EventData::Note { pitch, velocity, duration } => {
                OscMessage {
                    addr: "/note".to_string(),
                    args: vec![
                        OscType::Float(*pitch as f32),
                        OscType::Float(*velocity as f32),
                        OscType::Float(*duration as f32),
                    ],
                }
            }
            _ => return Ok(()),
        };

        let packet = OscPacket::Message(osc_msg);
        let msg_buf = rosc::encoder::encode(&packet)?;
        self.socket.send_to(&msg_buf, &self.target_addr)?;
        Ok(())
    }
}
```

### 4. Future: VST Host (`src/music/vst/`)

**Responsibility**: Host VST plugins for advanced audio processing

```rust
// Future implementation
pub struct VstHost {
    plugins: HashMap<String, VstPlugin>,
    host: vst::host::Host,
}

pub struct VstPlugin {
    instance: Box<dyn vst::plugin::Plugin>,
    parameters: HashMap<String, f32>,
}
```

## Tilde Language Integration

### New Music Functions in Stdlib

```rust
// src/stdlib/music.rs

/// Start playing a pattern
/// play ~pattern "channel_name"
pub fn eval_play(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// Set global tempo
/// tempo 120
pub fn eval_tempo(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// Configure output destinations
/// output "midi" "Ableton Live"
/// output "audio" "default"
/// output "osc" "127.0.0.1:57120"
pub fn eval_output(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// Stop a pattern
/// stop "channel_name"
pub fn eval_stop(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// Pattern transformation functions
/// speed ~pattern 2.0
pub fn eval_speed(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// transpose ~pattern 12
pub fn eval_transpose(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// rev ~pattern
pub fn eval_reverse(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// jux ~pattern (apply transformation to right channel only)
pub fn eval_jux(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;

/// stack [~pattern1, ~pattern2, ~pattern3] (layer patterns)
pub fn eval_stack(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String>;
```

### Mini-Notation Support

```rust
// Parse mini-notation strings as a special string type or function
// "c3 e3 g3 c4" -> Pattern
// "[c3 e3] [g3 c4]" -> Pattern with subdivisions
// "c3*2 ~ g3/2" -> Pattern with speed modifications and rests
// "c3,e3,g3" -> Chord pattern
// "<c3 d3 e3>" -> Sequence that cycles through
// "c3?" -> 50% chance of playing
// "c3?0.8" -> 80% chance of playing
```

## Example Tilde Usage

```tilde
# Initialize music system
music-init

# Set up outputs
output "audio" "default"
output "midi" "Ableton Live" 1    # MIDI channel 1
output "osc" "127.0.0.1:57120"

# Set global tempo
tempo 120

# Define patterns using mini-notation
~kick is "x ~ x ~"
~snare is "~ x ~ x"
~hihat is "x x x x"
~bass is "c2*2 [~ g2] f2 ~"
~melody is "<c4 d4 e4 f4 g4>"

# Apply transformations using Tilde's function chaining
~processed-bass:
    ~bass
    speed 2
    transpose 5

~processed-melody:
    ~melody
    speed 0.5
    transpose 12

# Start patterns on different outputs
play ~kick "drums" "audio"
play ~processed-bass "bass" "midi"
play ~processed-melody "lead" "osc"

# Live modifications (patterns update automatically)
~kick is "x ~ [x x] ~"
~bass is "[c2 g2]*2 f2 [a#2 ~]"

# Transform existing patterns
speed "bass" 0.5
transpose "lead" -12
stop "drums"

# Layer patterns
~full-drums is stack [~kick, ~snare, ~hihat]
play ~full-drums "drums"

# Pattern effects (future)
~reverb-bass:
    ~bass
    reverb 0.4 0.8

~distorted-lead:
    ~melody
    distortion 0.6
```

## Configuration System

### Music Engine Configuration
```rust
// src/music/config.rs
#[derive(Serialize, Deserialize)]
pub struct MusicConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub audio_driver: AudioDriverType,
    pub midi_devices: Vec<String>,
    pub osc_targets: Vec<OscTarget>,
    pub default_synths: HashMap<String, SynthType>,
}

#[derive(Serialize, Deserialize)]
pub struct OscTarget {
    pub name: String,
    pub address: String,
    pub port: u16,
}

pub enum AudioDriverType {
    Default,
    Alsa,
    CoreAudio,
    Wasapi,
}

pub enum SynthType {
    Sine,
    Saw,
    Square,
    Sample(String), // path to sample
}
```

### Integration with Evaluator
```rust
// src/evaluator.rs
impl Evaluator {
    pub fn music_engine(&mut self) -> &mut Option<MusicEngine> {
        &mut self.music_engine
    }

    pub fn init_music_engine(&mut self, config: MusicConfig) -> Result<(), String> {
        let engine = MusicEngine::new(config)?;
        self.music_engine = Some(engine);
        Ok(())
    }
}

pub struct MusicEngine {
    scheduler: Arc<Mutex<Scheduler>>,
    _audio_thread: Option<JoinHandle<()>>,
    config: MusicConfig,
}
```

## Threading Model

### Real-time Audio Thread
```rust
// High-priority thread for audio processing
fn audio_thread(scheduler: Arc<Mutex<Scheduler>>, outputs: Vec<Box<dyn OutputAdapter>>) {
    // Set thread to real-time priority
    let mut interval = tokio::time::interval(Duration::from_millis(1));

    loop {
        interval.tick().await;

        let events = {
            let mut sched = scheduler.lock().unwrap();
            sched.tick()
        };

        for event in events {
            for output in &mut outputs {
                if let Err(e) = output.send_event(&event) {
                    eprintln!("Output error: {}", e);
                }
            }
        }
    }
}
```

### Main Thread Communication
```rust
// Messages from Tilde REPL to audio thread
pub enum MusicMessage {
    AddPattern { name: String, pattern: Pattern },
    RemovePattern { name: String },
    UpdatePattern { name: String, pattern: Pattern },
    SetTempo(f64),
    AddOutput(Box<dyn OutputAdapter>),
    RemoveOutput(String),
}
```

## Startup Integration

```rust
// src/main.rs
fn main() {
    let mut evaluator = Evaluator::new();

    // Check for music configuration
    if let Ok(config) = load_music_config() {
        if let Err(e) = evaluator.init_music_engine(config) {
            eprintln!("Warning: Could not initialize music engine: {}", e);
            eprintln!("Music features will be disabled.");
        }
    }

    // Continue with normal Tilde startup...
    run_repl(evaluator);
}

fn load_music_config() -> Result<MusicConfig, String> {
    // Try to load from ~/.tilde/music.toml or similar
    // Fall back to sensible defaults
    Ok(MusicConfig::default())
}
```

## Phase Implementation Plan

### Phase 1: Core Foundation
- [ ] Pattern data structures and mini-notation parser
- [ ] Basic scheduler with tempo and cycle management
- [ ] Simple MIDI output adapter
- [ ] Basic Tilde integration (`play`, `tempo`, `stop` functions)

### Phase 2: Audio Output
- [ ] Local audio output with simple synths (sine, saw, sample playback)
- [ ] Audio thread with real-time scheduling
- [ ] Pattern transformation functions (`speed`, `transpose`, `reverse`)

### Phase 3: Advanced Patterns
- [ ] Complex mini-notation features (euclidean rhythms, probability)
- [ ] Pattern layering and composition (`stack`, `jux`)
- [ ] OSC output adapter

### Phase 4: Effects and Polish
- [ ] Audio effects chain
- [ ] Better terminal UI with pattern visualization
- [ ] Configuration management
- [ ] VST hosting (stretch goal)

## Key Benefits of This Design

1. **Modular**: Each component has a single responsibility
2. **Decoupled**: Pattern engine doesn't know about outputs, scheduler doesn't know about Tilde
3. **Extensible**: New output types, effects, transforms can be added easily
4. **Optional**: Music features don't break normal Tilde usage
5. **Performance**: Real-time scheduling happens in separate threads
6. **Cross-platform**: Abstract interfaces allow platform-specific implementations
7. **Future-proof**: Architecture supports VSTs, advanced audio processing, etc.

This architecture enables:
- Sending the same pattern to multiple outputs simultaneously
- Hot-swapping output destinations
- Adding new pattern transformation functions as regular Tilde stdlib functions
- Scaling from simple MIDI out to full audio production
- Live coding performance with real-time pattern modification

The design leverages Tilde's existing functional programming paradigm and extends it naturally into the musical domain, creating a powerful yet approachable live coding music environment.