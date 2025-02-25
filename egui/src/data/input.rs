//! The input needed by egui.

use crate::emath::*;

/// What the integrations provides to egui at the start of each frame.
///
/// Set the values that make sense, leave the rest at their `Default::default()`.
///
/// All coordinates are in points (logical pixels) with origin (0, 0) in the top left corner.
#[derive(Clone, Debug)]
pub struct RawInput {
    /// How many points (logical pixels) the user scrolled
    pub scroll_delta: Vec2,

    #[deprecated = "Use instead: `screen_rect: Some(Rect::from_pos_size(Default::default(), screen_size))`"]
    pub screen_size: Vec2,

    /// Position and size of the area that egui should use.
    /// Usually you would set this to
    ///
    /// `Some(Rect::from_pos_size(Default::default(), screen_size))`.
    ///
    /// but you could also constrain egui to some smaller portion of your window if you like.
    ///
    /// `None` will be treated as "same as last frame", with the default being a very big area.
    pub screen_rect: Option<Rect>,

    /// Also known as device pixel ratio, > 1 for high resolution screens.
    /// If text looks blurry you probably forgot to set this.
    /// Set this the first frame, whenever it changes, or just on every frame.
    pub pixels_per_point: Option<f32>,

    /// Monotonically increasing time, in seconds. Relative to whatever. Used for animations.
    /// If `None` is provided, egui will assume a time delta of `predicted_dt` (default 1/60 seconds).
    pub time: Option<f64>,

    /// Should be set to the expected time between frames when painting at vsync speeds.
    /// The default for this is 1/60.
    /// Can safely be left at its default value.
    pub predicted_dt: f32,

    /// Which modifier keys are down at the start of the frame?
    pub modifiers: Modifiers,

    /// In-order events received this frame.
    ///
    /// There is currently no way to know if egui handles a particular event,
    /// but you can check if egui is using the keyboard with [`crate::Context::wants_keyboard_input`]
    /// and/or the pointer (mouse/touch) with [`crate::Context::is_using_pointer`].
    pub events: Vec<Event>,
}

impl Default for RawInput {
    fn default() -> Self {
        #![allow(deprecated)] // for screen_size
        Self {
            scroll_delta: Vec2::ZERO,
            screen_size: Default::default(),
            screen_rect: None,
            pixels_per_point: None,
            time: None,
            predicted_dt: 1.0 / 60.0,
            modifiers: Modifiers::default(),
            events: vec![],
        }
    }
}

impl RawInput {
    /// Helper: move volatile (deltas and events), clone the rest
    pub fn take(&mut self) -> RawInput {
        #![allow(deprecated)] // for screen_size
        RawInput {
            scroll_delta: std::mem::take(&mut self.scroll_delta),
            screen_size: self.screen_size,
            screen_rect: self.screen_rect.take(),
            pixels_per_point: self.pixels_per_point.take(),
            time: self.time.take(),
            predicted_dt: self.predicted_dt,
            modifiers: self.modifiers,
            events: std::mem::take(&mut self.events),
        }
    }
}

/// An input event generated by the integration.
///
/// This only covers events that egui cares about.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// The integration detected a "copy" event (e.g. Cmd+C).
    Copy,
    /// The integration detected a "cut" event (e.g. Cmd+X).
    Cut,
    /// Text input, e.g. via keyboard or paste action.
    ///
    /// When the user presses enter/return, do not send a `Text` (just [`Key::Enter`]).
    Text(String),
    Key {
        key: Key,
        pressed: bool,
        modifiers: Modifiers,
    },

    PointerMoved(Pos2),
    PointerButton {
        pos: Pos2,
        button: PointerButton,
        pressed: bool,
        /// The state of the modifier keys at the time of the event
        modifiers: Modifiers,
    },
    /// The mouse left the screen, or the last/primary touch input disappeared.
    ///
    /// This means there is no longer a cursor on the screen for hovering etc.
    ///
    /// On touch-up first send `PointerButton{pressed: false, …}` followed by `PointerLeft`.
    PointerGone,
}

