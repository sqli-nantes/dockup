# dockup [![Build Status](https://travis-ci.org/sqli-nantes/dockup.svg?branch=master)](https://travis-ci.org/sqli-nantes/dockup)
You want to simplify call to complex but recurrent commands ? Dock UP !



## Description
**dockup** transforms any complex command line into a single executable runnable from everywhere 

The choosen programming language is [Rust](https://www.rust-lang.org/) simply because :
* It allows low level system calls
* It allows functional programming that makes code more understandable than usual sequential system programming
* Once compiled, your program has good chances to not being affected by memory issues or unexpected behavior

## Why
This tool allows to wrap a complex command into a simple alias

## How to dockup your project or application launch ?
1. _First_, __Create__ a YAML configuration file at the root of your project
Example `mydockup.yaml` :
```yaml
name : myapplication
command : docker run -it --rm -v /home/user1/var/data:/var/data -p 80:8080 -p 443:8443 mycontainer
```
2. _Next_,  __Install__ the new command
```bash
$ dockup install --config /path/to/mydockup.yaml
```
This will add the config file inside a $HOME_DIR/.dockup/mycommand/dockup.yaml and create the alias.

3. _Then_, __Execute__ the program
```bash
$ myapplication
```
it will launch the command configured behind the name `myapplication`, in the example, the `docker run`command

## Install dockup
1. Download the [last release](https://github.com/sqli-nantes/dockup/releases) of dockup
2. Copy the dockup binary to /usr/bin
```bash
sudo cp dockup /usr/bin
```

## Want more ?
- If you need to test a configuration file without installing it (i.e. copying the config file into the local `.dockup` directory and creating the binary symbolic link into `/usr/bin` just call :
```bash
dockup run --config  /path/to/dockup.yaml
```
which will execute the inner `command` described in the configuration file

