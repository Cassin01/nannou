use nannou::prelude::*;
use rand::prelude::*;
//use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    let length: usize = 40;
    let max_human: usize = 20;
    Model::new(length, max_human, app)
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    for h in &mut model.humans {
        if h.state == State::Live {
            h.update_timer();
        }
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    // キャンバスを取得
    let draw = app.draw();

    // 背景色を設定
    draw.background().color(BLACK);

    // 1辺100の正方形を原点に表示
    //draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

    // フレームに書き出し
    for h in &model.humans {
        if h.state == State::Live {
            println!("{}", h.xpos);
            h.draw(&draw);
        }
    }

    //model.game.iter().enumerate()
    //    .for_each(|(i, cell)| {
    //        let color = match cell {
    //            Cell::Live => WHITE,
    //            Cell::Dead => return,
    //        };
    //        let xi = i % model.grid_size.x;
    //        let yi = i / model.grid_size.x;
    //        let x = model.cell_size/2.0 - model.width/2.0
    //            + (xi as f32) * model.cell_size;
    //        let y = -model.cell_size/2.0 + model.height/2.0
    //            - (yi as f32) * model.cell_size;

    //        draw.ellipse()
    //            .x_y(x, y)
    //            //.w_h(model.cell_size, model.cell_size)
    //            .radius(model.cell_size)
    //            .color(Rgb::new(1.0, 0.0, rand::thread_rng().gen()));
    //        });
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    game: Vec<State>,
    humans: Vec<Human>,

}

impl Model {
    pub fn new(length: usize, max_human: usize, app: &App) -> Self {
        //let width = (grid_size.x as f32) * cell_size;
        //let height = (grid_size.y as f32) * cell_size;
        let mut rng = thread_rng();
        let game = vec![State::Live; length * length];
        let humans = vec![0; max_human].into_iter().map(|_| Human::new(app, &mut rng)).collect();
        Self {
            game,
            humans,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Live,
    Dead,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Stay,
    Light,
    Left,
    Up,
    Down,
}

impl Direction {
    fn new() -> Self {
        match rand::thread_rng().gen_range(0..5) {
            0 => Self::Stay,
            1 => Self::Light,
            2 => Self::Left,
            3 => Self::Up,
            4 => Self::Down,
            _ => {
                println!("Err at direction new()");
                Self::Down
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Human {
    xpos: f32,
    ypos: f32,
    state: State,
    direction: Direction,
    timer: u64,
    hp: u32,
}

impl Human {
    fn new(app: &App, rng: &mut ThreadRng) -> Self {
        let win = app.window_rect();
        Self {
            xpos: rng.gen_range((win.left()+1.)..win.right()) as f32,
            ypos: rng.gen_range((win.bottom()+1.)..win.top()) as f32,
            state: State::Live,
            direction: Direction::new(),
            timer: 0,
            hp: rng.gen_range(500..1000),
        }
    }
    fn update_timer(&mut self) {
        self.timer = self.timer + 1;
    }
    fn draw(&self, draw: &Draw) {
        let xpos = self.xpos;
        let ypos = self.ypos;
        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //match self.timer % 5 {
        //    0 => {
        //        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //        draw.rect().x_y(xpos - 3., ypos + 3.).w_h(6., 5.); // 胴体
        //        draw.rect().x_y(xpos - 3., ypos + 8.).w_h(3., 5.); // 左足
        //        draw.rect().x_y(xpos - 4., ypos + 3.).w_h(2., 5.); // 左腕
        //        draw.rect().x_y(xpos + 2., ypos + 3.).w_h(2., 5.); // 右腕
        //    }
        //    1 => {
        //        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //        draw.rect().x_y(xpos - 3., ypos + 3.).w_h(6., 5.); // 胴体
        //        draw.rect().x_y(xpos - 3., ypos + 8.).w_h(3., 4.);
        //        draw.rect().x_y(xpos, ypos + 8.).w_h(3., 1.);
        //    }
        //    2 => {
        //        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //        draw.rect().x_y(xpos - 3., ypos + 3.).w_h(6., 5.); // 胴体
        //        draw.rect().x_y(xpos - 3., ypos + 8.).w_h(3., 3.);
        //        draw.rect().x_y(xpos - 3., ypos + 8.).w_h(3., 3.);
        //    }
        //    3 => {
        //        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //        draw.rect().x_y(xpos - 3., ypos + 3.).w_h(6., 5.); // 胴体
        //        draw.rect().x_y(xpos - 3., ypos + 8.).w_h(3., 1.);
        //        draw.rect().x_y(xpos, ypos + 8.).w_h(3., 4.);
        //    }
        //    4 => {
        //        draw.ellipse().x_y(xpos, ypos).radius(3.);
        //        draw.rect().x_y(xpos - 3., ypos + 3.).w_h(6., 5.); // 胴体
        //        draw.rect().x_y(xpos, ypos + 8.).w_h(3., 5.); // 左足
        //        draw.rect().x_y(xpos - 4., ypos + 3.).w_h(2., 5.); // 左腕
        //        draw.rect().x_y(xpos + 2., ypos + 3.).w_h(2., 5.); // 右腕
        //    }
        //    _ => {}
        //}
    }
}
