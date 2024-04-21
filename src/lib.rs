pub fn reply(message: &str) -> &str {

    let trimmed_msg = message.trim();

    match trimmed_msg {
        msg if msg.is_empty() => "Fine. Be that way!",
        msg if msg.ends_with('?') && msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic()) => "Calm down, I know what I'm doing!",
        msg if msg.ends_with('?') && msg.to_uppercase() != msg && msg.chars().any(|c| c.is_alphabetic()) => "Sure.",
        msg if msg.ends_with('?') && msg.chars().all(|c| !c.is_alphabetic()) => "Sure.",
        msg if msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic()) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}