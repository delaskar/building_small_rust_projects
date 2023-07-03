# **Flappy Dragon**
This is a simple implementation of the Flappy Dragon game, created using the Rust programming language and the ***"Hands on Rust"*** book.

### **Game Overview**
Flappy Dragon is a side-scrolling game where the player controls a dragon that needs to navigate through a series of obstacles. The goal is to survive for as long as possible and earn points by passing through the gaps between the obstacles.

### **Controls**
Press the SPACE key to make the dragon flap and fly upward.
Use the Q key to quit the game.
Use the P key to play again after the game ends.

## **Gameplays**

### **Main Menus**
When the game starts, you will see the main menu with the following options:

- **(P)** Play Game: Start a new game.
- **(Q)** Quit Game: Exit the game.
Use the arrow keys to select an option and press Enter to confirm your selection.

### **Playing the Games**
Once you start the game, you will control a dragon positioned at the left side of the screen. The dragon will automatically descend due to gravity, and you must make it flap by pressing the SPACE key to avoid crashing into the obstacles.

Your objective is to pass through the gaps between the obstacles to earn points. The score is displayed at the top-right corner of the screen.

### **Game Overs**
The game ends if the dragon collides with an obstacle or touches the top or bottom of the screen. When the game ends, you will see the "Game Over" screen with your final score and the following options:

- **(P)s** Play Again: Start a new game.
- **(Q)s** Quit Game: Exit the game.
Use the arrow keys to select an option and press Enter to confirm your selection.

### **Getting Starteds**
To play Flappy Dragon on your local machine, follow these steps:

1. Ensure that you have Rust installed on your system.

2. Clone or download the Flappy Dragon repository from GitHub.

3. Open a terminal or command prompt and navigate to the project's directory.

4. Run the following command to build and run the game:

    - `cargo run`

Enjoy playing Flappy Dragon!

## **Dependencies**

Flappy Dragon utilizes the bracket-lib crate, which provides a simple console-based game development library for Rust. The `bracket-lib` library handles the rendering and input handling necessary for creating a game.