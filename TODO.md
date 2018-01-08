Engine
======

Short Term
----------

- add grass and trees
  - still 50/50 on whether grass is better as terrain tile or decoration
   so start with trees
  - split hexmap.rs into hexgeometry.rs and landscape.rs (done)
    - while I'm at it, regularize use of r,q and x,y (and i,j)
    - instead of one call to draw the background map, do
      - draw map (tiles, depth sorted)
      - draw objects (decorations, items, and actors -- depth sorted)
      - draw ui (compass rose)
  - need a things.rs
    - generate palm trees onto sand 
    - generate apple trees onto grass
      - possibly generate grass onto dirt? if so, are trees at a different
        "elevation"?
    - generate pine trees onto mountains
      - um, snow?
- I'm thinking the island should be about 100 hexes across rather than
  30-ish. This implies need to implement zooming and scrolling sooner
  rather than later. Maybe 60-ish will do, so just half size?
  - texture width is 16384 on both of my laptops; that gives 256 tiles at 64
    wide without any shenanigans. (So 128 wide seems very manageable.)
  - anyway, do this:
    - bigger map (start with smaller tiles)
    - zooming
     - prescale graphics tiles to different resolutions because SDL's
       render 
    - scrolling
    - change island generation routine to work with variable sizes
- error handling if texture type not found
- break from ideas and do some cleanup and *gasp* commenting (use ///)
- Add objects (start with boxes?)
- make tiles prettier (Clear water?)
- make struct holding information like orientation and stuff
  rather than dumping in an unattached variable in the event
  loop
- read texture tile metadata (position, size) from text file
- function for drawing backgroud rather than ugly block of code
  in game loop
- add a struct to hold all the textures together, too
- get sensible about integer datatypes instead of randomly using i32 for no
    good reason
- add slight border to one side and top of tiles so they don't visually
    merge together when they shouldn't
- fix: diagonal drawing routine shifted with even map size


Medium Term
-----------
- better data structure for hex stacks, 'cause the current thing is
   unreadably ugly
- make a Trait for maps and island map be a type which implements that?
- minimap?
- commit to fullscreen
 - deal with multiple aspect ratios and stuff
 - proper position of mouse clicks in different screen sizes
- coconut trees! apple trees with and without apples.
- figure out how to make the water pretty
- terrain water -- rivers (possibly also lakes, for larger maps)
- lava? obsidian?
- snow!
- read terrain _type_ from text file rather than hard-coding
- possibly separate sides and top of hexes, so they can more easily be
  variable height
- make a game setup function rather than dumping that at the beginning
    of the event loop
- architectural decision re universal event loop with different states
  _or_ different loops for game menu and game itself
- split engine from game
- moving things: actors!
  - people (first cut: wander randomly)
  - monkeys
  - turtles 
  - alpacas 
  - mountain lions? (or, island too small?)
  - fish / dolphins / sharks
- buildings. think I'm going to go for hex-sized huts rather than
   wall-building. Maybe leave room for both?
- transition hexes (from one terrain type to another)
- shadows
- proper error handling for all of those unwrap()s
- animate zoom
- animate rotation (hard because it needs to be in 3d... or at least
   pseudo-3d)
- tides!

Longer Term
-----------

- go from island to generic map stuff
- un-hardcode terrain types? or, make more. one or the other :)
- move island generation and the specific island terrain types
   an add-on
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
