use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn empty_go() {
    world().run("scenarios/festival_smart_contract.scen.json");
}
