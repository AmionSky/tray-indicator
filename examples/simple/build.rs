use winscribe::icon::Icon;
use winscribe::manifest::{DpiMode, Feature, Manifest};
use winscribe::ResBuilder;

fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        ResBuilder::new()
            .push(Icon::app("app.ico"))
            .push(Manifest::from([Feature::DpiAware(DpiMode::PerMonitorV2), Feature::ControlsV6]))
            .compile()
            .expect("Failed to include resource!");
    }
}
