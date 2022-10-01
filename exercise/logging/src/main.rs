use frogger::Frog;

fn main() {
    env_logger::init();
    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();
    skippy.sleep();
}
