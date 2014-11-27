Path of exile companion
====

This program is a utilty program for the free 2 play action rpg Path of Exile
(www.pathofexile.com). It calculates some basic item information for you while
playing.

I created this project for a couple of reasons.
* I wanted to learn rust
* I can not use the overlay based poe tools due do a driver issue with my ATI
  GPU.
* I want a program that runs on Linux as well as Windows (mac too but I dont
  have a mac)

Dependencies
====
As I have no idea how to read the clipboard in rust I use a small python script
to do it.

Python 2.7 tk support required (might work with 3 but it is untested)

How it works
====

In poe, hover over a item and press Ctrl + c to copy it's information to the
clipboard, then you can invoke this program to have it print some very basic
information about the item.

Future plans
====

There are a lot of missing functionality in this version and it is heavily
untested. Be sure to leave an issue if you try it out and it does not work for
you (very likely).
