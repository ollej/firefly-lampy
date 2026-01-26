# Lampy

A multiplayer game for the [Firefly Zero](https://fireflyzero.com)
handheld retro gaming console.

Race to be the first to collect fireflies for 20 points using matching colored
flashlights.

Guide the fireflies with a flashlight matching their color. Each button activates a different colored flashlight.
- N -- Red
- E -- Magenta
- S -- Green
- W -- Blue

Different colored fireflies are worth a different amount of points, and they
are less common when they're worth more points.

- Red -- 1 point
- Magenta -- 2 points
- Green -- 3 points
- Blue -- 5 points

![Screenshot of the game Lampy](screenshot.png)

## Controls

- Steer with the touchpad, the left joystick on controller while holding down left shoulder button, or arrow keys on keyboard.
- Use buttons to activate different colored flashlights.
- Trap fireflies in the flashlight matching their color and bring them to the collection point in the center of the play area.
- First player to collect fireflies worth a total of 20 points wins.
- Fireflies are worth different amount of points.

## Usage
- Press menu button to access Credits and Info screens.
- There is a badge for 10 wins.
- Some cheats are available via Firefly CLI:
  - restart -- Restart the level.
  - debug -- Show debug lines showing what point fireflies are attracted to.
  - add-points -- Add points to the current player.

## Installation

To play the game you need the Firefly Zero emulator (or the hardware once it's
released). Instructions on how to [install the Firefly Zero
emulator](https://docs.fireflyzero.com/user/installation/).

### OS X (Mac) or Linux
Run the following:
```sh
PLAY="$(curl https://fireflyzero.com/play.sh)"
bash -c "$PLAY" -- olle.lampy
```

### Windows
Istall firefly_cli and then run:
```
firefly_cli import olle.lampy
firefly_cli emulator --id olle.lampy
```

## Credits

- Programming: Olle & Catboots
- Graphics: Catboots
- Font: kenney (License CC0-1.0) & Pico-8 (CC0-1.0)
- Music and sound effects: (OpenGameArt.org)[https://opengameart.org/]

## License

Released under the MIT License.
