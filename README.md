# Yoga and Strength Calendar Event Creator
App for creating an icalendar file for strength and yoga training in connection to a cycling workout.
This was created to work well with TrainerRoad as supplementary workouts/stretches.  The name of each
event corresponds to a Wahoo SYSTM video.
  
## Compiling
Rust or pyton installation needed.  For the python versions `./python`,
with workout.tk.py being the GUI version.  For Rust, compile with `cargo build --release`

## Use
### Rust  
`./target/release/YogaStrengthCalendar`  
  

![App picture](./app.png)

#### Rust WASM
use Trunk to build for web target.  
  
- Install the required target with `rustup target add wasm32-unknown-unknown`.
- Install Trunk withk `cargo install --locked trunk`.
- Run trunk serve to build and serve on http://127.0.0.1:8080. Trunk will rebuild automatically if you edit the project.
- Open http://127.0.0.1:8080/index.html#dev in a browser. See the warning below.
  
Web Deploy  
  
- Run `trunk build --release`.
- It will generate a dist directory as a "static html" website
- Upload the dist directory to any of the numerous free hosting websites
