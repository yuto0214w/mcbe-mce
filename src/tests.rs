use windows::Win32::{
  Foundation::PSTR,
  UI::WindowsAndMessaging::{FindWindowA, GetClassNameA},
};

#[allow(dead_code)]
fn class_name() {
  unsafe {
    let mut app_name = "Minecraft".to_string();
    let hwnd = FindWindowA(None, PSTR(app_name.as_mut_ptr()));
    let mut buf = Vec::with_capacity(80);
    let class_name = PSTR(buf.as_mut_ptr());
    let length = GetClassNameA(hwnd, class_name, 80);
    buf.set_len(length as usize);
    println!("{:?} / {}", hwnd, String::from_utf8(buf).unwrap());
  }
}
