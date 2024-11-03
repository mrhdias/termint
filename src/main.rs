//
// Termint - Terminal Emulator
// Author: Henrique Dias
// Last Modification: 2024-11-03 11:52:17
//
// References:
// https://stackoverflow.com/questions/72114626/why-gtk4-seems-to-use-only-48x48-icons-for-displaying-minimized-application-in-a/
// https://stackoverflow.com/questions/71847278/gtk-4-and-applications-icons-how-to-include-an-application-icon-in-a-portable
// https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html
//
// ./termint -e "cal -3;echo 'quit? ';read"
//

// use gio::ApplicationFlags;

use gtk4::{
    gio::Cancellable,
    prelude::*,
    Application,
    ApplicationWindow,
    ScrolledWindow,
    CssProvider,
};
use vte4::{
    Pty,
    PtyFlags,
    Terminal,
    TerminalExt,
};

use std::{
    env,
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clap::{
    Command,
    Arg,
    ArgAction,
    value_parser,
};
use ini::Ini;

// const APP_ID: &str = "org.gtk_rs.Termint";
const APP_NAME: &str = "termint";
const APP_TITLE: &str = "Termint";
const VERSION: &str = "0.0.1";

const DEFAULT_WIDTH: usize = 680;
const DEFAULT_HEIGHT: usize = 364;

#[derive(Debug)]
struct AppTerm {
    app_id: String,
    ini_file: PathBuf,
    login_shell: String,
    working_dir: String,
    window_size: (usize, usize),
    command: String,
}

impl AppTerm {

    fn default_style() -> String {
        format!(r#"
.scrolled-window {{
    /* background-image: url(\".config/{}/background.jpg\"); */
    background-size: cover;
    background-repeat: no-repeat;
    background-position: center;
    background-color: rgba(255, 255, 255, 0);
}}
.terminal {{
    opacity: 0.92;
    font-size: 12px;
    font-family: monospace;
}}"#, APP_NAME)
    }

    fn default_styles_file(file_path: &PathBuf) {

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("failed to create file: {}", err);
            }
        };

        if let Err(err) = file.write_all(Self::default_style().as_bytes()) {
            panic!("failed to write to file: {}", err);
        }
    
    }

    fn default_ini(config_dir: Option<&PathBuf>) -> Ini {
        let mut ini = Ini::new();
    
        ini.with_section(None::<String>)
            .set("encoding", "utf-8");

        ini.with_section(Some("Settings"))
            .set("default_width", DEFAULT_WIDTH.to_string())
            .set("default_height", DEFAULT_HEIGHT.to_string())
            .set("icon_name", "computer");

        if let Some(dir) = config_dir {
            let styles_file = dir.join("styles.css");
        
            if !dir.join("styles.css").exists() {
                Self::default_styles_file(&styles_file);
                ini.with_section(Some("Settings"))
                    .set("styles_file", styles_file.to_str().unwrap().to_string());
            }
        }

        ini
    }

    fn make_terminal(
        login_shell: &str,
        working_directory: &str,
        command: &str,
    ) -> Terminal {

        // https://python-forum.io/thread-16720.html
        let terminal = Terminal::new();

        // set terminal font from a string
        // let font_description = pango::FontDescription::from_string("monospace 10");
        // terminal.set_font_desc(Some(&font_description));

        let flags = PtyFlags::DEFAULT;

        let has_cmd = !command.is_empty();

        // Setup command arguments
        let mut argv = vec![];

        argv.push(login_shell);
        if has_cmd {
            argv.push("-c");
            argv.push(&command);
        }

        let envv = vec![]; // Environment variables can be added here if needed

        // Spawn flags and optional child setup
        // let spawn_flags = gtk4::glib::SpawnFlags::SEARCH_PATH | gtk4::glib::SpawnFlags::DO_NOT_REAP_CHILD;
        let spawn_flags = gtk4::glib::SpawnFlags::SEARCH_PATH;

        // println!("has_cmd: {} command: {}", has_cmd, command);
        let child_setup = move || {
            // Child setup (e.g., change the working directory, etc.)
            // get the user
            if !has_cmd {
                if let Ok(user) = env::var("USER") {
                    println!("Welcome {}!", user);
                }
            }
        };

        let timeout = -1; // Set to -1 for no timeout
        let cancellable = Cancellable::new();
        let cancellable_ref = Some(&cancellable);
        let callback = |pid| {
            // Callback after spawn (e.g., handle the pid if needed)
            // println!("pid {:?}", pid);
            if let Err(err) = pid {
                eprintln!("Failed to spawn: {:?}", err);
                std::process::exit(1);
            }
        };

        // Create a new PTY
        let pty = Pty::new_sync(flags, cancellable_ref)
            .expect("Failed to create PTY");

        // Spawn the command asynchronously within the PTY
        // https://gnome.pages.gitlab.gnome.org/vte/gtk4/method.Pty.spawn_with_fds_async.html

        pty.spawn_async(
            if working_directory.is_empty() {
                None
            } else {
                Some(working_directory)
            },
            &argv, // the window closes after executing the command
            &envv,
            spawn_flags,
            child_setup,
            timeout,
            cancellable_ref,
            callback,
        );

        // Link the PTY to the terminal widget
        terminal.set_pty(Some(&pty));

        terminal
    }

    fn create(&self) {

        let ini_file = self.ini_file.clone();
        let (default_width, default_height) = self.window_size;
        let login_shell = self.login_shell.to_owned();
        let working_dir = self.working_dir.to_owned();
        let command = self.command.to_owned();

        // https://lazka.github.io/pgi-docs/Gio-2.0/flags.html

        /*
        let flags = if self.command.is_empty() {
            // Default::default()
            gio::ApplicationFlags::default()
        } else {
            // gio::ApplicationFlags::NON_UNIQUE | gio::ApplicationFlags::default()
            // gio::ApplicationFlags::NON_UNIQUE
            gio::ApplicationFlags::NON_UNIQUE // To not inherit the commands from the first instance
        };
        */
    
        let application = Application::builder()
            // .application_id(APP_ID) // add id from command line
            // .flags(gio::ApplicationFlags::default())
            // .flags(gio::ApplicationFlags::NON_UNIQUE)
            .build();

        if self.app_id.is_empty() {
            application.set_flags(gio::ApplicationFlags::NON_UNIQUE);
        } else {
            // println!("Using application id: {}", &self.app_id);
            application.set_application_id(Some(&format!("org.gtk_rs.{}", self.app_id)));
        }

        application.connect_activate(move |app| {

            let config = if ini_file.exists() {
                match Ini::load_from_file(&ini_file) {
                    Ok(config) => config,
                    Err(err) => {
                        panic!("failed to parse config file: {}", err);
                    }
                }
            } else {
                Self::default_ini(None)
            };

            let settings = match config.section(Some("Settings")) {
                Some(section) => section,
                None => {
                    eprintln!("Error: No settings section found in config.");
                    app.quit();
                    return;
                }
            };

            let win_width = if default_width == 0 {
                settings
                    .get("default_width")
                    .and_then(|width| width.parse::<usize>().ok())
                    .unwrap_or(DEFAULT_WIDTH)
            } else {
                default_width as usize
            };
            let win_width = win_width.max(100); // Ensure the minimum width is 100

            let win_height = if default_height == 0 {
                settings
                    .get("default_height")
                    .and_then(|height| height.parse::<usize>().ok())
                    .unwrap_or(DEFAULT_HEIGHT)
            } else {
                default_height as usize
            };
            let win_height = win_height.max(100); // Ensure the minimum height is 100

            let window = ApplicationWindow::builder()
                .application(app)
                .title(APP_TITLE)
                .default_width(win_width as i32)
                .default_height(win_height as i32)
                .width_request(win_width as i32)
                .height_request(win_height as i32)
                .build();
        
            window.connect_destroy(|_| {
                println!("Window destroyed.");
            });
        
            window.set_icon_name(settings.get("icon_name"));

            let css_provider = CssProvider::new();

            match settings.get("styles_file") {
                Some(styles_file) => {
                    css_provider.load_from_path(&styles_file);
                },
                None => {
                    css_provider.load_from_data(Self::default_style().as_str());
                }
            };
    
            let scrolled_window = ScrolledWindow::builder().build();
            scrolled_window.set_policy(
                gtk4::PolicyType::Never,
                gtk4::PolicyType::Automatic);
    
            let sw_style_context = scrolled_window.style_context();
            sw_style_context.add_class("scrolled-window");
            sw_style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            let terminal = Self::make_terminal(
                &login_shell, 
                &working_dir,
                &command);
    
            let term_style_context = terminal.style_context();
            term_style_context.add_class("terminal");
            term_style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    
            let win = window.clone();
            terminal.connect_window_title_changed(move |terminal| {
                if let Some(title) = terminal.window_title() {
                    // println!("Window title changed: {:?}", title);
                    win.set_title(Some(&title));
                }
            });

            // quit the application
            // let app_clone = app.clone();
            // terminal.connect_eof(move |_terminal|{
            //     app_clone.quit();
            // });

            // quit the application
            let win = window.clone();
            terminal.connect_eof(move |_terminal|{
                win.close();
            });

            // this code is for testing purposes only
            /*
            let app_clone = app.clone();
            // quit the application
            terminal.connect_commit(move |_terminal, input, _s| {
                unsafe {
                    println!("Debug Input: {} Buffer: {:?}", input, BUFFER);
                    if input == "\u{7f}" { // backspace
                        if !BUFFER.is_empty() {
                            BUFFER.pop();
                        }
                        return;
                    }

                    if BUFFER.len() < "exit\r".len() {
                        BUFFER.push_str(input);
                        if input == "\r" {
                            if BUFFER.eq_ignore_ascii_case("exit\r") {
                                println!("Debug Exit Input: {} Buffer: {:?}", input, BUFFER);
                                app_clone.quit();
                            }
                        }
                    }
                    if input == "\r" {
                        BUFFER.clear();
                    }
                }
            });
            */

            /*
            // alternative approach
            let buffer = Arc::new(Mutex::new(String::new()));
            terminal.connect_commit(move |_terminal, input, _s| {
                // println!("Debug Input: {} Buffer: {:?}", input, buffer);
                if buffer.lock().unwrap().len() < "exit\r".len() {
                    buffer.lock().unwrap().push_str(input);
                    if input == "\r" {
                        if buffer.lock().unwrap().eq_ignore_ascii_case("exit\r") {
                            app_clone.quit();
                        }
                    }
                }
                if input == "\r" {
                    buffer.lock().unwrap().clear();
                }
            });
            */

            scrolled_window.set_child(Some(&terminal));
            window.set_child(Some(&scrolled_window));
            // window.show();
            window.present();

            // if !command.is_empty() {
            //     terminal.feed_child(format!("{}\n", &command)
            //         .as_bytes());
            // }
        });

        let empty: Vec<String> = vec![];
        application.run_with_args(&empty);
    
        // application.run();

    }


    fn new(
        app_id: Option<&String>,
        login_shell: Option<&PathBuf>,
        working_dir: Option<&PathBuf>,
        window_size: Option<&String>,
        custom_config_dir: Option<&PathBuf>,
        create_default_settings: Option<&bool>,
        command: Option<&String>,
    ) -> Self {

        let config_dir = if let Some(dir) = custom_config_dir {
            dir.join(APP_NAME)
        } else {
            // get the path to the user's home directory
            let home= match env::var("HOME") {
                Ok(home) => home,
                Err(err) => {
                    panic!("unabled to get the home: {}", err);
                }
            };
    
            Path::new(&home)
                .join(".config")
                .join(APP_NAME)
        };

        let ini_file = config_dir.join(format!("{}.ini", APP_NAME));

        if *create_default_settings.unwrap_or(&false) {
            if !config_dir.is_dir() {
                if let Err(err) = fs::create_dir_all(&config_dir) {
                    panic!("failed to create configuration directory: {}", err);
                }
            }

            if !config_dir.join(&ini_file).exists() {
                // Self::default_ini_file(&config_dir);
                let conf = Self::default_ini(Some(&config_dir));
                conf.write_to_file(config_dir.join(format!("{}.ini", APP_NAME)))
                    .unwrap();
            }
        }

        AppTerm {
            app_id: match app_id {
                Some(id) => id.to_string(),
                None => "".to_string(),
            },
            ini_file,
            login_shell: match login_shell {
                Some(shell) => {
                    if !shell.is_file() {
                        panic!("The specified login shell does not exist: {}", shell.display());
                    }
                    shell.to_string_lossy().to_string()
                },
                None => {
                    env::var("SHELL").unwrap_or_else(|err| {
                        panic!("Unable to get the user's shell: {}", err);
                    })
                },
            },
            command: match command {
                Some(cmd) => cmd.to_string(),
                None => "".to_string(),
            },
            working_dir: match working_dir {
                Some(dir) => {
                    if !dir.is_dir() {
                        panic!("The specified working directory does not exist: {}", dir.display());
                    }
                    dir.to_string_lossy().to_string()
                },
                None => "".to_string(),
            },
            window_size: match window_size {
                Some(size) => {
                    let size_parts: Vec<&str> = size.split('x').collect();
                    if size_parts.len()!= 2 {
                        panic!("Invalid window size: {}", size);
                    }
                    (
                        size_parts[0].parse::<usize>().unwrap_or(DEFAULT_WIDTH),
                        size_parts[1].parse::<usize>().unwrap_or(DEFAULT_HEIGHT),
                    )
                },
                None => (0, 0)
            },
        }
    }
}

