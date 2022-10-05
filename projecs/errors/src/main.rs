use anyhow::Result;
use aquarium::Dolphin;
#[allow(clippy::vec_init_then_push)]

fn play_time(dolphin: &Dolphin) -> Result<Vec<String>> {
    let mut responses = vec![];

    let response = dolphin.say_your_name()?;
    responses.push(response);

    let flip_response = dolphin.flip()?;
    responses.push(flip_response);

    let shake_response = dolphin.shake_hands()?;
    responses.push(shake_response);

    Ok(responses)
}

fn main() -> Result<()> {
    let dolphins = vec![
        Dolphin {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Dolphin {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Dolphin {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Dolphin {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];

    for dolphin in &dolphins {
        let responses = play_time(dolphin)?;
        println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
        for response in responses {
            println!("  {}", response);
        }
    }

    Ok(())
}
