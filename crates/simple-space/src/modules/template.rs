use bevy::prelude::*;

pub trait Template {
    type Prop;
    fn construct(cmd: &mut Commands, prop: Self::Prop) -> Entity;
}

pub trait TemplateExt {
    fn template<T: Template>(&mut self, prop: T::Prop) -> Entity;
}

impl TemplateExt for Commands<'_, '_> {
    fn template<T: Template>(&mut self, prop: T::Prop) -> Entity {
        T::construct(self, prop)
    }
}
