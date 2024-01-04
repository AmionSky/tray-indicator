use tray_indicator::{MenuItem, Tray};

fn main() {
    println!("Simple tray-indicator example.");

    let mut tray = Tray::new(0x8189715b3bc5434da3ad10295dca7780, "Simple tray icon");
    tray.set_click(|| println!("The tray icon was clicked."));
    tray.set_menu(vec![
        MenuItem::label("A simple label"),
        MenuItem::separator(),
        MenuItem::button("Action button", || println!("Simple action.")),
        MenuItem::button("Exit", Tray::exit),
    ]);

    tray.display().expect("Failed to display tray icon");
}
