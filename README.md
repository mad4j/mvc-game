# mvc-game
Rust game template structure

``` mermaid
classDiagram
    class GameModel {
        +GameState state
        +f32 player_x
        +f32 player_y
        +f32 player_speed
        +new() GameModel
        +update_position(dx: f32, dy: f32, delta: f32)
    }

    class GameView {
        +new() GameView
        +render(model: &GameModel)
    }

    class GameController {
        -GameModel model
        -GameView view
        +new() GameController
        +run() 
    }

    class GameState {
        <<enum>>
        Menu
        Playing
        GameOver
    }

    %% Relazioni
    GameController --> GameModel : gestisce
    GameController --> GameView : invoca rendering
    GameView --> GameModel : legge stato
    GameModel --> GameState : usa
```
