#![windows_subsystem = "windows"]

use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,PlatformError
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    let label = Label::new(|data: &u32, _env: &_| format!("Hello World {}", data))
        .with_text_color(Color::rgb8(0, 0, 0))
        .with_text_size(24.0);

    let background = Painter::new(|ctx, _, env| {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &env.get(theme::BACKGROUND_LIGHT));
    });

    Flex::column()
        .with_child(background)
        .with_child(label)
        .cross_axis_alignment(CrossAxisAlignment::Center)

}