fn main() {

    let matches = Command::new(APP_NAME)
        .version(VERSION)
        .about("Minimal terminal emulator with mint flavor!")
        .arg(
            Arg::new("app_id")
                .help(format!("window application ID ({})", APP_NAME))
                .short('a')
                .long("app-id")
                .value_name("ID")
                .value_parser(value_parser!(String))
                .required(false))
        .arg(
            Arg::new("directory")
                .help("Sets a custom settings directory")
                .short('d')
                .long("dir")
                .value_name("PATH")
                .value_parser(value_parser!(PathBuf))
                .required(false))
        .arg(
            Arg::new("init_settings")
                .help("Create the directory with the default settings if they do not exist")
                .short('i')
                .long("init-settings")
                .required(false)
                .action(ArgAction::SetTrue)) // set true if the arg is added
        .arg(
            Arg::new("execute")
                .help("Execute the specified command (for compatibility with xterm -e)")
                .short('e')
                .long("execute")
                .value_name("CMD")
                .value_parser(value_parser!(String))
                .required(false))
        .arg(
            Arg::new("login_shell")
                .help("start shell as a login shell")
                .short('L')
                .long("login-shell")
                .value_name("PATH")
                .value_parser(value_parser!(PathBuf))
                .required(false))
        .arg(
            Arg::new("working_directory")
                .help("directory to start in (CWD)")
                .short('D')
                .long("working-directory")
                .value_name("PATH")
                .value_parser(value_parser!(PathBuf))
                .required(false))
        .arg(
            Arg::new("window_size_pixels")
                .help("initial width and height, in pixels")
                .short('w')
                .long("window-size-pixels")
                .value_name("WIDTHxHEIGHT")
                .value_parser(value_parser!(String))
                .required(false)
        ).get_matches();

    AppTerm::new(
        matches.get_one::<String>("app_id"),
        matches.get_one::<PathBuf>("login_shell"),
        matches.get_one::<PathBuf>("working_directory"),
        matches.get_one::<String>("window_size_pixels"),
        matches.get_one::<PathBuf>("directory"),
        matches.get_one::<bool>("init_settings"),
        matches.get_one::<String>("execute"),
    ).create();
}