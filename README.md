# dockup
Do you want an universal packaging and integration system ? Dock UP !

## Description
**dockup** is a tool built on [Docker](https://www.docker.com/) which allows to embed your application in an everywhere runnable package
The choosen programming language is [Rust](https://www.rust-lang.org/) simply because :
* It allows low level system calls
* It allows functional programming that makes code more understandable than usual sequential system programming
* Once compiled, your program has good chances to not being affected by memory issues or unexpected behavior

## How it works
Add a ```dockup.yaml``` file to your application or project root directory and execute this command to package your application :
```bash
dockup
```

### 1.First version
*Needs docker installed on both :*
* the packaging machine
* the executing machine

### Work in progress
* Create the loader module to isolate the configuration loading process
* Add a logger
* Add main arguments with command pattern
* Create the packager module to run the packaging command

