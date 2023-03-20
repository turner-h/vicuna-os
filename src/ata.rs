use alloc::vec::Vec;
use x86_64::structures::port::{ PortWrite, PortRead };

use crate::println;

static mut BUFFER: Vec<u16> = Vec::new();

pub unsafe fn send_read_master(lba_addr: u32, count: u8) -> Vec<u16> {
    //println!("{:X}{:X}{:X}{:X}", lba_addr >> 24, (lba_addr >> 16) as u8, (lba_addr >> 8) as u8, lba_addr as u8);


    PortWrite::write_to_port(0x1F6, 0xE0 | (lba_addr >> 24) & 0x0F);
    PortWrite::write_to_port(0x1F1, 0x00 as u8);
    PortWrite::write_to_port(0x1F2, count);
    PortWrite::write_to_port(0x1F3, lba_addr as u8);
    PortWrite::write_to_port(0x1F4, (lba_addr >> 8) as u8);
    PortWrite::write_to_port(0x1F5, (lba_addr >> 16) as u8);
    PortWrite::write_to_port(0x1F7, 0x20 as u8);
    
    while BUFFER == Vec::new() {
        //wait for buffer to be updated after irq
    }

    let mut buf: Vec<u16> = Vec::new();
    for val in &BUFFER {
        buf.push(*val);
    }

    BUFFER = Vec::new();
    
    buf
}

pub unsafe fn send_write_master(lba_addr: u32, count: u8) {
    PortWrite::write_to_port(0x1F6, 0xE0 | (lba_addr >> 24) & 0x0F);
    PortWrite::write_to_port(0x1F1, 0x00 as u8);
    PortWrite::write_to_port(0x1F2, count);
    PortWrite::write_to_port(0x1F3, lba_addr as u8);
    PortWrite::write_to_port(0x1F4, lba_addr >> 8 as u8);
    PortWrite::write_to_port(0x1F5, lba_addr >> 16 as u8);
    PortWrite::write_to_port(0x1F7, 0x30 as u8);

    for _i in 0..256 {
        PortWrite::write_to_port(0x1F0, 0x4141 as u16);
    }

    PortWrite::write_to_port(0x1F7, 0xE7 as u8);
}

pub unsafe fn read_port() {
    let mut val: u16;
    for _i in 0..256 { 
        val = PortRead::read_from_port(0x1F0);
        BUFFER.push(val);
    }
}