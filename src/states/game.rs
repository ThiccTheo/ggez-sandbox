use {
    super::state::Action,
    crate::game_objects::game_object::{create_view, GameObject},
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, InstanceArray},
        Context,
    },
    std::collections::HashMap,
};

pub struct Game {
    game_objects: Vec<Box<dyn GameObject>>,
    batches: HashMap<&'static str, InstanceArray>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_objects: Vec::new(),
            batches: HashMap::new(),
        }
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();

        for i in 0..self.game_objects.len() {
            let (this, others) = create_view(&mut self.game_objects, i);
            this.update(ctx, dt, others);
        }

        self.game_objects.retain(|obj| obj.is_active());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        for obj in self.game_objects.iter_mut() {
            if let Some(batch) = self.batches.get_mut(obj.id()) {
                obj.draw(ctx, &mut canvas, batch);
            }
        }

        for batch in self.batches.values_mut() {
            batch.clear();
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        Ok(())
    }
}
