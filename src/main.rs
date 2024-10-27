use std::marker::PhantomData;

use bevy::{
    ecs::system::StaticSystemParam,
    prelude::*,
    render::{render_resource::AsBindGroup, Render, RenderApp},
};

#[derive(AsBindGroup, TypePath, Debug, Clone, Default, Asset)]
pub struct MyMaterial {
    #[uniform(0)]
    a_number: f32,
    #[uniform(0)]
    _padding: Vec3,
}

pub trait SomeMaterial: AsBindGroup + Asset + Clone + Sized {}

impl SomeMaterial for MyMaterial {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CustomRenderPlugin::<MyMaterial>::default());
}

pub struct CustomRenderPlugin<M: SomeMaterial>(PhantomData<M>);

impl<M: SomeMaterial> Default for CustomRenderPlugin<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<M: SomeMaterial> Plugin for CustomRenderPlugin<M>
where
    M::Data: PartialEq + Eq + Clone,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<M>();
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, foo::<M>);
        }
    }
}

fn foo<M: SomeMaterial>(_material: &M, _param: StaticSystemParam<M::Param>) {
    // SystemParam now required to call the default implementation of as_bind_group e.g.
    // let Ok(bg) = material.as_bind_group(&layout, render_device, param) else {
    //     return;
    // };
}
