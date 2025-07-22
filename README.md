# Demo Rust OpenTelemetry stdout

Demonstration of:

- the Rust programming language
- the OpenTelemetry observability framework
- the OpenTelemetry SDK
- the OpenTelemetry exporter for standard output
- the OpenTelemetry appender for the tracing crate

The `init_tracer_provider`, `init_meter_provider`, and `init_logger_provider` functions are called to
initialize the respective components.

Each component is configured with a default exporter that outputs to standard output.

The `global` module is used to set the meter provider for the application.

## Usage

Run:

```sh
cargo run
```

## Output

The output has a bunch of sections.

Output looks like:

```stdout
Logs
Resource
   ->  telemetry.sdk.name=String(Static("opentelemetry"))
   ->  telemetry.sdk.language=String(Static("rust"))
   ->  service.name=String(Static("demo-rust-opentelemetry-stdout"))
   ->  telemetry.sdk.version=String(Static("0.30.0"))
```

### emit_log

In the code, see the demo function `emit_log`.

Output looks like:

```stdout
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "function-emit-log-my-name"
   Target (Scope): "function-emit-log-my-target"
   Observed Timestamp: 2025-07-22 07:31:01.332332
   SeverityText: "ERROR"
   SeverityNumber: Error
   Attributes:
     ->  event_id: Int(20)
     ->  user_name: String(Owned("function-emit-log-my-target-user-name"))
     ->  user_email: String(Owned("function-emit-log-my-user-email"))
```

### emit_span

In the code, see the demo function `emit_span`.

Output of the deliberate span error looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "function-emit-span-my-name"
   Target (Scope): "function-emit-span-my-target"
   TraceId: 9539f563bbf57a7abe51081ec0b47592
   SpanId: 3304d306a2c81b88
   TraceFlags: TraceFlags(1)
   Observed Timestamp: 2025-07-22 07:31:01.332568
   SeverityText: "ERROR"
   SeverityNumber: Error
   Attributes:
     ->  event_id: Int(20)
     ->  user_name: String(Owned("function-emit-span-my-target-user-name"))
     ->  user_email: String(Owned("function-emit-span-my-user-email"))
```

Output of the span resource looks like:

```stdout
Spans
Resource
   ->  telemetry.sdk.name=String(Static("opentelemetry"))
   ->  telemetry.sdk.language=String(Static("rust"))
   ->  service.name=String(Static("demo-rust-opentelemetry-stdout"))
   ->  telemetry.sdk.version=String(Static("0.30.0"))
```

Output of the span instrumentation looks like:

```stdout
Span #0
  Instrumentation Scope
    Name         : "stdout-example"
    Version  : "v1"
    Scope Attributes:
       ->  scope_key: scope_value

  Name        : example-span
  TraceId     : aa547ec795748c1b04869219b6defa31
  SpanId      : 2cf3c5dc13e2eef5
  TraceFlags  : TraceFlags(1)
  ParentSpanId: 0000000000000000
  Kind        : Internal
  Start time: 2025-07-22 06:55:12.957518
  End time: 2025-07-22 06:55:12.957560
  Status: Unset
  Attributes:
     ->  my-attribute: String(Static("my-value"))
  Events:
  Event #0
  Name      : example-event-name
  Timestamp : 2025-07-22 06:55:12.957526
  Attributes:
     ->  event_attribute1: String(Static("event_value1"))
```

### emit_metrics

In the code, see the demonstration function `emit_metrics`.

The function creates a meter named 'function-emit-metrics-meter' that will combine a counter and a histogram.

Create the meter, then the output looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "MeterProvider.NewMeterCreated"
   Target (Scope): "opentelemetry_sdk"
   Observed Timestamp: 2025-07-22 06:55:12.957691
   SeverityText: "DEBUG"
   SeverityNumber: Debug
   Body: String(Owned(""))
   Attributes:
     ->  name: String(Owned("MeterProvider.NewMeterCreated"))
     ->  meter_name: String(Owned("function-emit-metrics"))
```

Create a metric instrument named 'function-emit-metrics-counter' that is a
counter, then the output looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "Metrics.InstrumentCreated"
   Target (Scope): "opentelemetry_sdk"
   Observed Timestamp: 2025-07-22 06:55:12.957862
   SeverityText: "DEBUG"
   SeverityNumber: Debug
   Body: String(Owned(""))
   Attributes:
     ->  name: String(Owned("Metrics.InstrumentCreated"))
     ->  instrument_name: String(Owned("function-emit-metrics-counter"))
     ->  cardinality_limit: Int(2000)
