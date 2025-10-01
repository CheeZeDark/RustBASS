#![allow(non_snake_case)]

use std::{thread::sleep, time, ffi::CString};
use std::ptr;

#[link(name = "bass")]
extern "C" {
    fn BASS_Init(device: i32, freq: i32, flags: i32, win: *const u8, clsid: *const u8) -> bool;
    fn BASS_Start() -> bool;
    fn BASS_Stop() -> bool;
    fn BASS_StreamCreateFile(mem: bool, f: *const i8, offset1: u64, offset2: u64, length1: u32, length2: u32, flags: u32) -> i32;
    fn BASS_ChannelPlay(handle: i32, restart: bool);
    fn BASS_ErrorGetCode() -> i32;
}

fn main() {
    unsafe {
        // Initialize BASS
        if !BASS_Init(-1, 44100, 0, ptr::null(), ptr::null()) {
            println!("Failed to initialize BASS. Error code: {}", BASS_ErrorGetCode());
            return;
        }
        
        if !BASS_Start() {
            println!("Failed to start BASS. Error code: {}", BASS_ErrorGetCode());
            return;
        }
        
        // Convert filename to C string
        let filename = match CString::new("MumeiVirus.mp3") {
            Ok(f) => f,
            Err(e) => {
                println!("Filename error: {}", e);
                return;
            }
        };
        
        // Create stream
        let stream = BASS_StreamCreateFile(
            false, 
            filename.as_ptr(), 
            0, 0, 0, 0, 
            0x4
        );
        
        if stream == 0 {
            println!("Failed to create stream. Error code: {}", BASS_ErrorGetCode());
            return;
        }
        
        println!("BASS IS STARTED!!!");
        
        // Play the stream
        BASS_ChannelPlay(stream, false);
        
        // Keep the program running while the music plays
        loop {
            sleep(time::Duration::from_millis(100));
        }
    }
}
