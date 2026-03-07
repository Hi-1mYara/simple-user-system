<h1 align="center">
    <br>
    <img alt="ferris-the-rustacean" src="https://github.com/Hi-1mYara/users/blob/master/img/rustacean-orig-noshadow.svg?raw=true" width="256">
    <br>
    SUS
    <br>
</h1>

<h3 align="center">The Simple User System</h3>

<p align="center">
    <a href="https://rust-lang.org/">
        <img alt="[Rust] It's pretty quick!" src="https://img.shields.io/badge/It%27s_pretty_quick!-black?logo=rust&logoColor=white">
    </a>
    <a href="https://www.youtube.com/watch?v=XfELJU1mRMg">
        <img alt="check me out on Youtube" src="https://img.shields.io/badge/Check_me_out_on_Youtube!-red?logo=youtube&logoColor=white">
    </a>
    <a href="https://www.linuxmint.com/">
    <img alt="(Linux mint) It works on my machine" src="https://img.shields.io/badge/%22It_works_on_my_machine%22-86BE43?logo=linuxmint&logoColor=white">
    </a>
</p>

<p align="center">
  <a href="#overview">Overview</a> •
  <a href="#features">Features</a> •
  <a href="#planned-features">Planned features</a> •
  <a href="#installation">Installation</a> •
  <a href="#reporting-issues">Reporting issues</a> •
  <a href="#credits">Credits</a>
</p>


## Overview
The Simple User System is a framework for creating users, which serves as my testing ground for all things Rust (while I'm learning anyway). Do not use this, it is not tested for safety and is missing a lot of features (Though it doesn't do anything useful yet or connect to anything, so the danger should be minimal).

This was made while I was going through chapter 8 of the Rust Book, don't expect much. I plan to update this when my knowledge expands.

## Features
- User creation features including:
  - username, email and UUID
  - ability to make a user an admin
  - activity state  

(There really should be more here, huh. Again, more will be added as I learn)

## Planned features
- [ ] User deletion
- [ ] Command line arguments
- [ ] Actual security
- [ ] A nice looking TUI (maybe)
- [ ] Integration in another program (whatever that means)

## Installation
**Don't install this, you have no good reason to.**

If you really want to do so, however, you need to compile it from source.  
If you do not have Rust installed, follow the installation guide [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

When Rust has been installed, run the following commands:  
```bash
# Clone the repository
# run this command in a directory of your choosing
git clone https://github.com/Hi-1mYara/users
cd simple-user-system/

# Build the project
# the executable will be found in target/release/users
cargo build --release
```
If you wish to execute the program from the command line by its name alone, put it in /bin or /usr/bin

## Reporting issues
When you see a problem in the code, run into bugs or the README/license is wrong, please open an issue. Include in your report the steps to reproduce the errors and your operating system. If you think there is something about the configuration of your system that may be causing the problem, include that too.  

**Reporting guidelines**
- Be civil and respectful
- Keep it to the problem at hand
  - Only comment on the technical details
  - Keep any personal details out of the issue, for both your privacy and for reducing clutter in your report

## Credits
- Hi_1mYara (as in, me)
- The Rust Programming Language (affectionately called [The Book](https://doc.rust-lang.org/book/title-page.html))
  - the primary learning material provided by Rust itself

- Inspiration
  - [DocJade](https://github.com/DocJade)
    - The madman who made a filesystem ([Fluster](https://github.com/DocJade/fluster_rs)) to run factorio from floppy disks
    - a nice example of Rust in practice [(do check him out on youtube too)](https://www.youtube.com/@DocJade)
  - [Dysk](https://dystroy.org/dysk/) ([GitHub](https://github.com/Canop/dysk))
    - Inspired me to learn Rust in the first place, i wanted to create projects like it.
