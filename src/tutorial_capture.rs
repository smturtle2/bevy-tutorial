use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};

pub fn tutorial_capture_enabled() -> bool {
    matches!(
        std::env::var("BEVY_TUTORIAL_CAPTURE").as_deref(),
        Ok("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}

pub fn add_tutorial_screenshot(app: &mut App, path: impl Into<String>, frame: u32) {
    if tutorial_capture_enabled() {
        app.insert_resource(TutorialScreenshot {
            path: path.into(),
            target_frame: frame,
            current_frame: 0,
            requested: false,
        })
        .add_systems(Update, tutorial_screenshot_system);
    }
}

#[derive(Resource)]
struct TutorialScreenshot {
    path: String,
    target_frame: u32,
    current_frame: u32,
    requested: bool,
}

fn tutorial_screenshot_system(world: &mut World) {
    let mut screenshot = world.resource_mut::<TutorialScreenshot>();

    if screenshot.requested {
        return;
    }

    screenshot.current_frame += 1;

    if screenshot.current_frame < screenshot.target_frame {
        return;
    }

    let path = screenshot.path.clone();
    screenshot.requested = true;
    drop(screenshot);

    world.spawn(Screenshot::primary_window()).observe(
        move |captured: On<ScreenshotCaptured>, mut app_exit: MessageWriter<AppExit>| {
            save_to_disk(path.clone())(captured);
            app_exit.write(AppExit::Success);
        },
    );
}
