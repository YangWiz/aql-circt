# AQL-CIRCT

## Overview

AQL is a domain-specific language (DSL) designed to automatically transform single-core pipelines into multi-core pipelines while enforcing the intended memory consistency model. Though the theoretical foundation of AQL has been verified, it currently lacks a hardware description language (HDL) backend.

Converting AQL into Verilog using traditional tools presents challenges due to inconsistencies, usability issues, and the absence of a unified platform. Many existing tools rely on Verilog or VHDL as intermediate representations (IRs) and were independently developed, making integration difficult.

The **CIRCT project**, built on the MLIR/LLVM frameworks, offers modern IRs to address these issues. By leveraging CIRCT, we can compile AQL code into Verilog in a modular and consistent manner. This project explores representing AQL code using the `fsm` dialect within CIRCT and implements a compiler that translates AQL code into the `fsm` dialect IR, enabling Verilog code generation.

## Getting Started

AQL-CIRCT is built on the **CIRCT project** and **Rust**, and can be installed on Windows, macOS, and Linux systems.

### Install Dependencies

To run this compiler, first install the following:

- [Rust](https://www.rust-lang.org/tools/install)
- [CIRCT](https://circt.llvm.org/docs/GettingStarted/)

### Usage

1. First, generate the FSM dialect for CIRCT to produce the corresponding Verilog code:

   ```bash
   # Example:

   cargo run -- -i example.aql -o text.mlir
   ```

2. Next, use the /circt/build/bin/circt-opt tool to generate the Verilog code:
    ```
    # Example:

    ./circt-opt -convert-fsm-to-sv --lower-seq-to-sv -export-verilog ./text.mlir
    ```
