/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 15
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use std::{
  fs,
  path::PathBuf
};

use chrono::Local;
use log::{LevelFilter, info, trace, error, warn};
use log4rs::{
  append::{console::ConsoleAppender, file::FileAppender},
  encode::pattern::PatternEncoder,
  Config,
  config::{Appender, Logger, Root}
};

//Logging constants
const SERVER_LOG_PATTERN: &'static str =
  "(({d(%Y-%m-%d %H:%M:%S)})) server: [{h({l})}] {m}{n}";
const CHAT_LOG_PATTERN: &'static str= 
  "(({d(%Y-%m-%d %H:%M:%S)})) {m}{n}";

//Panic messages
const PANIC_DIR_MISSING: &'static str =
  "[FATAL STARTUP PANIC] - could not create directory for logfiles: ";

/*
  Loggin macros
*/
macro_rules! chat {
  ($player:expr, $msg:expr) => {
    trace!(target: "chat", "{}: {}", ($player), ($msg))
  };
}

pub fn start_logger() {
  /*
    This function sets-up the logger for the entire server. The logger consists
    of two parts; all messages are logged to *both*:
      (1) Stdout - basic console logger
      (2) Logfile - file that is appended during operation
  */

  //(1) First we need to configure stdout
  let server_stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(SERVER_LOG_PATTERN)))
    .build();
  let chat_stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(CHAT_LOG_PATTERN)))
    .build();

  //(2a) Next we setup the logfile. We must first make sure the log dir exists
  let log_dir = PathBuf::from(super::LOG_FOLDER);
  if !log_dir.exists() {
    fs::create_dir_all(log_dir.clone()).expect(PANIC_DIR_MISSING);
  }

  //(2b) Next we create the logfile based on the current time
  let now = Local::now().format("%Y-%m-%d_%H:%M:%S");
  let log_file_path = format!("{}/[SRVR_LOG]_{}.log", super::LOG_FOLDER, now);

  let server_logfile_out = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(SERVER_LOG_PATTERN)))
    .build(log_file_path.clone())
    .expect("[FATAL STARTUP PANIC] - could not instantiate logger: ");
  let chat_logfile_out = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(CHAT_LOG_PATTERN)))
    .build(log_file_path)
    .expect("[FATAL STARTUP PANIC] - could not instantiate logger: ");

  //(3) Now we actually set-up the logger
  let log_config = Config::builder()
    .appender(Appender::builder().build("server_stdout", Box::new(server_stdout)))
    .appender(Appender::builder().build("server_logfile", Box::new(server_logfile_out)))
    .appender(Appender::builder().build("chat_stdout", Box::new(chat_stdout)))
    .appender(Appender::builder().build("chat_logfile", Box::new(chat_logfile_out)))
    .logger(Logger::builder()
      .appender("chat_stdout")
      .appender("chat_logfile")
      .additive(false)
      .build("chat", LevelFilter::Trace)
    )
    .build(
      Root::builder()
      .appender("server_stdout")
      .appender("server_logfile")
      .build(LevelFilter::Info)
    )
    .expect("[FATAL STARTUP PANIC] - error while setting-up logging config: ");
  log4rs::init_config(log_config).expect("[FATAL STARTUP PANIC] - error while creating logger: ");

  info!("Starting srvr v{}...", super::VERSION);
  info!("Finished configuring logger");
}