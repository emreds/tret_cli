[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# TRet_cli
`tret_cli` is a simple cli program which brings the etymological background of Turkish words with a simple command. 

## Tech Stack
The whole project is written in Rust. I started it as a hobby project to practice my Rust skills.

## How to Install?
- To use the project, you should have rust compiler in your device. 
- After installing rust, just run the `cargo build --release` command in the project's folder.
- It will create a binary file located in  `${PROJECT_PATH}/target/tret_cli`
- After this step, please follow the instructions specific to your operating system.

### For Linux
1. Create directory ${HOME}/bin by running:
``` mkdir -p ${HOME}/bin ```
2. Save the binary to directory: `${HOME}/bin`
3. Open the file `${HOME}/.bashrc` to edit your shell configuration. 
4. Add the below line to the shell config file(if it is not already there), then save it.
```  export PATH="${HOME}/bin:${PATH}" ```

### For MacOS 
- Do the first and second steps above. 
- Then, edit your shell config file. Depending on the shell you are using. 
- For bash `${HOME}/.bashrc`.
- For Zsh `${HOME}/.zshrc`.

### For Windows
- Please follow the instructions here: https://zwbetz.com/how-to-add-a-binary-to-your-path-on-macos-linux-windows/


