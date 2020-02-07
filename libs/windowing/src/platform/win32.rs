mod message;

use std::{
    cell::{RefCell},
    collections::{BTreeMap},
    ffi::{OsString},
    os::windows::ffi::{OsStrExt},
    ptr,
    sync::mpsc::{self, Sender, Receiver},
};
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND},
    },
    um::{
        errhandlingapi::{GetLastError},
        libloaderapi::{GetModuleHandleW},
        winuser::{
            self, DefWindowProcW, MSG, PeekMessageW, DispatchMessageW,
            CreateWindowExW, ShowWindow, DestroyWindow, RegisterClassW, WNDCLASSW,
        },
    },
};
use crate::{
    WindowingResult, WindowingError,
    event::{Event},
};

thread_local!(static WINDOW_MESSENGERS: MessengerPool = Default::default());

pub struct WindowImpl {
    window_handle: HWND,
    receiver: Receiver<Event>,
}
impl WindowImpl {
    pub unsafe fn new() -> WindowingResult<WindowImpl> {
        let window_handle = create_window()?;
        ShowWindow(window_handle, winuser::SW_SHOWNORMAL);

        let (sender, receiver) = mpsc::channel();
        WINDOW_MESSENGERS.with(|messengers| messengers.insert(window_handle, sender));

        Ok(WindowImpl {
            window_handle,
            receiver,
        })
    }

    pub unsafe fn fetch_current_events(&mut self) -> impl Iterator<Item = Event> {
        let mut msg = MSG::default();
        while PeekMessageW(&mut msg, self.window_handle, 0, 0, winuser::PM_REMOVE) != 0 {
            // Call our messenger's window proc with this message
            // We don't need to translate the message first and that will just slow us down
            DispatchMessageW(&msg);
        }

        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        events.into_iter()
    }
}
impl Drop for WindowImpl {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.window_handle);
        }
        WINDOW_MESSENGERS.with(|messengers| messengers.remove(self.window_handle));
    }
}

#[derive(Default)]
struct MessengerPool {
    handles: RefCell<BTreeMap<HWND, Sender<Event>>>,
}
impl MessengerPool {
    fn insert(&self, window_handle: HWND, sender: Sender<Event>) {
        let mut handles = self.handles.borrow_mut();
        handles.insert(window_handle, sender);
    }

    fn get(&self, window_handle: HWND) -> Option<Sender<Event>> {
        let handles = self.handles.borrow_mut();
        handles.get(&window_handle).map(|sender| sender.clone())
    }

    fn remove(&self, window_handle: HWND) {
        let mut handles = self.handles.borrow_mut();
        handles.remove(&window_handle);
    }
}

// The window proc that every message will go through
unsafe extern "system" fn window_proc(window_handle: HWND,
message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // It will probably be more common to ignore a message than send an event
    //  So check the message first before accessing the mutex (sys call)
    if let Some(event) = self::message::convert_message(message, w_param, l_param) {
        WINDOW_MESSENGERS.with(|messengers| {
            if let Some(sender) = messengers.get(window_handle) {
                if let Err(e) = sender.send(event) {
                    // TODO Log this
                    println!("Failed to send the event from the messenger. {:?}", e);
                }
            }
        });
    }

    DefWindowProcW(window_handle, message, w_param, l_param)
}

unsafe fn create_window() -> WindowingResult<HWND> {
    let window_class_wchars: Vec<u16> = {
        let window_class_string = OsString::from("BraveWindowingClass\0");
        window_class_string.encode_wide().collect()
    };
    let window_name_wchars: Vec<u16> = {
        let name = OsString::from("Brave Window\0");
        name.encode_wide().collect()
    };

    let instance_handle = GetModuleHandleW(ptr::null());

    let mut window_class = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_proc);
    window_class.hInstance = instance_handle;
    window_class.lpszClassName = window_class_wchars.as_ptr();
    RegisterClassW(&window_class);

    let window_handle = CreateWindowExW(0,
        window_class_wchars.as_ptr(), window_name_wchars.as_ptr(),
        winuser::WS_OVERLAPPEDWINDOW,
        // Size and position
        10, 10, 400, 400,
        ptr::null_mut(),       // Parent window
        ptr::null_mut(),       // Menu
        instance_handle,
        ptr::null_mut(),       // Additional application data
    );
    if window_handle.is_null() {
        Err(WindowingError::BadCreation(
            format!("Return={:?}. ErrorCode from CreateWindow={:?}", window_handle, GetLastError())
        ))
    } else {
        Ok(window_handle)
    }
}
