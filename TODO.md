Engine
======

Short Term
----------

- hex stacks
- island generation routine
- Add objects (start with boxes)
- add grass and trees
- make tiles prettier (Clear water?)
- make struct holding information like orientation and stuff
  rather than dumping in an unattached variable in the event
  loop
- read texture tile metadata (position, size) from text file
- fixme: decide where orientation enum 
  datatype lives
- function for drawing backgroud rather than ugly block of code
  in game loop
- animate rotation
- add a struct to hold all the textures together, too
- note to self: I'm imagining the main loop to have these draw layers:
  - background
  - decorations
  - objects
  - actors
  - ui

Medium Term
-----------
- Scroll around map
 - vague ideas about map size and screen size
- minimap?
- commit to fullscreen
 - deal with multiple aspect ratios and stuff
- figure out how to make the water pretty
- terrain water -- rivers (possibly also lakes, for larger maps)
- lava? obsidian?
- read terrain _type_ from text file rather than hard-coding
- split engine from game
- transition hexes (from one terrain type to another)

Longer Term
-----------

- go from island to generic map stuff
- un-hardcode terrain types? or, make more. one or the other :)
- target games:
  - island survival (rimworld + sims + minecraft)
  - ai evolution simulation (plant / herbivore / carnivore)
  - civ-like city builder
  - roguelike generative hack-and-slash RPG
- add flat-top hex orientation (and six more directions)

Game
====

- Add people!
- 
