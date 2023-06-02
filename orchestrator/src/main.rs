/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM orchestrator implementation.
 *
 * Copyright (c) 2023 Viacheslav Dubeyko <slava@dubeyko.com>,
 * All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate daemonize;

use std::fs::File;
use daemonize::Daemonize;
use clap::{Arg, Command};
use std::{
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};
pub use fm_library::cxl_fm_lib::send_responce;
pub use fm_library::cxl_fm_lib::CxlFmOptions;

/*
 * CXL FM orchestrator version
 */
const CXL_FM_ORCHESTRATOR_VERSION: &str = "0.0.1";

/*
 * CXL FM orchestrator strings
 */
const CXL_FM_ORCHESTRATOR_NAME: &str = "fm_orchestrator";
const CXL_FM_ORCHESTRATOR_DESCRIPTOR: &str = "CXL Fabric Manager (FM) orchestrator";
const CXL_FM_ORCHESTRATOR_DEBUG_OPTION: &str = "debug";
const CXL_FM_ORCHESTRATOR_DEBUG_OPTION_SHORT: char = 'd';
const CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION: &str = "ip";
const CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION_SHORT: char = 'i';
const CXL_FM_ORCHESTRATOR_PORT_OPTION: &str = "port";
const CXL_FM_ORCHESTRATOR_PORT_OPTION_SHORT: char = 'p';

const CXL_FM_ORCHESTRATOR_WORKING_DIRECTORY: &str = "/tmp";
const CXL_FM_ORCHESTRATOR_LOG_FILE_PATH: &str = "/tmp/fm_orchestrator.log";
const CXL_FM_ORCHESTRATOR_ERROR_MESSAGES_FILE_PATH: &str = "/tmp/fm_orchestrator.err";
const CXL_FM_ORCHESTRATOR_USER: &str = "nobody";
const CXL_FM_ORCHESTRATOR_GROUP: &str = "bin";
const CXL_FM_ORCHESTRATOR_GROUP_ID: u32 = 2;
const CXL_FM_ORCHESTRATOR_UMASK: u32 = 0o777;

/*
 * Command line interface definition
 */
fn cli() -> Command {
	Command::new(CXL_FM_ORCHESTRATOR_NAME)
		.about(CXL_FM_ORCHESTRATOR_DESCRIPTOR)
		.version(CXL_FM_ORCHESTRATOR_VERSION)
		.arg_required_else_help(true)
		.arg(Arg::new(CXL_FM_ORCHESTRATOR_DEBUG_OPTION)
			.short(CXL_FM_ORCHESTRATOR_DEBUG_OPTION_SHORT)
			.long(CXL_FM_ORCHESTRATOR_DEBUG_OPTION)
			.action(clap::ArgAction::SetTrue))
		.arg(Arg::new(CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION)
			.short(CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION_SHORT)
			.long(CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
		.arg(Arg::new(CXL_FM_ORCHESTRATOR_PORT_OPTION)
			.short(CXL_FM_ORCHESTRATOR_PORT_OPTION_SHORT)
			.long(CXL_FM_ORCHESTRATOR_PORT_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
}

/*
 * Discover available FM instances
 */
pub fn discover_fm(stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("{}", fm_library::cxl_fm_lib::CXL_FM_DISCOVER_FM_COMMAND);
	}

	send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_NO_DATA, env);
}

/*
 * Start FM instance
 */
pub fn start_fm(stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("{}", fm_library::cxl_fm_lib::CXL_FM_START_FM_COMMAND);
	}

	send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_NO_DATA, env);
}

/*
 * Restart FM instance
 */
pub fn restart_fm(stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("{}", fm_library::cxl_fm_lib::CXL_FM_RESTART_FM_COMMAND);
	}

	send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_NO_DATA, env);
}

/*
 * Stop FM instance
 */
pub fn stop_fm(stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("{}", fm_library::cxl_fm_lib::CXL_FM_STOP_FM_COMMAND);
	}

	send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_NO_DATA, env);
}

/*
 * Connection request processing logic
 */
fn handle_connection(stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("Process request...");
	}

	let buf_reader = BufReader::new(stream);
	let request_line = buf_reader.lines().next().unwrap().unwrap();

	if env.is_debug {
		println!("Request: {:#?}", request_line);
	}

	match request_line.as_str() {
		fm_library::cxl_fm_lib::CXL_FM_DISCOVER_FM_COMMAND => {
			discover_fm(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_START_FM_COMMAND => {
			start_fm(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_RESTART_FM_COMMAND => {
			restart_fm(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_STOP_FM_COMMAND => {
			stop_fm(stream, env);
		},
		_ => send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_UNKNOWN_COMMAND, env),
	}
}

/*
 * Main logic of daemon
 */
fn fm_daemon_logic(env: &CxlFmOptions) {
	if env.is_debug {
		println!("{} {}: Daemonized!",
			 CXL_FM_ORCHESTRATOR_NAME, CXL_FM_ORCHESTRATOR_VERSION);
	}

	loop {
		let listener = TcpListener::bind(&env.ip_port).unwrap();

		if env.is_debug {
			println!("Ready to accept connections: {}",
				 env.ip_port);
		}

		for stream in listener.incoming() {
			handle_connection(&stream.unwrap(), env);
		}
	};
}

/*
 * Application logic
 */
fn main() {
	let stdout = File::create(CXL_FM_ORCHESTRATOR_LOG_FILE_PATH).unwrap();
	let stderr = File::create(CXL_FM_ORCHESTRATOR_ERROR_MESSAGES_FILE_PATH).unwrap();

	let matches = cli().get_matches();

	let ip = matches.get_one::<String>(CXL_FM_ORCHESTRATOR_IP_ADDRESS_OPTION).unwrap();
	let port = matches.get_one::<String>(CXL_FM_ORCHESTRATOR_PORT_OPTION).unwrap();
	let ip_port = format!("{ip}:{port}");

	let options = CxlFmOptions {
		ip_port: String::from(ip_port),
		is_debug: matches.get_flag(CXL_FM_ORCHESTRATOR_DEBUG_OPTION) == true,
	};

	if options.is_debug {
		println!("{} {}", CXL_FM_ORCHESTRATOR_NAME, CXL_FM_ORCHESTRATOR_VERSION);
	}

	let daemonize = Daemonize::new()
			// Every method except `new` and `start`
			// is optional, see `Daemonize` documentation
			// for default behaviour.
			.working_directory(CXL_FM_ORCHESTRATOR_WORKING_DIRECTORY)
			.user(CXL_FM_ORCHESTRATOR_USER)
			.group(CXL_FM_ORCHESTRATOR_GROUP)	// Group name
			.group(CXL_FM_ORCHESTRATOR_GROUP_ID)	// or group id.
			.umask(CXL_FM_ORCHESTRATOR_UMASK)	// Set umask, `0o027` by default.
			.stdout(stdout)				// Redirect stdout to log file.
			.stderr(stderr)				// Redirect stderr to error messages file.
			.privileged_action(|| "Executed before drop privileges");

	match daemonize.start() {
		Ok(_) => fm_daemon_logic(&options),
		Err(e) => eprintln!("Error, {}", e),
	}
}
