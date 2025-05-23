mod fall;
mod taunts;
mod landing;
mod guard;
mod escape;
mod throw;

pub fn install(agent: &mut smashline::Agent) {
    fall::install(agent);
    taunts::install(agent);
    landing::install(agent);
    guard::install(agent);
    escape::install(agent);
    throw::install(agent);
}