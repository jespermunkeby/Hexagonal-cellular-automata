use nannou::prelude::*;
use nannou::geom::vector::Vector2;
use hex2d;

struct HexCA{
    r: i32,
    hash_map: std::collections::HashMap<hex2d::Coordinate,bool>,
}

impl HexCA{
    fn new(n: i32)->Self{ //where n is the number of layers from cell att origin
        let center : hex2d::Coordinate<i32> = hex2d::Coordinate::new(0,0);
        let mut hash_map = std::collections::HashMap::new();
        hash_map.insert(center, false);
        for n_layer in 1..n{
            let ring = center.ring(n_layer, hex2d::Spin::CW(hex2d::Direction::YX));
            for cell in ring{
                hash_map.insert(cell, false);
            }
        }

        Self{
            r:n,
            hash_map: hash_map,
        }
    }

    fn draw_circles(&self ,_draw : &nannou::draw::Draw, spacing: hex2d::Spacing){
        let wh = match spacing{
            hex2d::Spacing::FlatTop(val) => val,
            hex2d::Spacing::PointyTop(val) => val,
        };

        for cell in &self.hash_map{
            let p = cell.0.to_pixel(spacing);
            if *cell.1{
                _draw.ellipse()
                     .rgba(1.0, 1.0, 1.0, 0.17)
                     //.color(WHITE)
                     .w_h(wh*1.0, wh*1.0)
                     .x_y(p.0, p.1);
            } else{
                _draw.ellipse()
                     .rgba(0.0, 0.0, 0.0, 0.333)
                     .w_h(wh*0.7, wh*0.7)
                     .x_y(p.0, p.1);
            }
        }
    }

    fn draw_lines(&self, _draw : &nannou::draw::Draw,spacing: hex2d::Spacing){
        for cell in &self.hash_map{
            if (cell.0.distance(hex2d::Coordinate::new(0,0)) < (self.r-1)) & cell.1{
                let p = cell.0.to_pixel(spacing);
                for neighbor in &cell.0.neighbors(){
                    if *self.hash_map.get(neighbor).unwrap(){
                        _draw.line()
                            .caps_round()
                            .weight(3.0)
                            .color(WHITE)
                            .points(Vector2::from(p),Vector2::from(neighbor.to_pixel(spacing)));
                    }
                }
            }
        }

    }

    fn n_neighbors_alive(&self, cell: hex2d::Coordinate) -> i32{
        let mut alive_count = 0;
        if cell.distance(hex2d::Coordinate::new(0,0)) < (self.r-1){ //TODO: currently ignores edge-cells. add optional wrap-around 
            for neighbor in &cell.neighbors(){
                match self.hash_map.get(neighbor){
                    Some(alive) => {
                        if *alive{
                            alive_count += 1;
                        }
                    },
                    None => println!("No such cell"),
                }
            }
        }
        alive_count
    }

    fn tick(&mut self){
        let mut new_state = self.hash_map.clone();
        for cell in &self.hash_map {
            new_state.insert(*cell.0,
                match (self.n_neighbors_alive(*cell.0),cell.1){
                    //1. Any live cell with two or three live neighbours survives.
                    //2. Any dead cell with three live neighbours becomes a live cell.
                    //3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                    (2,true) => true,
                    (3,_) => true,  
                    _ => false      
                }
            );
        }

        self.hash_map = new_state;
    }
}

struct Model {
    mouse_pressed: bool,
    ca : HexCA,
    spacing : hex2d::Spacing,
}

fn main() {

    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {  

    Model{
        mouse_pressed: false,
        ca : HexCA::new(20),
        spacing : hex2d::Spacing::FlatTop(15.0),
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event){
    match _event{
        Event::WindowEvent{id:_, simple : we} => {
            match we {
                Some(message) => {
                    match message {
                        MousePressed(_mbutton) => {
                            _model.mouse_pressed = true;
                        },

                        MouseReleased(_mbutton) => {
                            _model.mouse_pressed = false;
                        },

                        KeyPressed(Key::Space) => {
                            _model.ca.tick();
                        },

                        KeyPressed(Key::R) => {
                            for cell in _model.ca.hash_map.clone(){
                                _model.ca.hash_map.insert(cell.0, random_f32()>0.5);
                            }
                        },

                        KeyPressed(Key::C) => {
                            _model.ca = HexCA::new(_model.ca.r);
                        },

                        KeyPressed(Key::Up) => {
                            _model.spacing = match _model.spacing{
                                hex2d::Spacing::FlatTop(val) => hex2d::Spacing::FlatTop(val+10.0),
                                hex2d::Spacing::PointyTop(val) => hex2d::Spacing::PointyTop(val+10.0)
                            }
                        },
                        _ => ()
                    }
                },
                None =>(),
                }
        },
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update){
    if _model.mouse_pressed{
        let cell : hex2d::Coordinate<i32> = hex2d::Coordinate::from_pixel(_app.mouse.x, _app.mouse.y, _model.spacing);
        _model.ca.hash_map.insert(cell,true);
    }

    
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();

    draw.background().color(DARKSLATEGREY);

    _model.ca.draw_lines(&draw, _model.spacing);
    _model.ca.draw_circles(&draw, _model.spacing);
    
    
    draw.to_frame(_app, &_frame).unwrap();
}
