use dioxus::prelude::*;
use dioxus_motion::{KeyframeAnimation, prelude::*};
use easer::functions::Easing;

// Type alias to simplify complex keyframe type
#[allow(clippy::type_complexity)]
type KeyframeData<T> = Vec<(T, f32, Option<fn(f32, f32, f32, f32) -> f32>)>;

// Helper function to safely build keyframe animations
fn build_keyframes<T: dioxus_motion::animations::core::Animatable>(
    duration: Duration,
    keyframes: KeyframeData<T>,
) -> Result<KeyframeAnimation<T>, dioxus_motion::keyframes::KeyframeError> {
    let mut animation = KeyframeAnimation::new(duration);
    for (value, offset, easing) in keyframes {
        animation = animation.add_keyframe(value, offset, easing)?;
    }
    Ok(animation)
}

// A playful button that bounces on click
#[component]
pub fn RotatingButton() -> Element {
    let scale = use_motion_store(1.0f32);
    let rotation = use_motion_store(0.0f32);
    let glow = use_motion_store(0.0f32);

    let onclick = move |_| {
        // Smooth scale keyframe animation for bounce effect
        let scale_keyframes = build_keyframes(
            Duration::from_millis(800),
            vec![
                (1.0, 0.0, Some(easer::functions::Expo::ease_out)), // Start
                (1.15, 0.3, Some(easer::functions::Back::ease_out)), // Peak scale
                (0.95, 0.7, Some(easer::functions::Bounce::ease_out)), // Slight undershoot
                (1.0, 1.0, Some(easer::functions::Elastic::ease_out)), // Final rest
            ],
        );

        // Smooth rotation keyframe animation
        let rotation_keyframes = build_keyframes(
            Duration::from_millis(800),
            vec![
                (0.0, 0.0, Some(easer::functions::Cubic::ease_in_out)), // Start
                (180.0, 0.5, Some(easer::functions::Expo::ease_in_out)), // Half rotation
                (360.0, 1.0, Some(easer::functions::Back::ease_out)), // Full rotation with overshoot
            ],
        );

        // Quick glow effect with keyframes
        let glow_keyframes = build_keyframes(
            Duration::from_millis(600),
            vec![
                (0.0, 0.0, Some(easer::functions::Quart::ease_out)), // Start
                (1.0, 0.2, Some(easer::functions::Expo::ease_out)),  // Peak glow
                (0.3, 0.6, Some(easer::functions::Cubic::ease_in_out)), // Fade
                (0.0, 1.0, Some(easer::functions::Quart::ease_in)),  // Fade out
            ],
        );

        // Only animate if keyframe creation succeeded
        if let Ok(_scale_anim) = scale_keyframes {
            animate_to(
                &scale,
                1.15,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(800),
                    easing: easer::functions::Expo::ease_out,
                })),
            );
        }
        if let Ok(_rotation_anim) = rotation_keyframes {
            animate_to(
                &rotation,
                360.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(800),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        }
        if let Ok(_glow_anim) = glow_keyframes {
            animate_to(
                &glow,
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(600),
                    easing: easer::functions::Expo::ease_out,
                })),
            );
        }
    };

    rsx! {
        button {
            class: "relative px-8 py-4 bg-linear-to-r from-purple-500 to-pink-500
                   text-white rounded-xl font-bold text-lg overflow-hidden
                   transition-all duration-300 hover:shadow-xl hover:shadow-purple-500/20",
            style: "transform: scale({scale.current()()}) rotate({rotation.current()()}deg)",
            onclick,
            // Enhanced glow effect
            div {
                class: "absolute inset-0 bg-white/30 blur-xl",
                style: "opacity: {glow.current()()}",
            }
            "Click me!"
        }
    }
}
