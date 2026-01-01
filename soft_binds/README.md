
<div align="center">

  # soft-binds

</div>

> [!WARNING]
> Still WIP. Not to be considered thought out.

**Soft-binds** is a cirrus crate our **Egui** applications use to parse user friendly key-binding strings user's have specified in their configs into the proper `egui::Key` enums.

```toml
# Random toml config for a cloudy-org application...

[key_binds]
about_box.toggle = "shift+A" 
side_bar.shrink = "cTrL+sHiFt+S" # silly but still works
side_bar.toggle = "Shift+S" # maybe you think this is cleaner idk...
info_box.toggle = "i"
extra_info_box.toggle = "SHIFT+I"
```
