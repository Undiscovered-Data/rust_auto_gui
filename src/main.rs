use rustautogui::RustAutoGui;
use std::{thread, time};

fn main() {
    let rustautogui = RustAutoGui::new(false);
    let four_sec = time::Duration::from_millis(4000);
    thread::sleep(four_sec);
    rustautogui.keyboard_input("test", &false);
    rustautogui.keyboard_command("return");
    let short_pause = time::Duration::from_millis(30);
    let uchr = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '>', '?', '~'];
    for a in "!@#$%^&*()_+ next {}| : \" <> ? ".chars() {
        let b = a.to_string();
        if b == "?".to_string() { rustautogui.keyboard_input(&b, &true); }
        else if a.is_ascii_uppercase() { rustautogui.keyboard_input(&b, &true); }
        else if uchr.contains(&a) { rustautogui.keyboard_input(&b, &true); }
        else if a == '<' { rustautogui.keyboard_input(&"-", &false); }  //This is a hack
        else { rustautogui.keyboard_input(&b, &false); } 
        thread::sleep(short_pause);
    }
    rustautogui.keyboard_command("return");
    for c in "-=[] \\ ;' , . / next ~ `".chars() {
        let d = c.to_string();
        if c == '-' { rustautogui.keyboard_input(&"_", &false); }  //This is another hack
        else { rustautogui.keyboard_input(&d, &false); }
        thread::sleep(short_pause);
    }
    //Does not print ~ character
    
}
