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
        self.check_updates();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Optimizer W11");
            
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                if ui.link("GitHub").clicked() {
                    let _ = webbrowser::open("https://github.com/Simv135/Optimizer-W11");
                }
            });
            
            ui.separator();

            if !self.is_running && !self.done {
                ui.vertical_centered(|ui| {
                    if ui.button("Optimize").clicked() {
                        self.start_optimization();
                    }
                });
            }

            if self.is_running || self.done {
                ui.add(egui::ProgressBar::new(self.progress / 100.0).text(format!("{:.1}%", self.progress)));
                ui.label(&self.current_step);
            }

        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 150.0])
            .with_resizable(false)
            .with_title("Optimizer W11  -  v1.4.0"),
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
        ("Backup and cleaning update cache", step_backup_clean_update_cache),
        ("Energy optimizations", step_power_optimization), 
        ("BCD Tweaks", step_bcd_tweaks),
        ("NTFS Optimizations", step_ntfs_tweaks),
        ("Disabling Fast Startup", step_disable_fast_startup),
        ("DirectX Memory Optimizations", step_directx_memory_optimization),
        ("Timer Distribution", step_timer_distribution),
        ("Menu Delay Reduction", step_menu_delay_reduction),
        ("Windows Insider Experiments", step_windows_insider_experiments),
        ("MMCSS Configuration", step_mmcss_configuration),
        ("System Responsiveness", step_system_responsiveness),
        ("Windows Tips and Spotlight", step_windows_tips_spotlight),
        ("Shared Experiences", step_shared_experiences),
        ("Frequent/Recent Files", step_frequent_recent_files),
        ("Tailored Experiences", step_tailored_experiences),
        ("Search and Bing Configuration", step_search_bing_extended),
        ("Notifications Configuration", step_notifications_config),
        ("Windows Privacy Settings", step_windows_privacy_settings),
        ("Preinstalled Apps", step_preinstalled_apps),
        ("Windows Suggestions", step_windows_suggestions),
        ("Setting Synchronization", step_setting_synchronization),
        ("Windows Error Reporting", step_windows_error_reporting),
        ("Service Priorities", step_service_priorities),
        ("Full Screen Optimizations", step_full_screen_optimizations),
        ("Windowed Game Optimizations", step_windowed_game_optimizations),
        ("USB Selective Suspend", step_usb_selective_suspend),
        ("Mouse and Keyboard Optimizations", step_mouse_keyboard_optimizations),
        ("Network Configuration", step_network_config_extended), 
        ("Removing Unnecessary Apps", step_remove_apps),
        ("Service Configuration", step_configure_services_extended), 
        ("Graphics Optimizations", step_graphics_optimization_extended), 
        ("System Cleanup", step_system_cleanup_extended),
        ("Final Optimizations", step_final_optimizations),
        ("Restarting Explorer", step_restart_explorer), 
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
        
        let _ = sender.send(state);

        match step_function() {
            Ok(()) => (),
            Err(e) => println!("Error in {}: {}", description, e),
        }

        thread::sleep(Duration::from_millis(500));
    }

    let final_state = AppState {
        progress: 100.0,
        current_step: "Optimization complete!".to_string(),
        is_running: false,
        done: true,
    };
    let _ = sender.send(final_state);
}

fn step_backup_clean_update_cache() -> Result<(), String> {
    // Backup e pulizia SoftwareDistribution
    let _ = run_command("cmd", &["/c", "if exist \"C:\\Windows\\SoftwareDistribution.bak\" rmdir /s /q \"C:\\Windows\\SoftwareDistribution.bak\""]);
    let _ = run_command("cmd", &["/c", "if exist \"C:\\Windows\\SoftwareDistribution\" ren \"C:\\Windows\\SoftwareDistribution\" \"SoftwareDistribution.bak\""]);
    
    // Backup e pulizia catroot2
    let _ = run_command("cmd", &["/c", "if exist \"C:\\Windows\\System32\\catroot2.bak\" rmdir /s /q \"C:\\Windows\\System32\\catroot2.bak\""]);
    let _ = run_command("cmd", &["/c", "if exist \"C:\\Windows\\System32\\catroot2\" ren \"C:\\Windows\\System32\\catroot2\" \"catroot2.bak\""]);
    
    Ok(())
}

