// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use activitylib::msg;
use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};
use colored::*;

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}

enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // Add code here to spawn a thread to control the light bulb
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;
        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("the light color is {}", "     ".on_truecolor(r,g,b));
                        match light_status {
                            LightStatus::Off => println!("light is OFF"),
                            LightStatus::On => println!("light is ON")
                        }
                    },
                    LightMsg::On => {
                        println!("light is ON");
                        light_status = LightStatus::On
                    },
                    LightMsg::Off => {
                        println!("light is OFF");
                        light_status = LightStatus::Off
                    },
                    LightMsg::Disconnect => {
                        println!("light is Dissconected");
                        light_status = LightStatus::Off
                    }
                }
            } else {
                println!("light is Dissconected");
                light_status = LightStatus::Off            
            }
        }
        light_status
    })
}



fn main() {
    let (s, r) = unbounded();
    let light = spawn_light_thread(r);
    s.send(LightMsg::On);
    s.send(LightMsg::ChangeColor(23, 24, 50));
    s.send(LightMsg::ChangeColor(255, 255, 255));
    s.send(LightMsg::ChangeColor(23, 88, 233));
    s.send(LightMsg::Disconnect);
    s.send(LightMsg::Off);

    let light_status = light.join();

}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
