pub mod system_info {
    use std::{env, io};
    use sysinfo::{System};
    use os_info;
    use hostname::get;
    use env::var_os;
    use std::ffi::OsString;
    use sys_info::{mem_info};
    use std::process::Command;


    pub struct SystemInfo {
        pub username: Option<String>,
        pub os: Option<String>,
        pub host: Option<String>,
        pub architecture: Option<String>,
        pub kernel: Option<String>,
        pub uptime: Option<String>,
        pub packages_managers: Option<String>,
        pub shell: Option<String>,
        pub de: Option<String>,
        pub wm: Option<String>,
        pub cpu: Option<String>,
        pub gpu: Option<String>,
        pub memory: Option<String>,
        pub disk: Option<String>
    }

    impl SystemInfo {
        pub fn new() -> Self {
            let sys_info = System::new();

            let username = Self::get_username();
            let os = Self::get_os(&sys_info);
            let host = Self::get_host();
            let architecture = Self::get_architecture();
            let kernel = Self::get_kernel();
            let uptime = Self::get_uptime();
            let packages_managers = Self::get_package_managers();
            let shell = Self::get_shell();
            let de = Self::get_de();
            let wm = Self::get_wm();
            let cpu = Self::get_cpu(&sys_info);
            let gpu = Self::get_gpu();
            let memory = Self::get_memory();
            let disk = Self::get_disk();

            SystemInfo { username, os, host, architecture, kernel, uptime, packages_managers, shell, de, wm, cpu, gpu, memory, disk }
        }

        fn get_username() -> Option<String> {
            return var_os("USER").and_then(|os_string: OsString| os_string.into_string().ok());
        }

        fn get_os(sys_info: &System) -> Option<String> {
            return format!("{} {}", sys_info.os_type().to_string(), sys_info.version());
        }

        fn get_host() -> Option<String> {
            return get().and_then(|result: Result<OsString, io::Error>| result.into_string().ok())
        }

        fn get_architecture() -> Option<String> {
            if let Some(arch) = os_info::get().architecture().unwrap().to_string() {
                arch
            }
            None
        }

        fn get_kernel() -> Option<String> {
            let output = Command::new("uname").arg("-r").output().expect("fail");

            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string());
            }
            None
        }

        fn get_uptime() -> Option<String> {
            let output = Command::new("uptime").output().expect("fail");

            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string());
            }
            None
        }

        fn get_package_managers() -> Option<String> { None }

        fn get_shell() -> Option<String> {
            return var_os("SHELL").and_then(|os_string: OsString| os_string.into_string().ok());
        }

        fn get_de() -> Option<String> {
            return var_os("DESKTOP_SESSION").and_then(|os_string: OsString| os_string.into_string().ok());
        }

        fn get_wm() -> Option<String> { None }

        fn get_cpu(sys_info: &System) -> Option<String> {
            if let Some(cpu) = sys_info.global_cpu_info().name().to_string() {
                return cpu
            }
            None
        }

        fn get_gpu() -> Option<String> { None }

        fn get_memory() -> Option<String> {
            for info in mem_info() {
                info.try_into().ok().to_string()
            }
            None
        }

        fn get_disk() -> Option<String> { None }
    }
}