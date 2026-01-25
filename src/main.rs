#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const TITLE: &str = "Optimizer W11";
const VERSION: &str = "v1.5.0";

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use eframe::egui;
use std::sync::mpsc;

use std::os::windows::process::CommandExt;
const CREATE_NO_WINDOW: u32 = 0x08000000;

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
            ui.heading(TITLE);
            
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
            .with_title(&format!("{}  -  {}", TITLE, VERSION)),
        ..Default::default()
    };

    eframe::run_native(
        TITLE,
        options,
        Box::new(|_cc| Box::<OptimizerApp>::default()),
    )
}

fn run_optimization_steps(sender: mpsc::Sender<AppState>) {
    let steps: Vec<(&str, fn() -> Result<(), String>)> = vec![
		("Create Restore Point", step_create_restore_point),
        ("System Cleanup & Backup", step_system_cleanup_backup),
        ("Performance & Power Tweaks", step_performance_power_tweaks),
        ("Gaming & Multimedia Optimizations", step_gaming_multimedia_optimizations),
        ("Privacy & Telemetry Configuration", step_privacy_telemetry_config),
        ("Windows Experience & UI Tweaks", step_windows_experience_ui_tweaks),
        ("Network & Hardware Optimization", step_network_hardware_optimization),
        ("Unnecessary Components Removal", step_unnecessary_components_removal),
        ("System Services Configuration", step_system_services_config),
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

        thread::sleep(Duration::from_millis(300));
    }

    let final_state = AppState {
        progress: 100.0,
        current_step: "Optimization complete!".to_string(),
        is_running: false,
        done: true,
    };
    let _ = sender.send(final_state);
}

/* Create restore point */
fn step_create_restore_point() -> Result<(), String> {
    run_command(&format!(
        r#"powershell -Command "$n='OW11{}'; if(!(Get-ComputerRestorePoint|?{{$_.Description-eq$n}})){{Checkpoint-Computer -Description $n}}""#,
        VERSION
    ))?;
    Ok(())
}

/* System Cleanup & Backup */
fn step_system_cleanup_backup() -> Result<(), String> {
    // Backup and cleaning update cache
    run_command("if exist \"C:\\Windows\\SoftwareDistribution.bak\" rmdir /s /q \"C:\\Windows\\SoftwareDistribution.bak\"")?;
    run_command("if exist \"C:\\Windows\\SoftwareDistribution\" ren \"C:\\Windows\\SoftwareDistribution\" \"SoftwareDistribution.bak\"")?;
    
    run_command("if exist \"C:\\Windows\\System32\\catroot2.bak\" rmdir /s /q \"C:\\Windows\\System32\\catroot2.bak\"")?;
    run_command("if exist \"C:\\Windows\\System32\\catroot2\" ren \"C:\\Windows\\System32\\catroot2\" \"catroot2.bak\"")?;
    
    // System cleanup extended
    run_command("net stop wuauserv")?;
    
	run_command(r#"powershell -Command "if (Test-Path 'C:\Windows\SoftwareDistribution') { Remove-Item -Path 'C:\Windows\SoftwareDistribution' -Recurse -Force -ErrorAction Stop }""#)?;
	run_command(r#"powershell -Command "New-Item -Path 'C:\Windows\SoftwareDistribution' -ItemType Directory -Force | Out-Null""#)?;
		
    run_command("net start wuauserv")?;

	run_command(r#"powershell -Command "Get-ChildItem -Path 'C:\Windows\Temp' -Force | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue""#)?;
	run_command(r#"powershell -Command "Get-ChildItem -Path 'C:\Windows\Prefetch' -Force | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue""#)?;
	run_command(r#"powershell -Command "Get-ChildItem -Path $env:TEMP -Force | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue""#)?;

    Ok(())
}

/* Performance & Power Tweaks */
fn step_performance_power_tweaks() -> Result<(), String> {
    // Energy optimizations
    run_command("powercfg /restoredefaultschemes")?;
    run_command("powercfg /setactive SCHEME_MIN")?;
    run_command("powercfg -h on")?;
    
    // BCD Tweaks
    run_command("bcdedit /set useplatformclock No")?;
    run_command("bcdedit /set useplatformtick No")?;
    run_command("bcdedit /set disabledynamictick Yes")?;
    
    // NTFS Optimizations
    run_command("fsutil behavior set mftzone 4")?;
    run_command("fsutil behavior set disablelastaccess 1")?;
    run_command("fsutil behavior set disabledeletenotify 0")?;
    
    // Disabling Fast Startup
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\Power\" /v HiberbootEnabled /t REG_DWORD /d 0 /f")?;
    
    // Timer Distribution
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\kernel\" /v DistributeTimers /t REG_DWORD /d 1 /f")?;
    
    // Final optimizations
    run_command("fsutil behavior set memoryusage 2")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\FileSystem\" /v NTFSDisableLastAccessUpdate /t REG_DWORD /d 1 /f")
}

/* Gaming & Multimedia Optimizations */
fn step_gaming_multimedia_optimizations() -> Result<(), String> {
    // DirectX Memory Optimizations
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\I/O System\" /v PassiveIntRealTimeWorkerPriority /t REG_DWORD /d 18 /f")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\KernelVelocity\" /v DisableFGBoostDecay /t REG_DWORD /d 1 /f")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Control\\GraphicsDrivers\" /v DpiMapIommuContiguous /t REG_DWORD /d 1 /f")?;
    
    // MMCSS Configuration
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\" /v NoLazyMode /t REG_DWORD /d 1 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\" /v AlwaysOn /t REG_DWORD /d 1 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\" /v \"GPU Priority\" /t REG_DWORD /d 8 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\" /v Priority /t REG_DWORD /d 6 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\" /v \"Scheduling Category\" /t REG_SZ /d High /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\" /v \"SFIO Priority\" /t REG_SZ /d High /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\" /v \"Latency Sensitive\" /t REG_SZ /d True /f")?;
    
    // System Responsiveness
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\" /v SystemResponsiveness /t REG_DWORD /d 10 /f")?;
    
    // Full Screen Optimizations
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_DSEBehavior /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_FSEBehaviorMode /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_EFSEFeatureFlags /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_DXGIHonorFSEWindowsCompatible /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_HonorUserFSEBehaviorMode /t REG_DWORD /d 1 /f")?;
    
    // Windowed Game Optimizations
    run_command("reg add \"HKCU\\Software\\Microsoft\\DirectX\\UserGpuPreferences\" /v DirectXUserGlobalSettings /t REG_SZ /d \"VRROptimizeEnable=0;SwapEffectUpgradeEnable=1;\" /f")
}

