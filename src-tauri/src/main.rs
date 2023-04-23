// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Tokio
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn_blocking;
use tokio::sync::Mutex;
use tokio::runtime::Runtime;

// Tauri
use tauri::{Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::Builder;
use tauri::AppHandle;

// Keyboard
use rdev::{listen, EventType, Key as RdevKey};
use unicode_categories::UnicodeCategories;
use enigo::{Enigo, KeyboardControllable, Key as EnigoKey};

// Requests
use reqwest::Client;

// JSON & settings
use serde_json::json;
use serde::{Serialize, Deserialize};

/* FOR ENCRYPTION - NOT USED NOW
use aes_gcm::{
    aead::{ AeadInPlace, KeyInit},
     Aes256Gcm, Nonce,  Or `Aes128Gcm`
};
use rand::Rng;*/


// Other
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::fs;
use std::path::Path;
use once_cell::sync::OnceCell;

// VARIABLES
const SETTINGS_FILE: &str = "config.json";

#[derive(Default)]
struct AppState {
    buffer: String,
    trigger_buffer: String,
    trigger_detected: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct Settings {
    api_key: String,
    start_trigger: String,
    end_trigger: String,
    use_input_box: bool,
    max_tokens: i16,
    temperature: f32
}

impl Settings {
    fn new() -> Self {
        Self {
            api_key: String::new(),
            start_trigger: String::from("@gpto"),
            end_trigger: String::from(":"),
            use_input_box: false,
            max_tokens: 1024,
            temperature: 0.5
        }
    }
}

/*******
*
*
* STARTED IMPLEMENTATION OF ENCRYPTED Settings
*
*
*********/

// const ENCRYPTION_KEY: &[u8; 32] = &[
//     0x7b, 0x2a, 0x9f, 0x76, 0x82, 0x5d, 0x1c, 0x33,
//     0x51, 0x42, 0x7e, 0x8a, 0x63, 0x3b, 0x4d, 0x12,
//     0x0f, 0x88, 0x6a, 0x9b, 0x29, 0xc5, 0xd8, 0xee,
//     0xfa, 0x35, 0x90, 0x14, 0x17, 0x21, 0x44, 0x73,
// ];

// fn encrypt_settings(settings: &Settings, key: &[u8; 32]) -> Result<Vec<u8>, String> {
//     let cipher = Aes256Gcm::new(key.into());
//     let nonce = rand::thread_rng().gen::<[u8; 12]>();
//     let serialized_settings = serde_json::to_vec(settings).map_err(|e| e.to_string())?;

//     println!("Serialized settings: {:?}", serialized_settings);

//     let mut buffer: Vec<u8> = Vec::from(serialized_settings);
//     buffer.resize(buffer.len() + 16, 0);

//     cipher.encrypt_in_place(Nonce::from_slice(&nonce), b"", &mut buffer)
//         .map_err(|_| "Encryption failed")?;

//     let mut encrypted_data = nonce.to_vec();
//     encrypted_data.extend(buffer);
//     Ok(encrypted_data)
// }

// fn decrypt_settings(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Settings, String> {
//     if encrypted_data.len() < 12 + 16 {
//         return Err("Invalid encrypted data".to_string());
//     }

//     let cipher = Aes256Gcm::new(key.into());
//     let nonce = &encrypted_data[..12];
//     let ciphertext = &encrypted_data[12..];

//     let mut buffer: Vec<u8> = Vec::new();
//     buffer.extend_from_slice(ciphertext);

//     cipher.decrypt_in_place(Nonce::from_slice(nonce), b"", &mut buffer)
//         .map_err(|_| "Decryption failed")?;

//     let decrypted_data = buffer.into_iter().filter(|&byte| byte != 0).collect::<Vec<u8>>();
//     println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted_data));

//     let settings = serde_json::from_slice::<Settings>(&decrypted_data)
//         .map_err(|e| e.to_string())?;
//     Ok(settings)
// }


// fn read_settings_from_file(key: &[u8; 32]) -> Result<Settings, String> {
//     let mut file = match OpenOptions::new().read(true).write(true).create(true).open(CONFIG_FILE) {
//         Ok(f) => f,
//         Err(e) => {
//             return Err(format!("Error opening or creating file: {}", e));
//         }
//     };

//     let mut encrypted_data = Vec::new();
//     let read_result = file.read_to_end(&mut encrypted_data);

//     if let Err(e) = read_result {
//         // Create default settings only in that case
//         let default_settings = Settings {
//             api_key: String::from(""),
//             start_trigger: String::from("@gpto"),
//             end_trigger: String::from("\\"),
//             use_input_box: false,
//         };

//         // File might be empty or unreadable, write default settings
//         write_settings_to_file(&default_settings, key)?;

//         // Read the encrypted data from the file again
//         file.read_to_end(&mut encrypted_data)
//             .map_err(|e| format!("Error reading from file: {}", e))?;
//     }

//     // Settings where at least created in case of error
//     decrypt_settings(&encrypted_data, key)
// }

// fn write_settings_to_file(settings: &Settings, key: &[u8; 32]) -> Result<(), String> {
//     let encrypted_data = encrypt_settings(settings, key)?;
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(CONFIG_FILE)
//         .map_err(|e| format!("Error opening file: {}", e))?;
//     file.write_all(&encrypted_data)
//         .map_err(|e| format!("Error writing to file: {}", e))?;
//     Ok(())
// }



/***********************************************************
*
* write the settings from the file
*
* **********************************************************/
async fn write_settings_to_file(settings: &Settings /*, window: tauri::Window*/) -> Result<(), String> {
    println!("We are writing the settings");
    let serialized_settings = serde_json::to_string(settings).map_err(|e| e.to_string())?;
    fs::write(SETTINGS_FILE, serialized_settings).map_err(|e| e.to_string())?;
    Ok(())
}

/***********************************************************
*
* Reading the settings from the file
*
* **********************************************************/
async fn read_settings_from_file() -> Result<Settings, String> {
    if Path::new(SETTINGS_FILE).exists(){
        let settings_data = fs::read_to_string(SETTINGS_FILE).map_err(|e| e.to_string())?;
        let settings: Settings = serde_json::from_str(&settings_data).map_err(|e| e.to_string())?;
        Ok(settings)
    } else {
        let settings = Settings::new();
        write_settings_to_file(&settings).await.map_err(|e| e.to_string())?;
        Ok(settings)
    }
}


/* ***********************************************
*
* @function: save_settings()
* @goal: use when settings from front end are changed
*
************************************************ */
#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String> {
    write_settings_to_file(&settings).await.map_err(|e| e.to_string())
}

/* ***********************************************
*
* @function: load_settings()
* @goal: use when settings are loaded at start and
*       when settings windows open
*
************************************************ */
#[tauri::command]
async fn load_settings() -> Result<Settings, String> {
    read_settings_from_file().await.map_err(|e| e.to_string())
}

/* ***********************************************
*
* @function: submit_text()
* @goal: use when the input windows is used
*
************************************************ */
// TODO this method should be use from the input windows
// when click on "Send" input windows should close
// text should be sent to send_to_gpt_api() method
// Issue: the main loop wait for the end_trigger_word to be typed
#[tauri::command]
async fn submit_text(text: String) -> Result<String, String>  {
    println!("User entered text: {}", text);
    // if text is ok, send it to chat gpt
    let settings = read_settings_from_file().await?;
    println!("End trigger should be pressed: {}", settings.end_trigger);

    // This part is not working
    // So maybe we can just call the 
    let result = send_to_gpt_api(text).await;
    // let mut keyboard = Enigo::new();
    // let response = format!(" {}", settings.end_trigger);
    // keyboard.key_sequence(&response);
    // keyboard.key_sequence("Hello world");
    match result {
        Ok(output) => {
            let response = format!(" {}", output);
            Ok(response)
        }
        Err(e) => {
            let err_string = format!("Error: {}", e.to_string());
            eprintln!("{}", err_string);
            Ok(err_string)
        }
    }

}

/* ***********************************************
*
* @function: hide_current_window()
* @goal: use to hide the current windows instead of closing it
*
************************************************ */
#[tauri::command]
async fn hide_it(window: tauri::Window) {
    window.hide().unwrap();
}


/* ***********************************************
*
* @function: is_printable()
* @goal: check if a c: char is printable
*
************************************************ */
fn is_printable(c: char) -> bool {
    c.is_letter()
        || c.is_mark()
        || c.is_number()
        || c.is_punctuation()
        || c.is_symbol()
        || c.is_whitespace()
}

/* ***********************************************
*
* @function: select_all_and_delete()
* @goal: select all the text typed and delete it
*
************************************************ */
fn select_all_and_delete(keyboard: &mut Enigo) {
    thread::sleep(Duration::from_millis(50));

    // Press Ctrl + A
    keyboard.key_down(EnigoKey::Control);
    keyboard.key_click(EnigoKey::Layout('a'));
    keyboard.key_up(EnigoKey::Control);

    thread::sleep(Duration::from_millis(50));

    // Press Backspace
    keyboard.key_click(EnigoKey::Backspace);
}


/* ***********************************************
*
* @function: send_to_gpt_api()
* @goal: take the text: String input and send it to chatGPT
*
************************************************ */
async fn send_to_gpt_api(text: String) -> Result<String, Box<dyn std::error::Error>> {
    let settings = read_settings_from_file().await?;
    let client = Client::new();
    let api_key = settings.api_key;
    let url = "https://api.openai.com/v1/completions";
    let payload = json!({
        "model": "text-davinci-003",
        "prompt": text,
        "max_tokens": settings.max_tokens,
        "n": 1,
        "stop": null,
        "temperature": settings.temperature
    });

    let response = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            println!("Request error: {}", e);
            e
        })?;
    
    let json_response = response.json::<serde_json::Value>().await?;

    // handle results form Chat GPT
    if let Some(completions) = json_response.get("choices") {
        if let Some(first_completion) = completions.get(0) {
            if let Some(text) = first_completion.get("text") {
                return Ok(text.as_str().unwrap().to_string());
            } 
        } 
    }

    Err("Failed to retrieve response from GPT API".into())
}

