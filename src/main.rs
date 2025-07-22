//! # Demo Rust OpenTelemetry stdout
//! 
//! Demonstration of:
//! 
//! - the Rust programming language
//! - the OpenTelemetry observability framework
//! - the OpenTelemetry SDK
//! - the OpenTelemetry exporter for standard output
//! - the OpenTelemetry appender for the tracing crate
//! 
//! The `init_tracer_provider`, `init_meter_provider`, and `init_logger_provider` functions are called to
//! initialize the respective components.
//!
//! Each component is configured with a default exporter that outputs to standard output.
//! 
//! The `global` module is used to set the meter provider for the application.

/// The opentelemetry::global module in OpenTelemetry provides functions for
/// managing global instances of a tracer provider and meter provider.
/// 
/// These enable you to set and retrieve these providers, so you can
/// have consistent telemetry configuration throughout your application. 
///
use opentelemetry::global;

use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::Resource;

/// The tracing_subscriber::prelude::* provides convenient extensions and traits
/// that simplify the configuration and usage of tracing_subscriber for building
/// and customizing tracing instrumentation.
///  
/// For example, it allows you to use the with() method to add layers to a
/// subscriber, or to configure layers with methods like with_target() and
/// with_level() in tracing_subscriber::fmt::Layer. 
///
use tracing_subscriber::prelude::*;

/// Create a static resource that will be used for all telemetry data.
/// 
/// The `RESOURCE` provides metadata about the service that is generating telemetry data.
/// This resource includes the service name and can be extended with additional attributes.
/// This is useful for identifying the source of telemetry data in a distributed system.
/// The `LazyLock` ensures that the resource is initialized only once and is thread-safe.
/// 
static RESOURCE: std::sync::LazyLock<Resource> = std::sync::LazyLock::new(|| {
    Resource::builder()
        .with_service_name("demo-rust-opentelemetry-stdout")
        .build()
});


/// Initialize OpenTelemetry tracer provider. 
/// 
/// This uses the OpenTelemetry SDK and OpenTelemetry exporter for stdout.
/// 
/// This processor directly exports data to the configured exporter whenever a
/// span is completed. This happens because of `with_simple_exporter`.
/// 
/// This function also sets the global tracer provider.
/// 
fn init_tracer_provider() -> opentelemetry_sdk::trace::SdkTracerProvider {
    let exporter = opentelemetry_stdout::SpanExporter::default();
    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(exporter)
        .with_resource(RESOURCE.clone())
        .build();
    global::set_tracer_provider(provider.clone());
    provider
}

/// Initialize OpenTelemetry meter provider.
/// 
/// This uses the OpenTelemetry SDK and OpenTelemetry exporter for stdout.
/// 
/// This processor collects metrics based on a time interval and then sends them
/// to the configured exporter.  This happens because of `with_periodic exporter`.
/// 
/// This function also sets the global meter provider.
/// 
fn init_meter_provider() -> opentelemetry_sdk::metrics::SdkMeterProvider {
    let exporter = opentelemetry_stdout::MetricExporter::default();
    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(RESOURCE.clone())
        .build();
    global::set_meter_provider(provider.clone());
    provider
}

/// Initialize OpenTelemetry logger provider.
/// 
/// This uses the OpenTelemetry SDK and OpenTelemetry exporter for stdout.
/// 
/// This function creates a bridge from the tracing subscriber to the
/// OpenTelemetry export, by registering the bridge with tracing_subscriber.
/// 
fn init_logger_provider() -> opentelemetry_sdk::logs::SdkLoggerProvider {
    use opentelemetry_appender_tracing::layer;

    let exporter = opentelemetry_stdout::LogExporter::default();
    let provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_simple_exporter(exporter)
        .with_resource(RESOURCE.clone())
        .build();
    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();
    provider
}