/* Privacy & Telemetry Configuration */
fn step_privacy_telemetry_config() -> Result<(), String> {
    // Windows Insider Experiments
    run_command("reg add \"HKLM\\Software\\Microsoft\\PolicyManager\\current\\device\\System\" /v AllowExperimentation /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\PolicyManager\\default\\System\\AllowExperimentation\" /v value /t REG_DWORD /d 0 /f")?;
    
    // Privacy Settings
    let capabilities = [
        "activity", "appDiagnostics", "appointments", "bluetoothSync",
        "broadFileSystemAccess", "cellularData", "chat", "contacts",
        "documentsLibrary", "email", "gazeInput", "location", "phoneCall",
        "phoneCallHistory", "picturesLibrary", "radios", "userAccountInformation",
        "userDataTasks", "userNotificationListener", "videosLibrary"
    ];

    for capability in &capabilities {
        let path = format!("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\{}", capability);
        run_command(&format!("reg add \"{}\" /v Value /t REG_SZ /d Deny /f", path))?;
    }
    
    // Browser privacy settings
    run_command("reg add \"HKLM\\Software\\Policies\\Microsoft\\Edge\" /v MetricsReportingEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Policies\\Microsoft\\Edge\" /v DiagnosticData /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Policies\\Google\\Chrome\" /v MetricsReportingEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Policies\\Google\\Chrome\" /v SafeBrowsingEnabled /t REG_DWORD /d 0 /f")?;
    
    // Windows Suggestions
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
        run_command(&format!("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v {} /t REG_DWORD /d 0 /f", suggestion))?;
    }
    
    // Tailored Experiences
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Privacy\" /v TailoredExperiencesWithDiagnosticDataEnabled /t REG_DWORD /d 0 /f")?;
    
    // Windows Error Reporting
    run_command("reg add \"HKLM\\Software\\Policies\\Microsoft\\Windows\\Windows Error Reporting\" /v Disabled /t REG_DWORD /d 1 /f")?;
    run_command("reg add \"HKLM\\Software\\Policies\\Microsoft\\Windows\\Windows Error Reporting\" /v DoReport /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Microsoft\\Windows\\Windows Error Reporting\" /v Disabled /t REG_DWORD /d 1 /f")
}

