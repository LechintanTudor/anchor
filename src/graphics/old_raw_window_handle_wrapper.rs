use old_raw_window_handle as old;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct OldRawWindowHandleWrapper<W>(pub W);

unsafe impl<W> old::HasRawWindowHandle for OldRawWindowHandleWrapper<W>
where
    W: HasRawDisplayHandle + HasRawWindowHandle,
{
    fn raw_window_handle(&self) -> old::RawWindowHandle {
        use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

        match (self.0.raw_window_handle(), self.0.raw_display_handle()) {
            (RawWindowHandle::UiKit(window), RawDisplayHandle::UiKit(_)) => {
                let mut handle = old::UiKitHandle::empty();
                handle.ui_window = window.ui_window;
                handle.ui_view = window.ui_view;
                handle.ui_view_controller = window.ui_view_controller;
                old::RawWindowHandle::UiKit(handle)
            }
            (RawWindowHandle::AppKit(window), RawDisplayHandle::AppKit(_)) => {
                let mut handle = old::AppKitHandle::empty();
                handle.ns_window = window.ns_window;
                handle.ns_view = window.ns_view;
                old::RawWindowHandle::AppKit(handle)
            }
            (RawWindowHandle::Orbital(window), RawDisplayHandle::Orbital(_)) => {
                let mut handle = old::OrbitalHandle::empty();
                handle.window = window.window;
                old::RawWindowHandle::Orbital(handle)
            }
            (RawWindowHandle::Xlib(window), RawDisplayHandle::Xlib(display)) => {
                let mut handle = old::XlibHandle::empty();
                handle.window = window.window;
                handle.display = display.display;
                handle.visual_id = window.visual_id;
                old::RawWindowHandle::Xlib(handle)
            }
            (RawWindowHandle::Xcb(window), RawDisplayHandle::Xcb(display)) => {
                let mut handle = old::XcbHandle::empty();
                handle.window = window.window;
                handle.connection = display.connection;
                handle.visual_id = window.visual_id;
                old::RawWindowHandle::Xcb(handle)
            }
            (RawWindowHandle::Wayland(window), RawDisplayHandle::Wayland(display)) => {
                let mut handle = old::WaylandHandle::empty();
                handle.surface = window.surface;
                handle.display = display.display;
                old::RawWindowHandle::Wayland(handle)
            }
            (RawWindowHandle::Win32(window), RawDisplayHandle::Windows(_)) => {
                let mut handle = old::Win32Handle::empty();
                handle.hwnd = window.hwnd;
                handle.hinstance = window.hinstance;
                old::RawWindowHandle::Win32(handle)
            }
            (RawWindowHandle::WinRt(window), RawDisplayHandle::Windows(_)) => {
                let mut handle = old::WinRtHandle::empty();
                handle.core_window = window.core_window;
                old::RawWindowHandle::WinRt(handle)
            }
            (RawWindowHandle::Web(window), RawDisplayHandle::Web(_)) => {
                let mut handle = old::WebHandle::empty();
                handle.id = window.id;
                old::RawWindowHandle::Web(handle)
            }
            (RawWindowHandle::AndroidNdk(window), RawDisplayHandle::Android(_)) => {
                let mut handle = old::AndroidNdkHandle::empty();
                handle.a_native_window = window.a_native_window;
                old::RawWindowHandle::AndroidNdk(handle)
            }
            (RawWindowHandle::Haiku(window), RawDisplayHandle::Haiku(_)) => {
                let mut handle = old::HaikuHandle::empty();
                handle.b_window = window.b_window;
                handle.b_direct_window = window.b_direct_window;
                old::RawWindowHandle::Haiku(handle)
            }
            _ => panic!("Unsupported"),
        }
    }
}
