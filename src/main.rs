use std::{
    io,
    io::Write,
    sync::{Arc, Mutex},
    thread,
};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleA,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

// mod tests;

fn main() {
    let hook: Arc<Mutex<Option<HHOOK>>> = Arc::new(Mutex::new(None));
    let c_hook = Arc::clone(&hook);

    thread::spawn(move || unsafe {
        *c_hook.lock().unwrap() = Some(SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(ll_keyboard_proc),
            GetModuleHandleA(None),
            0,
        ));

        println!("Hooked!");
        print!("Press enter key to unhook . . . ");
        io::stdout().flush().unwrap();

        let mut msg = MSG::default();
        loop {
            let pm = GetMessageA(&mut msg, HWND(0), 0, 0);
            if pm.0 == 0 {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    });

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Error reading line");

    let r_hook = *hook.lock().unwrap();
    if let Some(hook) = r_hook {
        unsafe {
            UnhookWindowsHookEx(hook);
        }
        if ans.trim_end() == "return" {
            println!("Oh boy, I can't wait for Apple to release M1 Mac!");
        } else {
            println!("Unhooked!");
        }
    } else {
        println!("Ended before hooking, do nothing");
    }
}

unsafe extern "system" fn ll_keyboard_proc(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let kbs = *(lp.0 as *const KBDLLHOOKSTRUCT);
    if kbs.vkCode == VK_E as u32 && wp.0 == WM_KEYDOWN as usize {
        SetCursorPos(960, 540);
    }
    // Q. Why None? Specify Hook!
    // A. I mean it's optional, and static hook isn't option
    CallNextHookEx(None, code, wp, lp)
}
