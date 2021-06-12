use nannou::prelude::*;
use rand::prelude::*;
//use nannou::ui::prelude::*;

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(240.));

    let max_human: usize = 20;
    Model::new(max_human, app)
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    let width = model.width.clone() as isize;
    let height = model.height.clone() as isize;
    for h in &mut model.humans {
        if h.state == State::Live {
            h.update_timer();
            h.drive(width, height, &mut model.game);
            h.coll(width, height, &mut model.game);
            h.encount(width, height, &mut model.game);
            h.hp -= 1;
            println!("hp: {}", h.hp);
            if h.hp < 0 {
                h.state = State::Dead;
                model.game[h.xpos][h.ypos] = Object::None;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // キャンバスを取得
    let draw = app.draw();

    // 背景色を設定
    draw.background().color(BLACK);

    // 1辺100の正方形を原点に表示

    let win = app.window_rect();

    draw.rect().x_y(0.0, 30.0).w_h(10.0, 10.0).color(BLUE);

    // フレームに書き出し
    for h in &model.humans {
        if h.state == State::Live {
            h.draw(&draw, win);
        }
    }

    let number_of_people =
        model.humans.iter().fold(
            0,
            |acc, h| if h.state == State::Live { acc + 1 } else { acc },
        );
    if number_of_people <= 2 {
        for h in &model.humans {
            if h.state == State::Live {
                dbg!(h.strategy.clone());
            }
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
    width: usize,
    height: usize,
    game: Vec<Vec<Object>>,
    humans: Vec<Human>,
}

impl Model {
    pub fn new(max_human: usize, app: &App) -> Self {
        let win = app.window_rect();
        let width = win.w() as usize;
        let height = win.h() as usize;
        let mut game = vec![vec![Object::None; height]; width];
        let humans: Vec<Human> = vec![0; max_human]
            .into_iter()
            .map(|_| {
                Human::new(
                    width,
                    height,
                    Strategy::TFT(Person::Good),
                    Character::Introverted,
                )
            })
            .collect();
        for h in humans.iter() {
            game[h.xpos][h.ypos] = Object::Human(h.strategy.person());
        }
        Self {
            width,
            height,
            game,
            humans,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Object {
    None,
    Human(Person),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Live,
    Dead,
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct Human {
    xpos: usize,
    ypos: usize,
    state: State,
    direction: Direction,
    timer: u64,
    hp: i64,
    strategy: Strategy,
    character: Character,
}

impl Human {
    fn new(width: usize, height: usize, strategy: Strategy, character: Character) -> Self {
        let mut rng = thread_rng();
        Self {
            xpos: rng.gen_range(0..width),
            ypos: rng.gen_range(0..height),
            state: State::Live,
            direction: Direction::new(),
            timer: 0,
            //hp: rng.gen_range(500..1000),
            hp: rng.gen_range(50..100),
            strategy,
            character,
        }
    }
    fn update_timer(&mut self) {
        self.timer = self.timer + 1;
    }
    fn draw(&self, draw: &Draw, win: Rect) {
        let xpos = self.xpos;
        let ypos = self.ypos;
        draw.ellipse()
            .x_y(xpos as f32 - win.w() / 2.0, ypos as f32 - win.h() / 2.0)
            .radius(3.);
    }

    // 人を動かす
    fn drive(&mut self, width: isize, height: isize, game: &mut Vec<Vec<Object>>) {
        if thread_rng().gen_range(0..100) < 2 {
            self.direction = Direction::new();
        }
        match self.direction {
            Direction::Stay => {}
            Direction::Light => {
                game[self.xpos][self.ypos] = Object::None;
                self.xpos = ((self.xpos as isize + 1 + width) % width) as usize;
            }
            Direction::Left => {
                game[self.xpos][self.ypos] = Object::None;
                self.xpos = ((self.xpos as isize - 1 + width) % width) as usize;
            }
            Direction::Up => {
                self.ypos = ((self.ypos as isize + 1 + height) % height) as usize;
                game[self.xpos][self.ypos] = Object::None;
            }
            Direction::Down => {
                game[self.xpos][self.ypos] = Object::None;
                self.ypos = ((self.ypos as isize - 1 + height) % height) as usize;
            }
        }
        game[self.xpos][self.ypos] = Object::Human(self.strategy.person());
    }

    fn coll(&mut self, width: isize, height: isize, game: &mut Vec<Vec<Object>>) {
        if self.character == Character::Introverted {
            if thread_rng().gen_range(0..100) < 2 {
                for i in -10..10 {
                    for j in -10..10 {
                        if let Object::Human(_) = game
                            [((self.xpos as isize + i + width) % width) as usize]
                            [((self.ypos as isize + j + height) % height) as usize]
                        {
                            //相手から2画素分近づく
                            if i < 0 {
                                self.xpos = ((self.xpos as isize - 2 + width) % width) as usize;
                            }
                            if i > 0 {
                                self.xpos = ((self.xpos as isize + 2 + width) % width) as usize;
                            }

                            if j < 0 {
                                self.ypos = ((self.ypos as isize - 2 + height) % height) as usize;
                            }
                            if j > 0 {
                                self.ypos = ((self.ypos as isize + 2 + height) % height) as usize;
                            }
                        }
                        game[self.xpos][self.ypos] = Object::Human(self.strategy.person());
                    }
                }
            }
            return;
        }
        for i in -10..10 {
            for j in -10..10 {
                if let Object::Human(_) = game[((self.xpos as isize + i + width) % width) as usize]
                    [((self.ypos as isize + j + height) % height) as usize]
                {
                    //相手から2画素分逃げるようにする
                    if i < 0 {
                        self.xpos = ((self.xpos as isize + 2 + width) % width) as usize;
                    }
                    if i > 0 {
                        self.xpos = ((self.xpos as isize - 2 + width) % width) as usize;
                    }

                    //相手から2画素分逃げるようにする
                    if j < 0 {
                        self.ypos = ((self.ypos as isize + 2 + height) % height) as usize;
                    }
                    if j > 0 {
                        self.ypos = ((self.ypos as isize - 2 + height) % height) as usize;
                    }
                }
                game[self.xpos][self.ypos] = Object::Human(self.strategy.person());
            }
        }
    }

    fn encount(&mut self, width: isize, height: isize, game: &mut Vec<Vec<Object>>) {
        for i in -2..2 {
            for j in -2..2 {
                if let Object::Human(other_person) = game
                    [((self.xpos as isize + i + width) % width) as usize]
                    [((self.ypos as isize + j + height) % height) as usize]
                {
                    match (other_person, self.strategy.person()) {
                        (Person::Bad, Person::Bad) => self.hp -= 10,
                        (Person::Bad, Person::Good) => self.hp -= 5,
                        (Person::Good, Person::Bad) => self.hp += 1,
                        (Person::Good, Person::Good) => self.hp += 2,
                    }
                }
                game[self.xpos][self.ypos] = Object::Human(self.strategy.person());
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Person {
    Good,
    Bad,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum Character {
    Sociable,
    Introverted,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum Strategy {
    AlwaysDefects(Person),
    AlwaysHonest(Person),
    TFT(Person),
}
impl Strategy {
    fn new(self) -> Self {
        match self {
            Self::AlwaysDefects(_) => Self::AlwaysHonest(Person::Bad),
            Self::AlwaysHonest(_) => Self::AlwaysHonest(Person::Good),
            Self::TFT(_) => Self::AlwaysHonest(Person::Good),
        }
    }
    fn person(&self) -> Person {
        match self {
            Self::AlwaysDefects(x) => x.clone(),
            Self::AlwaysHonest(x) => x.clone(),
            Self::TFT(x) => x.clone(),
        }
    }
}
// #[derive(Copy, Clone)]
// struct AlwaysDefects {
//     person: Person
// }
//impl AlwaysDefects {
//    fn new() -> Self {
//        Self {
//            person: Person::Bad
//        }
//    }
//}
//#[derive(Copy, Clone)]
//struct AlwaysHonest {
//    person: Person
//}
//impl AlwaysHonest {
//    fn new() -> Self {
//        Self {
//            person: Person::Good
//        }
//    }
//}
//#[derive(Copy, Clone)]
//struct TFT {
//    person: Person
//}
//impl TFT {
//    fn new() -> Self {
//        Self {
//            person: Person::Good
//        }
//    }
//}
