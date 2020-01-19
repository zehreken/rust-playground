pub fn start_cpal() {
    use cpal::traits::HostTrait;
    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host
        .default_output_device()
        .expect("no output device available");
}
