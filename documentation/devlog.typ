#import "@preview/tufte-memo:0.1.1": *

#let hline = [#line(length:100%,stroke:0.5pt+black)]

#show: template.with(
  title: [Folk Computers as Game Interfaces],
  shorttitle: [Folk Game Interfaces],
  subtitle: [Development Log],
  authors: (
    (
      name: "Melody Henrich",
    ),
  ),
  toc: false,
  draft: false,
)

= Introduction

This document is the development log outlined in the Higher Level Design
Document (HLDD) submitted along with the initial directed study
request.#note[The plan of study is available in the same GitHub repository as
this log.]
As such, this document contains the following:
- Week-by-week progress updates
- Meeting notes from update meetings with faculty supervisor
- Playtest notes from any playtests conducted during the project

The week-by-week progress updates will take the form of notes on what
deliverables were and weren't completed during the week, and the plan for the
following week.#note[These weekly updates will begin from after the first
meeting with my faculty advisor Prof. Murphy on 2024-07-12.] Additionally, any
changes in scope or project direction will be documented in the weekly logs,
and later worked into the finalized HLDD.

The meeting notes will be initialy taken in bullet form, but reformatted into
prose for this log. The same process will be used for playtest notes.

// INFO: Weekly Updates Section
#pagebreak()

= Week 1

== Deliverables completed
- Initial hardware setup
- Prelim. software architecture design
- Documentation layout
- Timeline established
- Set up software project

== Deliverables to-do
- Redo software design for new deliverables
- Read camera input into software
- Detect AprilTags
- Drawing hard-coded shapes

#hline

== Calibration moved from hand-calibration to hybrid.
Aspect ratios between the camera and projector match, but there is a slight
fisheye in the camera and there will always be a slight offset. In order to
achieve maximum accuracy, a vector offset must be _combined_ with a skew-like
projection tranformation to account for any distortion in the image.

== Design change removing the editor.
Due to scope concerns, the editor side of the project will be scrapped in favor
of manual authoring of data files. This is now a reach goal, allowing project
goals to remain MVP-oriented and achievable within the timeframe of the
directed study.

// INFO: Meeting Notes Section
#pagebreak()

= Meeting Notes

== Kickoff Meeting 2024-07-12

- Work begins!
- Most things are already pre-planned
- Scrapping editor, slimming down scope
  - Reach goal
- Think about 3D applications
- Timeline looks good

== Update Meeting 2024-07-18

- 
