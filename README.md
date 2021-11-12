# updatr
Simple CLI for batch updating records in Current RMS.

Please note, these instructions are for compiling and running the application on Windows 10 only.

## How to Run
### 1. Install the Rust Compiler
Follow [this link](https://www.rust-lang.org/tools/install), which will give you instructions for installing the Rust compiler onto your machine.

### 2. Clone this Repository
Find a suitable directory and clone the repository as you would any repository on GitHub. Alternatively, if you don't have Git (or the Github CLI) installed on your machine, simply download the repository as a zip file.

### 3. Set your Environment Variables
Follow [this guide](https://www.alphr.com/environment-variables-windows-10/), for how to set Environment Variables on your machine. The program depends on two variables: CURRENT_API_KEY and CURRENT_SUBDOMAIN. The values of which should be pretty self-explanatory.

### 4. Compile the Program
In *Command Prompt* (or *PowerShell*) navigate to the directory where you cloned this repository, and run `cargo build`.

### 5. Run the Program
The program will be compiled to the directory `src\target` in the reposistory. For example, if you cloned the repository to `C:\projects\`, the program will be compiled under `C:\projects\updatr\src\target\updatr.exe`. To run it, navigate to the `target` directory in *Command Prompt* and enter `updatr.exe C:\temp\products.csv`, where `C:\temp\products.csv` is the absolute path to the list of product records you wish to update.