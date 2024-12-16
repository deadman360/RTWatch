use std::{io::ErrorKind, time::Duration};

use ffmpeg::{codec::{profile::H264, traits::Encoder}, decoder::find, Codec};
use ffmpeg_next::codec::{context, traits::Encoder};
use scrap::{Capturer, Display, Frame};

fn main(){
    capture_screen();
}

fn capture_screen(){
    let display: Display = Display::primary().expect("Couldn't find primary display");
    let mut capturer: Capturer = Capturer::new(display).expect("Error creating capturer");
    let width: usize = capturer.width();
    let heigh: usize = capturer.height();

    loop {
        match capturer.frame() {
            Ok(frame) => {

                // comprimir o frame e enviar para o p2p
                println!("Frame capturado  tamanho_frame : {}x{}", width, heigh)
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5));
                continue;
            },
            Err(e) => panic!("Error {}", e),
        
        }
    }
}
fn compress(frame: &Frame){
    
}