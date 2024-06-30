use tracing_subscriber::FmtSubscriber;




pub fn init(){
    let subscriber = FmtSubscriber::builder()
    .without_time()
    .with_max_level(tracing::Level::TRACE)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
}