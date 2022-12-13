mod states {
    pub mod app;
    pub mod game;
    pub mod state;
}

mod game_objects {
    pub mod game_object;
}

use {
    ggez::{
        conf::{WindowMode, WindowSetup},
        event::run,
        ContextBuilder,
    },
    states::{app::App, game::Game, state::Action},
    std::path::PathBuf,
};

fn main() {
    let mut asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    asset_path.push("assets");

    let ctx_builder = ContextBuilder::new(App::ID, App::AUTHOR);
    let win_mode = WindowMode::default().dimensions(App::WIDTH, App::HEIGHT);
    let win_setup = WindowSetup::default().title(App::ID);

    let (ctx, e_loop) = ctx_builder
        .add_resource_path(asset_path)
        .window_mode(win_mode)
        .window_setup(win_setup)
        .build()
        .unwrap();

    let mut app = App::new();
    app.add_action(Action::Create(Box::new(Game::new())));

    run(ctx, e_loop, app);
}
