use macroquad::prelude::*;
use std::process::Command;
use ::rand::{Rng, thread_rng};
use std::env;

fn play_video(media_path: &str) -> bool {
    let vlc_path = "vlc";
    
    let status = Command::new(vlc_path)
        .arg(media_path)
        .spawn()
        .expect("Failed to start VLC player")
        .wait()
        .expect("Failed to wait on VLC");

    let success = status.success();
    if success {
        println!("VLC played the video successfully.");
    } else {
        println!("VLC did not play the video successfully.");
    }
    success
}

// Need to change main to return Result from macroquad
#[macroquad::main("My Window")]
async fn main() {
    // Get command line args or use random number as fallback
    let random_number: u32 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| {
            let mut rng = ::rand::thread_rng();
            rng.gen_range(1..10)
        });
    
    // Main game loop
    loop {
        clear_background(WHITE);
        
        // Draw text with the random number
        draw_text(
            &format!("Number Of Inches: {}", random_number),
            20.0,
            40.0,
            30.0,
            BLACK
        );
        
        // Your existing logic can go here, but you'll need to trigger it with a condition
        // For example, you might want to play the videos when a key is pressed
        if is_key_pressed(KeyCode::Space) {
            if random_number >= 5 && random_number <= 7 {
                print!("long penis");
                play_video("assets/video1.mkv");
            } else if random_number < 5 {
                println!("small penis");
                play_video("assets/video2.mkv");
            } else if random_number > 7 {
                println!("you won the lottery");
                play_video("assets/video3.mkv");
            }
            std::process::exit(0); // Exit the program after playing the video
        }

        next_frame().await;
    }

	







}


