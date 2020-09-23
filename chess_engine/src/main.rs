use chess_engine::Board; 
use chess_engine::Team; 

use ggez;
use ggez::{graphics, Context, GameResult};
use ggez::event::{self, MouseButton};
use ggez::nalgebra as na;
use std::path; 

const BOARD_OFFSET_X: usize = 10; 
const BOARD_OFFSET_Y: usize = 10; 
const BOARD_WIDTH: usize = 580; 
const BOARD_HEIGHT: usize = 580; 
const TILE_WIDTH: usize = BOARD_WIDTH/8; 
const TILE_HEIGHT: usize = BOARD_HEIGHT/8; 

fn new_tile(ctx: &mut ggez::Context, x: f32, y: f32, w: f32, h: f32, color: graphics::Color) -> graphics::Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x,
            y,
            w,
            h,
        },
        color,
    )
    .unwrap()
}

fn draw_grid(ctx: &mut ggez::Context) -> GameResult<()>{
    for x in 0..8{
        for y in 0..8{
            let color;
            if (y%2 == 0 && x%2 == 0) || (y%2 == 1 && x%2 == 1){
                color = graphics::Color::from_rgb(177, 228, 185); 
            }
            else{
                color = graphics::Color::from_rgb(112, 162, 163); 
            }
            
            let tile = new_tile(
                ctx, 
                (x*TILE_WIDTH+BOARD_OFFSET_X) as f32, 
                (y*TILE_HEIGHT+BOARD_OFFSET_Y) as f32, 
                TILE_WIDTH as f32,
                TILE_HEIGHT as f32,
                color,
            ); 
            graphics::draw(ctx, &tile, (na::Point2::new(0.0, 0.0),))?; 
        }
    }
    Ok(())
}

fn draw_piece(ctx: &mut ggez::Context, x: usize, y: usize, board: &mut Board) -> (){
    if !board.is_empty((x, y)){
        let name = board.get_name((x, y)).unwrap(); 

        let mut image_path: String = "/pieces/".to_string()+&name;
        
        if board.is_team((x, y), Team::White){
            image_path = image_path + "White.png"; 
        }
        else{
            image_path = image_path + "Black.png"; 
        }

        let image = graphics::Image::new(ctx, image_path).unwrap(); 
        graphics::draw( 
            ctx,
            &image,
            (na::Point2::new(
                (x*TILE_WIDTH+BOARD_OFFSET_X+5) as f32, 
                (y*TILE_HEIGHT+BOARD_OFFSET_Y+5) as f32, 
            ),),
        );
    }
}

fn add_pieces(board: &mut Board, ctx: &mut ggez::Context) -> GameResult<()>{
    for x in 0..8{
        for y in 0..8{
            draw_piece(ctx, x, y, board); 
        }
    }
    Ok(())
}

fn place_piece(x: i64, y: i64) {
    if x != -1 && y != -1{
        println!("X - {}, Y - {}", x, y); 
    }
}

fn coordinates_to_tile(x: f32, y: f32) -> (i64, i64) {
    if x < BOARD_OFFSET_X as f32 || y < BOARD_OFFSET_Y as f32 || x > (BOARD_OFFSET_X as f32 + BOARD_WIDTH as f32) || y > (BOARD_OFFSET_Y as f32 + BOARD_HEIGHT as f32) {
        return (-1, -1)
    }

    let x1 = (x as usize - BOARD_OFFSET_X)/TILE_WIDTH;
    let y1 = (y as usize - BOARD_OFFSET_Y)/TILE_HEIGHT;  
    (x1 as i64, y1 as i64)
}

struct MainState {
    board: Board,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            board: Board::new(None),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        draw_grid(ctx)?; 
        add_pieces(&mut self.board, ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, 
        _ctx: &mut Context,
        _button: MouseButton, 
        _x: f32, 
        _y: f32
    ){
        match _button{
            MouseButton::Left => {
                let (x, y) = coordinates_to_tile(_x, _y); 
                place_piece(x, y); 
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult { 
    let cb = ggez::ContextBuilder::new("Chess", "ggez")
        .add_resource_path(path::PathBuf::from("./images"))
    ;
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}