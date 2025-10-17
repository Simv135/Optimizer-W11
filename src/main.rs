#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use eframe::egui;
use std::sync::mpsc;
use std::os::windows::process::CommandExt;

struct OptimizerApp {
    progress: f32,
    current_step: String,
    is_running: bool,
    done: bool,
    receiver: Option<mpsc::Receiver<AppState>>,
}

impl Default for OptimizerApp {
    fn default() -> Self {
        Self {
            progress: 0.0,
            current_step: "Ready for optimization".to_string(),
            is_running: false,
            done: false,
            receiver: None,
        }
    }
}

impl OptimizerApp {
    fn start_optimization(&mut self) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.done = false;
        self.progress = 0.0;

        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);

        thread::spawn(move || {
            run_optimization_steps(sender);
        });
    }

    fn check_updates(&mut self) {
        if let Some(receiver) = &self.receiver {
            while let Ok(state) = receiver.try_recv() {
                self.progress = state.progress;
                self.current_step = state.current_step;
                self.is_running = state.is_running;
                self.done = state.done;
            }
        }
    }
}

#[derive(Clone)]
struct AppState {
    progress: f32,
    current_step: String,
    is_running: bool,
    done: bool,
}

impl eframe::App for OptimizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Controlla se ci sono nuovi messaggi dal thread di ottimizzazione
        self.check_updates();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Optimizer W11");
            
            // Link GitHub in alto a destra
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                if ui.link("GitHub").clicked() {
                    let _ = webbrowser::open("https://github.com/Simv135/Optimizer-W11");
                }
            });
            
            ui.separator();

            // Pulsante Ottimizza
            if !self.is_running && !self.done {
                ui.vertical_centered(|ui| {
                    if ui.button("Optimize").clicked() {
                        self.start_optimization();
                    }
                });
            }

            // Barra di progresso
            if self.is_running || self.done {
                ui.add(egui::ProgressBar::new(self.progress / 100.0).text(format!("{:.1}%", self.progress)));
                ui.label(&self.current_step);
            }

        });

        // Richiedi un repaint continuo per aggiornare la GUI
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 150.0])
            .with_resizable(false)
            .with_title("Optimizer W11  -  v.1.0.0"),
        ..Default::default()
    };

    eframe::run_native(
        "Optimizer W11",
        options,
        Box::new(|_cc| Box::<OptimizerApp>::default()),
    )
}

fn run_optimization_steps(sender: mpsc::Sender<AppState>) {
    let steps: Vec<(&str, fn() -> Result<(), String>)> = vec![
        ("Energy optimizations", step_power_optimization), 
        ("Service Configuration", step_configure_services), 
        ("Privacy Settings", step_privacy_settings), 
        ("Disabling Windows AI", step_disable_windows_ai), 
        ("Verify and disable Recall", step_disable_recall), 
        ("Search Configuration", step_search_cortana), 
        ("Registry Cleaner", step_clean_registry), 
        ("Graphics Optimizations", step_graphics_optimization), 
        ("Restarting Explorer", step_restart_explorer), 
        ("Network Configuration", step_network_config), 
        ("TCP/IP Optimizations", step_tcp_optimization), 
        ("Time Synchronization", step_time_sync), 
        ("Disabling Telemetry", step_disable_telemetry), 
        ("System Cleanup", step_system_cleanup),
    ];

    let total_steps = steps.len();

    for (i, (description, step_function)) in steps.iter().enumerate() {
        let progress = (i as f32 / total_steps as f32) * 100.0;
        let state = AppState {
            progress,
            current_step: format!("{}...", description),
            is_running: true,
            done: false,
        };
        
        // Invia lo stato attuale alla GUI
        let _ = sender.send(state);

        // Esegue lo step di ottimizzazione
        match step_function() {
            Ok(()) => (),
            Err(e) => println!("Error in {}: {}", description, e),
        }

        thread::sleep(Duration::from_millis(500));
    }

    // Invio stato finale
    let final_state = AppState {
        progress: 100.0,
        current_step: "Optimization complete!".to_string(),
        is_running: false,
        done: true,
    };
    let _ = sender.send(final_state);
}

// Funzioni di ottimizzazione modificate
fn step_power_optimization() -> Result<(), String> {
    run_command("powercfg", &["/restoredefaultschemes"])?;
    run_command("powercfg", &["/setactive", "SCHEME_MIN"])?;
    run_command("powercfg", &["-h", "on"])
}

