Tapecable[^1] is a (currently) fictional hardware platform that adds more
physical elements to computing. A Tapecable system should evoke the feeling of
data physically occupying space, instead of simply being "inside the computer".

[^1]: Name tentative.

Therefore, Tapecable follows these general design goals:

- Working with a Tapecable system is tactile: users have to touch and interact
  with several parts of the hardware when working on and using the system.
- Nothing is centralised: a Tapecable system has no central processor, nor input
  & output. Instead, it is a constellation of atomic components.
- Permanent storage media are limited & hot-swappable: a Tapecable system does
  not have a single, abstract file system. Instead, selecting data to operate on
  is done by literally swapping out storage media.
- Programming a Tapecable system is no different from other interactions: if
  components support running software, they should load it like any other data.

Concretely, a Tapecable system is a collection of _modules_ that are wired
together by plugging in cables to modules' _ports_, which are always designated
as either an _input_ or an _output_. Each module continuously reads in data from
its inputs (if any) and produces data on its outputs (if any). Beyond that,
modules may consist of any kind of hardware. For example, they can include
sensors or readers for data carriers, print or visually display data, and
feature physical switches to configure a fixed mode of operation.

## Modules

### Keyboard

- It's a computer keyboard.
- Output: the current input as characters.

### Terminal

- Square CRT monitor that displays characters in a grid.
- Very limited: you'll have to plug in and out of modules to check different
  data, or set up multiple terminals.
  - Probably no scrollback buffer, or a very small one?
- Input: the stream of characters to display.

### Processor

- Executes code.
- Form of code TBD - either a minimal language or some kind of machine code.
- Input: program code.
- Inputs: signals to be processed.
- Outputs: processed signal.
- Comes in multiple port amounts, but input amount always matches output amount.

### Patchbay

- Like for audio.
- Inputs & outputs: depend on switch configuration, but the unit has 8 columns
  of front in- & outputs.

### Printer

- Prints on a roll of thermal paper.
- Features a manual lever/button to cut off.
  - Or simply a row of teeth to help you tear off?
- Input: signal to print.
- Switch: whether to run in character or graphing mode.

### Oscilloscope

- It's an oscilloscope: continuously graphs a signal.
- Input: the signal to graph.
- Slider: X range.
- Slider: Y range.

### Casette Reader

- Reads the signal off of a tape casette until the end of the tape.
- Output: the read signal.
- Button: start.
- Button: stop & rewind.
- Button: rewind & eject.

### Cartridge Reader

- Reads a data cartridge.
- Output: the read data.
- Button: (re)start read.
- Switch: loop (automatically restart read after data end).

### Casette Writer

- Writes a signal to a tape casette until either data end or tape run out.
- Input: the signal to write.
- Button: record.
- Button: rewind & eject.

### Cartridge Writer

- Writes a data cartridge
- Input: the data to write.
- Button: wipe cartridge.