```

Create a metric instrument named 'function-emit-metrics-histogram' that is a
histogram, then the output looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "Metrics.InstrumentCreated"
   Target (Scope): "opentelemetry_sdk"
   Observed Timestamp: 2025-07-22 06:55:12.958017
   SeverityText: "DEBUG"
   SeverityNumber: Debug
   Body: String(Owned(""))
   Attributes:
     ->  name: String(Owned("Metrics.InstrumentCreated"))
     ->  instrument_name: String(Owned("function-emit-metrics-histogram"))
     ->  cardinality_limit: Int(2000)
```

The meter output looks like:

```stdout
Metrics
Resource
   ->  telemetry.sdk.language=String(Static("rust"))
   ->  telemetry.sdk.name=String(Static("opentelemetry"))
   ->  telemetry.sdk.version=String(Static("0.30.0"))
   ->  service.name=String(Static("demo-rust-opentelemetry-stdout"))
  Instrumentation Scope #0
    Name         : function-emit-metrics-meter
```

The counter output looks like:

```stdout
Metric #0
    Name         : function-emit-metrics-counter
    Description  : 
    Unit         : 
    Type         : Sum
    Sum DataPoints
    Monotonic    : true
    Temporality  : Cumulative
    StartTime    : 2025-07-22 07:07:50.808288
    EndTime      : 2025-07-22 07:07:50.808645
    DataPoint #0
      Value        : 2
      Attributes   :
         ->  name: apple
         ->  color: green
    DataPoint #1
      Value        : 12
      Attributes   :
         ->  color: yellow
         ->  name: banana
    DataPoint #2
      Value        : 2
      Attributes   :
         ->  name: apple
         ->  color: red
```

The histogram output looks like:

```stdout
Metric #1
    Name         : function-emit-metrics-histogram
    Description  : 
    Unit         : 
    Type         : Histogram
    Temporality  : Cumulative
    StartTime    : 2025-07-22 07:07:50.808475
    EndTime      : 2025-07-22 07:07:50.808712
    Histogram DataPoints
    DataPoint #0
      Count        : 2
      Sum          : 12.0
      Min          : 1.0
      Max          : 11.0
      Attributes   :
         ->  name: banana
         ->  color: yellow
      Buckets
         -inf to 0 : 0
         0 to 5 : 1
         5 to 10 : 0
         10 to 25 : 1
         …
    DataPoint #1
      Count        : 1
      Sum          : 2.0
      Min          : 2.0
      Max          : 2.0
      Attributes   :
         ->  name: apple
         ->  color: red
      Buckets
         -inf to 0 : 0
         0 to 5 : 1
         5 to 10 : 0
         …
    DataPoint #2
      Count        : 2
      Sum          : 2.0
      Min          : 1.0
      Max          : 1.0
      Attributes   :
         ->  name: apple
         ->  color: green
      Buckets
         -inf to 0 : 0
         0 to 5 : 2
         5 to 10 : 0
         …

```

### main shutdown

In the code, see the function `main`.

The logger provider shutdown output looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "LoggerProvider.ShutdownInvokedByUser"
   Target (Scope): "opentelemetry_sdk"
   Observed Timestamp: 2025-07-22 06:55:12.958451
   SeverityText: "DEBUG"
   SeverityNumber: Debug
   Body: String(Owned(""))
   Attributes:
     ->  name: String(Owned("LoggerProvider.ShutdownInvokedByUser"))
```

The meter provider shutdown output looks like:

```stdout
Logs
Log #0
   Instrumentation Scope: InstrumentationScope { name: "", version: None, schema_url: None, attributes: [] }
   EventName: "MeterProvider.Shutdown"
   Target (Scope): "opentelemetry_sdk"
   Observed Timestamp: 2025-07-22 06:55:12.958088
   SeverityText: "DEBUG"
   SeverityNumber: Debug
   Body: String(Owned("User initiated shutdown of MeterProvider."))
   Attributes:
     ->  name: String(Owned("MeterProvider.Shutdown"))
```
