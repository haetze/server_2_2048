# server_2_2048
Simple Game server that allows local programs to connect to it and implement a GUI for the game without needed to write the 
logic themself.
## The Protocol
### Messages you can send
* "new n\n" (n is a number that represents the dimensions of the square)
* "right\n"
* "left\n" 
* "up\n"
* "down\n"
In all cases the directions says which direction the slide is going in. For example "right\n" is a swipe from left to right.
* exit 
Stops the game.
### Messages you receive 
* "Empty\n", no game has been started 
* "Lost\n", the game is over and you can't make a move any more.
* [None...];....;[None..];
Messages that represent the state of the game board.  None means the field is empty. Some(n) means n is on the board at that plays.
