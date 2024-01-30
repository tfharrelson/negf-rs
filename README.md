# negf-rs
Non-equilibrium Green's function method implement in Rust

## Overview

The goal of this package is to provide a straightforward framework for calculating the transmission function
through an interfacial region for a given set of Hamiltonians. This is nice for accurately calculating
current probabilities through interfaces and/or defect regions. These calculations may be combined with more
traditional bulk current density calculations to create accurate device level simulations of electronics.

## Roadmap

- Rebuild core using faer-rs for speed and upcoming features.
- Add python bindings with pyo3 and maturin.
- Look into gpu support.
