// fake temporary winit, until winit adds wasi support

pub mod window {
    use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};

    use super::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Window { }
    impl HasWindowHandle for Window {
        fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, wgpu::rwh::HandleError> {
            let window_handle = unsafe {
                wgpu::rwh::WindowHandle::borrow_raw(wgpu::rwh::RawWindowHandle::Wasi)
            };
            Ok(window_handle)
        }
    }
    impl HasDisplayHandle for Window {
        fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
            let display_handle: wgpu::rwh::DisplayHandle = unsafe {
                wgpu::rwh::DisplayHandle::borrow_raw(wgpu::rwh::RawDisplayHandle::Wasi)
            };
            Ok(display_handle)
        }
    }
    impl Window {
        pub fn inner_size(&self) -> PhysicalSize {
            PhysicalSize { width: 100, height: 100 }
            // PhysicalSize { width: 800, height: 600 }
        }

        pub fn request_redraw(&self) {}
    }

    #[derive(Clone, Default)]
    pub struct WindowBuilder { }
    impl WindowBuilder {
        pub fn new() -> Self {
            Default::default()
        }
        pub fn build<T: 'static>(
            self,
            _window_target: &EventLoopWindowTarget<T>,
        ) -> Result<Window, ()> {
            Ok(Window { })
        }
        pub fn with_title<T: Into<String>>(mut self, _title: T) -> Self {
            self
        }

    }
}

pub mod event_loop {
    use std::{marker::PhantomData, ops::Deref};
    use crate::winit::event::Event;

    use super::event::WindowId;

    pub struct EventLoop<T: 'static> {
        pub(crate) event_loop: PhantomData<T>,
        pub(crate) _marker: PhantomData<*mut ()>,

        pub(crate) window_target: EventLoopWindowTarget<T>,
    }
    impl<T: 'static> EventLoop<T> {
        pub fn new() -> Result<EventLoop<()>, ()> {
            Ok(EventLoop {
                event_loop:PhantomData,
                _marker: PhantomData,
                window_target: EventLoopWindowTarget { p: PhantomData, _marker: PhantomData, },
            })
        }
        pub fn run<F>(self, mut event_handler: F) -> Result<(), ()>// -> Result<(), EventLoopError>
        where
            F: FnMut(Event, &EventLoopWindowTarget<T>),
        {
            event_handler(Event::NewEvents, &self.window_target);
            loop {
                super::sleep(1000);
                let event = Event::WindowEvent {
                    window_id: WindowId(0),
                    event: super::event::WindowEvent::RedrawRequested,
                };
                event_handler(event, &self.window_target);
            }
            // Ok(())
        }
    }

    pub struct EventLoopWindowTarget<T: 'static> {
        pub(crate) p: PhantomData<T>,
        pub(crate) _marker: PhantomData<*mut ()>,
    }
    impl<T> EventLoopWindowTarget<T> {
        pub fn exit(&self) {

        }
    }

    impl<T> Deref for EventLoop<T> {
        type Target = EventLoopWindowTarget<T>;
        fn deref(&self) -> &EventLoopWindowTarget<T> {
            &self.window_target
        }
    }
}

fn sleep(milis: u32) {
    for i in 0..milis+1 {
        if i == milis {
            return;
        }
    }
}

pub mod event {
    // use crate::winit::window::PhysicalSize;

    use super::dpi::PhysicalSize;

    #[derive(Debug, Clone, PartialEq)]
    pub enum WindowEvent {
        CloseRequested,
        RedrawRequested,
        Resized(PhysicalSize),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Event {
        NewEvents,
        WindowEvent {
            window_id: WindowId,
            event: WindowEvent,
        },
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WindowId(pub(crate) u64);

    /// Describes the reason the event loop is resuming.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StartCause {
        // /// Sent if the time specified by [`ControlFlow::WaitUntil`] has been reached. Contains the
        // /// moment the timeout was requested and the requested resume time. The actual resume time is
        // /// guaranteed to be equal to or after the requested resume time.
        // ///
        // /// [`ControlFlow::WaitUntil`]: crate::event_loop::ControlFlow::WaitUntil
        // ResumeTimeReached {
        //     start: Instant,
        //     requested_resume: Instant,
        // },

        // /// Sent if the OS has new events to send to the window, after a wait was requested. Contains
        // /// the moment the wait was requested and the resume time, if requested.
        // WaitCancelled {
        //     start: Instant,
        //     requested_resume: Option<Instant>,
        // },

        /// Sent if the event loop is being resumed after the loop's control flow was set to
        /// [`ControlFlow::Poll`].
        ///
        /// [`ControlFlow::Poll`]: crate::event_loop::ControlFlow::Poll
        Poll,

        /// Sent once, immediately after `run` is called. Indicates that the loop was just initialized.
        Init,
    }

