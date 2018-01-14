Engine
======

Short Term
----------
- sky!
- add grass and trees
  - done: palm trees on sand
  - done: grass as decoration
  - todo: make grass thin rather than tile-shaped
  - todo apple trees onto grass
  - todo: pine trees onto mountains
  - todo: snow
- I'm thinking the island should be about 100 hexes across rather than
  30-ish. 64 fits in 16384 at my current big background texture size of 256
    - bigger map (start with smaller tiles)
    - change island generation routine to work with variable sizes
- error handling if texture type not found
- break from ideas and do some cleanup and *gasp* commenting (use ///)
- make draw_offset_[xy] optional in spritesheet.toml
- Add objects (start with boxes?)
- make tiles prettier (Clear water?)
- make struct holding information like orientation and stuff
  rather than dumping in an unattached variable in the event
  loop
- add slight border to one side and top of tiles so they don't visually
    merge together when they shouldn't


Medium Term
-----------
- better data structure for hex stacks, 'cause the current thing is
   unreadably ugly
- make a Trait for maps and island map be a type which implements that?
- minimap?
- rework pan/scroll key handling to allow more combos
- commit to fullscreen
 - deal with multiple aspect ratios and stuff
 - proper position of mouse clicks in different screen sizes
- coconut trees! apple trees with and without apples.
- figure out how to make the water pretty
- terrain water -- rivers (possibly also lakes, for larger maps)
- ocean water tiles
- clay next to the rivers
- lava? obsidian? (obsidian is good for sharp tools?)
- snow!
- structures:
  - Q: rimworld-style wall building, or prefab tile-sized buildings?
    - quarries
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
  - bees! (need flowers. beehives, then, of course)
- buildings. think I'm going to go for hex-sized huts rather than
   wall-building. Maybe leave room for both?
- transition hexes (from one terrain type to another)
- shadows
- proper error handling for all of those unwrap()s
- animate zoom
- animate rotation (hard because it needs to be in 3d... or at least
   pseudo-3d)
- tides!
- time passing
  - night day
  - sun, moon in sky

Longer Term
-----------

- go from island to generic map stuff
- change generation routine to an embedded script. (rhai? rlua?)
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
