#include <GLFW/glfw3.h>
#include <stdlib.h>
#include "opengl_wrapper_lib.h"
// Global variable for the GLFW window
GLFWwindow* window;

// Function to create a game window
void create_game_window(const char *title, int width, int height) {
    int argc = 0;
    char * argv = "test_game";
    glutInit(&argc, argv);

    if (!glfwInit()) {
        exit(EXIT_FAILURE);
    }

    window = glfwCreateWindow(width, height, title, NULL, NULL);

    if (!window) {
        glfwTerminate();
        exit(EXIT_FAILURE);
    }

    glfwMakeContextCurrent(window);

    // Set up orthographic projection
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(0, width, height, 0, -1, 1); // Origin at top-left
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
}

// Function to create a sprite
Sprite* create_sprite(float x, float y, int width, int height, int r, int g, int b) {
    Sprite *sprite = (Sprite*)malloc(sizeof(Sprite));
    sprite->width = width;
    sprite->height = height;
    sprite->color[0] = r;
    sprite->color[1] = g;
    sprite->color[2] = b;
    sprite->x = x;
    sprite->y = y;
    return sprite;
}

int test() {
    return 1;
}

// Function to render a sprite
void render_sprite(Sprite *sprite) {
    // Convert sprite position and size to window coordinates
    float x1 = sprite->x;
    float y1 = sprite->y;
    float x2 = sprite->x + sprite->width;
    float y2 = sprite->y + sprite->height;

    glColor3ub(sprite->color[0], sprite->color[1], sprite->color[2]);
    glBegin(GL_QUADS);
    glVertex2f(x1, y1);
    glVertex2f(x2, y1);
    glVertex2f(x2, y2);
    glVertex2f(x1, y2);
    glEnd();
}

// Function to update a sprite position
void update_sprite_position(Sprite *sprite, float x, float y) {
    sprite->x = x;
    sprite->y = y;
}

// Function to update the game window
void update_game_window() {
    glfwSwapBuffers(window);
    glfwPollEvents();
}

// Function to clear the screen
void clear_screen() {
    glClear(GL_COLOR_BUFFER_BIT);
}

// Function to check if the window should close
int window_should_close() {
    return glfwWindowShouldClose(window);
}

// Function to get key state
int get_key(GLFWwindow* window, int key) {
    return glfwGetKey(window, key);
}

// Function to get the window pointer
GLFWwindow* get_window() {
    return window;
}

// Function to draw a text using GLUT. glutinit must be called before hand (from claud ai)
void renderText(const char* text, float x, float y, float scale, float r, float g, float b) {
    glPushMatrix();
    glLoadIdentity();
    
    // Set position
    glTranslatef(x, y, 0);
    glScalef(scale, scale, 1.0f);
    
    // Set color
    glColor3f(r, g, b);
    
    // Enable bitmap rendering
    glRasterPos2f(0, 0);
    
    // Render each character
    for (const char* c = text; *c != '\0'; c++) {
        glutBitmapCharacter(GLUT_BITMAP_HELVETICA_18, *c);
    }
    
    glPopMatrix();
}