    /// Describes a keyboard input targeting a window.
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct KeyEvent {
        // /// Represents the position of a key independent of the currently active layout.
        // ///
        // /// It also uniquely identifies the physical key (i.e. it's mostly synonymous with a scancode).
        // /// The most prevalent use case for this is games. For example the default keys for the player
        // /// to move around might be the W, A, S, and D keys on a US layout. The position of these keys
        // /// is more important than their label, so they should map to Z, Q, S, and D on an "AZERTY"
        // /// layout. (This value is `KeyCode::KeyW` for the Z key on an AZERTY layout.)
        // ///
        // /// ## Caveats
        // ///
        // /// - Certain niche hardware will shuffle around physical key positions, e.g. a keyboard that
        // /// implements DVORAK in hardware (or firmware)
        // /// - Your application will likely have to handle keyboards which are missing keys that your
        // /// own keyboard has.
        // /// - Certain `KeyCode`s will move between a couple of different positions depending on what
        // /// layout the keyboard was manufactured to support.
        // ///
        // ///  **Because of these caveats, it is important that you provide users with a way to configure
        // ///  most (if not all) keybinds in your application.**
        // ///
        // /// ## `Fn` and `FnLock`
        // ///
        // /// `Fn` and `FnLock` key events are *exceedingly unlikely* to be emitted by Winit. These keys
        // /// are usually handled at the hardware or OS level, and aren't surfaced to applications. If
        // /// you somehow see this in the wild, we'd like to know :)
        // pub physical_key: keyboard::PhysicalKey,

        // // Allowing `broken_intra_doc_links` for `logical_key`, because
        // // `key_without_modifiers` is not available on all platforms
        // #[cfg_attr(
        //     not(any(windows_platform, macos_platform, x11_platform, wayland_platform)),
        //     allow(rustdoc::broken_intra_doc_links)
        // )]
        // /// This value is affected by all modifiers except <kbd>Ctrl</kbd>.
        // ///
        // /// This has two use cases:
        // /// - Allows querying whether the current input is a Dead key.
        // /// - Allows handling key-bindings on platforms which don't
        // /// support [`key_without_modifiers`].
        // ///
        // /// If you use this field (or [`key_without_modifiers`] for that matter) for keyboard
        // /// shortcuts, **it is important that you provide users with a way to configure your
        // /// application's shortcuts so you don't render your application unusable for users with an
        // /// incompatible keyboard layout.**
        // ///
        // /// ## Platform-specific
        // /// - **Web:** Dead keys might be reported as the real key instead
        // /// of `Dead` depending on the browser/OS.
        // ///
        // /// [`key_without_modifiers`]: crate::platform::modifier_supplement::KeyEventExtModifierSupplement::key_without_modifiers
        // pub logical_key: keyboard::Key,

        // /// Contains the text produced by this keypress.
        // ///
        // /// In most cases this is identical to the content
        // /// of the `Character` variant of `logical_key`.
        // /// However, on Windows when a dead key was pressed earlier
        // /// but cannot be combined with the character from this
        // /// keypress, the produced text will consist of two characters:
        // /// the dead-key-character followed by the character resulting
        // /// from this keypress.
        // ///
        // /// An additional difference from `logical_key` is that
        // /// this field stores the text representation of any key
        // /// that has such a representation. For example when
        // /// `logical_key` is `Key::Named(NamedKey::Enter)`, this field is `Some("\r")`.
        // ///
        // /// This is `None` if the current keypress cannot
        // /// be interpreted as text.
        // ///
        // /// See also: `text_with_all_modifiers()`
        // pub text: Option<SmolStr>,

        // /// Contains the location of this key on the keyboard.
        // ///
        // /// Certain keys on the keyboard may appear in more than once place. For example, the "Shift" key
        // /// appears on the left side of the QWERTY keyboard as well as the right side. However, both keys
        // /// have the same symbolic value. Another example of this phenomenon is the "1" key, which appears
        // /// both above the "Q" key and as the "Keypad 1" key.
        // ///
        // /// This field allows the user to differentiate between keys like this that have the same symbolic
        // /// value but different locations on the keyboard.
        // ///
        // /// See the [`KeyLocation`] type for more details.
        // ///
        // /// [`KeyLocation`]: crate::keyboard::KeyLocation
        // pub location: keyboard::KeyLocation,

        // /// Whether the key is being pressed or released.
        // ///
        // /// See the [`ElementState`] type for more details.
        // pub state: ElementState,

        // /// Whether or not this key is a key repeat event.
        // ///
        // /// On some systems, holding down a key for some period of time causes that key to be repeated
        // /// as though it were being pressed and released repeatedly. This field is `true` if and only if
        // /// this event is the result of one of those repeats.
        // pub repeat: bool,

        // /// Platform-specific key event information.
        // ///
        // /// On Windows, Linux and macOS, this type contains the key without modifiers and the text with all
        // /// modifiers applied.
        // ///
        // /// On Android, iOS, Redox and Web, this type is a no-op.
        // pub(crate) platform_specific: platform_impl::KeyEventExtra,
    }
}

pub mod dpi {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct PhysicalSize<P=u32> {
        pub width: P,
        pub height: P,
    }
}

pub mod keyboard {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum NamedKey {
        /// The `Alt` (Alternative) key.
        ///
        /// This key enables the alternate modifier function for interpreting concurrent or subsequent
        /// keyboard input. This key value is also used for the Apple <kbd>Option</kbd> key.
        Alt,
        /// The Alternate Graphics (<kbd>AltGr</kbd> or <kbd>AltGraph</kbd>) key.
        ///
        /// This key is used enable the ISO Level 3 shift modifier (the standard `Shift` key is the
        /// level 2 modifier).
        AltGraph,
        /// The `Caps Lock` (Capital) key.
        ///
        /// Toggle capital character lock function for interpreting subsequent keyboard input event.
        CapsLock,
        /// The `Control` or `Ctrl` key.
        ///
        /// Used to enable control modifier function for interpreting concurrent or subsequent keyboard
        /// input.
        Control,
        /// The Function switch `Fn` key. Activating this key simultaneously with another key changes
        /// that key’s value to an alternate character or function. This key is often handled directly
        /// in the keyboard hardware and does not usually generate key events.
        Fn,
        /// The Function-Lock (`FnLock` or `F-Lock`) key. Activating this key switches the mode of the
        /// keyboard to changes some keys' values to an alternate character or function. This key is
        /// often handled directly in the keyboard hardware and does not usually generate key events.
        FnLock,
        /// The `NumLock` or Number Lock key. Used to toggle numpad mode function for interpreting
        /// subsequent keyboard input.
        NumLock,
        /// Toggle between scrolling and cursor movement modes.
        ScrollLock,
        /// Used to enable shift modifier function for interpreting concurrent or subsequent keyboard
        /// input.
        Shift,
        /// The Symbol modifier key (used on some virtual keyboards).
        Symbol,
        SymbolLock,
        // Legacy modifier key. Also called "Super" in certain places.
        Meta,
        // Legacy modifier key.
        Hyper,
        /// Used to enable "super" modifier function for interpreting concurrent or subsequent keyboard
        /// input. This key value is used for the "Windows Logo" key and the Apple `Command` or `⌘` key.
        ///
        /// Note: In some contexts (e.g. the Web) this is referred to as the "Meta" key.
        Super,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum Key { // <Str = SmolStr> {
        /// A simple (unparameterised) action
        Named(NamedKey),

        // /// A key string that corresponds to the character typed by the user, taking into account the
        // /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
        // /// effect.
        // Character(Str),

        // /// This variant is used when the key cannot be translated to any other variant.
        // ///
        // /// The native key is provided (if available) in order to allow the user to specify keybindings
        // /// for keys which are not defined by this API, mainly through some sort of UI.
        // Unidentified(NativeKey),

        /// Contains the text representation of the dead-key when available.
        ///
        /// ## Platform-specific
        /// - **Web:** Always contains `None`
        Dead(Option<char>),
    }
}
