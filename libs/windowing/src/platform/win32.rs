mod message;

use std::{
    ffi::{OsString},
    mem,
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
        libloaderapi::{GetModuleHandleW},
        winuser::{
            self, DefWindowProcW, MSG, PeekMessageW, DispatchMessageW,
            CreateWindowExW, ShowWindow, DestroyWindow, RegisterClassW, WNDCLASSW,
            CREATESTRUCTW, SetWindowLongPtrW, GetWindowLongPtrW,
        },
    },
};
use crate::{
    WindowingResult, WindowingError,
    event::{Event},
};

pub struct WindowImpl {
    window_handle: HWND,
    messenger: *mut Messenger,
    receiver: Receiver<Event>,
}
impl WindowImpl {
    pub unsafe fn new() -> WindowingResult<WindowImpl> {
        let (sender, receiver) = mpsc::channel();
        let messenger = Box::into_raw(Box::new(Messenger::new(sender)));
        let window_handle = create_window(messenger)?;
        ShowWindow(window_handle, winuser::SW_SHOWNORMAL);

        Ok(WindowImpl {
            window_handle,
            messenger,
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
            Box::from_raw(self.messenger);
        }
    }
}

struct Messenger {
    sender: Sender<Event>,
}
impl Messenger {
    fn new(sender: Sender<Event>) -> Messenger {
        Messenger { sender }
    }
}

// The window proc that every message will go through
unsafe extern "system" fn window_proc(window_handle: HWND,
message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let messenger: *mut Messenger = {
        if message == winuser::WM_CREATE {
            let create_struct: *const CREATESTRUCTW = mem::transmute(l_param);
            let messenger: *mut Messenger = mem::transmute((*create_struct).lpCreateParams);
            SetWindowLongPtrW(window_handle, winuser::GWLP_USERDATA, messenger as isize);
            messenger
        } else {
            let long_ptr = GetWindowLongPtrW(window_handle, winuser::GWLP_USERDATA);
            mem::transmute(long_ptr)
        }
    };

    if let Some(event) = self::message::convert_message(message, w_param, l_param) {
        if let Err(e) = (*messenger).sender.send(event) {
            println!("Failed to send the event from the messenger. {:?}", e);
        }
    }
    DefWindowProcW(window_handle, message, w_param, l_param)
}

unsafe fn create_window(messenger: *mut Messenger) -> WindowingResult<HWND> {
    let window_class_wchars: Vec<u16> = {
        let window_class_string = OsString::from("BraveWindowingClass");
        window_class_string.encode_wide().collect()
    };
    let window_name_wchars: Vec<u16> = {
        let name = OsString::from("Brave Window");
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
        winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT,
        winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT,
        ptr::null_mut(),       // Parent window
        ptr::null_mut(),       // Menu
        instance_handle,  // Instance handle
        messenger as *mut _,        // Additional application data
    );
    if window_handle.is_null() {
        Err(WindowingError::BadCreation(
            format!("ErrorCode from CreateWindow={:?}", window_handle)
        ))
    } else {
        Ok(window_handle)
    }
}
