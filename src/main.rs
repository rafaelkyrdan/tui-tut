use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, ScrollView, TextView};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut siv = cursive::default();
    create_ui(&mut siv);
    siv.run();
}

fn create_ui(siv: &mut cursive::Cursive) {
    let output = execute_command();

    let content = match output {
        Ok(text) => text,
        Err(e) => format!("Error executing command: {}", e),
    };
    
    let scroll_view = ScrollView::new(
        TextView::new(content)
            .with_name("output_view")
            .fixed_width(80),
    )
    .scroll_x(true)
    .scroll_y(true)
    .min_size((60, 10));

    let buttons = LinearLayout::horizontal()
        .child(cursive::views::Button::new("Refresh", |s| {
            refresh_output(s);
        }))
        .child(cursive::views::Button::new("Quit", |s| s.quit()));

    let timestamp = TextView::new(format_timestamp())
        .with_name("timestamp")
        .fixed_width(80);

    let main_layout = LinearLayout::vertical()
        .child(scroll_view)
        .child(timestamp)
        .child(buttons);

    siv.add_layer(Dialog::around(main_layout).title("Directory Contents"));
}

fn execute_command() -> Result<String, String> {
    Command::new("ls")
        .arg("-la")
        .output()
        .map_err(|e| e.to_string())
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .map_err(|e| e.to_string())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("Command failed: {}", error))
            }
        })
}

fn refresh_output(siv: &mut cursive::Cursive) {
    let output = execute_command();
    
    let content = match output {
        Ok(text) => text,
        Err(e) => format!("Error executing command: {}", e),
    };

    siv.call_on_name("output_view", |view: &mut TextView| {
        view.set_content(content);
    });
    
    siv.call_on_name("timestamp", |view: &mut TextView| {
        view.set_content(format_timestamp());
    });
}

fn format_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let datetime = chrono::DateTime::from_timestamp(now as i64, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    format!("Last updated: {}", datetime)
}