use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::collections::HashSet;
use winit::event::{ElementState, MouseButton, WindowEvent, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::core::math::vec2::Vec2;

/// Serializable game event that wraps the relevant parts of WindowEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    CloseRequested,
    Resized { width: u32, height: u32 },
    RedrawRequested,
    MouseInput {
        button: MouseButtonType,
        state: ElementStateType,
    },
    KeyboardInput {
        key_code: KeyCodeType,
        state: ElementStateType,
    },
    CursorMoved {
        x: f32,
        y: f32,
    },
}

/// Serializable mouse button type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MouseButtonType {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

/// Serializable element state (pressed/released)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ElementStateType {
    Pressed,
    Released,
}

/// Serializable key code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCodeType {
    Escape,
    Space,
    ShiftLeft,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    KeyA,
    KeyD,
    KeyW,
    KeyS,
    KeyZ,
    KeyX,
    F9,
    F10,
    F11,
    F12,
    // Add more as needed
    Unknown,
}

impl From<KeyCode> for KeyCodeType {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Escape => KeyCodeType::Escape,
            KeyCode::Space => KeyCodeType::Space,
            KeyCode::ShiftLeft => KeyCodeType::ShiftLeft,
            KeyCode::ArrowLeft => KeyCodeType::ArrowLeft,
            KeyCode::ArrowRight => KeyCodeType::ArrowRight,
            KeyCode::ArrowUp => KeyCodeType::ArrowUp,
            KeyCode::ArrowDown => KeyCodeType::ArrowDown,
            KeyCode::KeyA => KeyCodeType::KeyA,
            KeyCode::KeyD => KeyCodeType::KeyD,
            KeyCode::KeyW => KeyCodeType::KeyW,
            KeyCode::KeyS => KeyCodeType::KeyS,
            KeyCode::KeyZ => KeyCodeType::KeyZ,
            KeyCode::KeyX => KeyCodeType::KeyX,
            KeyCode::F9 => KeyCodeType::F9,
            KeyCode::F10 => KeyCodeType::F10,
            KeyCode::F11 => KeyCodeType::F11,
            KeyCode::F12 => KeyCodeType::F12,
            _ => KeyCodeType::Unknown,
        }
    }
}

/// Event paired with a frame number for recording/replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FramedEvent {
    pub frame: u128,
    pub event: GameEvent,
}

/// Recording of a game session
#[derive(Debug, Serialize, Deserialize)]
pub struct EventRecording {
    pub events: Vec<FramedEvent>,
}

pub struct EventSystem {
    pub events: Vec<GameEvent>,
    
    // Live state tracking (former InputHelper)
    // keys_pressed: HashSet<KeyCodeType>,
    // pub mouse_position: Vec2,

    // Recording state
    recording: bool,
    recorded_events: Vec<FramedEvent>,
    current_frame: u128,
    
