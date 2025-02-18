use bevy::prelude::*;

pub trait Template {
    type Prop;
    fn construct(cmd: EntityCommands<'_>, prop: Self::Prop) -> EntityCommands<'_>;
}

pub trait TemplateExt {
    fn template<T: Template>(&mut self, prop: T::Prop) -> EntityCommands<'_>;
}

impl TemplateExt for Commands<'_, '_> {
    fn template<T: Template>(&mut self, prop: T::Prop) -> EntityCommands<'_> {
        let cmd = self.spawn_empty();
        T::construct(cmd, prop)
    }
}

impl TemplateExt for ChildBuilder<'_> {
    fn template<T: Template>(&mut self, prop: T::Prop) -> EntityCommands<'_> {
        let cmd = self.spawn_empty();
        T::construct(cmd, prop)
    }
}
