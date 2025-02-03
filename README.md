# The Snake Game Engine with Rust, built from the Ground Up (Udacity Intro to Rust)

![readme_image_assets/snake_game.jpg](readme_image_assets/snake_game.jpg)

![readme_image_assets/snake_game2.jpg](readme_image_assets/snake_game2.png)

This repository contain a rust implementation of the Snake Game. 

## Getting Started

### Setup in Unix Environment

For this project, you'll need to have Rust installed in your machine. If you haven't installed Rust yet, you can do so with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Also, because we are dealing with C code in this project, you'll need to have a C compiler installed on your machine. You can install the `build-essential` package, which includes the GNU C Compiler (GCC) and other necessary tools. You'll need to have `GLFW` installed in your machine. GLFW is a C library that will be the foundation of our game engine. Finally `Glut` is also used for text rendering on the screen. 

All these dependencies can be installed via the makefile `setup` rule

```bash
make setup
```

### Running the Test C Game

To start with your project, clone this repository to your local machine:

```bash
git clone git@github.com:jazracherif/cd13678-intro-to-rust-starter.git
```

To ensure you are set up correctly, you can run the test C game that comes with this project. You can build and run the test game with:

```bash
cd intro-to-rust-starter/starter
make run-c
```

You should see the following pop-up window:

![readme_image_assets/img.png](readme_image_assets/img.png)

### Rust Projects

There are two rust project in this repo. One is the `my_game_engine` library, and provide basic game macros and several tests for the library.

The other is the `rust_snake_game` binary, the actual snake game that can be played by running `cargo run` inside that project. The game will spawn several snake, one of which is the user controlled snake, one is a buddy follower, and a third is an automous snake. Food will be spawned now and then, some poisonous (Red color) and other edible. The more edible food is eaten, the higher the score. Enjoy!


## License

[License](LICENSE.txt)
