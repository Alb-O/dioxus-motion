use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
fn BouncingLetter(letter: char, delay: f32) -> Element {
    let transform = use_motion_store(Transform::identity());

    use_effect(move || {
        let delay = Duration::from_secs_f32(delay);
        animate_to(
            &transform,
            Transform {
                y: -30.0,
                scale: 1.5,
                rotation: 5.0 * (std::f32::consts::PI / 180.0),
                x: 0.0,
            },
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(1),
                easing: easer::functions::Sine::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite)
            .with_delay(delay),
        );
    });

    rsx! {
        span {
            class: "text-4xl font-bold text-indigo-600 inline-block origin-bottom
                   transition-transform duration-300",
            style: "transform: translateY({transform.current()().y}px)
                            scale({transform.current()().scale})",
            "{letter}"
        }
    }
}

#[component]
pub fn BouncingText(text: String) -> Element {
    rsx! {
        div { class: "flex space-x-1",
            {
                text.chars()
                    .enumerate()
                    .map(|(i, char)| {
                        rsx! {
                            BouncingLetter { letter: char, delay: i as f32 * 0.1 }
                        }
                    })
            }
        }
    }
}