/* Windows Experience & UI Tweaks */
fn step_windows_experience_ui_tweaks() -> Result<(), String> {
    // Delay Reduction
    run_command("reg add \"HKCU\\Control Panel\\Desktop\" /v MenuShowDelay /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\CLASSES\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32\" /ve /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" /v HideFileExt /t REG_DWORD /d 0 /f")?;
    
    // Windows Tips and Spotlight
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v SoftLandingEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v RotatingLockScreenOverlayEnabled /t REG_DWORD /d 0 /f")?;
    
    // Shared Experiences
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\CDP\" /v CdpSessionUserAuthzPolicy /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\CDP\" /v NearShareChannelUserAuthzPolicy /t REG_DWORD /d 0 /f")?;
    
    // Frequent/Recent Files
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\" /v ShowFrequent /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\" /v ShowRecent /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\" /v TelemetrySalt /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer\" /v NoRecentDocsHistory /t REG_DWORD /d 1 /f")?;
    
    // Search and Bing Configuration
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\" /v HistoryViewEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\" /v DeviceHistoryEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\" /v BingSearchEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKLM\\Software\\Policies\\Microsoft\\Windows\\Windows Search\" /v AllowCortana /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Search\" /v CortanaEnabled /t REG_DWORD /d 0 /f")?;
    
    // Notifications Configuration
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\PushNotifications\" /v ToastEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\" /v NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\" /v NOC_GLOBAL_SETTING_ALLOW_CRITICAL_TOASTS_ABOVE_LOCK /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings\\QuietHours\" /v Enabled /t REG_DWORD /d 0 /f")?;
    
    // Setting Synchronization
    let sync_groups = [
        "Accessibility", "AppSync", "BrowserSettings", "Credentials",
        "DesktopTheme", "Language", "PackageState", "Personalization",
        "StartLayout", "Windows"
    ];

    for group in &sync_groups {
        let path = format!("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\SettingSync\\Groups\\{}", group);
        run_command(&format!("reg add \"{}\" /v Enabled /t REG_DWORD /d 0 /f", path))?;
    }
    
    // Graphics Optimizations
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\" /v EnableTransparency /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Desktop\" /v DragFullWindows /t REG_SZ /d 0 /f")
}

/* Network & Hardware Optimization */
fn step_network_hardware_optimization() -> Result<(), String> {
    // Network Configuration Extended
    run_command("netsh int tcp reset")?;
    run_command("netsh winsock reset")?;
    run_command("ipconfig /flushdns")?;

    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters\" /v DefaultTTL /t REG_DWORD /d 64 /f")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters\" /v Tcp1323Opts /t REG_DWORD /d 1 /f")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters\" /v MaxUserPort /t REG_DWORD /d 65534 /f")?;
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Services\\Tcpip\\Parameters\" /v TcpTimedWaitDelay /t REG_DWORD /d 30 /f")?;
    
    // USB Selective Suspend
    run_command("reg add \"HKLM\\System\\CurrentControlSet\\Services\\USB\" /v DisableSelectiveSuspend /t REG_DWORD /d 1 /f")?;
    
    // Mouse and Keyboard Optimizations
    run_command("reg add \"HKCU\\Control Panel\\Mouse\" /v MouseSpeed /t REG_SZ /d 0 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Mouse\" /v MouseThreshold1 /t REG_SZ /d 0 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Mouse\" /v MouseThreshold2 /t REG_SZ /d 0 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Mouse\" /v MouseSensitivity /t REG_SZ /d 10 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Keyboard\" /v KeyboardDelay /t REG_SZ /d 0 /f")?;
    run_command("reg add \"HKCU\\Control Panel\\Keyboard\" /v KeyboardSpeed /t REG_SZ /d 31 /f")
}

