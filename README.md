# Bevy 2D sprite animations
This is just a very simple plugin to manage sprite animations in bevy

# Example
```rust
let mut animation = AnimatedSprite::default();
animation.add_animation("idle", vec![1,2,3], 0.1);
animation.queue_animation("idle", true, None);
```

# TODO
 - Transistions
 - Transform based animations
 - Queueing