fn step_power_optimization() -> Result<(), String> {
    run_command("powercfg", &["/restoredefaultschemes"])?;
    run_command("powercfg", &["/setactive", "SCHEME_MIN"])?;
    run_command("powercfg", &["-h", "on"])
}

fn step_bcd_tweaks() -> Result<(), String> {
    run_command("bcdedit", &["/set", "useplatformclock", "No"])?;
    run_command("bcdedit", &["/set", "useplatformtick", "No"])?;
    run_command("bcdedit", &["/set", "disabledynamictick", "Yes"])
}

fn step_ntfs_tweaks() -> Result<(), String> {
    run_command("fsutil", &["behavior", "set", "mftzone", "4"])?;
    run_command("fsutil", &["behavior", "set", "disablelastaccess", "1"])?;
    run_command("fsutil", &["behavior", "set", "disabledeletenotify", "0"])
}

fn step_disable_fast_startup() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power", "/v", "HiberbootEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_directx_memory_optimization() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers", "/v", "DpiMapIommuContiguous", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_timer_distribution() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\kernel", "/v", "DistributeTimers", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_menu_delay_reduction() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\Control Panel\\Desktop", "/v", "MenuShowDelay", "/t", "REG_DWORD", "/d", "0", "/f"])?;
	//menu contestuale classico
	run_command("reg", &["add", "HKCU\\SOFTWARE\\CLASSES\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32", "/ve", "/f"])?;
	//mostra estensione file
	run_command("reg", &["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "/v", "HideFileExt", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_windows_insider_experiments() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\PolicyManager\\current\\device\\System", "/v", "AllowExperimentation", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\AllowExperimentation", "/v", "value", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_mmcss_configuration() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile", "/v", "NoLazyMode", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile", "/v", "AlwaysOn", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", "/v", "GPU Priority", "/t", "REG_DWORD", "/d", "8", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", "/v", "Priority", "/t", "REG_DWORD", "/d", "6", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", "/v", "Scheduling Category", "/t", "REG_SZ", "/d", "High", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", "/v", "SFIO Priority", "/t", "REG_SZ", "/d", "High", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", "/v", "Latency Sensitive", "/t", "REG_SZ", "/d", "True", "/f"])
}

fn step_system_responsiveness() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile", "/v", "SystemResponsiveness", "/t", "REG_DWORD", "/d", "10", "/f"])
}

fn step_windows_tips_spotlight() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "SoftLandingEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "RotatingLockScreenOverlayEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_shared_experiences() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CDP", "/v", "CdpSessionUserAuthzPolicy", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CDP", "/v", "NearShareChannelUserAuthzPolicy", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_frequent_recent_files() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer", "/v", "ShowFrequent", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer", "/v", "ShowRecent", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer", "/v", "TelemetrySalt", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer", "/v", "NoRecentDocsHistory", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_tailored_experiences() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Privacy", "/v", "TailoredExperiencesWithDiagnosticDataEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_search_bing_extended() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "/v", "HistoryViewEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "/v", "DeviceHistoryEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "/v", "BingSearchEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search", "/v", "AllowCortana", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Search", "/v", "CortanaEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_notifications_config() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\PushNotifications", "/v", "ToastEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings", "/v", "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings", "/v", "NOC_GLOBAL_SETTING_ALLOW_CRITICAL_TOASTS_ABOVE_LOCK", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\\QuietHours", "/v", "Enabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_windows_privacy_settings() -> Result<(), String> {
    let capabilities = [
        "activity", "appDiagnostics", "appointments", "bluetoothSync",
        "broadFileSystemAccess", "cellularData", "chat", "contacts",
        "documentsLibrary", "email", "gazeInput", "location", "phoneCall",
        "phoneCallHistory", "picturesLibrary", "radios", "userAccountInformation",
        "userDataTasks", "userNotificationListener", "videosLibrary"
    ];

    for capability in &capabilities {
        let path = format!("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\{}", capability);
        run_command("reg", &["add", &path, "/v", "Value", "/t", "REG_SZ", "/d", "Deny", "/f"])?;
    }

    Ok(())
}

