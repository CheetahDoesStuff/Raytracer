# Raytracer
***This is a raytracer made with help from the raytracer in one weekend series***

Mkay so i wanna start things off with just saying that this project, does not steer that far from the books, and i am sad that i couldnt write more of my own code, but i have gotten really demotivated and will probably move on to the next project.

[image](https://cdn.hackclub.com/019e9952-ac21-7a44-88c4-e614966b3cdc/image.png)

## Installation
First of all you need to install oidn (Intel Open Image Denoise). Installing it depends on your OS so youll need to google that one. Do note that if it complains about a missing file, which it did for me, i cant give you an answer on how to fix it as it may vary depending on the system. I will recommend you to google that too.

Now if you are unable to install oidn you can prevent it from being used by disabling it, see features section below.

So this program isnt really installable, scenes are sadly hardcoded etc, and therefore the only way to run the raytracer is cloning the repo, and compiling/running the program:
```bash
git clone https://github.com/CheetahDoesStuff/Raytracer
cargo run --release
```
Now this kinda assumes that cargo is installed, if not, it will be installed when you install rustup. Guides are on google.

I cant really distribute this as a build when the scenes themselves, that you may wanna change, are baked into the program.

This will output an image.png, thats your render!

### Features
Now the program comes with 2 "features", basically they are toggleable parts of the code to enable/disable some features.

Currently there are the following features:
- threading - Makes the raytracer run on all threads
- denoise - enables/disables the use of oidn. Will affect output quality

Now by default both of these are on. To enable only a select one, change your run command to this:
```bash
cargo run --release --no-default-features --feature FEATURE_TO_ENABLE
```

## Usage
Now by default this will render a basic cornell box, which can easily be changed, kinda. So im main.rs on line 21, you will find this:
```rs
    let (world, camera) = raytracer::scenes::cornell::scene();
```
Now if you change `cornell` to any other file in the scenes folder it will render said scene instead.

Now if you want to write your own scene, i am going to assume you are experienced enough in rust that you can read some rust docs and therefore i have compiled the docs for you and they are availble at /doc/raytracer/index.html. Now there arent any docstrings in the codebase, so no examples in the docs, but there are plenty demo scenes for you to read through.

I have made a blank scene for you to copy paste and write your own scene into. Remember to add the file to the mod.rs (`pub mod FILENAME;`) or it will not work.

Im sorry for the lack of documentation but the scenes are written in semi-advanced rust, and if you can write a scene i think you can work with some rust docs and examples (again, sorry).
