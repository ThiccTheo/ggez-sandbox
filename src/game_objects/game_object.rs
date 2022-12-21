use {
    ggez::{
        graphics::{Canvas, InstanceArray},
        Context,
    },
    std::{any::Any, iter::Chain, slice::IterMut},
};

pub trait GameObject {
    fn id(&self) -> &'static str;
    fn is_active(&self) -> bool;
    fn update(&mut self, others: View, addons: &mut Vec<Box<dyn GameObject>>, ctx: &mut Context, dt: f32);
    fn draw(&mut self, others: View, batch: &mut InstanceArray, ctx: &mut Context, canvas: &mut Canvas);
    fn as_any(&mut self) -> &mut dyn Any;
}

pub fn cast<T>(obj: &mut dyn Any) -> &T
where
    T: 'static,
{
    obj.downcast_mut().unwrap()
}

pub type View<'a> = Chain<IterMut<'a, Box<dyn GameObject>>, IterMut<'a, Box<dyn GameObject>>>;

pub fn create_view(
    game_objects: &mut [Box<dyn GameObject>],
    idx: usize,
) -> (&mut Box<dyn GameObject>, View) {
    let (before, tmp) = game_objects.split_at_mut(idx);
    let (this, after) = tmp.split_first_mut().unwrap();
    let others = before.iter_mut().chain(after.iter_mut());
    (this, others)
}
