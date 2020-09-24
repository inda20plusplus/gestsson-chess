use chess_engine::Board; 
use chess_engine::Team; 
use chess_engine::pieces::default::*; 

use ggez;
use ggez::{graphics, Context, GameResult};
use ggez::event::{self, MouseButton};
use ggez::nalgebra as na;
use std::path; 
use dialog::DialogBox; 

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

fn mark_movables(ctx: &mut ggez::Context, board: &mut Board) -> GameResult<()>{
    let moves = board.get_movable(); 

    if board.held_piece != None{
        for point in moves.iter(){
            let color = graphics::Color::from_rgb(0, 200, 170); 
            let tile = new_tile(
                ctx, 
                (point.0*TILE_WIDTH+BOARD_OFFSET_X+15) as f32, 
                (point.1*TILE_HEIGHT+BOARD_OFFSET_Y+15) as f32, 
                (TILE_WIDTH-30) as f32,
                (TILE_HEIGHT-30) as f32,
                color,
            ); 
            graphics::draw(ctx, &tile, (na::Point2::new(0.0, 0.0),))?; 
        }
    }
    Ok(())
}

fn draw_piece(ctx: &mut ggez::Context, x: usize, y: usize, board: &mut Board){
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
        ).unwrap();
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

fn display_text(ctx: &mut ggez::Context, txt: &str, x: f32, y: f32, font_size: f32){
    let font = graphics::Font::new(ctx, "/Raleway-Black.ttf").unwrap();
    let text = graphics::Text::new((txt, font, font_size)); 
    graphics::draw(
        ctx,
        &text, 
        (na::Point2::new(
            (x) as f32, 
            (y) as f32, 
        ),),
    ).unwrap();
}

fn display_state(ctx: &mut ggez::Context, board: &mut Board){
    let current_player_message: &str; 
    match board.current_player{
        Team::White => current_player_message = "Current player: White",
        _ => current_player_message = "Current player: Black",
    }
    display_text(ctx, current_player_message, 600.0, 20.0, 15.0); 

    let check_message: &str; 
    match board.check{
        true => check_message = "Check: true",
        _ => check_message = "Check: false",
    }
    display_text(ctx, check_message, 600.0, 60.0, 15.0);

    if board.finished{
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.2].into());
        let winner_txt: &str; 
        match board.winner{
            Some(Team::Black) => winner_txt = "Black wins!",
            Some(Team::White) => winner_txt = "White wins!",
            _ => winner_txt = "Stalemate",
        }
        display_text(ctx, winner_txt, 200.0, 200.0, 100.0); 
    }
}

fn place_piece(board: &mut Board, x: i64, y: i64) {
    if x != -1 && y != -1{
        let moves = board.get_movable(); 

        for point in moves.iter(){
            if point.0 as i64 == x && point.1 as i64 == y{
                board.move_piece(*point); 
                if board.can_promote(){
                    promote(board); 
                }
            }
        }

        board.deselect(); 

        if board.held_piece == None {
            if board.is_team((x as usize, y as usize), board.current_player){
                board.select((x as usize, y as usize)); 
            }
        }
    }
}

fn promote(board: &mut Board){
    let name = dialog::Input::new("Promote to: Queen (Q, q), Knight (N, n), Rook (R, r), Bishop (B, b)")
            .title("Promotion")
            .show()
            .expect("Could not display dialog box")
            .unwrap();
    if name == "Q" || name == "q"{
        board.promote(queen(board.current_enemy)); 
        return; 
    }
    else if name == "N" || name == "n"{
        board.promote(knight(board.current_enemy)); 
        return; 
    }
    else if name == "R" || name == "r"{
        board.promote(rook(board.current_enemy)); 
        return; 
    }
    else if name == "B" || name == "b"{
        board.promote(bishop(board.current_enemy)); 
        return; 
    }
    promote(board); 
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
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        draw_grid(ctx)?; 
        mark_movables(ctx, &mut self.board)?; 
        add_pieces(&mut self.board, ctx)?;
        display_state(ctx, &mut self.board); 
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
                place_piece(&mut self.board, x, y); 
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult { 
    let cb = ggez::ContextBuilder::new("Chess", "ggez")
        .add_resource_path(path::PathBuf::from("./gui/resources"))
    ;
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}