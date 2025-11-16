use crate::{hook_fn, rfg::memory::{MemoryBlockDefinition, MemoryLayoutDefinition}, utils::{address::addr, patchers::patch_string_pool_size, write_value}};
use anyhow::Result;
use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect};

hook_fn!(
    addr(0x8E5DC0),
    extern "C" fn() -> *const MemoryLayoutDefinition,
    get_memory_layout_definition
    {
        let memory_layout = get_memory_layout_definition_original()();
        unsafe {
            let blocks = std::slice::from_raw_parts_mut(
                (*memory_layout).block_definitions,
                (*memory_layout).block_count as usize
            );
            let blocks_size = (*memory_layout).block_count as usize * std::mem::size_of::<MemoryBlockDefinition>();
            let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);
        
            // Change memory protection to read/write/execute
            let res = VirtualProtect(
                (*memory_layout).block_definitions as *mut _,
                blocks_size,
                PAGE_EXECUTE_READWRITE,
                &mut old_protect
            );
            
            if res.as_bool() {
                // Increase size of "RFG_MempoolProvider" block (index 2)
                // Vanilla size is 1,146,000,000
                let rfg_mempool_provider_block = &mut blocks[2];
                rfg_mempool_provider_block.size = 1_346_000_000;
                
                // Restore original protection
                VirtualProtect(
                    (*memory_layout).block_definitions as *mut _,
                    blocks_size,
                    old_protect,
                    std::ptr::null_mut()
                );
            }
        }

        memory_layout
    }
);

fn patch_memory_pools() {
    // Common_page_pool. Vanilla is 13369344
    write_value::<u32>(addr(0x1D8660), 26738688);

    // Permanent_mempool. Vanilla is 15728640
    write_value::<u32>(addr(0x1D8E36), 31457280);

    // MP_effect_preload_mempool. Vanilla size is 5767168. Only cpu_mem_size is patched since gpu_mem_size = 0
    write_value::<u32>(addr(0x54F35A), 11534336);

    // MP_item_preload_mempool. Vanilla size is 80735232
    write_value::<u32>(addr(0x54F3A3), 161470464); // TODO: May need to increase size of pool used by Stream_mp_effect_preload_allocator (source allocator)

    // Item_preload_pool. Vanilla is 134217728
    write_value::<u32>(addr(0x1D8F17), 268435456);
    write_value::<u32>(addr(0x1D8F2C), 268435456);
}

