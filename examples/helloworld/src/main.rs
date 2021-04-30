use microbio;

fn main() {
    let service = microbio::service::new_service();

    service.init();

    match service.run() {
        Ok(_) => (),
        Err(_) => panic!("Service run failed"),
    };
}
