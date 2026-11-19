#![allow(dead_code)]

pub struct Builder {
    filter: Vec<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self { filter: Vec::new() }
    }

    pub fn add_filter(&mut self, text: &str) -> &mut Self {
        self.filter.push(text.into());
        self
    }

    pub fn init(&mut self) {
        let mut filter: Vec<String> = Vec::with_capacity(self.filter.len());
        filter.append(&mut self.filter);
        let logger = SimpleLogger::new(filter);

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(logger)).unwrap()
    }
}

// ------------------------------------------------------ //

#[derive(Debug)]
struct SimpleLogger {
    filter: Vec<String>,
}

impl SimpleLogger {
    const fn new(filter: Vec<String>) -> Self {
        Self { filter: filter }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if !self.filter.is_empty() && !self.filter.contains(&metadata.target().to_string()) {
            return false;
        }
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let dt = chrono::Local::now().format("%H:%M:%S").to_string();
            println!(
                "\x1b[36;2m{}\x1b[0m {} {}",
                &dt,
                &record.level().color(),
                // record.target(), \x1b[90m[{}]\x1b[0m
                record.args(),
            );
        }
    }

    fn flush(&self) {}
}

// static LOGGER: SimpleLogger = SimpleLogger::new();

// pub fn init_log() -> Result<(), log::SetLoggerError> {
// log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
// log::set_max_level(log::LevelFilter::Trace);
// log::set_logger(&LOGGER)
// }

trait LevelColor {
    fn color(&self) -> String;
}

impl LevelColor for log::Level {
    fn color(&self) -> String {
        match self {
            log::Level::Info => format!("\x1b[32;2m[{}]\x1b[0m", self.as_str()),
            log::Level::Error => format!("\x1b[31;2m[{}]\x1b[0m", self.as_str()),
            log::Level::Debug => format!("\x1b[33;2m[{}]\x1b[0m", self.as_str()),
            log::Level::Warn => format!("\x1b[33;2m[{}]\x1b[0m", self.as_str()),
            log::Level::Trace => format!("\x1b[90m[{}]\x1b[0m", self.as_str()),
        }
        .replace("\"", "")
    }
}

// trait IsMyApp {
//     fn is_my_app(&self) -> bool;
//
//     fn check_name(&self, name: &str) -> bool {
//         true
//         // name.starts_with(APP_NAME)
//     }
// }
//
// impl IsMyApp for log::Metadata<'_> {
//     fn is_my_app(&self) -> bool {
//         self.check_name(self.target())
//     }
// }
//
// impl IsMyApp for log::Record<'_> {
//     fn is_my_app(&self) -> bool {
//         self.check_name(self.target())
//     }
// }

/*

0–15: Стандартні 16 ANSI-кольорів.
16–231: Кольорова сітка 6×6×6 (різні комбінації червоного, зеленого та синього).
232–255: Градації сірого кольору (від майже чорного до майже білого).


\x1b[1m — Жирний / Яскравий (Bold)

\x1b[2m — тьмяний (Dim)

\x1b[3m — Курсив (Italic, підтримується не всюди)

\x1b[4m — Підкреслений (Underline)

\x1b[5m — Slow blink
\x1b[6m — Rapid blink

\x1b[7m — Інверсія (міняє місцями колір тексту і фону)

\x1b[0m — Скидання всіх модифікаторів та кольорів (Reset)


\x1b[31m — Червоний

\x1b[32m — Зелений

\x1b[33m — Жовтий

\x1b[34m — Синій

\x1b[35m — Пурпуровий (Magenta)

\x1b[36m — Блакитний (Cyan)

\x1b[1m — Жирний (Bold)

\x1b[0m — Скидання всіх стилів

https://en.wikipedia.org/wiki/ANSI_escape_code
https://github.com/colored-rs/colored
 */
