use x86_64::structures::paging::PageTable;
use x86_64::registers::control::Cr3;
use x86_64::VirtAddr;

pub unsafe fn active_level_4_page_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_page_table, _) = Cr3::read();

    let phys_addr = level_4_page_table.start_address();
    let virt = physical_memory_offset + phys_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}