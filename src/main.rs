use nannou::{geom::Polygon, prelude::*};

use crate::model::{Light, Model};

mod model;
fn model(app: &App) -> Model {
    app.new_window().view(view).event(event).build().unwrap();
    Model {
        light: Light::new(Vec2::ZERO),
        walls: Vec::new(),
        wall_building: None,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MousePressed(MouseButton::Left) => model.light.pos = app.mouse.position(),
        WindowEvent::MousePressed(MouseButton::Right) if app.keys.down.contains(&Key::LShift) => {
            if let Some(wall_building) = &model.wall_building {
                model.walls.push(Polygon {
                    points: wall_building.clone(),
                });
                model.wall_building = None
            }
        }
        WindowEvent::MousePressed(MouseButton::Right) => match model.wall_building.as_mut() {
            Some(wall_building) => {
                wall_building.push(app.mouse.position());
            }
            None => model.wall_building = Some(vec![app.mouse.position()]),
        },
        WindowEvent::MouseMoved(pos) => {
            if app.mouse.buttons.left().is_down() {
                model.light.pos = pos;
            }
        }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.ellipse()
        .xy(model.light.pos)
        .z(1.)
        .radius(model.light.radius)
        .color(YELLOW);

    let window_rect = app.main_window().rect();

    let shadows = model.get_shadow_polygons(window_rect);

    for polygon in shadows {
        draw.polygon().points(polygon.points).z(2.).color(BLACK);
    }

    for wall in &model.walls {
        draw.polygon()
            .points(wall.points.clone())
            .z(3.)
            .color(DARKGRAY);
    }

    if let Some(wall_building) = &model.wall_building {
        draw.path()
            .stroke()
            .weight(2.)
            .points(wall_building.clone());
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}

fn main() {
    nannou::app(model).update(update).run();
}
