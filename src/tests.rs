#[cfg(test)]
mod test {

    #[cfg(test)]
    mod characteristics_test {
        use crate::{dnd5, DND5Npc, Npc};
        use crate::dnd5::{CARAC_BOUNDS, DND5Npc};

        #[test]
        fn generate_npc_carac() {
            let npc = dnd5::Npc::generate_npc();

            assert!(isCaracValid(&npc.str));
            assert!(isCaracValid(&npc.wis));
            assert!(isCaracValid(&npc.dex));
            assert!(isCaracValid(&npc.con));
            assert!(isCaracValid(&npc.cha));
        }

        fn isCaracValid(carac: &u32) -> bool {
            carac >= &CARAC_BOUNDS.0 && carac <= &CARAC_BOUNDS.1
        }

        #[test]
        fn carac_is_random() {
            let npc1 = DND5Npc::generate_npc();
            let npc2 = DND5Npc::generate_npc();
            let carac_npc1 = (npc1.con, npc1.dex, npc1.cha, npc1.wis, npc1.str, npc1.int);
            let carac_npc2 = (npc2.con, npc2.dex, npc2.cha, npc2.wis, npc2.str, npc2.int);

            assert_ne!(carac_npc1, carac_npc2);
        }
    }

    #[test]
    fn npc_has_random_alignment() {
        let npc = DND5Npc::generate_npc();
        let npc2 = DND5Npc::generate_npc();

        assert_ne!(npc.alignment, npc2.alignment)
    }
}
