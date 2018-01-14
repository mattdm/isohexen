This is an experiment in learning Rust. It is, eventually, to be an island
survival game, drawing inspiration from Rimworld, The Sims, Minecraft, and
The Settlers (the Amiga game, not Catan — despite the hexes).

It will also be basic engine for that game, which might be extensible to
other things I feel like playing with. (Maybe something more Civ like?
Ambitious Master of Magic clone? A procedural hack-and-slash RPG?)

If this ever gets to the point where it might be useful for someone else,
I'll increase the version to at least 1.0.0. Right now, it definitely isn't.

This crate is currently a binary, but the plan is to split this into an
engine and a game — or maybe eventually several games — which use that
engine.

Currently, draws an "island", which you can rotate with page up and page
down keys or by clicking on the compass rose. You can regenerate the island
by pressing `G`.

See the [TODO list](TODO.md) for plans and vague thoughts.

![State of the Art](screenshots/20180113-660a6db.png)
