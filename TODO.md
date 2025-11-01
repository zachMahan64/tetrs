### To-Do list
#### Core
- Collision
    - Rot: impl by cloning piece call rotation, check if theres a conflict -> if no make piece = cloned piece (which rotates it)
    - Movement: simpler, so don't clone piece, just look into board and see if tiles w/ blocks +/- 1 are collinear
    - Downward sticking: for "sticky" mode, don't make it stick until the next tick (i.e. not immediately)
    - Bug test core gameplay before moving on
- Line clearing
- Score/non-persistent highscore w/ names
- Difficulty (levels 1-10, maybe more?)
- Advanced stats (live) -> inputs per second (average overall and last ~3 seconds)
- Music w/ rodio (toggle-able)
#### Extra
- Persisent highscore (file, decide where) and advanced stats?
- Holding (toggle-able)
- VFX (like cool graphic on a tetrs)
- SFX (like satisfying sounds on line clears/tetrs)
- Toggle-able sticky or slidy pieces (essentially NES/GB vs modern sytle tetris)
#### Very Extra
- Online leaderboard w/ http server DB