/// Mouse button (or similar for touch input)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerButton {
    /// The primary mouse button is usually the left one.
    Primary = 0,
    /// The secondary mouse button is usually the right one,
    /// and most often used for context menus or other optional things.
    Secondary = 1,
    /// The tertiary mouse button is usually the middle mouse button (e.g. clicking the scroll wheel).
    Middle = 2,
}

/// Number of pointer buttons supported by egui, i.e. the number of possible states of [`PointerButton`].
pub const NUM_POINTER_BUTTONS: usize = 3;

/// State of the modifier keys. These must be fed to egui.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Modifiers {
    /// Either of the alt keys are down (option ⌥ on Mac).
    pub alt: bool,
    /// Either of the control keys are down.
    /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
    pub ctrl: bool,
    /// Either of the shift keys are down.
    pub shift: bool,
    /// The Mac ⌘ Command key. Should always be set to `false` on other platforms.
    pub mac_cmd: bool,
    /// On Windows and Linux, set this to the same value as `ctrl`.
    /// On Mac, this should be set whenever one of the ⌘ Command keys are down (same as `mac_cmd`).
    /// This is so that egui can, for instance, select all text by checking for `command + A`
    /// and it will work on both Mac and Windows.
    pub command: bool,
}

impl Modifiers {
    pub fn is_none(&self) -> bool {
        self == &Self::default()
    }

    pub fn any(&self) -> bool {
        !self.is_none()
    }
}

/// Keyboard keys.
///
/// Includes all keys egui is interested in (such as `Home` and `End`)
/// plus a few that are useful for detecting keyboard shortcuts.
///
/// Many keys are omitted because they are not always physical keys (depending on keyboard language), e.g. `;` and `§`,
/// and are therefor unsuitable as keyboard shortcuts if you want your app to be portable.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Key {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    /// Either from the main row or from the numpad.
    Num0,
    /// Either from the main row or from the numpad.
    Num1,
    /// Either from the main row or from the numpad.
    Num2,
    /// Either from the main row or from the numpad.
    Num3,
    /// Either from the main row or from the numpad.
    Num4,
    /// Either from the main row or from the numpad.
    Num5,
    /// Either from the main row or from the numpad.
    Num6,
    /// Either from the main row or from the numpad.
    Num7,
    /// Either from the main row or from the numpad.
    Num8,
    /// Either from the main row or from the numpad.
    Num9,

    A, // Used for cmd+A (select All)
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K, // Used for ctrl+K (delete text after cursor)
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U, // Used for ctrl+U (delete text before cursor)
    V,
    W, // Used for ctrl+W (delete previous word)
    X,
    Y,
    Z, // Used for cmd+Z (undo)
}

impl RawInput {
    pub fn ui(&self, ui: &mut crate::Ui) {
        #![allow(deprecated)] // for screen_size
        let Self {
            scroll_delta,
            screen_size: _,
            screen_rect,
            pixels_per_point,
            time,
            predicted_dt,
            modifiers,
            events,
        } = self;

        ui.label(format!("scroll_delta: {:?} points", scroll_delta));
        ui.label(format!("screen_rect: {:?} points", screen_rect));
        ui.label(format!("pixels_per_point: {:?}", pixels_per_point))
            .on_hover_text(
                "Also called HDPI factor.\nNumber of physical pixels per each logical pixel.",
            );
        if let Some(time) = time {
            ui.label(format!("time: {:.3} s", time));
        } else {
            ui.label("time: None");
        }
        ui.label(format!("predicted_dt: {:.1} ms", 1e3 * predicted_dt));
        ui.label(format!("modifiers: {:#?}", modifiers));
        ui.label(format!("events: {:?}", events))
            .on_hover_text("key presses etc");
    }
}
