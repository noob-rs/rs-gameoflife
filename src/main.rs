use nannou::prelude::*;

mod gameoflife;

const CELLS_COUNT: (u32, u32) = (30, 30);

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    last_update: f32,
    last_update_keys: f32,
    last_update_mouse: f32,
    iteration_cycle: f32,
    game: gameoflife::Game,
    running: bool,
}

fn model(app: &App) -> Model {
    let m = Model {
        _window: app.new_window().view(view).build().unwrap(),
        last_update: 0.0,
        last_update_keys: 0.0,
        last_update_mouse: 0.0,
        iteration_cycle: 0.5,
        game: gameoflife::Game::new(CELLS_COUNT.0, CELLS_COUNT.1),
        running: false,
    };

    for (x, y) in [(0, 3), (1, 3), (2, 3), (2, 2), (1, 1)] {
        m.game.cells[(y * CELLS_COUNT.0 + x) as usize]
            .borrow_mut()
            .alive = true;
    }

    m
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let current_time = app.duration.since_start.as_secs_f32();

    if app.keys.down.len() > 0 && current_time - model.last_update_keys > 0.05 {
        model.last_update_keys = current_time;
        if app.keys.down.contains(&Key::Plus) {
            model.iteration_cycle /= 1.1;
        } else if app.keys.down.contains(&Key::Minus) {
            model.iteration_cycle *= 1.1;
        }
        if app.keys.down.contains(&Key::Space) {
            model.running = !model.running;
        }
    }

    if app.mouse.buttons.left().is_down() && current_time - model.last_update_mouse > 0.5 {
        model.last_update_mouse = current_time;
        for cell in model.game.cells.iter() {
            let mut cell = cell.borrow_mut();
            if cell.drawrect.contains(app.mouse.position()) {
                cell.alive = !cell.alive;
            }
        }
    }

    if model.running && current_time - model.last_update > model.iteration_cycle {
        model.game.iteration();
        model.last_update = current_time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let drawrect = app.window_rect().pad(30.0);
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.text(format!("Cycle time: {:.2}s", model.iteration_cycle).as_str())
        .xy(app.window_rect().mid_top() + vec2(0.0, -10.0))
        .font_size(15)
        .color(BLACK);

    let cellsize_x = drawrect.w() / CELLS_COUNT.0 as f32;
    let cellsize_y = drawrect.h() / CELLS_COUNT.1 as f32;

    for cell in model.game.cells.iter() {
        let mut cell = cell.borrow_mut();
        cell.drawrect = Rect::from_x_y_w_h(
            drawrect.left() + cell.index.0 as f32 * cellsize_x + cellsize_x / 2.0,
            drawrect.top() + cell.index.1 as f32 * -cellsize_y - cellsize_y / 2.0,
            cellsize_x,
            cellsize_y,
        );
        draw.rect()
            .xy(cell.drawrect.xy())
            .wh(cell.drawrect.wh())
            .color(if cell.alive { BLACK } else { WHITE })
            .stroke_weight(1.0)
            .stroke(LIGHTGRAY);
        // draw.text(format!("{:?}", cell.index).as_str())
        //     .xy(cell.drawrect.top_left() + vec2(20.0, -10.0))
        //     .font_size(10)
        //     .color(if cell.alive { WHITE } else { BLACK });
    }

    draw.to_frame(app, &frame).unwrap();
}
