# aql-circt

## Overview
AQL is a domain-specific language (DSL) designed to automatically transform single-core pipelines into multi-core pipelines that enforce the intended memory consistency model. While AQL's theoretical foundations have been established and verified, it currently lacks a hardware description language (HDL) backend. Converting the DSL into Verilog using traditional tools is challenging due to inconsistencies, usability issues, and the lack of a unified platform. Most existing tools were developed independently and typically rely on Verilog or VHDL as intermediate representations (IRs), making integration difficult. The CIRCT project, a new initiative based on the MLIR/LLVM frameworks, offers a modern set of IRs that address these challenges. Leveraging CIRCT, we can compile AQL code into Verilog in a modular and consistent manner. This project explores the feasibility of representing AQL code using the fsm dialect within CIRCT. Additionally, I implemented a compiler that targets AQL code to the fsm dialect IR in CIRCT, enabling the generation of Verilog code.

## Getting Started
AQL-CIRCT is based on the CIRCT project and Rust, and you can build CIRCT and install Rust on Windows, macOS, and Linux systems.
### Install Dependencies
You can firstly install [rust](https://www.rust-lang.org/tools/install) and [circt](https://circt.llvm.org/docs/GettingStarted/) to run this compiler.
### Usage
1. 

