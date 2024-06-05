use p3_baby_bear::BabyBear;
use p3_keccak_machine::KeccakMachine;
use p3_machine::machine::Machine;

pub fn main() {
    let machine = KeccakMachine;

    machine.write_schema_to_file::<BabyBear>("schema.dbml");
}
