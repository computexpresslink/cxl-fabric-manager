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

pub mod cxl_ppb_command {
	use std::net::{TcpStream};
	use fm_library::cxl_fm_lib::CxlFmOptions;
	use fm_library::cxl_fm_lib::send_responce;
	use fm_library::cxl_fm_lib::CXL_FM_GET_PPB_CONFIG_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_PPB_BIND_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_PPB_UNBIND_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_NO_DATA;

	/*
	 * Send PCI-to-PCI Bridge (PPB) configuration request
	 */
	pub fn config(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_PPB_CONFIG_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Bind Virtual PCI-to-PCI Bridge (vPPB) inside a CXL switch
	 */
	pub fn bind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_PPB_BIND_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Unbind Virtual PCI-to-PCI Bridge (vPPB) inside a CXL switch
	 */
	pub fn unbind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_PPB_UNBIND_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}
}
