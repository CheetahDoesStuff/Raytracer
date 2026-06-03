# Raytracer
***This is a raytracer made with help from the raytracer in one weekend series***

Mkay so i wanna start things off with just saying that this project, does not steer that far from the books, and i am sad that i couldnt write more of my own code, but i have gotten really demotivated and will probably move on to the next project.

![image](image.png)

## Installation
So this program isnt really installable, scenes are sadly hardcoded etc, and therefore the only way to run the raytracer is cloning the repo, and compiling/running the program:
```bash
git clone https://github.com/CheetahDoesStuff/Raytracer
cargo run --release
```
Now this kinda assumes that cargo is installed, if not, it will be installed when you install rustup. Guides are on google.

I cant really distribute this as a build when the scenes themselves, that you may wanna change, are baked into the program.

This will output an image.png, thats your render!

## Usage
Now by default this will render a basic cornell box, which can easily be changed, kinda. So im main.rs on line 21, you will find this:
```rs
    let (world, camera) = raytracer::scenes::cornell::scene();
```
Now if you change `cornell` to any other file in the scenes folder it will render said scene instead.

Now if you want to write your own scene, i am going to assume you are experienced enough in rust that you can read some rust docs and therefore i have compiled the docs for you and they are availble at /doc/raytracer/index.html. Now there arent any docstrings in the codebase, so no examples in the docs, but there are plenty demo scenes for you to read through.

I have made a blank scene for you to copy paste and write your own scene into. Remember to add the file to the mod.rs (`pub mod FILENAME;`) or it will not work.

Im sorry for the lack of documentation but the scenes are written in semi-advanced rust, and if you can write a scene i think you can work with some rust docs and examples (again, sorry).