# Rust Snake

## Introduction

During my college studies, I developed a Snake game using Python in the terminal environment, which proved enjoyable. For this project, I am implementing Snake in Rust for personal interest. Departing from convention, I am creating a graphical user interface (GUI) using Vulkan rather than a terminal-based approach.

My first step is creating engine for this.

## Dependencies

The graphics library selected is **Vulkano**, providing high-level abstractions over complex Vulkan operations with minimal `unsafe` code-simpler than alternatives.

| Category          | Crate(s)                  | Purpose                            |
|-------------------|---------------------------|------------------------------------|
| Window Management | `winit`                   | Window creation and event handling |
| Logging           | `tracing`                 | Structured logging                 |
| Physics           | `rapier2d`                | 2D physics simulation              |
| Image Processing  | `image`                   | Image encode and decode            |
| Profiling         | `tracy_client`            | Performance tracking and leaks     |

## Development Process

This project proceeds without a formal plan. My primary goal is to acquire practical Vulkan experience while applying Rust knowledge. Development occurs intermittently; comprehension challenges occasionally require days or weeks to resolve.

## Conclusion

This implementation does not exemplify engine development practices. Instead, it highlights common pitfalls to avoid, particularly frequent memory leaks.

## Building

It's build steps **for GNU/Linux and FreeBSD**

First step is building shaders
```sh
for shader in "./shaders"/*; do
  if [[ "${shader}" == *.frag || "${shader}" == *.vert ]]; then
    glslangValidator -V "$shader" -o "$shader.spv"
  fi
done
```
And then you can build engine and game with `cargo build` or `cargo run` for game

## TODOs
[ ] - Добавить обработку событий для объектов
[ ] - Создать возможность добавлять текст
[ ] - Горячее обновление объектов(например при изменении текста в объекте текста он должен автоматически обновлять дескриптор сет)
