# Energy Match

## Game idea

You are presented with a window containing multiple ordered columns of values. <br>
However only 3 values per column are visible at a time. <br>
The purpose of the game is to cycle through each column so that the values visible in your window match the target window while not running out of energy. <br>
There is one special column per game. Usually this column is the center column. <br> 
Upon cycling any column to the either side of this column. Any other column on that side of the special column will be rotated the same amount as the column you rotated. <br>

## Rules
If you run out of energy you automatically lose. //might allow to keep playing to check if you could have gotten it next time. <br>
Each swipe up or down cycles one value and uses one energy. You can do multiple swipes in one command. <br>


## Commands 
There is only one command, the cycle command. <br>
The basic structure of the cycle command is as follows: <br>
```
cycle {index of root affected column} {direction} {amount} 
```

### examples:
```
cycle 8 up 3
cycle 2 down 4
```


## Planned Features (WIP)
- The game automatically finds the most optimal way to play pre-game and then adjusts the amount of moves for the user.
- Main menu
- Generated levels
- The ability to save and load levels you've created or downloaded. (json to allow for manual level creation)
- Level editor?
- Adjustable column amount
- Adjustable column sides.
- Adjustable window height.
- looping columns. if your column currently has has value one and you go down even lower it'll jump to the max.

## Example game window

## Default values
Amount of columns: 10 <br>
Amount of sides per column: 8 <br>
window height: 1 value <br>
In generated levels the selected value visible from the window is random. but the columns are still ordered from 1 to X <br>
//To be playtested <br>

## Max values
Amount of columns: 20 <br>
Amount of sides per column: 99 <br>

## Minimum values 
Amount of columns: 1 <br>
Amount of sides per column: 1 <br>
All indexes start from 1. So both column indexing in the cycle command and the column values start from one. 


## Main menu
```
.----------------------------.
|                            |
|       ENERGY MATCH         |
|                            |
|----------------------------|
|                            |
|      [1] Generate level    |
|      [2] Load level        |
|      [2] Create level      |
|      [3] Settings          |
|      [4] Quit              |
|                            |
'----------------------------'

```

## Move up

```
                        LEVEL 1
         .--.--.--.--.--.------.--.--.--.--.--.                                      TARGET WINDOW
         |3 |2 |4 |7 |3 |#|1 |#|6 |8 |8 |1 |5 |                          .--.--.--.--.--.------.--.--.--.--.--.
      => |4 |3 |5 |8 |4 |#|2 |#|7 |1 |1 |2 |6 | <=  |Energy left: 9|  => |6 |6 |6 |6 |6 |#|5 |#|7 |7 |7 |7 |7 | <=
         |5 |4 |6 |1 |5 |#|3 |#|8 |2 |2 |3 |7 |                          '--'--'--'--'--'------'--'--'--'--'--'  
         '--'--'--'--'--'------'--'--'--'--'--'
         
  index: [0, 1, 2, 3, 4,   5,   6, 7, 8, 9, 10]


Enter a command: cycle 4 up 2


                        LEVEL 1
         .--.--.--.--.--.------.--.--.--.--.--.                                      TARGET WINDOW
         |1 |8 |2 |1 |3 |#|1 |#|6 |8 |8 |1 |5 |                          .--.--.--.--.--.------.--.--.--.--.--.
      => |2 |1 |3 |2 |4 |#|2 |#|7 |1 |1 |2 |6 | <=  |Energy left: 7|  => |6 |6 |6 |6 |6 |#|5 |#|7 |7 |7 |7 |7 | <=
         |3 |2 |2 |3 |5 |#|3 |#|8 |2 |2 |3 |7 |                          '--'--'--'--'--'------'--'--'--'--'--'
         '--'--'--'--'--'------'--'--'--'--'--'

  index: [0, 1, 2, 3, 4,   5,   6, 7, 8, 9, 10]

```

## Move down

```     
                        LEVEL 1
         .--.--.--.--.--.------.--.--.--.--.--.                                      TARGET WINDOW
         |3 |2 |4 |7 |3 |#|1 |#|6 |8 |8 |1 |5 |                          .--.--.--.--.--.------.--.--.--.--.--.
      => |4 |3 |5 |8 |4 |#|2 |#|7 |1 |1 |2 |6 | <=  |Energy left: 9|  => |6 |6 |6 |6 |6 |#|5 |#|7 |7 |7 |7 |7 | <=
         |5 |4 |6 |1 |5 |#|3 |#|8 |2 |2 |3 |7 |                          '--'--'--'--'--'------'--'--'--'--'--' 
         '--'--'--'--'--'------'--'--'--'--'--'

  index: [0, 1, 2, 3, 4,   5,   6, 7, 8, 9, 10]


Enter a command: cycle 3 down 1


                        LEVEL 1
         .--.--.--.--.--.------.--.--.--.--.--.                                      TARGET WINDOW
         |2 |1 |3 |7 |3 |#|1 |#|6 |8 |8 |1 |5 |                          .--.--.--.--.--.------.--.--.--.--.--.
      => |3 |2 |4 |8 |4 |#|2 |#|7 |1 |1 |2 |6 | <=  |Energy left: 8|  => |6 |6 |6 |6 |6 |#|5 |#|7 |7 |7 |7 |7 | <=
         |4 |3 |5 |1 |5 |#|3 |#|8 |2 |2 |3 |7 |                          '--'--'--'--'--'------'--'--'--'--'--'   
         '--'--'--'--'--'------'--'--'--'--'--'

  index: [0, 1, 2, 3, 4,   5,   6, 7, 8, 9, 10]

```

## Game over - victory

```  
      .    .     .     .
.-----|----|-----|-----|-----.
|                            |
|         VICTORY!           |
|                            |
|     LEVEL COMPLETED!       |
|                            |
'----------------------------'

```

## Game over - loss

```                    
.----------------------------.
|                            |
|         GAME OVER          |
|                            |
|    Ran out of energy...    |
|                            |
'----------------------------'
```