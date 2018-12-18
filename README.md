# Reversi

A library for arbitrary player, size and dimension reversi

## Rules

## Gamestate File

A reversi gamestate file is structured like this:  
`i k d`  
`i` being the number of the player who is next to move. These numbers start counting at 0.  
`k` is the total amount of players in the game.  
`d` being the dimensions of the board. In a standard game of reversi this is 2.

The next line specifies the size of the board in each dimension.
Each following line specifying one stone on the field which needs a coordinate in each dimesion.

This would be the starting position of the standard reversi game:

```
0 2 2
8 8
0 3 3
0 4 4
1 4 3
1 3 4
```

Player 0 starts.
It is a two player game.
The board has two dimensions.
The board is of size 8x8.
Player 0 has a stone on position 3,3
Player 0 has a stone on position 4,4
Player 1 has a stone on position 4,3
Player 1 has a stone on position 3,4
