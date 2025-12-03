

// https://github.com/id-Software/Quake-III-Arena/blob/dbe4ddb10315479fc00086f08e25d968b4b43c49/code/qcommon/qcommon.h#L931

// typedef enum {
//   // bk001129 - make sure SE_NONE is zero
// 	SE_NONE = 0,	// evTime is still valid
// 	SE_KEY,		// evValue is a key code, evValue2 is the down flag
// 	SE_CHAR,	// evValue is an ascii char
// 	SE_MOUSE,	// evValue and evValue2 are reletive signed x / y moves
// 	SE_JOYSTICK_AXIS,	// evValue is an axis number and evValue2 is the current state (-127 to 127)
// 	SE_CONSOLE,	// evPtr is a char*
// 	SE_PACKET	// evPtr is a netadr_t followed by data bytes to evPtrLength
// } sysEventType_t;

// typedef struct {
// 	int				evTime;
// 	sysEventType_t	evType;
// 	int				evValue, evValue2;
// 	int				evPtrLength;	// bytes of data pointed to by evPtr, for journaling
// 	void			*evPtr;			// this must be manually freed if not NULL
// } sysEvent_t;

// Ideally we map from WindowEvent to our own custom event to hide winit from the rest of the codebase
// but for now we will just store WindowEvents directly. This may change when we need joystick/gamepad support for example, or network events.
use winit::event::{WindowEvent};

pub struct EventSystem {
    pub events: Vec<WindowEvent>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            events: vec![],
        }
    }

    // pub fn push<T: Entity + 'static>(&mut self, entity: T) -> &mut Self {
    //     self.events.push(Box::new(entity));
    //     self
    // }

    pub fn queue_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.events.push(event)
            },
            WindowEvent::Resized(_size) => {
                self.events.push(event)
            },
            WindowEvent::RedrawRequested => {
                self.events.push(event);
            }
            WindowEvent::MouseInput { .. } => {
                self.events.push(event);
            },
            WindowEvent::KeyboardInput { .. } => {
                self.events.push(event);
            },
            _ => {}
        }
    }

    pub fn process_events(&mut self) {
        for event in self.events.iter() {
            self.process_event(event);
        }
        self.events.clear();
    }

    pub fn process_event(&self, _event: &WindowEvent) {

    }
}