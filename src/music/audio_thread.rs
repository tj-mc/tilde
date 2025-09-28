use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::music::scheduler::Scheduler;
use crate::music::output::OutputCollection;

pub enum AudioThreadCommand {
    Stop,
    Tick,
}

pub struct AudioThreadManager {
    thread_handle: Option<JoinHandle<()>>,
    command_sender: Option<Sender<AudioThreadCommand>>,
}

impl AudioThreadManager {
    pub fn new() -> Self {
        AudioThreadManager {
            thread_handle: None,
            command_sender: None,
        }
    }

    pub fn start(&mut self, scheduler: Arc<Mutex<Scheduler>>, outputs: Arc<Mutex<OutputCollection>>) {
        if self.thread_handle.is_some() {
            return; // Already running
        }

        let (tx, rx) = unbounded();
        self.command_sender = Some(tx);

        let handle = thread::spawn(move || {
            println!("🎵 Audio thread started - real-time ticking at 20Hz");
            let mut last_tick = Instant::now();

            loop {
                // Check for stop command (non-blocking)
                match rx.try_recv() {
                    Ok(AudioThreadCommand::Stop) => {
                        println!("🛑 Audio thread stopping");
                        break;
                    }
                    Ok(AudioThreadCommand::Tick) => {
                        // Manual tick requested
                    }
                    Err(_) => {
                        // No command, continue with regular ticking
                    }
                }

                // Automatic tick every 50ms (20Hz)
                let now = Instant::now();
                if now.duration_since(last_tick) >= Duration::from_millis(50) {
                    // Lock both scheduler and outputs for the duration of the tick
                    if let (Ok(mut sched), Ok(mut outs)) = (scheduler.try_lock(), outputs.try_lock()) {
                        let events = sched.tick();
                        for event in &events {
                            let _ = outs.send_event_to_all(event);
                        }
                        last_tick = now;
                    }
                }

                // Sleep for a short time to prevent CPU spinning
                thread::sleep(Duration::from_millis(10));
            }
        });

        self.thread_handle = Some(handle);
    }

    pub fn stop(&mut self) {
        if let Some(sender) = &self.command_sender {
            let _ = sender.send(AudioThreadCommand::Stop);
        }

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        self.command_sender = None;
    }

    pub fn manual_tick(&self) {
        if let Some(sender) = &self.command_sender {
            let _ = sender.send(AudioThreadCommand::Tick);
        }
    }

    pub fn is_running(&self) -> bool {
        self.thread_handle.is_some()
    }
}

impl Drop for AudioThreadManager {
    fn drop(&mut self) {
        self.stop();
    }
}