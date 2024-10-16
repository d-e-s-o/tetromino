0.1.3
-----
- Added `xlock` screen lock "mode"
- Improved key event handling logic to eliminate chance of missed key
  presses
- Made initial AI state configurable via configuration
- Updated `glutin` dependency to `0.31.0`
- Updated `winit` dependency to `0.29.2`


0.1.2
-----
- Fixed update of internal field state on final stone collision to
  prevent potential invariant violation
- Added auto-playing AI


0.1.1
-----
- Handle stone rotation and horizontal stone movements while pausing to
  clear complete lines
- Switched to using Backspace key for restarting the game


0.1.0
-----
- Initial release
