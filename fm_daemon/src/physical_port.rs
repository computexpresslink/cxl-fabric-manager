/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM configuration tool implementation.
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

pub mod cxl_physical_port_command {
	use std::net::{TcpStream};
	use fm_library::cxl_fm_lib::CxlFmOptions;
	use fm_library::cxl_fm_lib::send_responce;
	use fm_library::cxl_fm_lib::CXL_FM_GET_PHYSICAL_PORT_INFO_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_BIND_PHYSICAL_PORT_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_UNBIND_PHYSICAL_PORT_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_NO_DATA;

	/*
	 * Get state of physical port
	 */
	pub fn get_info(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_PHYSICAL_PORT_INFO_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Control physical port
	 */
	pub fn control(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Bind physical port to Virtual PCI-to-PCI Bridge (vPPB)
	 */
	pub fn bind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_BIND_PHYSICAL_PORT_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Unbind physical port from Virtual PCI-to-PCI Bridge (vPPB)
	 */
	pub fn unbind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_UNBIND_PHYSICAL_PORT_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}
}