/* ***********************************************
*
* @function: start_keylogger()
* @goal: handling all the keystrokes from the user
*
************************************************ */
async fn start_keylogger(app_handle: AppHandle, app_settings: Arc<RwLock<Settings>>){
    let (tx, mut rx) = unbounded_channel();

    // creating the app_state
    let app_state = Arc::new(Mutex::new(AppState {
        buffer: String::from(""), // To store the prompt, question to send to chat GPT
        trigger_buffer: String::from(""), // Where we store the keystorke to detect start trigger word
        trigger_detected: false, // When we have detected the start trigger word
    }));

    let mut app_state = app_state.lock().await;

    // GETTING THE APP SETTINGS
    let settings = {
        let settings_read = app_settings.read().unwrap();
        settings_read.clone()
    };

    let first_char_clone = settings.start_trigger.clone();
    let first_char = &first_char_clone[0..1];

    // SAVING SETTINGS TO LOCAL VARIABLES
    let start_trigger = settings.start_trigger.clone();
    let end_trigger = settings.end_trigger.clone(); 
    let use_input_box = settings.use_input_box.clone();

    drop(settings); // Closing settings read lock

    spawn_blocking(move || {
        if let Err(err) = listen(move |event| {
            if let Err(_) = tx.send(event) {
                println!("Error sending event to the receiver");
                std::process::exit(0);
            }
        }) {
            println!("Error starting keylogger: {:?}", err);
        }
    });

    // KEYSTROKES LOOP
    loop {
        if let Some(event) = rx.recv().await {
            if let Some(ref string) = event.name {
                if app_state.trigger_detected != true { // trigger word has not been detected
                    if *string == first_char // we check for first char of the start word
                     || app_state.trigger_buffer.len() > start_trigger.len() {
                        // We need to clear when the length is bigger and alos when the first char is 
                        // detected
                        app_state.trigger_buffer.clear(); 
                    }
                    if string.chars().all(is_printable) { // We only check for printable keystrokes
                        app_state.trigger_buffer.push_str(string);
                    }
                }
                // check if the trigger buffer contain our start trigger
                if app_state.trigger_buffer == start_trigger { 
                    app_state.trigger_detected = true; // Yes -> we set to true
                    app_state.trigger_buffer.clear(); // clear trigger buffer, not needed any more

                    // TODO let user being able to send text using the input windows
                    // For now windows open but when click on send test is not send to chat GPT
                    if use_input_box == true { // check if user want to use input windows based on his settings
                        let input_window = app_handle.get_window("input").unwrap();
                        input_window.show().unwrap();
                    }
                }
                // Here the trigger was detected so we can start record the user prompt
                else if app_state.trigger_detected { 
                    if string.chars().all(is_printable) { // We only add printable keystorkes to the buffer
                        app_state.buffer.push_str(string);
                    }
                    if *string == end_trigger { // When end word trigger is detected it's the end of the prompt
                        if !app_state.buffer.is_empty() {
                            let prompt = app_state.buffer.clone(); // save the prompt
                            app_state.buffer.clear(); // clear everything
                            app_state.trigger_detected = false; // rested trigger detected to false

                            let mut keyboard = Enigo::new(); // create a virtual keyboard
                            let result = send_to_gpt_api(prompt).await; // send prompt to chat gpt
                            thread::sleep(Duration::from_millis(500)); // then wait a bit
                            select_all_and_delete(&mut keyboard); // select all text and prompt and delete

                            // Get the chat GPT response
                            match result {
                                Ok(output) => {
                                    let response = format!(" {}", output);
                                    keyboard.key_sequence(&response); // print using the virtual keyboard
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }

                        }
                    }
                }
            } else if let EventType::KeyRelease(key) = event.event_type {
                if key == RdevKey::Backspace { // when backspace, delete from trigger buffer and buffer
                    app_state.trigger_buffer.pop();
                    app_state.buffer.pop();
                }
            }
        }
    };
}

/* ***********************************************
*
* @function: main()
* @goal: Main Tauri builder function
*
************************************************ */
#[tokio::main]
async fn main() {
    let keylogger_handle = OnceCell::new();
    let rt = Runtime::new().unwrap();

    // loading the settings
    let settings = Arc::new(RwLock::new(read_settings_from_file().await.unwrap()));

    // TRAY MENU ITEMS
    let quit_menu = CustomMenuItem::new("quit".to_string(), "Quit");
    let settings_menu = CustomMenuItem::new("settings".to_string(), "Show Settings");
    let input_window_menu = CustomMenuItem::new("input_window".to_string(), "Show Input");
    let about_menu = CustomMenuItem::new("about".to_string(), "About");

    let tray_menu = SystemTrayMenu::new()
    .add_item(about_menu)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(settings_menu)    
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(input_window_menu)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit_menu);

    // BUILDER 
    Builder::default()
        .invoke_handler(tauri::generate_handler![submit_text, save_settings, load_settings, hide_it])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
        
            // Menu click handeling
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    // main menu
                    "about" => {
                        let window = app.get_window("about").unwrap();
                        window.show().unwrap();
                    }
                    "settings" => {
                        let window = app.get_window("settings").unwrap();
                        window.open_devtools();
                        window.show().unwrap();
                    }
                    "input_window" => {
                        let window = app.get_window("input").unwrap();
                        window.open_devtools();
                        window.show().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {
            // Clone the app_handle inside the closure
            let app_handle = app.app_handle().clone();
            // Clone the shared settings state
            let shared_settings = settings.clone();

            // Spawn the keylogger task only once
            keylogger_handle.get_or_init(|| {
                rt.spawn(start_keylogger(app_handle, shared_settings))
            });
            }
        });


}