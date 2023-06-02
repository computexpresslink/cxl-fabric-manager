/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM daemon implementation.
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

pub mod cxl_fm_discover_command {
	use std::net::{TcpStream};
	use fm_library::cxl_fm_lib::CxlFmOptions;
	use fm_library::cxl_fm_lib::send_responce;
	use fm_library::cxl_fm_lib::CXL_FM_DISCOVER_CXL_DEVICE_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_DISCOVER_CXL_SWITCH_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_DISCOVER_LD_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_NO_DATA;

	/*
	 * Discover available CXL devices
	 */
	pub fn discover_cxl_devices(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_DISCOVER_CXL_DEVICE_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Discover available CXL switches
	 */
	pub fn discover_cxl_switches(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_DISCOVER_CXL_SWITCH_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Discover available logical devices
	 */
	pub fn discover_logical_devices(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_DISCOVER_LD_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}
}
