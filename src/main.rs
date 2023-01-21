extern crate sdl2;

fn main(){
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();
    

    let mut event_pump = sdl.event_pump().unwrap();
    loop {
        for _event in event_pump.poll_iter(){
            // handle user input here
        }

        // render window contents here
    }
}