//! Defines passes

use crate::config::Config;
use colored::*;

const INFO_INDENT: &'static str = "   ";

/// A pass
struct Pass<T> {
    exec: Box<Fn(&T)>,
    name: String,
}

impl<T> Pass<T> {
    /// Creates a new pass
    pub fn new(exec: Box<Fn(&T)>, name: &str) -> Self {
        Self {
            exec,
            name: String::from(name),
        }
    }

    /// Runs the pass
    pub fn run(&self, input: &T) {
        (*self.exec)(input);
    }
}

/// Sequence of passes
struct PassSequence<'a, T> {
    passes: Vec<Pass<T>>,
    config: &'a Config,
    ty: String,
}

impl<'a, T: ToString> PassSequence<'a, T> {
    /// Create a new sequence of passes
    pub fn new(config: &'a Config, ty: &str) -> Self {
        Self {
            passes: Vec::new(),
            config,
            ty: String::from(ty),
        }
    }

    /// Add a pass to the sequence
    pub fn add_pass(&mut self, pass: Pass<T>) {
        self.passes.push(pass);
    }

    /// Run passes in the sequence (except ignored ones)
    pub fn run(&self, input: &T) {
        let data = input.to_string().red().bold();
        let ty = self.ty.yellow().bold();

        println!("{} {}({})", "found".white().bold(), ty, data);

        for pass in self.passes.iter() {
            if !self.config.is_ignored(&pass.name) {
                println!("{} {}", "➔".white().bold(), &pass.name.magenta().bold());

                pass.run(input);
            } else {
                println!(
                    "{} {} {}",
                    "✘".white().bold(),
                    &pass.name.magenta().bold(),
                    "ignored".white().bold()
                );
            }
        }
    }
}

macro_rules! pass_sequence {
    ($input:expr, $config:expr, $ty:expr; $( $pass:expr ),*) => {
        let mut seq = PassSequence::new($config, $ty);

        $( seq.add_pass(Pass::new(Box::new($pass), stringify!($pass))); )*

        seq.run($input)
    };
}

macro_rules! indent_println {
    ($add_indent_level:expr; $( $args:expr ),*) => {
        println!("{}{}{}", INFO_INDENT, " ".repeat($add_indent_level), format!($( $args ),*))
    };

    ($( $args:expr ),*) => {
        indent_println!(0; $( $args ),*)
    }
}

macro_rules! info {
    ($add_indent_level:expr; $name:expr, $value:expr) => {
        indent_println!(
            $add_indent_level;
            "{} {}",
            $name.to_string().blue().bold(),
            $value.to_string().green().bold()
        )
    };
    ($name:expr, $value:expr) => {
        info!(0; $name, $value)
    };
}

macro_rules! na {
    ($add_indent_level:expr; $name:expr) => {
        info!($add_indent_level; $name, "n/a".cyan().bold())
    };
    ($name:expr) => {
        na!(0; $name)
    };
}

pub mod ip_addr;
pub mod integer;
pub mod string;
