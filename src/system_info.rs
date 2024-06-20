pub mod system_info {
    use std::{env};
    use sysinfo::{System};
    use os_info;
    use hostname::get;
    use env::var_os;
    use std::collections::{BTreeMap, HashMap};
    use std::ffi::OsString;
    use sys_info::{mem_info};
    use std::process::Command;
    use indexmap::IndexMap;


    pub struct SystemInfo {
        pub username: Option<String>,
        pub os: Option<String>,
        pub host: Option<String>,
        pub architecture: Option<String>,
        pub kernel: Option<String>,
        pub desktop: Option<String>,
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
            let os = Self::get_os();
            let host = Self::get_host();
            let architecture = Self::get_architecture();
            let kernel = Self::get_kernel();
            let desktop = Self::get_desktop();
            let uptime = Self::get_uptime();
            let packages_managers = Self::get_package_managers();
            let shell = Self::get_shell();
            let de = Self::get_de();
            let wm = Self::get_wm();
            let cpu = Self::get_cpu(&sys_info);
            let gpu = Self::get_gpu();
            let memory = Self::get_memory();
            let disk = Self::get_disk();

            SystemInfo { username, os, host, architecture, kernel, desktop, uptime, packages_managers, shell, de, wm, cpu, gpu, memory, disk }
        }

        pub fn to_hashmap(&self) -> IndexMap<&str, &Option<String>> {
            let mut hashmap = IndexMap::new();
            hashmap.insert("Username", &self.username);
            hashmap.insert("Os", &self.os);
            hashmap.insert("Host", &self.host);
            hashmap.insert("Architecture", &self.architecture);
            hashmap.insert("Kernel", &self.kernel);
            hashmap.insert("Desktop", &self.desktop);
            hashmap.insert("Uptime", &self.uptime);
            hashmap.insert("Packages_managers", &self.packages_managers);
            hashmap.insert("Shell", &self.shell);
            hashmap.insert("De", &self.de);
            hashmap.insert("Wm", &self.wm);
            hashmap.insert("Cpu", &self.cpu);
            hashmap.insert("Gpu", &self.gpu);
            hashmap.insert("Memory", &self.memory);
            hashmap.insert("Disk", &self.disk);
            hashmap
        }

        fn get_username() -> Option<String> {
            return var_os("USER").and_then(|os_string: OsString| os_string.into_string().ok());
        }

        fn get_os() -> Option<String> {
            let osinfo = os_info::get();
            if Some(&osinfo).is_some() {
                return Some(format!("{} {}", osinfo.os_type().to_string(), osinfo.version()));
            } return None
        }

        fn get_host() -> Option<String> {
            return get().ok().and_then(|result| result.into_string().ok())
        }

        fn get_architecture() -> Option<String> {
            if let Some(arch) = os_info::get().architecture() {
                return Some(arch.to_string());
            } None
        }

        fn get_kernel() -> Option<String> {
            let output = Command::new("uname").arg("-r").output().expect("fail");

            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).replace("\n", "").to_string());
            } None
        }

        fn get_desktop() -> Option<String> {
            return var_os("XDG_SESSION_DESKTOP").and_then(|os_string: OsString| os_string.into_string().ok())
        }

        fn get_uptime() -> Option<String> {
            let output = Command::new("uptime").output().expect("fail");

            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).replace("\n", "").to_string());
            } None
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
            let cpu_name = sys_info.global_cpu_info().name().to_string();
            if !cpu_name.is_empty() {
                return Some(cpu_name);
            } None
        }

        fn get_gpu() -> Option<String> { None }

        fn get_memory() -> Option<String> {
            for info in mem_info() {
                let available_memory = info.avail/1024/1024;
                let total = info.total/1024/1024;
                return Some(format!("{}Gi/{}Gi", available_memory.to_string(), total.to_string()));
            } None
        }

        fn get_disk() -> Option<String> { None }
    }
}