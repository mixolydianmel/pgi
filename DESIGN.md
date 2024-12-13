# PGI: Design

## Elevator Pitch

> PGI brings the power and flexibility of a virtual tabletop onto your physical
> tabletop by tracking markers and superimposing game objects onto them via a
> projector.

## Design Pillars

Usability
: It should be easy to set up, calibrate, and use the interface for tabletop
gameplay. This maximizes the range of tabletop organizers that can make use of
the interface.

Compatibility
: The interface should maintain its usefulness across multiple different
tabletop games and scenarios. The type of game or the game scenario should have
no effect on the usefulness of the interface.

Flexibility
: It should be easy to transport the interface and set it up in a wide variety
of indoor locations. Game organizers should be location-flexible to meet the
needs of their play group.

Affordability
: The interface should be as inexpensive as possible to acquire and use. The
cheaper the interface, the more people can use it.

## Core Gameplay Loop

1. Assemble the PGI
1. Place tags in the play space
    - Players can use any number of tags ≤ 30
1. Play the tabletop game with PGI assistance
    - Players move tags to move in-game objects
    - Players modify in-game objects using the computer
    - Continue playing until the game is ended
1. Disassemble the PGI

## Hardware

The hardware for this PGI setup is as follows:
- 1x Webcam
- 1x Projector
- 1x Mounting surface
- 2x Camera Mounts
- 1x Computer (e.g. laptop)
- 1x HDMI cable

This list is comprehensive for the transportable components of the PGI. For
recommendations on which hardware to purchase, visit [the Folk Computer
project's setup page](https://folk.computer/pilot). If the user is already in
possession of any of these components, they are likely usable. There are,
however, a few stipulations:

- The projector and camera should have mounting points compatible with the
  mounts. Most threaded "camera" mounts should work.
- The mounts should be able to be fastened to the mounting surface, and easily
  adjusted once fastened. For a pole, clamp mounts are most convenient.
- The camera should have as high a refresh rate as possible. For example, the
  user should prefer to run a 1080p@30fps webcam at 720p@60fps for better
  tracking.
- The camera should be connected to the computer.
- The mounting surface should be sturdy.

There are also several locational things to be considered while finding an area
to set up the interface.

- The projector's charging cable should be able to reach a power socket.
  Extension cords help.
- The HDMI cable should be able to reach the projector from the computer.
- The camera and projector should be mounted close together such that the image
  from the camera lines up as close as possible to the projector's area.
- The play space should be dimly lit, and the projection surface should be as
  desaturated and light as possible.
- The mounting surface should be secured without damaging the surrounding
  environment. Watch for ceiling tiles that can be pushed out.

Considering all these variables is tricky, but can make or break a setup.

## Software

The software component of this project is completely free and open source. PGI
uses the Bevy game engine in the Rust programming language to run the display.
The render loop is broken up into several stages:

1. Image capture
1. Tag detection
1. Input processing
1. Element updates
1. Element rendering

### Image Capture

PGI uses the Nokhwa library for Rust to access and read images from the camera.
Images are captured as frequently as possible on the main thread. Capturing and
processing images asynchronously would not provide consistent detection data.

### Tag Detection

AprilTags are a product of the APRIL Robotics team at the University of
Michigan. PGI uses a Rust port of the AprilTag C library to detect tag
positions in-frame and extract IDs.

### Input Processing

Input processing happens in parallel. The only inputs currently processed by
PGI are `Esc` to close the window, and `0-5` to decrement player health.

### Element Updates

Virtual game objects are updated using the new positional data from the
detected tags. Tag IDs are used to match in-game objects with the correct
data.

### Element Rendering

PGI uses Bevy's UI library to render. Once positioned, bevy's ECS automatically
renders the UI elements in the update phase.

> [!NOTE]
> Bevy UI is currently very early in development, and therefore very unstable.
> There are very few features. I am researching a new approach.

Since this project is built in Rust, the project should be able to run on any
platform that can compile it and works with the libraries used.
