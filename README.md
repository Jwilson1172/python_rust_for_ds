# Python with Rust bindings

This repo is both a rust tutorial for myself but also documentation to showcase how a data scientist may use rust to speed up some python processes that are undgodly slow in python alone.

I'll be doing that by writing rust extensions that interact with the standard C library for CPython

---

### Cool I guess but what does that mean for me?

If you are working with python for ds and want some things to go faster you may be able to use some of my code to do that.

### getting started

If you want to fork the repo and start developing for this repo then your actually looking for [Contributing.md](./.github/CONTRIBUTING.md) head over there first then come back here.

if you are trying to git clone this and using it for building a python wheel with rust translated C bindings then goto `building a python wheel with rust bindings from docker`

if you are looking to extend your rust with the data science functions that have been written for python then take a look at `building a crate for rust development of python friendly function bindings`

### TODO:
- [ ] building a python wheel with rust bindings from docker.
- [ ] building a crate for rust development of python friendly function bindings.
- [ ] setup CI/CD to handle testing and release building
- [ ] setup contrib tracking, github actions for bug and issue tracking
- [ ] setup kaban board for feature/issue tracking.