fn patch_string_pools() {
    // Upgrades_string_pool. Vanilla is 2048
    patch_string_pool_size(addr(0x390D3C), addr(0x390D6F), 4096);

    // Area_defense_string_pool. Vanilla is 1024
    patch_string_pool_size(addr(0x39624A), addr(0x39627D), 2048);

    // Riding_shotgun_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x399E3E), addr(0x399E73), 2048);

    // Delivery_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x39A620), addr(0x39A651), 1024);

    // Raid_string_pool. Vanilla is 1024
    patch_string_pool_size(addr(0x39B10B), addr(0x39B13E), 2048);

    // Wrecking_crew_string_pool. Vanilla is 3072
    patch_string_pool_size(addr(0x39EDD6), addr(0x39EDF4), 6144);

    // Cont_attack_string_pool. Vanilla is 896
    patch_string_pool_size(addr(0x3A0C6C), addr(0x3A0C9F), 1792);

    // Squad_name_string_pool. Vanilla is 2048. The game has two functions which initialize it. One makes it 768 bytes so I just picked the bigger one.
    patch_string_pool_size(addr(0x3A2D5E), addr(0x3A2D8F), 4096);

    // Managed_layer_name_string_pool. Vanilla is 6144
    patch_string_pool_size(addr(0x3A573C), addr(0x3A576C), 12288);

    // Activity_mission_menu_string_pool. Vanilla is 6656
    // Not patched yet. Has static array so it requires a different patching technique
    //patch_string_pool_size(addr(0x, addr(0x, 2048); 

    // Activity_data_name_string_pool. Vanilla is 2560
    patch_string_pool_size(addr(0x3B375C), addr(0x3B378F), 2560);

    // Mission_name_string_pool. Vanilla is 1450
    patch_string_pool_size(addr(0x3B514C), addr(0x3B517F), 2900);

    // Stats_string_pool. Vanilla is 30720
    patch_string_pool_size(addr(0x3D3A09), addr(0x3D3A3C), 61440);

    // Handbook_string_pool. Vanilla is 5000
    patch_string_pool_size(addr(0x47715C), addr(0x47718F), 10000);

    // UI_string_pool. Vanilla is 10000
    patch_string_pool_size(addr(0x4B703D), addr(0x4B7072), 20000);

    // Tool_tips_string_pool. Vanilla is 6144
    patch_string_pool_size(addr(0x4EB766), addr(0x4EB797), 12288);
    patch_string_pool_size(addr(0x4EBE4F), addr(0x4EBE80), 12288);

    // Credits_string_pool. Vanilla is 131072
    patch_string_pool_size(addr(0x509664), addr(0x5096A1), 262144);

    // Multi_xp_string_pool. Vanilla is 6144
    patch_string_pool_size(addr(0x5A354E), addr(0x5A355A), 12288);

    // Multi_game_character_string_pool. Vanilla is 1400
    patch_string_pool_size(addr(0x5DCFFD), addr(0x5DD00C), 2800);
    patch_string_pool_size(addr(0x5FCE58), addr(0x5FCE67), 2800);

    // Backpack_string_pool. Vanilla is 1024
    patch_string_pool_size(addr(0x5DF0A3), addr(0x5DF0C5), 2048);

    // Human_hap_string_pool. Vanilla is 2048
    // Not patched yet. Has static array so it requires a different patching technique
    //patch_string_pool_size(addr(0x, addr(0x, 4096); 

    // Asd_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x6609AF), addr(0x6609E2), 1024);

    // Gameplay_properties_name_string_pool. Vanilla is 22528
    patch_string_pool_size(addr(0x67ABDE), addr(0x67ABF9), 45056);

    // Convoy_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x68E551), addr(0x68E582), 1024);

    // Courier_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x68EEEB), addr(0x68EF1C), 1024);

    // Action_node_string_pool. Vanilla is 3072
    patch_string_pool_size(addr(0x6A4295), addr(0x6A42C8), 6144);

    // Demolitions_master_string_pool. Vanilla is 2048
    patch_string_pool_size(addr(0x6A6988), addr(0x6A69B8), 4096);

    // House_arrest_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x6A7068), addr(0x6A7098), 1024);

    // Roadblock_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x6AB0AC), addr(0x6AB0C5), 1024);

    // Human_info_string_pool. Vanilla is 6200
    patch_string_pool_size(addr(0x6BCCC5), addr(0x6BCCF2), 12400);

    // Ambient_spawn_string_pool. Vanilla is 1024
    patch_string_pool_size(addr(0x7548B0), addr(0x7548E1), 2048);

    // Spawn_res_data_name_string_pool. Vanilla is 512
    patch_string_pool_size(addr(0x75C83C), addr(0x75C86F), 1024);

    // Spawn_group_string_pool. Vanilla is 3584
    patch_string_pool_size(addr(0x75F8AC), addr(0x75F8DF), 7168);

    // Vehicle_family_name_pool. Vanilla is 512
    patch_string_pool_size(addr(0x768B0C), addr(0x768B3F), 1024);

    // Vehicle_interaction_string_pool. Vanilla is 8192
    patch_string_pool_size(addr(0x782404), addr(0x78242A), 16384);

    // Vehicle_info_string_pool. Vanilla is 8192
    patch_string_pool_size(addr(0x7ACE82), addr(0x7ACEAF), 16384);
    patch_string_pool_size(addr(0x7AFE93), addr(0x7AFEC3), 16384);
}

pub fn apply() -> Result<()> {
    get_memory_layout_definition_register()?;

    patch_memory_pools();
    patch_string_pools();

    Ok(())
}