---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Assignment 3
author:
- Olof Ljunggren
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
#- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Visual computing fundamentals TDT4195 Assignment 2

## Task 1
Task 1: More polygons than you can shake a stick at [1 point]

### c: Extend the fragment shader to take in the normals that the vertex shader outputs.
Crater normals:
![
    Crater normal visualization.
](images/normal_visualization.png)

### d: Simple lighting.
Simple ighting:
![
    Simple lighting implemented.
](images/helicopter_drawn.png)


## Task 2
Helicopter Parenting [1.0 point]

### c: Draw helicopter.
Helicopter drawn:
![
    Helicopter drawn.
](images/helicopter_drawn.png)


## Task 5
Help! My lighting is wrong! [1 point]

### a: Initial helicopter lighting.
Without normal correction:
![
   Before light correction, first side.
](images/light.png)

![
   Before light correction, other side.
](images/dark.png)

### c: Corrected helicopter lighting.
After normal correction:
![
    After light correction, first side.
](images/light_corrected.png)

![
   After light correction, other side.
](images/light_corrected2.png)


## Task 6
Time to turn this thing up to 11 5 [0.5 point]

### a: Multiple helicopters.
Five moving helicopters:

![
   Five moving helicopters.
](images/5copters.png)


## Task 7
Optional Challenges [At most 0.51 points]

### a: Phong lighting.
I created three components in the fragment shader. One for ambient light, one for diffuse component and a specular component. These were summed up and multiplied with the vertex initial color. To accomplish this, I needed the position and color of the light source and the position of the camera. Then I also improved the normals by multiplying them with the model matrix.

![
   Phong lighting.
](images/phong.png)

### d: Door animation.
To accomplish animation, I created a few variables which told me if the door was open or not and if we were to C (close) or O (open) the door. Then I did a sin/cos animation which had a slide in the x and z direction. 0.5 seconds outwards and then 1.5 seconds sliding the door.