/// Example of how to emit a log entry.
/// 
/// Output looks like:
/// 
/// ```stdout
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "function-emit-log-my-name"
///    Target (Scope): "function-emit-log-my-target"
///    Observed Timestamp: 2025-07-22 07:31:01.332332
///    SeverityText: "ERROR"
///    SeverityNumber: Error
///    Attributes:
///      ->  event_id: Int(20)
///      ->  user_name: String(Owned("function-emit-log-my-target-user-name"))
///      ->  user_email: String(Owned("function-emit-log-my-user-email"))
/// ```
/// 
fn emit_log() {
    use tracing::error;
    error!(
        name: "function-emit-log-my-name", 
        target: "function-emit-log-my-target", 
        event_id = 20, 
        user_name = "function-emit-log-my-target-user-name", // e.g. "otel"
        user_email = "function-emit-log-my-user-email" // e.g. "otel@opentelemetry.io"
    );
}

/// Example of how to emit a span.
/// 
/// Output of the deliberate span error looks like:
/// 
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "function-emit-span-my-name"
///    Target (Scope): "function-emit-span-my-target"
///    TraceId: 9539f563bbf57a7abe51081ec0b47592
///    SpanId: 3304d306a2c81b88
///    TraceFlags: TraceFlags(1)
///    Observed Timestamp: 2025-07-22 07:31:01.332568
///    SeverityText: "ERROR"
///    SeverityNumber: Error
///    Attributes:
///      ->  event_id: Int(20)
///      ->  user_name: String(Owned("function-emit-span-my-target-user-name"))
///      ->  user_email: String(Owned("function-emit-span-my-user-email"))
/// ```
/// 
/// Output of the span resource looks like:
/// 
/// ```stdout
/// Spans
/// Resource
///    ->  telemetry.sdk.name=String(Static("opentelemetry"))
///    ->  telemetry.sdk.language=String(Static("rust"))
///    ->  service.name=String(Static("demo-rust-opentelemetry-stdout"))
///    ->  telemetry.sdk.version=String(Static("0.30.0"))
/// ```
/// 
/// Output of the span instrumentation looks like:
/// 
/// ```stdout
/// Span #0
///   Instrumentation Scope
///     Name         : "stdout-example"
///     Version  : "v1"
///     Scope Attributes:
///        ->  scope_key: scope_value
/// 
///   Name        : example-span
///   TraceId     : aa547ec795748c1b04869219b6defa31
///   SpanId      : 2cf3c5dc13e2eef5
///   TraceFlags  : TraceFlags(1)
///   ParentSpanId: 0000000000000000
///   Kind        : Internal
///   Start time: 2025-07-22 06:55:12.957518
///   End time: 2025-07-22 06:55:12.957560
///   Status: Unset
///   Attributes:
///      ->  my-attribute: String(Static("my-value"))
///   Events:
///   Event #0
///   Name      : example-event-name
///   Timestamp : 2025-07-22 06:55:12.957526
///   Attributes:
///      ->  event_attribute1: String(Static("event_value1"))
/// ```
/// 
fn emit_span() {
    use opentelemetry::{trace::{Tracer, TraceContextExt}, InstrumentationScope};
    use opentelemetry::KeyValue;

    let scope = InstrumentationScope::builder("stdout-example")
        .with_version("v1")
        .with_attributes([
            KeyValue::new("scope_key", "scope_value")])
        .build();

    let tracer = global::tracer_with_scope(scope);
    tracer.in_span("example-span", |cx| {
        let span = cx.span();
        span.set_attribute(KeyValue::new("my-attribute", "my-value"));
        span.add_event(
            "example-event-name",
            vec![KeyValue::new("event_attribute1", "event_value1")],
        );
        use tracing::error;
        error!(
            name: "function-emit-span-my-name", 
            target: "function-emit-span-my-target", 
            event_id = 20, 
            user_name = "function-emit-span-my-target-user-name", // e.g. "otel"
            user_email = "function-emit-span-my-user-email" // e.g. "otel@opentelemetry.io"
        );
    })
}

