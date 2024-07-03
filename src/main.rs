//
// Termint - Terminal Emulator
// Author: Henrique Dias
// Last Modification: 2024-07-04 00:15:51
//
// References:
// https://stackoverflow.com/questions/72114626/why-gtk4-seems-to-use-only-48x48-icons-for-displaying-minimized-application-in-a/
// https://stackoverflow.com/questions/71847278/gtk-4-and-applications-icons-how-to-include-an-application-icon-in-a-portable
// https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html
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
    TerminalExt
};

use std::{
    env,
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process
};

use clap::{Command, Arg, ArgAction, value_parser};
use ini::{Ini, Properties};

// use std::sync::{Arc, Mutex};

// static mut BUFFER: String = String::new();

const APP_ID: &str = "org.gtk_rs.Termint";
const APP_NAME: &str = "termint";
const APP_TITLE: &str = "Termint";
const VERSION: &str = "0.0.1";

fn make_terminal(cmd: &str) -> Terminal {
    // https://python-forum.io/thread-16720.html
    let terminal = Terminal::new();

    // set terminal font from a string
    // let font_description = pango::FontDescription::from_string("monospace 10");
    // terminal.set_font_desc(Some(&font_description));

    let flags = PtyFlags::DEFAULT;

    let working_directory = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut args = vec![];

    // get the user's shell
    let shell = match env::var("SHELL") {
        Ok(shell) => shell,
        Err(err) => {
            panic!("unabled to get the user's shell: {}", err);
        }
    };

    args.push(shell.as_str());
    if !cmd.is_empty() {
        args.push("-c");
        args.push(cmd);
    }

    let envv = vec![];

    let spawn_flags = gtk4::glib::SpawnFlags::SEARCH_PATH | gtk4::glib::SpawnFlags::DO_NOT_REAP_CHILD;
    let child_setup = || {
        // get the user
        match env::var("USER") {
            Ok(user) => println!("Welcome {}!", user),
            Err(err) => println!("unabled to get the user: {}", err)
        };
    };
    let timeout = -1;
    let cancellable = Cancellable::new();
    let cancellable_ref = Some(&cancellable);
    let callback = |_pid| {
         // println!("pid {:?}", pid);
         return;
    };

    // Spawn a new PTY
    let pty = Pty::new_sync(flags, cancellable_ref)
        .expect("Failed to create PTY");

    pty.spawn_async(
        Some(&working_directory), // working_directory
        &args,               // argv
        &envv,                    // envv
        spawn_flags,              // spawn_flags
        child_setup,              // child_setup,
        timeout,                  // timeout
        cancellable_ref,          // cancellable
        callback,                 // callback
    );

    terminal.set_pty(Some(&pty));

    return terminal;
}

fn build_ui(settings: &Properties, command: &str) {

    let default_width: u32 = settings.get("default_width")
        .unwrap()
        .parse()
        .expect("default_width value conversion error");
    let default_height: u32 = settings.get("default_height")
        .unwrap()
        .parse()
        .expect("default_height value conversion error");
    let styles_file = settings.get("styles_file")
        .unwrap()
        .to_string();
    let icon_name = settings.get("icon_name")
        .unwrap()
        .to_string();

    let cmd = command.to_string();

    // https://lazka.github.io/pgi-docs/Gio-2.0/flags.html
    let flags = if cmd.is_empty() {
        Default::default()
    } else {
        gio::ApplicationFlags::NON_UNIQUE
    };

    let application = Application::builder()
        .application_id(APP_ID)
        .flags(flags)
        .build();

    // let application = Application::new(
    //    Some(APP_ID),
    //     // Default::default(),
    //     gio::ApplicationFlags::NON_UNIQUE,
    // );

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_TITLE)
            .default_width(default_width as i32)
            .default_height(default_height as i32)
            .width_request(default_width as i32)
            .height_request(default_height as i32)
            .build();

        window.connect_destroy(|_| {
            println!("Window destroyed.");
        });

        window.set_icon_name(Some(&icon_name));

        let css_provider = CssProvider::new();
        css_provider.load_from_path(&styles_file);

        let scrolled_window = ScrolledWindow::builder().build();
        scrolled_window.set_policy(
            gtk4::PolicyType::Never,
            gtk4::PolicyType::Automatic);

        let sw_style_context = scrolled_window.style_context();
        sw_style_context.add_class("scrolled-window");
        sw_style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let terminal = make_terminal(&cmd);

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
    });

    let empty: Vec<String> = vec![];
    application.run_with_args(&empty);

    // application.run();

}

