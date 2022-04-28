Rust Universal Machine
Connor Gray

The program successfully executes all provided binary instruction files.

Structure:
rum
   - cargo.toml
   - src
      - main.rs
      - lib.rs
      ---- Modules ----
      - rum.rs
      - instructions.rs
      - bitunpack.rs
      -----------------
      
---------------------------------
|     File     |   Knows about  |
| ------------ | -------------- |
| main         | rum            |
| instructions | rum, bitunpack |
| rum          | instructions   |
| bitunpack    |                |
---------------------------------

main:
- Creates vm struct from rum, boots and executes vm

rum:
- Calls functions from instructions module based on opcode of binary instruction
---------------- Hardware Abstractions ----------------
- Stores 8 registers in Vec<u32>
- Memory implemented as 2D space, Vec<Vec<u32>>
- Unmapped segments stored temporarily in Vec<u32>
- Program counter kept track of with u32 value

instructions:
- Functions for given instruction take mutable reference to vm as parameter
   - Modifies public fields of vm struct
- Unpacks data from instruction using functions from bitunpack module

bitunpack:
- Uses bitwise operators to extract data from instructions

# of instructions for files provided by Carl
--------------------------------------------------------------------------------------------------------
| um(z) file | # of instructions | Time to execute (s) | Instructions per second | 50 mil estimate (s) |
| ---------- | ----------------- | ------------------- | ----------------------- | ------------------- |
| Sandmark   |      85 070 522   |         0.489       |       163 743 398       |        0.305        |
| Midmark    |   2 113 497 591   |        10.177       |       207 673 930       |        0.241        |
--------------------------------------------------------------------------------------------------------

-----------------------------------------
|        Task          | Time spent (h) |
| -------------------- | -------------- |
| Analyzing assignment |       3        |
|   Preparing Design   |       3        |
|   Problem Solving    |       6        |
-----------------------------------------