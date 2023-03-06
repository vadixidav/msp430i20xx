union Vector {
    handler: unsafe extern "msp430-interrupt" fn(),
    reserved: usize,
}

extern "msp430-interrupt" {
    fn NmiOscillatorFaultFlashAccessViolationInterrupt();
    fn Ta1Ccr0Interrupt();
    fn Ta1Ccr1Ccr2OverflowInterrupt();
    fn VmonInterrupt();
    fn WdtInterrupt();
    fn Uca0Interrupt();
    fn Ucb0Interrupt();
    fn Sd24Interrupt();
    fn Ta0Ccr0Interrupt();
    fn Ta0Ccr1Ccr2OverflowInterrupt();
    fn P1Interrupt();
    fn P2Interrupt();
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static __INTERRUPTS: [Vector; 15] = [
    Vector { reserved: 0 },
    Vector {
        handler: P2Interrupt,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: P1Interrupt,
    },
    Vector {
        handler: Ta0Ccr1Ccr2OverflowInterrupt,
    },
    Vector {
        handler: Ta0Ccr0Interrupt,
    },
    Vector {
        handler: Sd24Interrupt,
    },
    Vector {
        handler: Ucb0Interrupt,
    },
    Vector {
        handler: Uca0Interrupt,
    },
    Vector {
        handler: WdtInterrupt,
    },
    Vector {
        handler: VmonInterrupt,
    },
    Vector {
        handler: Ta1Ccr1Ccr2OverflowInterrupt,
    },
    Vector {
        handler: Ta1Ccr0Interrupt,
    },
    Vector {
        handler: NmiOscillatorFaultFlashAccessViolationInterrupt,
    },
];