fn step_configure_services() -> Result<(), String> {
    let services = [
        ("LanmanServer", "demand"),
        ("CryptSvc", "demand"),
        ("LanmanWorkstation", "demand"),
        ("DusmSvc", "demand"),
        ("DiagTrack", "demand"),
        ("StiSvc", "demand"),
        ("BITS", "demand"),
        ("DPS", "demand"),
        ("TrkWks", "demand"),
        ("MapsBroker", "demand"),
        ("iphlpsvc", "demand"),
        ("WSearch", "disabled"),
        ("TabletInputService", "disabled"),
        ("WMPNetworkSvc", "disabled"),
        ("Fax", "disabled"),
    ];

    for (service, start_type) in &services {
        configure_service(service, start_type)?;
    }
    Ok(())
}

fn configure_service(service: &str, start_type: &str) -> Result<(), String> {
    let output = create_command("sc")
        .args(&["query", service])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("Service Query Error {}: {}", service, e))?;

    if output.success() {
        run_command("sc", &["config", service, &format!("start={}", start_type)])?;
        
        if start_type == "disabled" {
            let _ = run_command("sc", &["stop", service]);
        }
    }
    Ok(())
}

fn step_privacy_settings() -> Result<(), String> {
    let registry_entries = [
        ("HKLM\\Software\\Policies\\Microsoft\\Windows\\AppPrivacy", "LetAppsRunInBackground", "2"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings", "NOC_GLOBAL_SETTING_ALLOW_TOASTS_ABOVE_LOCK", "1"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings", "NOC_GLOBAL_SETTING_ALLOW_CRITICAL_TOASTS_ABOVE_LOCK", "1"),
        ("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "SubscribedContent-310093Enabled", "1"),
        ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer", "HideRecentlyaddedApps", "0"),
        ("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer", "EnableAutoTray", "1"),
        ("HKCU\\Software\\Microsoft\\GameBar", "UseNexusForGameBarEnabled", "0"),
        ("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR", "AppCaptureEnabled", "1"),
        ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "ShowCopilotButton", "0"),
        ("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "TaskbarDa", "0"),
        ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Widgets", "AllowWidgets", "0"),
        ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Dsh", "AllowNewsAndInterests", "0"),
    ];

    for (key, value, data) in &registry_entries {
        run_command("reg", &["add", key, "/v", value, "/t", "REG_DWORD", "/d", data, "/f"])?;
    }
    Ok(())
}

fn step_disable_windows_ai() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\Software\\Policies\\Microsoft\\Windows\\WindowsAI", "/v", "DisableAIDataAnalysis", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    run_command("reg", &["add", "HKLM\\Software\\Policies\\Microsoft\\Windows\\WindowsAI", "/v", "DisableAIDataAnalysis", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    Ok(())
}

fn step_disable_recall() -> Result<(), String> {
    let output = create_command("Dism")
        .args(&["/Online", "/Get-FeatureInfo", "/FeatureName:Recall", "/English"])
        .output()
        .map_err(|e| format!("Recall verification error: {}", e))?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("Enabled") {
            run_command("Dism", &["/Online", "/Disable-Feature", "/FeatureName:Recall", "/NoRestart"])?;
        }
    }
    Ok(())
}

fn step_search_cortana() -> Result<(), String> {
    let registry_entries = [
        ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search", "AllowCortana", "0"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "CortanaEnabled", "0"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled", "0"),
    ];

    for (key, value, data) in &registry_entries {
        run_command("reg", &["add", key, "/v", value, "/t", "REG_DWORD", "/d", data, "/f"])?;
    }
    Ok(())
}

fn step_clean_registry() -> Result<(), String> {
    let packages = [
        "46928bounde.EclipseManager_2.2.4.51_neutral__a5h4egax66k6y",
        "ActiproSoftwareLLC.562882FEEB491_2.6.18.18_neutral__24pqs290vpjk0",
        "Microsoft.MicrosoftOfficeHub_17.7909.7600.0_x64__8wekyb3d8bbwe",
        "Microsoft.PPIProjection_10.0.15063.0_neutral_neutral_cw5n1h2txyewy",
        "Microsoft.XboxGameCallableUI_1000.15063.0.0_neutral_neutral_cw5n1h2txyewy",
        "Microsoft.XboxGameCallableUI_1000.16299.15.0_neutral_neutral_cw5n1h2txyewy",
    ];

    for package in &packages {
        let paths = [
            format!("HKCR\\Extensions\\ContractId\\Windows.BackgroundTasks\\PackageId\\{}", package),
            format!("HKCR\\Extensions\\ContractId\\Windows.Launch\\PackageId\\{}", package),
            format!("HKCR\\Extensions\\ContractId\\Windows.Protocol\\PackageId\\{}", package),
        ];

        for path in &paths {
            let _ = run_command("reg", &["delete", path, "/f"]);
        }
    }
    Ok(())
}

