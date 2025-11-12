// MAIN.RS TEMPLATE BEFORE WMCORE CREATED
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None)?; 
    let screen = &conn.setup().roots[screen_num];

    let win_id = conn.generate_id()?;
    let (width, height) = (1920, 1080); 
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0, 0, // x, y
        width, height,
        0, // border_width
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::SUBSTRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_REDIRECT | EventMask::BUTTON_PRESS | EventMask::KEY_PRESS), // Пример маски событий
    )?;

    conn.map_window(win_id)?; 
    conn.flush()?; 


    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::ConfigureRequest(configure_event) => {
                println!("Configure request for window: {}", configure_event.window);
                // ./wmcore/ 
            }
            Event::ButtonPressEvent(button_event) => {
                if button_event.event == win_id {
                    println!("Mouse button pressed on DE window");
                }
            }
            Event::KeyPress(key_event) => {
                println!("Key pressed: {}", key_event.detail);
            }
            _ => {} 
        }
    }

    Ok(())
}
