### **The Add Compiler for Snek Expressions**  

#### **📌 Overview**  
This project compiles **Snek expressions** into **x86-64 assembly**, supporting basic operations like `add1`, `sub1`, and `negate`.  

#### **⚙️ Usage**  
To compile a `.snek` file into assembly and run it:  
```sh
make test/add.run
./test/add.run
```
To force recompilation:  
```sh
make clean
make test/add.run
```

#### **📂 Project Structure**  
- `src/main.rs` → Entry point, compiles Snek expressions.  
- `src/compiler.rs` → Converts expressions to assembly.  
- `src/interpreter.rs` → Evaluates expressions (debugging).  
- `test/` → Contains `.snek` test cases.  

#### **Requirements**  
- Rust (`cargo install rustup`)  
- NASM (`brew install nasm`)  

#### **Example**  
```snek
(add1 (sub1 (negate 5))) 
```
✅ **Expected Output:** `-5`  