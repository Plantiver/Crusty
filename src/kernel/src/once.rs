use crate::apscore;
use crate::bspcore;
use crate::requests::MP_REQUEST;

pub fn start_cores() -> ! {
    let mp_response = MP_REQUEST.get_response().unwrap();
    for cpu in mp_response.cpus() {
        cpu.goto_address.write(apscore::main)
    }
    bspcore::main()
}
