# dockup
Do you want an universal packaging and integration system ? Dock UP !

[![Build Status](https://travis-ci.org/sqli-nantes/dockup.svg?branch=master)](https://travis-ci.org/sqli-nantes/dockup)

## Description
**dockup** is a tool built on [Docker](https://www.docker.com/) which allows to embed your application in an everywhere runnable package
The choosen programming language is [Rust](https://www.rust-lang.org/) simply because :
* It allows low level system calls
* It allows functional programming that makes code more understandable than usual sequential system programming
* Once compiled, your program has good chances to not being affected by memory issues or unexpected behavior

## Why
This tool allows to wrap a complex command into a simple alias

## How it works
1. Describe the command and the name of the alias to associate into a YAML structured file
2. Install the new command
```bash
dockup install --config mydockup.yaml
```
This will add the config file inside a $HOME_DIR/.dockup/mycommand/dockup.yaml and create the alias.
Next, the alias when used, will simply call a
```bash
dockup run --config  $HOME_DIR/.dockup/mycommand/dockup.yaml
```
which will execute the given `command`

Add a ```dockup.yaml``` file to your application or project root directory and execute this command to package your application :
```bash
dockup run --config dockup.yaml
```

### 1.First version
*Needs docker installed on both :*
* the packaging machine
* the executing machine

### Work in progress
* Add main arguments with command pattern
* Create the packager module to run the packaging command

