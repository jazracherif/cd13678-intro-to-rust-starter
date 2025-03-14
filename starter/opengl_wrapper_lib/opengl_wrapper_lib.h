#ifndef OPENGL_WRAPPER_LIB_H
#define OPENGL_WRAPPER_LIB_H
// #include "../glfw-3.4/include/GLFW/glfw3.h"
#include <GLFW/glfw3.h>
#include <GL/glut.h>

// Structure to represent a sprite
typedef struct {
    int width;
    int height;
    int color[3]; // RGB color
    float x, y; // Position
} Sprite;

// Function to create a game window
void create_game_window(const char *title, int width, int height);

// Function to create a sprite
Sprite* create_sprite(float x, float y, int width, int height, int r, int g, int b);

// Function to render a sprite
void render_sprite(Sprite *sprite);

// Function to update a sprite position
void update_sprite_position(Sprite *sprite, float x, float y);

// Function to update the game window
void update_game_window();

// Function to clear the screen
void clear_screen();

// Function to check if the window should close
int window_should_close();

// Function to get key state
int get_key(GLFWwindow* window, int key);

// Function to get the window pointer
GLFWwindow* get_window();

void renderText(const char* text, float x, float y, float scale, float r, float g, float b);

#endif // OPENGL_WRAPPER_LIB_H
