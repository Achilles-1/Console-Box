# Console Box

A super simple app for drawing rectangles using characters.
The app uses Rust with the [`colored`](https://crates.io/crates/colored) module for colors in the terminal.

**Note: I'm still new to Rust. This app is just for my practice. I will be updating the code as I learn. I might optimize it and add more features.**

## Output Examples:

![One.png](/outputs/one.png)

# Points To Note

## The Program Uses Two Characters

The program uses two characters for one unit in width, height and border width.

`1 width/height/border width = 2 chars`

### Why?

Well, while printing out in the terminal, the box becomes distorted. Using 2 chars fixes it.

### Feel free to open issues 😀
