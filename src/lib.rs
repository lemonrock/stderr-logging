// This file is part of stderr. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT. No part of stderr, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT.


extern crate ansi_term;
#[macro_use] extern crate lazy_static;
#[cfg(unix)] extern crate libc;
extern crate log;
extern crate rust_extra;
extern crate time;


use ::ansi_term::ANSIGenericString;
use ::ansi_term::Colour;
use ::ansi_term::Style;
use ::log::Log;
use ::log::LogLevel;
use ::log::LogMetadata;
use ::log::LogRecord;
use ::log::SetLoggerError;
use ::rust_extra::unlikely;
use ::std::borrow::Cow;
use ::std::env::var;
use ::std::fmt::Debug;
use ::std::io;
use ::std::io::Write;


include!("stderr.rs");
include!("stderrln.rs");
include!("supportsAnAnsiTerminal.rs");
include!("StandardErrorAnsiLog.rs");
include!("TerminalAware.rs");