fn step_graphics_optimization() -> Result<(), String> {
    let registry_entries = [
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "EnableTransparency", "0"),
        ("HKCU\\Control Panel\\Desktop", "DragFullWindows", "0"),
        ("HKCU\\Control Panel\\Desktop", "MenuShowDelay", "0"),
    ];

    for (key, value, data) in &registry_entries {
        let type_str = if key.contains("Desktop") { "REG_SZ" } else { "REG_DWORD" };
        run_command("reg", &["add", key, "/v", value, "/t", type_str, "/d", data, "/f"])?;
    }

    run_command("reg", &["add", "HKCU\\SOFTWARE\\CLASSES\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32", "/ve", "/t", "REG_SZ", "/d", "", "/f"])?;
    Ok(())
}

fn step_restart_explorer() -> Result<(), String> {
    let _ = run_command("taskkill", &["/f", "/im", "explorer.exe"]);
    thread::sleep(Duration::from_secs(3));
    run_command("explorer.exe", &[])?;
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn step_network_config() -> Result<(), String> {
    let interfaces = ["Ethernet", "Wi-Fi"];
    let dns_servers = ["1.1.1.1", "1.0.0.1"];

    for interface in &interfaces {
        run_command("netsh", &[
            "interface", "ipv4", "set", "dnsservers", 
            interface, "static", dns_servers[0], "primary"
        ])?;
        
        run_command("netsh", &[
            "interface", "ipv4", "add", "dnsservers", 
            interface, dns_servers[1], "index=2"
        ])?;
    }
    Ok(())
}

fn step_tcp_optimization() -> Result<(), String> {
    run_command("ipconfig", &["/flushdns"])?;
    
    let tcp_settings = [
        ("rss", "enabled"),
        ("autotuninglevel", "normal"),
        ("dca", "enabled"),
        ("ecncapability", "enabled"),
    ];

    for (setting, value) in &tcp_settings {
        run_command("netsh", &["int", "tcp", "set", "global", &format!("{}={}", setting, value)])?;
    }
    
    run_command("netsh", &["int", "tcp", "set", "global", "initialrto=1000"])?;
    Ok(())
}

fn step_time_sync() -> Result<(), String> {
    run_command("w32tm", &["/config", "/manualpeerlist:pool.ntp.org", "/syncfromflags:manual", "/reliable:yes", "/update"])?;
    run_command("net", &["stop", "w32time"])?;
    run_command("net", &["start", "w32time"])
}

fn step_disable_telemetry() -> Result<(), String> {
    let registry_entries = [
        ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", "AllowTelemetry", "0"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "SoftLandingEnabled", "0"),
        ("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "SilentInstalledAppsEnabled", "0"),
    ];

    for (key, value, data) in &registry_entries {
        run_command("reg", &["add", key, "/v", value, "/t", "REG_DWORD", "/d", data, "/f"])?;
    }
    Ok(())
}

fn step_system_cleanup() -> Result<(), String> {
    run_command("net", &["stop", "wuauserv"])?;
    
    let _ = run_command("rd", &["/s", "/q", "C:\\Windows\\SoftwareDistribution"]);
    let _ = run_command("md", &["C:\\Windows\\SoftwareDistribution"]);
    
    run_command("net", &["start", "wuauserv"])?;

    let cleanup_paths = [
        "C:\\Windows\\Temp\\*.*",
        "C:\\WINDOWS\\Prefetch\\*.*",
        "%TEMP%\\*.*",
    ];

    for path in &cleanup_paths {
        let _ = run_command("del", &["/s", "/f", "/q", path]);
    }

    run_command("fsutil", &["behavior", "set", "memoryusage", "2"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem", "/v", "NTFSDisableLastAccessUpdate", "/t", "REG_DWORD", "/d", "1", "/f"])?;

    Ok(())
}

// Crea un comando con la console nascosta (solo Windows)
fn create_command(program: &str) -> Command {
    let mut command = Command::new(program);
    
    #[cfg(windows)]
    {
        // Nasconde la finestra della console
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    
    command
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let output = create_command(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("Execution error {}: {}", program, e))?;

    if output.success() {
        Ok(())
    } else {
        Ok(())
    }
}