/* Unnecessary Components Removal */
fn step_unnecessary_components_removal() -> Result<(), String> {
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v PreInstalledAppsEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v SilentInstalledAppsEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v OemPreInstalledAppsEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v ContentDeliveryAllowed /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager\" /v SubscribedContentEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR\" /v AppCaptureEnabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\System\\GameConfigStore\" /v GameDVR_Enabled /t REG_DWORD /d 0 /f")?;
    run_command("reg add \"HKCU\\Software\\Policies\\Microsoft\\Windows\\GameDVR\" /v AllowGameDVR /t REG_DWORD /d 0 /f")?;
    run_command("reg delete \"HKCU\\Software\\Classes\\ms-gamebar\" /f")?;
    run_command("reg delete \"HKLM\\Software\\Classes\\ms-gamebar\" /f")?;
    run_command("reg delete \"HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\GameBar.exe\" /f")?;
    run_command("reg delete \"HKCU\\Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\ms-gamebar\" /f")?;
    run_command("reg delete \"HKLM\\Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\ms-gamebar\" /f")?;
    run_command("reg add \"HKCU\\Software\\Classes\\ms-gamebar\" /f")?;
    run_command("reg add \"HKCU\\Software\\Classes\\ms-gamebar\" /v \"URL Protocol\" /t REG_SZ /d \"\" /f")?;
    run_command("reg add \"HKCU\\Software\\Classes\\ms-gamebar\\shell\\open\\command\" /t REG_SZ /d \"cmd /c exit\" /f")?;
    run_command("reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer\" /v DisableFeedbackRequests /t REG_DWORD /d 1 /f")?;
    
    let apps = [
        "Microsoft.3DBuilder", "Microsoft.BingFinance", "Microsoft.BingNews",
        "Microsoft.BingSports", "Microsoft.BingWeather", "Microsoft.Getstarted",
        "Microsoft.MicrosoftOfficeHub", "Microsoft.MicrosoftSolitaireCollection",
        "Microsoft.People", "Microsoft.SkypeApp", "Microsoft.WindowsCamera",
        "Microsoft.windowscommunicationsapps", "Microsoft.WindowsFeedbackHub", "FeedbackHub",
        "Microsoft.WindowsMaps", "Microsoft.WindowsPhone", "Microsoft.WindowsSoundRecorder",
        "Microsoft.ZuneMusic", "Microsoft.ZuneVideo", "WebExperience", "Microsoft.Whiteboard",
        "Microsoft.MicrosoftStickyNotes", "Microsoft.MixedReality.Portal", "Microsoft.Office.OneNote",
        "Microsoft.Outlook", "Teams", "Microsoft.Teams", "Microsoft.Windows.Client.WebExperience",
        "Microsoft.XboxApp", "Microsoft.XboxGameCallableUI", "Microsoft.XboxIdentityProvider",
        "XboxGamingOverlay", "GameBar"
    ];

    for app in &apps {
        run_command(&format!("PowerShell -Command \"Get-AppxPackage -allusers *{}* | Remove-AppxPackage\"", app))?;
    }
    
    Ok(())
}

/* System Services Configuration */
fn step_system_services_config() -> Result<(), String> {
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
        ("WSearch", "demand"),
        ("TabletInputService", "disabled"),
        ("WMPNetworkSvc", "disabled"),
        ("Fax", "disabled"),
        ("XboxGipSvc", "demand"),
        ("UDCService", "demand"),
        ("XboxNetApiSvc", "demand")
    ];

    for (service, start_type) in &services {
        configure_service(service, start_type)?;
    }
    Ok(())
}

fn configure_service(service: &str, start_type: &str) -> Result<(), String> {
    // First check if service exists
    let check_cmd = format!("sc query {} >nul 2>&1", service);
    match run_command(&check_cmd) {
        Ok(_) => {
            // Service exists, configure it
            run_command(&format!("sc config {} start={}", service, start_type))?;
            
            if start_type == "disabled" {
                // Try to stop it, but don't fail if it's already stopped
                let _ = run_command(&format!("sc stop {}", service));
            }
            Ok(())
        }
        Err(_) => {
            // Service doesn't exist, just skip it
            println!("[INFO] Service {} not found, skipping", service);
            Ok(())
        }
    }
}

fn run_command(command: &str) -> Result<(), String> {
    println!("{}", command);
    
    let normalized_command = if command.to_lowercase().starts_with("powershell") {
        let parts: Vec<&str> = command.splitn(2, "-Command").collect();
        if parts.len() == 2 {
            parts[1].trim().trim_matches('"').to_string()
        } else {
            command.to_string()
        }
    } else {
        command.to_string()
    };
    
    // Check if is a PowerShell command
    let is_powershell = normalized_command.contains("Get-AppxPackage") || 
                       normalized_command.contains("Remove-AppxPackage") ||
                       normalized_command.contains("|") ||
                       normalized_command.contains("$") ||
                       normalized_command.contains("{");
    
    if is_powershell {
        // PowerShell commands
        Command::new("powershell")
            .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &normalized_command])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("Failed to execute PowerShell command '{}': {}", normalized_command, e))?;
        
        Ok(())
    } else {
        // for other commands use cmd /C
        Command::new("cmd")
            .args(&["/C", &normalized_command])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("Execution error '{}': {}", normalized_command, e))?;
        
        Ok(())
    }
}