use std::fmt::Write;

use chrono::Local;
use tracing::Level;
use tracing::Subscriber;
use tracing::field::Field;
use tracing::field::Visit;
use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;

// Reexport tracing macros and structs
pub use tracing::instrument;
pub use tracing::instrument::Instrument;
pub use tracing::{debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span};

// Reeport the whole crate
extern crate tracing;
pub mod tr {
    pub use tracing::*;
}

// #=======================#
// #=== COLOR CONSTANTS ===#

pub const RESET: &str = "\x1B[0m";
pub const BOLD: &str = "\x1B[1m";
pub const DIM: &str = "\x1B[2m";
pub const ITALIC: &str = "\x1B[3m";
pub const UNDERLINE: &str = "\x1B[4m";
pub const REVERSED: &str = "\x1B[7m";

pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";
pub const WHITE: &str = "\x1B[37m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";

// #======================#
// #=== LOGGING MACROS ===#

/// ## Header Info
/// Tracing info log variant with colored header.
/// ```
/// # use tracing_logs::*;
/// hinfo!(BLUE, "CONFIG", "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! hinfo {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::info!(
            _header_color = $color,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Header Warning
/// Tracing warn log variant with colored header.
/// ```
/// # use tracing_logs::*;
/// hwarn!(YELLOW, "HTTP", "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! hwarn {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::warn!(
            _header_color = $color,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Header Error
/// Tracing error log variant with colored header.
/// ```
/// # use tracing_logs::*;
/// herror!(RED, "CRASH", "Application panicked!")
/// ```
#[macro_export]
macro_rules! herror {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::error!(
            _header_color = $color,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Header Info Extended
/// Tracing info log variant with colored header. Text can be colored too.
/// ```
/// # use tracing_logs::*;
/// hinfo_ext!(BLUE, "CONFIG", BLUE, "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! hinfo_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::info!(
            _header_color = $color1,
            _text_color = $color2,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Header Warning Extended
/// Tracing warn log variant with colored header. Text can be colored too.
/// ```
/// # use tracing_logs::*;
/// hwarn_ext!(YELLOW, "HTTP", YELLOW, "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! hwarn_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::warn!(
            _header_color = $color1,
            _text_color = $color2,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Header Error Extended
/// Tracing error log variant with colored header. Text can be colored too.
/// ```
/// # use tracing_logs::*;
/// herror_ext!(RED, "CRASH", RED, "Application panicked!")
/// ```
#[macro_export]
macro_rules! herror_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::error!(
            _header_color = $color1,
            _text_color = $color2,
            _header_text = $label,
            $fmt $(, $arg)*
        );
    }
}

/// ## Info Colored
/// Tracing info log variant with colored text.
/// ```
/// # use tracing_logs::*;
/// cinfo!(BLUE, "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! cinfo {
    ($color:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::info!(
            _text_color = $color,
            $fmt $(, $arg)*
        );
    }
}

/// ## Warning Colored
/// Tracing warn log variant with colored text.
/// ```
/// # use tracing_logs::*;
/// cwarn!(YELLOW, "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! cwarn {
    ($color:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::warn!(
            _text_color = $color,
            $fmt $(, $arg)*
        );
    }
}

/// ## Error Colored
/// Tracing error log variant with colored text.
/// ```
/// # use tracing_logs::*;
/// cerror!(RED, "Application panicked!")
/// ```
#[macro_export]
macro_rules! cerror {
    ($color:expr, $fmt:expr $(, $arg:expr)*) => {
        tr::error!(
            _text_color = $color,
            $fmt $(, $arg)*
        );
    }
}

// #=========================#
// #=== TRACING FORMATTER ===#

use tracing_subscriber::{
    EnvFilter, Layer,
    layer::SubscriberExt,
    util::{SubscriberInitExt, TryInitError},
};

/// Initialize tracing subscriber.
pub fn tracing_init(filter: &str) {
    try_tracing_init(filter).expect("Failed to initialize tracing subscriber");
}