fn step_preinstalled_apps() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "PreInstalledAppsEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "SilentInstalledAppsEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "OemPreInstalledAppsEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "ContentDeliveryAllowed", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", "SubscribedContentEnabled", "/t", "REG_DWORD", "/d", "0", "/f"])
}

fn step_windows_suggestions() -> Result<(), String> {
    let suggestions = [
        "SystemPaneSuggestionsEnabled", "SubscribedContent-338388Enabled",
        "SubscribedContent-314559Enabled", "SubscribedContent-280815Enabled",
        "SubscribedContent-314563Enabled", "SubscribedContent-338393Enabled",
        "SubscribedContent-353694Enabled", "SubscribedContent-353696Enabled",
        "SubscribedContent-310093Enabled", "SubscribedContent-202914Enabled",
        "SubscribedContent-338387Enabled", "SubscribedContent-338389Enabled",
        "SubscribedContent-353698Enabled"
    ];

    for suggestion in &suggestions {
        run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "/v", suggestion, "/t", "REG_DWORD", "/d", "0", "/f"])?;
    }
    Ok(())
}

fn step_setting_synchronization() -> Result<(), String> {
    let sync_groups = [
        "Accessibility", "AppSync", "BrowserSettings", "Credentials",
        "DesktopTheme", "Language", "PackageState", "Personalization",
        "StartLayout", "Windows"
    ];

    for group in &sync_groups {
        let path = format!("HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\SettingSync\\Groups\\{}", group);
        run_command("reg", &["add", &path, "/v", "Enabled", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    }
    Ok(())
}

fn step_windows_error_reporting() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting", "/v", "Disabled", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Error Reporting", "/v", "DoReport", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKLM\\SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting", "/v", "Disabled", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_service_priorities() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\I/O System", "/v", "PassiveIntRealTimeWorkerPriority", "/t", "REG_DWORD", "/d", "18", "/f"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\KernelVelocity", "/v", "DisableFGBoostDecay", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_full_screen_optimizations() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SYSTEM\\GameConfigStore", "/v", "GameDVR_DSEBehavior", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SYSTEM\\GameConfigStore", "/v", "GameDVR_FSEBehaviorMode", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SYSTEM\\GameConfigStore", "/v", "GameDVR_EFSEFeatureFlags", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SYSTEM\\GameConfigStore", "/v", "GameDVR_DXGIHonorFSEWindowsCompatible", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\SYSTEM\\GameConfigStore", "/v", "GameDVR_HonorUserFSEBehaviorMode", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_windowed_game_optimizations() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\DirectX\\UserGpuPreferences", "/v", "DirectXUserGlobalSettings", "/t", "REG_SZ", "/d", "VRROptimizeEnable=0;SwapEffectUpgradeEnable=1;", "/f"])
}

fn step_usb_selective_suspend() -> Result<(), String> {
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\USB", "/v", "DisableSelectiveSuspend", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_mouse_keyboard_optimizations() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\Control Panel\\Mouse", "/v", "MouseSpeed", "/t", "REG_SZ", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Mouse", "/v", "MouseThreshold1", "/t", "REG_SZ", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Mouse", "/v", "MouseThreshold2", "/t", "REG_SZ", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Mouse", "/v", "MouseSensitivity", "/t", "REG_SZ", "/d", "10", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Keyboard", "/v", "KeyboardDelay", "/t", "REG_SZ", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Keyboard", "/v", "KeyboardSpeed", "/t", "REG_SZ", "/d", "31", "/f"])
}

fn step_network_config_extended() -> Result<(), String> {
    run_command("netsh", &["int", "tcp", "reset"])?;
	run_command("netsh", &["winsock", "reset"])?;
    run_command("ipconfig", &["/flushdns"])?;

    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters", "/v", "DefaultTTL", "/t", "REG_DWORD", "/d", "64", "/f"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters", "/v", "Tcp1323Opts", "/t", "REG_DWORD", "/d", "1", "/f"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters", "/v", "MaxUserPort", "/t", "REG_DWORD", "/d", "65534", "/f"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters", "/v", "TcpTimedWaitDelay", "/t", "REG_DWORD", "/d", "30", "/f"])
}

