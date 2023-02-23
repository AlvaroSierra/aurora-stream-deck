# Aurora Stream Deck

This is an unofficial plugin for the Elgato Streamdeck for IVAO's Aurora ATC client.

# Installation

As of 03/2023, the plugin is not available in the ElGato Marketplace.

Installation of development version:
1. Download the latest release from [releases](https://github.com/AlvaroSierra/aurora-stream-deck/releases)
2. Unzip the rar into %Appdata%/Elgato/Plugins
3. Restart the Streamdeck app

If installation has been successful the commands should now appear.

# Issues

To report problems please provide the relevant log file available in _C:\Users\alvar\AppData\Roaming\Elgato\StreamDeck\logs_.

# Contributing

## Pre-requisites
* Install [Git](https://git-scm.com/downloads)
* Install [Rust](https://www.rust-lang.org/)
* (Optional) Install Make

## Clone the repo

To contribute to the project, you must:
1. Fork the repository to make your own copy of it
2. Edit local files
3. Commit and push your code changes to your own fork
4. Once ready to implement the changes, create a Pull Request and wait approval

## Building

To build the plugging there will be a missing dependency for aurora_api which is stored in a separate private repository due to the limitations in IVAO's Third Party. To access this repository please request Third Party access from IVAO for Aurora development and requests the owner for access.

The plugin is made up of three parts:
- Resources: images and manifest of the plugin
- Property inspector: The property inspector rendered by the Stream Deck application made with raw html and javascript
- Plugin: Made in rust and compiles to an exe.

These three parts are all compiled and moved to the Plugins directory using the makefile provided. These makefile also provides the commands to compile the individual parts
