// Interrupt Stack Table stores pointers of 7 good stacks to switch to during interrupt (in case the current stack is corrupted)
// The IDT is stored in the Task State Segment (TSS)
// TSS uses a segmenting system instead of paging, so we need to use 'Global Descriptor Table' that contains segments


// So we need to
// 1. Create a TSS and initialise its IST value
// 2. Create a GDT with the TSS and kernel code segmenting
// 3. load the GDT in lib

use x86_64::{
    structures::{tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}}, 
    VirtAddr
};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 3;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {
            code_selector,
            tss_selector
        })
    };
}

pub fn init() {
    use x86_64::instructions::{
        segmentation::{CS, Segment},
        tables::load_tss
    };

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}