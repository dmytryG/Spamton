# Spamton, G [keyboard] spammer

### Brifely about
Spamton is a cross-platform keyboard emulator witch can be used for
*friendly* spam in messages (please, don't be toxic bustards with this thing).
Due to Spamton emulates key-pesses and CTRL-V events on your keyboard it can
be used almost with any moder PC messangers and you can use it without any
fear for get baned because of spam-soft (in case if you're not toxic bustard, of course)
This education product is open-source, so you can modify it as you want.
If you're made your modifications and want to share it, just made a fork of
this repo or made pull request.
### Settings
In the file .env you can find all the available settings for this program:
+ use_clipboard - parameter that speed-up auto-typing by setting your clipboard
    and CTRL-V it to the messenger window (can be *true* of *false*)
+ source - Sets the file, where prepared for sending phrases are separated by lines
  (if it file are in root of the program folder, you don't have to specify
   full path to it)
+ delayment - Just delay between sending messages (integer in milliseconds)
+ speed - When it is keyboard typing, sets the delay between pressing keys 
+ is_autosend - Defines if program will automatically press "return" key after message
    is typed (can be *true* of *false*)
### Source
This file describes the messages that will be sent in random sequence
Each message have to be placed on one line. Content of the file should structured
as shown below:

---

    Message 1
    Message 2
    Message 3

---
### System requirements
As was mentioned before, Spamtom is cross-platform application, it can
run under Windows, Linux and macOS systems (Windows and macOS haven't been
debugged yet). Spamton wrote using rust programming language and require
cargo in order to run. How to install all the necessary basis software
perfectly described in official manual [CLICK ME](https://doc.rust-lang.org/book/ch01-01-installation.html)
> If you're using linux, you also have to install several libs

    sudo apt install xorg-dev libxcb-composite0-dev libxdo-dev

---

### In order to run
+ Open console
+ cd /path/to/root/folder
+ cargo run
+ ???
+ Profit




                                                                       ██
                                                   ▄▄▄▄▄▄▄          ▄▄▄███▌
                                               ▀█████████████▀   █████████▌
                                             ▐████████████████████████████▌
      ▄▐▄▐▄▄▐▄▄▐▄▐▄▄▐▄▄▐▄▐▄▄▐▄              ▀▄████████████████████████████▌
        ▀▀▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓           ▐█████████████████████████████████
           ▀▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓           ▐█████████████████████████████████  ▄▄▄█▌
             ▀▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▄          ▐█████████████████████████████████▄████
             ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓          ▐████████████████████████████████████
              ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓           ▀██▀    ▀█████▀       ▀▀▀████████▀▀
              ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓            ████████ ▐█   ▐███████▌ ████████
              ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓           ▐█        ██▄▄▄█▌░░░░░░░▓█  ▀██████▌
              ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌           ▐█        ██▀▀▀█▌░░░░░░░▓█    ▀████▌
              ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌            ▐█        █▌   █▌ ░░▄░ ░▓█     ████▌
              ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▀               ▀▀▀▀▀▀▀▀      ▀▀▀▀▀▀▀▀  ▓▒▓▒▓████▌
             ▄▓▓▓▓▓▓▓▓▓▓▓▓▀                ▄▄▄                    ▐███▀▒░████▌
           ▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀          ▀▀▀▀▀▀                        ▐█  ▄██████▀
          ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▀   ▄▄▄▄▄▀▀▀                     ▄▄▄▄▄▄▀▀███ ▐██████▄▄▄▄▄█▌
         ▄▓▓▓▓▓▓▓▓▓▓▓▓▌      ▐░░░▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▓██████        ███ ▐████████████▌
         ▓▓▓▓▓▓▓▓▓▓▓▀                         ▄▄▄█▌              ███ ▐█████████▀▀  ▄
        ▓▓▓▓▓▓▓▓▓▓▀                   ▄▄▄▄▄▄▄▄▀▀▀██▄           ▄▄███ ▐█████████▄▀▀██
        ▓▓▓▓▓▓▓▓▀         ▐██████████▌            █████████████  ███ ▐█   ▐███████▌
       ▄▓▓▓▓▓▓▓▀▀▀▀▀▀▀▀▀▀▀▀ ░░░░░            ▐▄▄▄██▀             ███ ▐█       ▀▀▀
      ██▓▓▓▓▓▓▓░░░░░          ░░░░░        ░█████              ▄████ ▐█
      ▐█▒▓▓▓▓▓▓▓░░░░░          ░░░░░      ▄▓██▌   ████████████████    ▐█
      ██░░░▓▓▓▓▓▓░░░░░░▌        ░░░░░ ░▐▄▄▄▀  █▌                      ▄▄█▄▄
      ██░░░░▓▓▓▓▓░░░░░░▌        ░░░░░░█▌      █████              ▓███████▒░
      ██░░░░░░░░░░░░░░░▌        ░▄▄▄▀▀        ▄▓██████▀▀▀▀▀▀▀▀▀████████████▄▄
      ▀▀█▒░░░░░░░░░░░░░   ░░░░░▐▓▀▀▀       ▄▄███████▌░        ▄▄░█████████████▄
       ▐█▓▒▒▒░░░░░░░▒▒░░░▓█████▌           ░▓█████████▒▌   ░▐▒████████████████▒▌


