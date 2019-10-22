use crate::Res;
use tokio::net::process::Command;


pub struct MuxPort {
    child: Command,
}


impl MuxPort {
    pub fn launch_commands(cmds: Vec<Command>) -> Res<Vec<MuxPort>> {
        // Result::collect(cmds.map(|c| c.spawn()))
        let mut mps = vec![];

        for mut cmd in cmds {
            let mp = MuxPort {
                child: cmd.spawn()?,
            };

            mps.push(mp);
        }

        Ok(mps)
    }
}
