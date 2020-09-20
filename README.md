# gestsson-chess

Jag försökte skriva det så att gui inte ska behöva fippla med pjäser individuellt.

att göra ett drag består av två stadier, första är att markera en pjäs och få ut vad man med pjäsen kan göra, och sedan flytta pjäsen. 
```rust
let mut board = Board::new(None);

board.select((1, 6));

let moves = board.get_movable();
board.move_piece(moves.first().unwrap());
```
både select och move_piece returnerar om de lyckats markera respektive flytta pjäsen. 

för highligting finns några användbara funktioner,
```rust
get_selectable(&self) -> Vec<Point> //lista med markerbara platser
get_movable(&self) -> Vec<Point> //lista med gångbara platser (inkl attacker)

is_enemy(&self, Point) -> bool //kollar om en punk är en fiende
is_friendly(&self, Point) -> bool //kollar om en punkt är vänlig
```

Jag medger att mitt sätt att göra pjäser är efterblivet och minnesineffektivt. Du kan få tag på en pjäs namn mha
```rust
get_name(&self, Point) -> Option<String> //returnerar namnet på pjäsen (vilken typ av pjäs det är)
```
där name kan vara
```rust
"Pawn"
"King"
"Bishop"
"Rook"
"Knight"
"Queen"
```
Jag rekommenderar att ta vara på detta genom att slänga in namnet i någon resource path / hashmap, samt checka efter namn på så få ställen som möjligt. 