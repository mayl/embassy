#![no_std]
#![no_main]

//! Lowest-current floor measurement: park the MSPM0L1306 in STANDBY (the
//! deep-sleep mode the datasheet rates at ~1.0 µA on LP-MSPM0L1306), with no
//! wakeup source enabled, and stay there forever. Drives nothing, runs no
//! executor, no timers — strictly the SoC's quiescent current after a clean
//! SYSCTL configuration.
//!
//! The mode is a single constant; swap `Dsleep::STANDBY` for `Dsleep::STOP`
//! (~50–120 µA, SYSOSC still available) or `Dsleep::SHUTDOWN` (~80 nA, but
//! wake re-runs the boot code) to compare floors.
//!
//! Use `cortex_m_rt::entry` directly (no `embassy_executor::main`) so the
//! embassy time driver isn't spun up — same reason `et_calib_busy_loop.rs`
//! does. The only code the CPU executes after init is the WFI.
//!
//! NOTE: the debugger keeps the MCU in RUN as long as DAP is attached. Measure
//! via `repro-cli` (it calls `DAP_Disconnect` → `XDS_ConnectET` →
//! `ET_DCDC_RestartMCU` which releases the debugger before profiling) or
//! power-cycle the LaunchPad after flashing.

use cortex_m_rt::entry;
use embassy_mspm0::pac::SYSCTL;
use embassy_mspm0::pac::sysctl::vals::Dsleep;
use embassy_mspm0::Config;
// Pulled in only to satisfy the defmt global-logger requirement of
// embassy-mspm0's "defmt" feature; nothing logs.
use {defmt_rtt as _, panic_halt as _};

/// Deep-sleep target. STANDBY ≈ 1 µA datasheet typ; STOP ≈ 50–120 µA;
/// SHUTDOWN ≈ 80 nA (wakeup → boot reset).
const DSLEEP_MODE: Dsleep = Dsleep::STANDBY;

#[entry]
fn main() -> ! {
    let _p = embassy_mspm0::init(Config::default());

    // Lowest-power clock policy:
    //  - SYSOSC fully off in idle modes (DISABLE) — ULPCLK falls back to LFCLK.
    //  - DISABLESTOP: SYSOSC also off in STOP mode.
    //  - USE4MHZSTOP: if SYSOSC isn't disabled in STOP, drop it to 4 MHz.
    SYSCTL.sysosccfg().modify(|w| {
        w.set_disable(true);
        w.set_disablestop(true);
        w.set_use4mhzstop(true);
    });
    // STOPCLKSTBY selects the deeper STANDBY sub-mode (kill ULPCLK/LFCLK to
    // peripherals — only TIMG0/TIMG1 can still run, and we have neither on).
    SYSCTL.mclkcfg().modify(|w| {
        w.set_stopclkstby(true);
    });
    // Select the deep-sleep target the next WFI will land in.
    SYSCTL.pmodecfg().modify(|w| {
        w.set_dsleep(DSLEEP_MODE);
        // Drop the SRAM controller in STOP (it's a load even with retention).
        w.set_syssramonstop(false);
    });

    // SCB.SCR.SLEEPDEEP=1 — required for WFI to enter the SYSCTL-selected
    // deep-sleep mode (STOP/STANDBY/SHUTDOWN). Without it, WFI is just CPU
    // SLEEP and PMODECFG.DSLEEP is ignored.
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.SCB.set_sleepdeep();

    // No wakeup source armed → the chip stays in STANDBY until reset/POR.
    loop {
        cortex_m::asm::wfi();
    }
}