/// Example of how to emit metrics.
/// 
/// This function creates a meter named 'function-emit-metrics-meter'. The meter
/// will combine a counter and a histogram.
/// 
/// Create the meter, then the output looks like:
/// 
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "MeterProvider.NewMeterCreated"
///    Target (Scope): "opentelemetry_sdk"
///    Observed Timestamp: 2025-07-22 06:55:12.957691
///    SeverityText: "DEBUG"
///    SeverityNumber: Debug
///    Body: String(Owned(""))
///    Attributes:
///      ->  name: String(Owned("MeterProvider.NewMeterCreated"))
///      ->  meter_name: String(Owned("function-emit-metrics"))
/// ```
/// 
/// Create a metric instrument named 'function-emit-metrics-counter' that is a
/// counter, then the output looks like:
/// 
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "Metrics.InstrumentCreated"
///    Target (Scope): "opentelemetry_sdk"
///    Observed Timestamp: 2025-07-22 06:55:12.957862
///    SeverityText: "DEBUG"
///    SeverityNumber: Debug
///    Body: String(Owned(""))
///    Attributes:
///      ->  name: String(Owned("Metrics.InstrumentCreated"))
///      ->  instrument_name: String(Owned("function-emit-metrics-counter"))
///      ->  cardinality_limit: Int(2000)
/// ```
/// 
/// Create a metric instrument named 'function-emit-metrics-histogram' that is a
/// histogram, then the output looks like:
/// 
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "Metrics.InstrumentCreated"
///    Target (Scope): "opentelemetry_sdk"
///    Observed Timestamp: 2025-07-22 06:55:12.958017
///    SeverityText: "DEBUG"
///    SeverityNumber: Debug
///    Body: String(Owned(""))
///    Attributes:
///      ->  name: String(Owned("Metrics.InstrumentCreated"))
///      ->  instrument_name: String(Owned("function-emit-metrics-histogram"))
///      ->  cardinality_limit: Int(2000)
/// ```
/// 
/// The meter output looks like:
/// 
/// ```stdout
/// Metrics
/// Resource
///    ->  telemetry.sdk.language=String(Static("rust"))
///    ->  telemetry.sdk.name=String(Static("opentelemetry"))
///    ->  telemetry.sdk.version=String(Static("0.30.0"))
///    ->  service.name=String(Static("demo-rust-opentelemetry-stdout"))
///   Instrumentation Scope #0
///     Name         : function-emit-metrics-meter
/// ```
/// 
/// The counter output looks like:
/// 
/// ```stdout
/// Metric #0
///     Name         : function-emit-metrics-counter
///     Description  : 
///     Unit         : 
///     Type         : Sum
///     Sum DataPoints
///     Monotonic    : true
///     Temporality  : Cumulative
///     StartTime    : 2025-07-22 07:07:50.808288
///     EndTime      : 2025-07-22 07:07:50.808645
///     DataPoint #0
///       Value        : 2
///       Attributes   :
///          ->  name: apple
///          ->  color: green
///     DataPoint #1
///       Value        : 12
///       Attributes   :
///          ->  color: yellow
///          ->  name: banana
///     DataPoint #2
///       Value        : 2
///       Attributes   :
///          ->  name: apple
///          ->  color: red
/// ```
/// 
/// The histogram output looks like:
/// 
/// ```stdout
/// Metric #1
///     Name         : function-emit-metrics-histogram
///     Description  : 
///     Unit         : 
///     Type         : Histogram
///     Temporality  : Cumulative
///     StartTime    : 2025-07-22 07:07:50.808475
///     EndTime      : 2025-07-22 07:07:50.808712
///     Histogram DataPoints
///     DataPoint #0
///       Count        : 2
///       Sum          : 12.0
///       Min          : 1.0
///       Max          : 11.0
///       Attributes   :
///          ->  name: banana
///          ->  color: yellow
///       Buckets
///          -inf to 0 : 0
///          0 to 5 : 1
///          5 to 10 : 0
///          10 to 25 : 1
///          …
///     DataPoint #1
///       Count        : 1
///       Sum          : 2.0
///       Min          : 2.0
///       Max          : 2.0
///       Attributes   :
///          ->  name: apple
///          ->  color: red
///       Buckets
///          -inf to 0 : 0
///          0 to 5 : 1
///          5 to 10 : 0
///          …
///     DataPoint #2
///       Count        : 2
///       Sum          : 2.0
///       Min          : 1.0
///       Max          : 1.0
///       Attributes   :
///          ->  name: apple
///          ->  color: green
///       Buckets
///          -inf to 0 : 0
///          0 to 5 : 2
///          5 to 10 : 0
///          …
/// ```
/// 
fn emit_metrics() {
    use opentelemetry::KeyValue;
    let meter = global::meter("function-emit-metrics-meter");
    let counter = meter.u64_counter("function-emit-metrics-counter").build();
    counter.add(
        1,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "green"),
        ],
    );
    counter.add(
        1,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "green"),
        ],
    );
    counter.add(
        2,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "red"),
        ],
    );
    counter.add(
        1,
        &[
            KeyValue::new("name", "banana"),
            KeyValue::new("color", "yellow"),
        ],
    );
    counter.add(
        11,
        &[
            KeyValue::new("name", "banana"),
            KeyValue::new("color", "yellow"),
        ],
    );

    let histogram = meter.f64_histogram("function-emit-metrics-histogram").build();
    histogram.record(
        1.0,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "green"),
        ],
    );
    histogram.record(
        1.0,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "green"),
        ],
    );
    histogram.record(
        2.0,
        &[
            KeyValue::new("name", "apple"),
            KeyValue::new("color", "red"),
        ],
    );
    histogram.record(
        1.0,
        &[
            KeyValue::new("name", "banana"),
            KeyValue::new("color", "yellow"),
        ],
    );
    histogram.record(
        11.0,
        &[
            KeyValue::new("name", "banana"),
            KeyValue::new("color", "yellow"),
        ],
    );
}