    // Replay state
    replaying: bool,
    replay_events: Vec<FramedEvent>,
    replay_index: usize,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            events: vec![],
            recording: false,
            recorded_events: vec![],
            current_frame: 0,
            replaying: false,
            replay_events: vec![],
            replay_index: 0,
        }
    }

    /// Update the current frame number
    pub fn set_frame(&mut self, frame: u128) {
        self.current_frame = frame;
    }

    /// Start recording events
    pub fn start_recording(&mut self) {
        self.recording = true;
        self.recorded_events.clear();
        println!("Started recording events");
    }

    /// Stop recording events
    pub fn stop_recording(&mut self) {
        self.recording = false;
        println!("Stopped recording. Captured {} events", self.recorded_events.len());
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.recording
    }

    /// Export recorded events to a JSON file
    pub fn export_recording(&self, path: &str) -> io::Result<()> {
        let recording = EventRecording {
            events: self.recorded_events.clone(),
        };
        
        let json = serde_json::to_string_pretty(&recording)?;
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())?;
        
        println!("Exported {} events to {}", self.recorded_events.len(), path);
        Ok(())
    }

    /// Load events from a JSON file
    pub fn load_replay(&mut self, path: &str) -> io::Result<()> {
        let json = fs::read_to_string(path)?;
        let recording: EventRecording = serde_json::from_str(&json)?;
        
        self.replay_events = recording.events;
        self.replay_index = 0;
        
        println!("Loaded {} events from {}", self.replay_events.len(), path);
        Ok(())
    }

    /// Start replaying loaded events
    pub fn start_replay(&mut self) {
        if self.replay_events.is_empty() {
            println!("No replay events loaded");
            return;
        }
        
        self.replaying = true;
        self.replay_index = 0;
        println!("Started replay with {} events", self.replay_events.len());
    }

    /// Stop replaying events
    pub fn stop_replay(&mut self) {
        self.replaying = false;
        println!("Stopped replay");
    }

    /// Check if currently replaying
    pub fn is_replaying(&self) -> bool {
        self.replaying
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent, _scale_factor: f64) {
        if let Some(game_event) = self.window_event_to_game_event(event) {
            self.queue_event(game_event);
        }
    }

    /// Convert WindowEvent to GameEvent for serialization
    fn window_event_to_game_event(&mut self, event: &WindowEvent) -> Option<GameEvent> {
        match event {
            WindowEvent::MouseInput { button, state, .. } => {
                let button_type = match button {
                    MouseButton::Left => MouseButtonType::Left,
                    MouseButton::Right => MouseButtonType::Right,
                    MouseButton::Middle => MouseButtonType::Middle,
                    MouseButton::Back => MouseButtonType::Back,
                    MouseButton::Forward => MouseButtonType::Forward,
                    MouseButton::Other(n) => MouseButtonType::Other(*n),
                };
                let state_type = match state {
                    ElementState::Pressed => ElementStateType::Pressed,
                    ElementState::Released => ElementStateType::Released,
                };
                Some(GameEvent::MouseInput {
                    button: button_type,
                    state: state_type,
                })
            }
            WindowEvent::KeyboardInput { 
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(key_code),
                    state,
                    ..
                },
                ..
            } => {
                let key_type = KeyCodeType::from(*key_code);
                let state_type = match state {
                    ElementState::Pressed => {
                        //self.keys_pressed.insert(key_type);
                        ElementStateType::Pressed
                    }
                    ElementState::Released => {
                        //self.keys_pressed.remove(&key_type);
                        ElementStateType::Released
                    }
                };
                Some(GameEvent::KeyboardInput {
                    key_code: key_type,
                    state: state_type,
                })
            }
            // WindowEvent::CursorMoved { position, .. } => {
            //     self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
            //     Some(GameEvent::CursorMoved {
            //         x: self.mouse_position.x,
            //         y: self.mouse_position.y,
            //     })
            // }
            _ => None,
        }
    }

    pub fn queue_event(&mut self, event: GameEvent) {
        // Record the event if recording is active (only mouse, keyboard and cursor events)
        if self.recording {
            match &event {
                GameEvent::MouseInput { .. } | GameEvent::KeyboardInput { .. } | GameEvent::CursorMoved { .. } => {
                    self.recorded_events.push(FramedEvent {
                        frame: self.current_frame,
                        event: event.clone(),
                    });
                }
                _ => {}
            }
        }

        // Queue all events for processing
        self.events.push(event);
    }

    /// Get replay events for the current frame and inject them into the event queue
    pub fn inject_replay_events(&mut self) {
        if !self.replaying {
            return;
        }

        // Find all events for the current frame
        while self.replay_index < self.replay_events.len() {
            let framed_event = &self.replay_events[self.replay_index];
            
            if framed_event.frame > self.current_frame {
                // Future event, stop here
                break;
            }
            
            if framed_event.frame == self.current_frame {
                // Directly queue GameEvent - it will also update state
                let event = framed_event.event.clone();
                self.queue_event(event);
            }
            
            self.replay_index += 1;
        }

        // Check if replay is complete
        if self.replay_index >= self.replay_events.len() {
            println!("Replay complete");
            self.stop_replay();
        }
    }

    pub fn process_events(&mut self) {
        // In replay mode, inject replay events first
        if self.replaying {
            self.inject_replay_events();
        }        
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
