mod args;
mod sensor;

use actix_web::{web, App, HttpServer, Responder};
use bytes::Bytes;
use lazy_static::lazy_static;
use prometheus::{register_gauge, Encoder, Gauge, Opts, TextEncoder};

lazy_static! {
    static ref TEMPERATURE_GAUGE: Gauge =
        register_gauge!(Opts::new("temperature", "The temperature from sensor")
            .namespace("home_system")
            .subsystem("room_node")
            .const_label("location", &args::ARGS.location))
        .unwrap();
    static ref PRESSURE_GAUGE: Gauge =
        register_gauge!(Opts::new("pressure", "The pressure from sensor")
            .namespace("home_system")
            .subsystem("room_node")
            .const_label("location", &args::ARGS.location))
        .unwrap();
}

fn get_metrics() -> impl Responder {
    let sensor_result = sensor::sensor_read();
    TEMPERATURE_GAUGE.set(sensor_result.temperature);
    PRESSURE_GAUGE.set(sensor_result.pressure);

    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Bytes::from(buffer)
}

fn main() -> std::io::Result<()> {
    // Read sensor eagerly to make sure i2c dev path is correct
    let _ = sensor::sensor_read();

    HttpServer::new(|| App::new().service(web::resource("metrics").to(get_metrics)))
        .bind(("0.0.0.0", args::ARGS.port))?
        .run()
}