/// Try to initialize tracing subscriber.
pub fn try_tracing_init(filter: &str) -> Result<(), TryInitError> {
    // Define the filter
    let filter = EnvFilter::new(filter);

    // Create the formatted logging layer
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(TracingFormatter).with_filter(filter);

    // Create the tracing registry
    tracing_subscriber::registry().with(fmt_layer).try_init()
}


struct ConfigExtractor<'a> {
    buf: &'a mut String,
    text_color: Option<String>,
    header_color: Option<String>,
    header_text: Option<String>,
}

impl<'a> Visit for ConfigExtractor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(self.buf, "{:?}", value).unwrap();
        } else {
            write!(self.buf, ", {} = {:?}", field.name(), value).unwrap();
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "_text_color" => {
                self.text_color = Some(value.to_string());
            },
            "_header_color" => {
                self.header_color = Some(value.to_string());
            },
            "_header_text" => {
                self.header_text = Some(value.to_string());
            },
            "message" => {
                write!(self.buf, "{}", value).unwrap();
            },
            _ => {
                write!(self.buf, ", {} = {}", field.name(), value).unwrap();
            },
        }
    }
}


pub struct TracingFormatter;
impl<S, N> fmt::FormatEvent<S, N> for TracingFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> fmt::FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &fmt::FmtContext<'_, S, N>, mut writer: fmt::format::Writer<'_>, event: &tracing::Event<'_>) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();

        // Timestamp & Name
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let mut prefix = String::new();
        write!(prefix, "{DIM}{timestamp}{RESET} ")?;

        // Extract the fields
        let mut fields_buf = String::new();
        let mut visitor = ConfigExtractor { buf: &mut fields_buf, text_color: None, header_color: None, header_text: None };
        event.record(&mut visitor);

        match *metadata.level() {
            Level::INFO => write!(prefix, "{GREEN}{:>5}{RESET} ", metadata.level()),
            Level::WARN => write!(prefix, "{YELLOW}{:>5}{RESET} ", metadata.level()),
            Level::ERROR => write!(prefix, "{RED}{:>5}{RESET} ", metadata.level()),
            _ => write!(prefix, "{} ", metadata.level()),
        }?;

        write!(prefix, "{DIM}>> ")?;
        if let Some(scope) = ctx.event_scope() {
            let mut iter = scope.from_root().peekable();
            while let Some(sp) = iter.next() {
                if iter.peek().is_some() {
                    write!(prefix, "{} > ", sp.name())?;
                } else {
                    write!(prefix, "{}", sp.name())?;
                }
            }
        }
        write!(prefix, " ⣿ {RESET}")?;

        if let Some(text_color) = visitor.text_color {
            if let Some(header_color) = visitor.header_color
                && let Some(header_text) = visitor.header_text
            {
                let header = format!("{}{BOLD}{:>12}:{RESET} ", header_color, format!("[{}]", header_text));
                let mut iterator = fields_buf.lines();
                if let Some(line) = iterator.next() {
                    writeln!(writer, "{prefix} {header}{text_color}{line}{RESET}")?;
                }
                for line in iterator {
                    writeln!(writer, "{prefix} {text_color}{line}{RESET}")?;
                }
            } else {
                for line in fields_buf.lines() {
                    writeln!(writer, "{prefix} {text_color}{line}{RESET}")?;
                }
            }
        } else if let Some(header_color) = visitor.header_color
            && let Some(header_text) = visitor.header_text
        {
            let header = format!("{}{BOLD}{:>12}:{RESET} ", header_color, format!("[{}]", header_text));
            let mut iterator = fields_buf.lines();
            if let Some(line) = iterator.next() {
                writeln!(writer, "{prefix} {header}{line}{RESET}")?;
            }
            for line in iterator {
                writeln!(writer, "{prefix} {line}{RESET}")?;
            }
        } else {
            for line in fields_buf.lines() {
                writeln!(writer, "{prefix} {line}{RESET}")?;
            }
        }


        Ok(())
    }
}
