
<div align="center">

  # sillybinds

  <sub>*Why is it called **silly**-binds? Because I was about to parse old and obscure linux key-binds... then I looked back and said that would be silly as fuck...* **also it can still parse silly stuff like this** `cTrL+sHiFt+a`</sub>

</div>

> [!WARNING]
> Don't take this readme seriously, this crate is still in development.

**sillybinds** is a **simple** cirrus crate our **Egui** applications use to parse **Wayland-style key-binding strings** user's have specified in their configs into the proper `egui::Key` stuff, with focus on **modern, simple and very common key-bind formats** — ignoring Vim formats and legacy linux X11 binds.

```toml
# Random toml config for a cloudy-org application...

[key_binds]
about_box.toggle = "shift+A" 
side_bar.shrink = "cTrL+sHiFt+S" # silly but still works
side_bar.toggle = "Shift+S" # maybe you think this is cleaner idk...
info_box.toggle = "i"
extra_info_box.toggle = "SHIFT+I"
```