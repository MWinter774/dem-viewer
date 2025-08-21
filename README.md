# dem-viewer
## Prerequisites
1. Rust
## Project Structure
### DEM/
Contains GeoTIFF files that can be used to render inside the engine
### shaders/
These are the shaders used in order to render the objects in the engine
### src/engine/
Contains the source code of the engine
### src/engine/camera_view
This is the code handling the window that shows the 2D world from the camera view, to later be 
matched with points in the 3D world
### src/engine/epnp
Code handling data structures regarding EPnP algorithm, the main component here is the epnp_manager.rs, which is handling calculating the camera position based on points from the camera view window and the 3D points that the user has picked
### src/engine/feature_matching
Everything regarding feature matching algorithm and the engine's logic behind it
### src/engine/input_system
Wrappers for reading mouse and keyboard input
### src/engine/models
Objects representing 3D world drawable objects
### src/engine/opengl
OpenGL object wrappers
### src/engine/renderers
Renderers for everything that the engine is capable of rendering to the screen
### src/engine/shader_programs
Wrappers to each shader the is in [shaders/](#shaders) directory
### src/engine/transformations
Contains code handling object's position, rotation and scale in 3D world

## Building + Running
Simply run:

`cargo run`

## Keybinds
`P` - Camera view point picking

`B` - Calculate estimated camera position