fn default_styles_file(file_path: &PathBuf) {

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("failed to create file: {}", err);
        }
    };

    let styles_content = "
.scrolled-window {
    /* background-image: url(\".config/{}/background.jpg\"); */
    background-size: cover;
    background-repeat: no-repeat;
    background-position: center;
    background-color: rgba(255, 255, 255, 0);
}
.terminal {
    opacity: 0.92;
    font-size: 12px;
    font-family: monospace;
}".replace("{}", APP_NAME);

    if let Err(err) = file.write_all(styles_content.as_bytes()) {
        panic!("failed to write to file: {}", err);
    }

}

fn default_config_file(config_dir: &PathBuf) {

    let mut conf = Ini::new();

    conf.with_section(None::<String>)
        .set("encoding", "utf-8");

    let styles_file = config_dir.join("styles.css");

    if !config_dir.join("styles.css").exists() {
        default_styles_file(&styles_file);
    }

    conf.with_section(Some("Settings"))
        .set("default_width", "680")
        .set("default_height", "364")
        .set("icon_name", "computer")
        .set("styles_file", styles_file.to_str().unwrap().to_string());

    conf.write_to_file(config_dir.join(format!("{}.ini", APP_NAME))).unwrap();
}

fn main() {

    let matches = Command::new(APP_NAME)
        .version(VERSION)
        .about("Minimal terminal emulator with mint flavor!")
        .arg(
            Arg::new("directory")
                .help("Sets a custom settings directory")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
                .value_parser(value_parser!(PathBuf))
                .required(false))
        .arg(
            Arg::new("init")
                .help("Create the directory with the default settings if they do not exist")
                .short('i')
                .long("init")
                .required(false)
                .action(ArgAction::SetTrue)) // set true if the arg is added
        .arg(
            Arg::new("command")
                .help("Execute the specified command")
                .short('c')
                .long("command")
                .value_name("COMMAND")
                .value_parser(value_parser!(PathBuf))
                .required(false)
        ).get_matches();

    let path = matches.get_one::<PathBuf>("command");

    // TODO: Check if the path is an executable file.
    let cmd = match path {
        Some(path) => {
            if path.exists() && path.is_file() {
                path.to_string_lossy().to_string()
            } else {
                eprintln!("the path {} is not valid", path.display());
                process::exit(1);
            }
        },
        None => "".to_string(),
    };

    let config_dir = || -> PathBuf {
        let custom_dir = matches.get_one::<PathBuf>("directory");
        if !custom_dir.is_none() {
            let dir_path = custom_dir.unwrap().display().to_string();
            return Path::new(&dir_path).join(APP_NAME);
        }
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
    }();

    let create: bool = *matches.get_one("init").unwrap();

    let ini_file = config_dir.join(format!("{}.ini", APP_NAME));
    if create {
        if !config_dir.is_dir() {
            if let Err(err) = fs::create_dir_all(&config_dir) {
                panic!("failed to create configuration directory: {}", err);
            }
        }

        if !config_dir.join(&ini_file).exists() {
            default_config_file(&config_dir);
        }
    }

    if !config_dir.join(&ini_file).exists() {
        default_config_file(&config_dir);
    }

    let config = match Ini::load_from_file(&ini_file) {
        Ok(config) => config,
        Err(err) => {
            panic!("failed to parse config file: {}", err);
        }
    };

    let settings = config.section(Some("Settings")).unwrap();

    build_ui(settings, &cmd);
}
