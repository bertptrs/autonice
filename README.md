# Autonice

[![Build Status](https://travis-ci.org/bertptrs/autonice.svg?branch=master)](https://travis-ci.org/bertptrs/autonice)

Automatically nice your naughty programs!

Have you ever started a large compilation job on all your machine's
cores, only to remember that it was going to take a while anyway and
that you'd like to keep using your device in the meantime? I know I
have.

Autonice runs in the background, and periodically checks the running
process list for your selected programs, and automatically ups their
niceness by a specified amount.

## Installation

Since I am nowhere close to a final build artifact, I am going to
recommend the following:

1. Clone this repository
2. `cargo install`
3. (optional) add the cargo bin directory to your `PATH`

## Usage

The basic usage of this program is `autonice` any program you want
niced. The default behaviour is to scan every 5 seconds, and increase
the niceness of matched programs by 1. You can configure additional
options, for this, see `autonice --help`.

### Configuration

Most command line arguments can be configured in the config file as
well, located at `~/.config/autonice/config.yml`. An example is shown
below, with the default values:

```yml
step: 1 # Steps to increase niceness by
interval: 5 # Interval between process polls, in seconds
whitelist: [] # List of (partial) program names that are to be nice'd
blacklist: [] # Array of (partial) process names that should never be
              # nice'd
```

## Contributing

The project is licensed as GPLv3 since I didn't feel like thinking about
it for a long time. If this is a problem for you, let me know so I can
work something out.

Other than that, feel free to pull request for something you want to
add, or open an issue for something that should be changed.
