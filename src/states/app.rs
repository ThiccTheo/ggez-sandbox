use {
    super::state::{Action, State},
    ggez::{event::EventHandler, Context},
    std::collections::VecDeque,
};

pub struct App {
    states: Vec<Box<State>>,
    actions: VecDeque<Action>,
}

impl App {
    pub const WIDTH: f32 = 800.0;
    pub const HEIGHT: f32 = 600.0;
    pub const ID: &str = "GGEZ Sandbox";
    pub const AUTHOR: &str = "Theo Lee";

    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            actions: VecDeque::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }
}

impl EventHandler<()> for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ()> {
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::Create(state) => self.states.push(state),
                Action::Destroy => drop(self.states.pop()),
                Action::Change(state) => {
                    self.states.pop();
                    self.states.push(state);
                }
            }
        }

        let cur_state = self.states.last_mut().unwrap();
        cur_state.update(ctx).unwrap();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
        let cur_state = self.states.last_mut().unwrap();

        if let Err(action) = cur_state.draw(ctx) {
            self.add_action(action);
        }

        Ok(())
    }
}
