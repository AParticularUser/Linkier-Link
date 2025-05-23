mod tilts;
mod smashes;
mod aerials;
mod other;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    other::install(agent);
}