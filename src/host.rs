use os_release::OsRelease;
use tracing::trace;
use which::which;

#[derive(Debug, PartialEq)]
pub enum OsType {
    MacOS,
    Debian,
    Alpine,
    Unknown,
}

#[derive(Debug)]
pub struct Host {
    pub os_type: OsType,
    pub root_cmd: String,
}

impl Host {
    pub fn detect() -> Self {
        trace!("Detecting host OS");
        let os_type = match std::env::consts::OS {
            "macos" => {
                trace!("Detected macOS");
                OsType::MacOS
            }
            "linux" => {
                if let Ok(os) = OsRelease::new() {
                    match os.id.as_str() {
                        "debian" => {
                            trace!("Detected Debian");
                            OsType::Debian
                        }
                        "alpine" => {
                            trace!("Detected Alpine");
                            OsType::Alpine
                        }
                        _ => {
                            trace!("Unknown Linux distribution: {}", os.id);
                            OsType::Unknown
                        }
                    }
                } else {
                    trace!("Failed to read /etc/os-release");
                    OsType::Unknown
                }
            }
            _ => {
                trace!("Unknown OS: {}", std::env::consts::OS);
                OsType::Unknown
            }
        };

        let root_cmd = if which("sudo").is_ok() {
            trace!("Found sudo");
            "sudo".to_string()
        } else if which("doas").is_ok() {
            trace!("Found doas");
            "doas".to_string()
        } else {
            trace!("No sudo or doas found");
            "".to_string()
        };

        Host { os_type, root_cmd }
    }

    pub fn package_manager_cmd(&self) -> Option<&str> {
        match self.os_type {
            OsType::MacOS => Some("brew install"),
            OsType::Debian => Some("apt install -y"),
            OsType::Alpine => Some("apk add"),
            OsType::Unknown => {
                trace!("No package manager for unknown OS");
                None
            }
        }
    }
}

impl ToString for OsType {
    fn to_string(&self) -> String {
        match self {
            OsType::MacOS => "macos".to_string(),
            OsType::Debian => "debian".to_string(),
            OsType::Alpine => "alpine".to_string(),
            OsType::Unknown => "unknown".to_string(),
        }
    }
}