fn step_remove_apps() -> Result<(), String> {
    let apps = [
        "Microsoft.3DBuilder", "Microsoft.BingFinance", "Microsoft.BingNews",
        "Microsoft.BingSports", "Microsoft.BingWeather", "Microsoft.Getstarted",
        "Microsoft.MicrosoftOfficeHub", "Microsoft.MicrosoftSolitaireCollection",
        "Microsoft.People", "Microsoft.SkypeApp", "Microsoft.WindowsCamera",
        "Microsoft.windowscommunicationsapps", "Microsoft.WindowsFeedbackHub",
        "Microsoft.WindowsMaps", "Microsoft.WindowsPhone", "Microsoft.WindowsSoundRecorder",
        "Microsoft.XboxApp", "Microsoft.XboxGameCallableUI", "Microsoft.XboxIdentityProvider",
        "Microsoft.ZuneMusic", "Microsoft.ZuneVideo", "WebExperience", "Microsoft.Whiteboard",
		"Microsoft.MicrosoftStickyNotes", "Microsoft.MixedReality.Portal", "Microsoft.Office.OneNote",
		"Microsoft.Outlook", "Teams"
    ];

    for app in &apps {
        let _ = run_command("PowerShell", &["-Command", &format!("Get-AppxPackage -allusers *{}* | Remove-AppxPackage", app)])?;
    }
	
    Ok(())
}

fn step_configure_services_extended() -> Result<(), String> {
    let services = [
        ("LanmanServer", "demand"),
        ("CryptSvc", "demand"),
        ("LanmanWorkstation", "demand"),
        ("DusmSvc", "demand"),
        ("DiagTrack", "disabled"),
        ("dmwappushservice", "disabled"),
        ("StiSvc", "demand"),
        ("BITS", "demand"),
        ("DPS", "demand"),
        ("TrkWks", "demand"),
        ("MapsBroker", "disabled"),
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

fn step_graphics_optimization_extended() -> Result<(), String> {
    run_command("reg", &["add", "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "/v", "EnableTransparency", "/t", "REG_DWORD", "/d", "0", "/f"])?;
    run_command("reg", &["add", "HKCU\\Control Panel\\Desktop", "/v", "DragFullWindows", "/t", "REG_SZ", "/d", "0", "/f"])
}

fn step_system_cleanup_extended() -> Result<(), String> {
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
    
    Ok(())
}

fn step_final_optimizations() -> Result<(), String> {
    run_command("fsutil", &["behavior", "set", "memoryusage", "2"])?;
    run_command("reg", &["add", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem", "/v", "NTFSDisableLastAccessUpdate", "/t", "REG_DWORD", "/d", "1", "/f"])
}

fn step_restart_explorer() -> Result<(), String> {
    let _ = run_command("taskkill", &["/f", "/im", "explorer.exe"]);
    thread::sleep(Duration::from_secs(3));
    run_command("explorer.exe", &[])?;
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

// Funzioni helper
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

// Crea un comando con la console nascosta
fn create_command(program: &str) -> Command {
    let mut command = Command::new(program);
    
    #[cfg(windows)]
    {
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
