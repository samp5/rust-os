# Writing an OS in Rust
- As an honors supplement to CS361: Systems Programming, I am following Phillip Oppermann's [guide](https://github.com/phil-opp/blog_os/tree/main) on writing an small OS in Rust.

## Project Goals and Prospective Timeline

| Milestone                	| Description                                                                           	| Possible Extensions                                                        	| Target Completion Date 	| Completed?  	|
|--------------------------	|---------------------------------------------------------------------------------------	|----------------------------------------------------------------------------	|:----------------------:	|:-----------:	|
| Bare Bones Build         	| Set up the build environment to build for running on bare metal. Create a boot image. 	| None                                                                       	| 02-07                  	| [Notes!](progress_updates/milestone1.md)|
| VGA Buffers + Testing    	| Write to screen in a VM and create a testing framework                                	| Create a logging system which can be used to report debugging information  	| 02-14                  	|             	|
| CPU Exceptions           	| Implement the interrupt descriptor table and register preservation                    	| TBD...                                                                     	| 02-21                  	|             	|
| Double Fault Exceptions  	| Handle double faults! Implement handling errors in errors                             	| TBD...                                                                     	| 02-28                  	|             	|
| Hardware Interrupts      	| Forward all hardware interrupts to the CPU                                            	| Support APIC over Intel 8259 PIC                                           	| 03-07                  	|             	|
| Paging Implementation    	| Page table implementation (4-level 0x86_64)                                           	| Comparable page table implementations (Inverted, hierarchical, segmented)  	| 03-14                  	|             	|
| Heap Allocation          	| Re-implement Rust allocation interface for dynamic memory support                     	| TBD...                                                                     	| 03-21                  	|             	|
| Heap Allocators          	| Implement allocator designs (bump allocation, linked lists, fixed-block)              	| Research and implement additional allocaor designs                         	| 03-28                  	|             	|
| Cooperative Multitasking 	| Futures, Polls, Pins, oh my.                                                          	| TBD...                                                                     	| 04-02                  	|             	|
| Preemptive Multitasking  	| Async keyboard Task using Wakers and Atomic Wakers.                                   	| TBD...                                                                     	| 04-09                  	|             	|
| Threads and Scheduling?  	| ...                                                                                   	| TBD...                                                                     	| 04-16                  	|             	|
| Userspace?               	| ...                                                                                   	| TBD...                                                                     	| 04-23                  	|             	|
