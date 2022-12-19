use {
    super::state::Action,
    crate::game_objects::game_object::{create_view, GameObject},
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, Image, InstanceArray},
        Context,
    },
    std::collections::BTreeMap,
};

pub struct Game {
    game_objects: Vec<Box<dyn GameObject>>,
    batches: BTreeMap<&'static str, InstanceArray>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_objects: Vec::new(),
            batches: BTreeMap::new(),
        }
    }

    fn add_batch(
        ctx: &Context,
        batches: &mut BTreeMap<&'static str, InstanceArray>,
        id: &'static str,
    ) {
        let batch = InstanceArray::new(
            &ctx.gfx,
            Image::from_path(&ctx.gfx, format!("\\{id}.png")).unwrap(),
        );
        batches.insert(id, batch);
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();
        let mut addons = Vec::<Box<dyn GameObject>>::new();

        for i in 0..self.game_objects.len() {
            let (this, others) = create_view(&mut self.game_objects, i);
            addons.extend(this.update(ctx, dt, others));
        }

        self.game_objects.retain(|obj| obj.is_active());
        self.game_objects.extend(addons);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        for obj in self.game_objects.iter_mut() {
            if let Some(batch) = self.batches.get_mut(obj.id()) {
                obj.draw(ctx, &mut canvas, batch);
            }
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        for batch in self.batches.values_mut() {
            batch.clear()
        }

        Ok(())
    }
}
