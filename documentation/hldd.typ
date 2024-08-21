#import "@preview/cheq:0.1.0": checklist

#set text(font:"DejaVu Sans")

= Physicalized Game Interfaces
_Melody Henrich_

== Overview
This directed study’s purpose is to experiment with folk computers in the
context of games. A folk computer is a three part setup, consisting of a
computer (e.g. Mini PC or Raspberry Pi, or perhaps a laptop), a projector, and
a camera. The camera is then used to read visual input in the form of AprilTags
(like QR codes, but specialized for robotics) that contain encoded computer
instructions, placed on a flat surface. These instructions are processed by the
computer, and the graphical results are projected back onto the flat surface on
or near the ArilTags, depending on what code they contain. The key aspects of
folk computers is that they endow physical objects with computational
properties on a human/room-wide scale, while making those physical objects
user-scriptable. While this project is folk computer inspired, it may not take
the form of a full folk computer, but something that abides by the same
principles nonetheless.


#pagebreak()

== Final Deliverables
- A game interface that adheres to the core folk computer aims
- A development log that contains the following
  - Week-by-week progress updates
  - Meeting notes from update meetings with faculty supervisor
  - Playtest notes from any playtests conducted during the project
- A pitch deck for the use of the product as a game interface
  - Use case and benefits of interface over others
  - Retrospective on the development process, key decisions, etc.
  - What would continued development look like?

== Action Items

=== Design
- [ ] Investigate viability of using self-made software for the product instead
  of the folk software
- [ ] Plan and assemble a portable setup for the product
- [ ] Decide the game experience to be accomplished with the product
- [ ] Playtest the game experience

=== Programming
- [ ] Set up the interface by doing one of the following
  - [ ] Set up the software side of a folk computer
  - [ ] Write and set up custom software to accomplish the same objectives
- [ ] Implement the game experience using the interface
- [ ] Make any necessary changes from playtesting

#pagebreak()

== Schedule
#table(
  columns: 2,
  stroke:none,
  align: left+top,
  [*Date Due*],[*Deliverables*],
  [2024-07-15],[
    - [ ] Investigate viability of using self-made software for the product
      instead of the folk software
    - [ ] Plan and assemble a portable setup for the product
    - [ ] Decide the game experience to be accomplished with the product
  ],
  [2024-07-22],[
    - [ ] Set up the interface by doing one of the following
      - [ ] Set up the software side of a folk computer
      - [ ] Write and set up custom software to accomplish the same objectives
    - [ ] Begin to implement the game experience using the interface
  ],
  [2024-07-29],[
    - [ ] Continue to implement the game experience using the interface
    - [ ] Playtest the game experience
    - [ ] Make any necessary changes from playtesting
  ],
  [2024-08-05],[
    - [ ] Playtest the game experience
    - [ ] Make any necessary changes from playtesting
    - [ ] Finish implementing the game experience using the interface
  ],
  [2024-08-12],[
    - [ ] Wrap-up development and finalize git repository
    - [ ] Create the pitch deck
    - [ ] Prepare the development log for final submission
  ],
)

#pagebreak()

== References
- #link("https://folk.computer/")[The official Folk Computer website]
- #link("https://folk.computer/pilot")[Folk computer hardware advice]
All photos taken from the official website.
