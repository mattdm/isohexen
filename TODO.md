Engine
======

Short Term
----------

- add grass and trees
- Add objects (start with boxes)
- make tiles prettier (Clear water?)
- make island generation routine use areas of earth and grass
  rather than going by rings
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
- get sensible about integer datatypes instead of randomly using i32 for no
    good reason
- diagonal drawing routine shifted with even map size

Medium Term
-----------
- Scroll around map
 - vague ideas about map size and screen size
- better data structure for hex stacks, 'cause the current thing is
   unreadably ugly
- minimap?
- commit to fullscreen
 - deal with multiple aspect ratios and stuff
 - proper position of mouse clicks in different screen sizes
- figure out how to make the water pretty
- terrain water -- rivers (possibly also lakes, for larger maps)
- lava? obsidian?
- read terrain _type_ from text file rather than hard-coding
- possibly separate sides and top of hexes, so they can more easily be
  variable height
- split engine from game
- transition hexes (from one terrain type to another)
- shadows
- proper error handling for all of those unwrap()s

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
