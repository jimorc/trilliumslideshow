# trilliumslideshow
## About

trilliumslideshow is a program (well, it will be eventually) that processes input files including JPEGs
containing the images that should be used in a slide show, as well as a file that contains information about
the JPEGs. Slide show functionality is also included.

trilliumslideshow contains two parts:
- part 1 allows the user to:
  - select the slides to be displayed.
  - sort the slides according to information in the information file.
  - generate title slides based on information in the information file.
  - create a new information file containing the title slides and images. This file would be input to the
    second part of the program to control the order in which the slides are displayed.
- part 2 runs the slideshow based on information in a selected information file.

trilliumslideshow is based on the design of two programs: 
[trilliumshowfx](https://github.com/jimorc/trilliumshowfx) and
[flexishow](https://sourceforge.net/projects/flexishow/).

### Limitations of trilliumshowfx and flexishow

trilliumshowfx was initially designed for a specific purpose: to automate a number of
manual processes that are required to take the output from the WordPress Entry Wizard plugin, generate
title slides, sort the images, and create a .XLS file for input to flexishow.

XLS files suffer from at least two problems:
1. XLS files are proprietary to Microsoft Excel. This limits the number of applications and operating systems
that the file contents can be viewed in.
2. XLS is the old MS Excel format that was replaced by XLSX in Excel 2007. While most programming
languages that are used for generating computer desktop applications have libraries for reading and
writing both XLS and XLSX, a number of newer languages only support XLSX. I will be using one of those
newer programming languages to create trilliumslideshow.

A potential alternative would be to update flexishow to also accept XLSX files, but that only potentially
solves one problem. There are others:

3. While Microsoft claims that XLSX is a standard, reality shows otherwise. Officially, XLSX is an open
standard called Office Open XML, supposedly documented as standards ECMA-376 and ISO 29500. However, Microsoft
only partially follows this standard and implements many proprietary, undocumented, additions that mean
XLSX files cannot always be interchanged with other spreadsheet programs. In other words, while Microsoft
was responsible for ramming Office Open XML through two standards organizations, they do not follow the
standard themselves!
4. While both trilliumshowfx and flexishow were written in Java, they use different GUI libraries, so they look
very different. This can be quite jarring to users when switching from one program to the other. It might be
possible to change the UI of one or the other program, the resulting UIs still
are not native to the operating system that the programs execute on.
5. Only CSV files are allowed as input to trilliumshowfx. This limits the number of sources of image
information available to the program.
6. flexishow has not been updated in a number of years. I could not find the source code for the program,
nor any way to generate an issue (bug report, change request, etc.).

## Status
### Development

Development of trilliumslideshow is just beginning, so at the moment, it is of limited to no use.

Initial development is being done on Linux (specifically Kubuntu). Windows 11 and MacOS (Apple silicon
only) versions will be provided when the Linux development is complete.

## Documentation

### License
trilliumslideshow is licensed under the MIT License. A copy of the license in included in this
project's [LICENSE file](LICENSE).

### Program Documentation

Documentation on how to use the program will be provided in this repository's Wiki when the first release
of the program is created.

### Contributing Instructions

Instructions on how to contribute to trilliumslideshow are provided in [CONTRIBUTING.md](CONTRIBUTING.md).

### How to Build From Source

To be added, probably in CONTRIBUTING.md.