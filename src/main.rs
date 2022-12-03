use nannou::prelude::*;

mod gameoflife;

const CELLS_COUNT: (u32, u32) = (10, 10);

fn main() {
    nannou::app(model).update(update).run();
    // let mut game = gameoflife::Game::new(CELLS_COUNT.0, CELLS_COUNT.1);
    // game.cells[3 + 0].borrow_mut().alive = true;
    // game.cells[3 + 1].borrow_mut().alive = true;
    // game.cells[3 + 2].borrow_mut().alive = true;
    // game.iteration();
}

struct Model {
    _window: window::Id,
    last_update: f32,
    game: gameoflife::Game,
}

fn model(app: &App) -> Model {
    let m = Model {
        _window: app.new_window().view(view).build().unwrap(),
        last_update: 0.0,
        game: gameoflife::Game::new(CELLS_COUNT.0, CELLS_COUNT.1),
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
    if current_time - model.last_update > 0.5 {
        // if app.mouse.buttons.pressed().count() > 0 {
        model.game.iteration();
        model.last_update = current_time;
        // }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let drawrect = app.window_rect().pad(20.0);

    let draw = app
        .draw()
        .translate(vec3(drawrect.x.start, -drawrect.y.start, 0.0));
    draw.background().color(WHITE);

    let cellsize_x = drawrect.w() / CELLS_COUNT.0 as f32;
    let cellsize_y = drawrect.h() / CELLS_COUNT.1 as f32;

    for cell in model.game.cells.iter() {
        let cell = cell.borrow();
        draw.rect()
            .x_y(
                cell.index.0 as f32 * cellsize_x + cellsize_x / 2.0,
                cell.index.1 as f32 * -cellsize_y - cellsize_y / 2.0,
            )
            .w_h(cellsize_x, cellsize_y)
            .color(if cell.alive { BLACK } else { WHITE })
            .stroke_weight(1.0)
            .stroke(LIGHTGRAY);
        draw.text(format!("{:?}", cell.index).as_str())
            .x_y(
                cell.index.0 as f32 * cellsize_x + 15.0,
                cell.index.1 as f32 * -cellsize_y - 10.0,
            )
            .font_size(10)
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