/// Demonstrate OpenTelemetry and how to emit a log, a span, and some metrics. 
/// 
/// This main function does three things:
/// 
/// - Initialize the OpenTelemetry providers.
/// - Emit an example log, an example span, and an example set of metrics.
/// - Shut down the OpenTelemetry providers.
/// 
/// ### main shutdown
///
/// The logger provider shutdown output looks like:
///
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "LoggerProvider.ShutdownInvokedByUser"
///    Target (Scope): "opentelemetry_sdk"
///    Observed Timestamp: 2025-07-22 06:55:12.958451
///    SeverityText: "DEBUG"
///    SeverityNumber: Debug
///    Body: String(Owned(""))
///    Attributes:
///      ->  name: String(Owned("LoggerProvider.ShutdownInvokedByUser"))
/// ```
///
/// The meter provider shutdown output looks like:
///
/// ```stdout
/// Logs
/// Log #0
///    Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
///    EventName: "MeterProvider.Shutdown"
///    Target (Scope): "opentelemetry_sdk"
///    Observed Timestamp: 2025-07-22 06:55:12.958088
///    SeverityText: "DEBUG"
///    SeverityNumber: Debug
///    Body: String(Owned("User initiated shutdown of MeterProvider."))
///    Attributes:
///      ->  name: String(Owned("MeterProvider.Shutdown"))
// ```
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Initialize the providers.
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meter_provider();
    let logger_provider = init_logger_provider();

    // // Emit examples.
    emit_log();
    emit_span();
    emit_metrics();

    // Shut down the providers.
    tracer_provider.shutdown()?;
    meter_provider.shutdown()?;
    logger_provider.shutdown()?;

    // Done.
    Ok(())
}
