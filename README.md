# Just-Another-Coffee

Just Another Coffee ( jac ) is a light useless script that just print a coffee cup in your command line.
To use it, simply type jac.

# Installation
To use jac, you need Rust and git on your computer. Simply clone the repo using :

```
git clone https://github.com/XanderTheRat/Just-Another-Coffee
```

Then, build the script using cargo : 

```
cargo build --release
```

You can move the binary file to `/usr/local/bin` to execute it from any repository, or use the following cargo line from the root repository ( *Where is you Cargo.toml* ) : 

```
cargo install --path .
```

Enjoy your coffee.

# Usage
To use it, simply use `jac` on your terminal and a coffee cup will be printed on your screen. It exist different option arguments that can be passed :
> **-q** : This argument question the user about their willing of a coffee cup. If answered no, it will show `"No more cup"`.

> **-n** : Answer automatically no to the question, even if **-q** is not provided.

> **-y** : Ask the question and automatically answer yes to it, showing the coffee cup.

> **-c** : Color the cup and the coffee. Colors can be change in script, and it match the ASCII caracter ( *▓: Coffee, ▒: Bright cup, ░: Dark cup* )

> **-d** : Show debug info ( *such as variable state...* )

> **-h** : Show an usage of the command, and print in your CMD this help page.

- These options are available in long format, respectively:
> **--question**

> **--no**

> **--yes**

> **--colored**

> **--debug**

> **--graduate-colorated** : Color the cup following x/y position of the pixel

> **--help**


## Customisation 
By default, the printed coffee is the following.
```

                 ░░░░░░░░░░░░░░░░░░                 
              ░░░░░░▒▒▓▓▓▓▓▓▓▓▒▒▒░░░░░              
             ░░░░▒▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░             
         ░░░░░░░▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░             
      ░░░▒▒▒▒▒░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░              
      ░▒▒    ▒▒░░░▒░░░░░░░░░░░▒▒▒▒▒▒░░              
      ░▒▒    ░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░              
       ▒▒▒▒▒▒░▒▒░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░         
      ░░░░▒▒▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░       
    ░░░░░░░░░░░▒▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░    
   ░░░░░░░░░░░░░░▒░░▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░   
   ░░▒▒▒▒▒▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░   
   ░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░   
    ░░▒▒▒▒▒▒▒▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░    
     ░░░░▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░     
        ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░        
           ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░           
                  ▒▒▒▒░▒░░░░▒▒▒▒▒▒▒
```
You can change it at any time by editing `coffee` file and puting your ascii cup in it.
Any ascii format is tolerated as long as it can be printed in your terminal.
To change the color, edit the print_colored_coffee function in lib.rs file.