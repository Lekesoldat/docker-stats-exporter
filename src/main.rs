mod error;
mod convert_to_bytes;
mod docker;

use prometheus::{Registry, Gauge, TextEncoder, Encoder};
use prometheus::core::{AtomicF64, GenericGauge};
use anyhow::{Result, anyhow};
use axum::{routing::{get}, Router};
use serde::{Deserialize, Serialize};
use crate::convert_to_bytes::convert_to_bytes;
use crate::docker::DockerContainerStats;
use crate::error::ApiResult;

fn percent_gauge(name: String, mut percent_string: String, help: String) -> Result<GenericGauge<AtomicF64>> {
    percent_string.pop();
    let value: f64 = percent_string.parse()?;
    get_gauge(name, help, value)
}

fn get_gauge(name: String, help: String, value: f64) -> Result<GenericGauge<AtomicF64>> {
    let gauge = Gauge::new(name.replace("-", "_"), help)?;
    gauge.set(value);
    Ok(gauge)
}

fn parse_io_str(str: String) -> Result<f64> {
    let backwards_unit = str.chars().rev().take_while(|c| c.is_alphabetic()).collect::<String>();
    let unit = backwards_unit.chars().rev().collect::<String>();
    let index = str.len() - unit.len();
    let value = &str[0..index];
    let float_value = value.parse::<f64>()?;
    let result = convert_to_bytes(float_value, unit)?;
    Ok(result)
}

fn parse_netio_str(netio_string: &str) -> Result<(f64, f64)> {
    let mut input_output: Vec<&str> = netio_string.split(" / ").collect();
    let (Some(output), Some(input)) = (input_output.pop(), input_output.pop()) else {
        return Err(anyhow!("Bad netio string: '{}'", netio_string));
    };

    let inp = parse_io_str(input.to_string())?;
    let out = parse_io_str(output.to_string())?;

    Ok((inp, out))
}

fn gauges_for_container(stat: &DockerContainerStats) -> Result<Vec<GenericGauge<AtomicF64>>> {
    let cpu_gauge = percent_gauge(format!("{}_cpu_usage", stat.container), stat.cpuPerc.clone(), format!("CPU Usage for the '{}' container", stat.container))?;
    let mem_gauge = percent_gauge(format!("{}_mem_usage", stat.container), stat.memPerc.clone(), format!("MEM Usage for the '{}' container", stat.container))?;
    let (input, output) = parse_netio_str(stat.netIO.as_str())?;
    let net_input_gauge = get_gauge(format!("{}_network_input_bytes", stat.container), format!("Help"), input)?;
    let net_output_gauge = get_gauge(format!("{}_network_output_bytes", stat.container), format!("Help"), output)?;

    Ok(vec![cpu_gauge, mem_gauge, net_input_gauge, net_output_gauge])
}

fn get_prometheus_format(stats: Vec<DockerContainerStats>) -> Result<String> {
    let registry = Registry::new();
    for container_stats in &stats {
        for gauge in gauges_for_container(container_stats)? {
            registry.register(Box::new(gauge))?;
        }
    }

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer)?;

    let str = String::from_utf8(buffer)?;
    Ok(str)
}

async fn docker_stats_metrics() -> ApiResult<String> {
    let stats = docker::stats()?;
    let prometheus_stuff = get_prometheus_format(stats)?;
    Ok(prometheus_stuff)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/docker-stats/metrics", get(docker_stats_metrics));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3069").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
