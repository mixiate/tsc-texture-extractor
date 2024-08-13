fn convert(bytes: &[u8]) -> image::RgbaImage {
    let bytes = &bytes[16..];
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let width = usize::from(u16::from_be_bytes(
        bytes[null_position + 21..null_position + 23].try_into().unwrap(),
    ));
    let height = usize::from(u16::from_be_bytes(
        bytes[null_position + 23..null_position + 25].try_into().unwrap(),
    ));

    let texture_type = bytes[null_position + 29];

    let palette_count = usize::from(u16::from_be_bytes(
        bytes[null_position + 25..null_position + 27].try_into().unwrap(),
    ));
    let palette_bit_count = usize::from(bytes[null_position + 32]);

    let palette_length = (palette_count * palette_bit_count) / 8;

    match texture_type {
        0x1 => {
            let bytes = &bytes[null_position + 37..];

            let mut pixels = Vec::new();
            for i in 0..width * height {
                pixels.push(bytes[i * 3]);
                pixels.push(bytes[(i * 3) + 1]);
                pixels.push(bytes[(i * 3) + 2]);
                pixels.push(255);
            }

            let image = image::RgbaImage::from_raw(width as _, height as _, pixels).unwrap();
            image::imageops::flip_vertical(&image)
        }
        0x81 => crate::gamecube::decode_cmpr(&bytes[null_position + 37..], width, height),
        0x82 => crate::gamecube::decode_rgb565(&bytes[null_position + 37..], width, height),
        0x85 => crate::gamecube::decode_rgba8(&bytes[null_position + 37..], width, height),
        0x89 => {
            assert!(palette_bit_count == 32 && palette_count == 16);

            let palette = &bytes[bytes.len() - palette_length..];

            crate::gamecube::decode_i4(&bytes[null_position + 37..], width, height, palette)
        }
        0x8a => {
            assert!(palette_bit_count == 32 && palette_count == 256);

            let palette = &bytes[bytes.len() - palette_length..];

            crate::gamecube::decode_i8(&bytes[null_position + 37..], width, height, palette)
        }
        _ => panic!(),
    }
}

pub fn extract_textures(textures_path: &std::path::Path, output_path: &std::path::Path, specular_file_names: &[&str]) {
    std::fs::create_dir_all(output_path).unwrap();

    let textures = std::fs::read(textures_path).unwrap();

    let files = crate::arc::read_be(&textures);

    for (name, _, bytes) in files {
        let image = convert(bytes);
        crate::save_texture(image, &name, output_path, specular_file_names.contains(&name.as_str()));
    }
}

pub const SPECULAR_FILE_NAMES: [&str; 1151] = [
    "_garbage",
    "accessorycanofpetfood",
    "accessorycleaningspounge",
    "accessorycoffeecup",
    "accessorylitterboxbag",
    "accessorylitterboxscoopsurface",
    "accessorypetfoodbox",
    "accessorypetfoodboxmammal",
    "accessoryringboxfabric",
    "accessoryshovelsurface",
    "af_bc_meshstonecuff_texture1",
    "af_bc_watch_ovaldiamond_texture1",
    "af_grandparent_portrait_1",
    "af_grandparent_portrait_2",
    "af_hh_dummy",
    "af_jw_additional1_texture1",
    "af_sh_alienshort",
    "af_sl_biker_short_black_ln",
    "af_sl_biker_short_black_rl",
    "af_to_wrapfur_ivory",
    "am_grandparent_portrait_1",
    "am_grandparent_portrait_2",
    "am_sl_furcoat_ln_blue",
    "am_to_constructionvest",
    "appliance_bar",
    "appliance_blender_cheap_main",
    "appliance_blender_expensive_main",
    "appliance_counterfeit",
    "appliance_dishwasher_traditional",
    "appliance_food_processor_groove",
    "appliance_fridge_retro",
    "appliance_grill_sausage",
    "appliance_modern_fridge_detail",
    "appliance_refrigerater_cheap",
    "appliance_stove_alien",
    "appliance_stove_bbq_charcoal",
    "appliance_stove_modern_body",
    "appliance_stove_retro",
    "appliance_stove_x_stainless",
    "appliance_vacuum_dc",
    "aquarium_floor_02_parts",
    "aquariumSourceLevel2",
    "aquariumSourceLevel3",
    "art_floor_machine_claw",
    "art_painting_badger",
    "art_painting_happy_gnome",
    "art_painting_kids_atomic",
    "art_painting_lone_daisy",
    "art_painting_pet_crazy4",
    "art_painting_pet_crazy5",
    "art_painting_pet_crazy6",
    "art_sculpture_birdbath_main",
    "art_wall_back_canvas",
    "art_wall_frame_fancy",
    "art_wall_frame_metal",
    "art_wall_frame_wood",
    "art_wall_painting_romantic_large",
    "art_wall_pet_dog_avg",
    "art_wall_pet_dog_awesome",
    "art_wall_pet_dog_cheap",
    "art_wall_poster",
    "art_wall_romantic_large_frame",
    "art_wall_romantic_large_gilding",
    "art_wall_vent_circular",
    "bamboo_soap",
    "banner_cat_day",
    "banner_dog_day",
    "bathtub_oriental_brown",
    "bed_antique_blue",
    "bed_double_atomic_wood",
    "bed_single_atomic_blue",
    "bed_single_cot",
    "bed_single_cot_wood",
    "bed_single_double_brass",
    "bed_single_petcrazy",
    "bedsheet_mission",
    "beejaphone_guitar",
    "billboard_verizon02_white",
    "billboard_verizon02_white_02",
    "biodome_submarine",
    "black",
    "bookcase_books",
    "bookshelf_comic_covers",
    "bookshelf_comics_humidor",
    "bookshelf_expensive",
    "bookshelfPetCrazy",
    "bookshelfPetCrazyBook",
    "brickwall_lot02",
    "butterpecan",
    "cabinet_armoire_punk_base",
    "cabinet_armoire_punk_door",
    "car_art",
    "car_junker",
    "car_limo",
    "car_police",
    "car_sports",
    "car_sports_wheel",
    "car_taxi",
    "car_taxi_checkers",
    "car_town",
    "car_windshield",
    "carry_bills_orange",
    "carry_bills_red",
    "carry_bills_yellow",
    "carry_bread_dough",
    "carry_bread_loaf",
    "carry_cutting_board_stage_2",
    "carry_gnome",
    "carry_newspaper",
    "carry_newspaper_old",
    "carry_snack_chips",
    "carry_soup_in_pan",
    "carry_tray_boxes_cartons_etc",
    "cas_lightning_grad",
    "cas_mirror_stainless",
    "cas_pictureframe_frames",
    "cas_pictureframe_matte",
    "cas_pictureframe_photos",
    "cas_placeholder",
    "ceiling",
    "chair_dining_hostel",
    "chair_dining_outdoors",
    "chair_music_studio",
    "chocolate",
    "chopper_paint_blue",
    "chrome_tool",
    "clogged_water",
    "computer_screen_00",
    "computer_screen_01",
    "computer_screen_02",
    "computer_screen_03",
    "computer_screen_04",
    "computer_screen_05",
    "computer_screen_06",
    "computer_screen_07",
    "concrete_46_floor",
    "condo_greytile",
    "condo_innerlot_stucco",
    "condo_innerlot_woodsiding",
    "condo_wood_deck",
    "containedpetgoldfish_brush",
    "containedpetgoldfish_food",
    "containedpetgoldfish_gravel",
    "containedpetmammalgpig",
    "containedpetmammalstand",
    "count_blanc_bathroom_counter_face",
    "count_blanc_bathroom_counter_side",
    "count_blanc_bathroom_counter_top",
    "counter_alien_straight",
    "counter_butcherblock",
    "counter_c_kitchen_ikea",
    "counter_cheap_sides",
    "counter_commercial_side",
    "counter_commercial_top",
    "counter_formica_gray",
    "counter_groovy",
    "counter_industrial",
    "counter_mod_salmon",
    "counter_outdoor_straight",
    "counter_retro_bottom",
    "counter_sides_retro",
    "counter_slat_board",
    "counter_slat_board_top",
    "counter_tiled",
    "counter_tiled_sides",
    "counter_toolchest",
    "counter_western",
    "counter_western_top",
    "counter_wiseguys",
    "counter_x_butcherblock",
    "counterAntiqueClean",
    "counterAntiqueDirty",
    "counterMissionBase",
    "counterMissionClean",
    "counterMissionDirty",
    "counterMissionGlass",
    "counterPetCrazyClean",
    "counterPetCrazyDirty",
    "cozmo_brass",
    "cozmo_door_glass_brass",
    "cozmo_metal_02",
    "cozmo_toilet_drain",
    "cozmo_wood_01",
    "cozmo_wood_02",
    "cozmo_wood_03",
    "cozmo_wood_molding_01",
    "d_hed_sm_ccream",
    "dark_wood",
    "dark_wood_edged",
    "datapad",
    "decorative_gnome_cheat",
    "dirt_01",
    "door_alien_airlock",
    "door_barred",
    "door_expensive_fancy",
    "door_expensive_plain_metal1",
    "door_expensive_plain_metal2",
    "door_fancy_inlay_body",
    "door_metal_smallwindow",
    "door_nauti",
    "door_nauti_fancy",
    "door_painted_plain",
    "door_painted_plain_detail",
    "door_plain",
    "door_solid_screen",
    "door_solid_screen_cloth",
    "door_solid_screen_cloth_02",
    "door_western_plain",
    "door_western_plain_wood",
    "door_western_saloon",
    "doorAntique1",
    "doorAntique2",
    "doorMission1",
    "doorMission2",
    "doorPetCrazy1",
    "drag_bike_green",
    "drinkbar_biker_bottle",
    "drinkbar_biker_drinkglass",
    "drinkbar_biker_shaker",
    "drinkbar_biker_tray",
    "drinkbar_tapioca",
    "drinkbar_tapioca_shaker",
    "drinkbar_tapioca_tank",
    "drinkbar_wiseguy_bottle",
    "drinkbar_wiseguy_drinkglass",
    "drinkbar_wiseguy_shaker",
    "drinkbar_wiseguy_tray",
    "electric_guitar",
    "electric_guitar_amplifier",
    "electronics_alarm_burglar_02",
    "electronics_alarm_burgler",
    "electronics_alarm_clock",
    "electronics_alarm_fire_main",
    "electronics_arcade_people_invaders",
    "electronics_arcade_regular",
    "electronics_arcade_scifi_inuse_loop1",
    "electronics_arcade_scifi_inuse_loop2",
    "electronics_arcade_scifi_on_loop1",
    "electronics_arcade_scifi_ship",
    "electronics_arcade_screen",
    "electronics_computer_cheap_main",
    "electronics_computer_cheap_screen_blog_a",
    "electronics_computer_cheap_screen_sims2_a",
    "electronics_computer_cheap_screen_sims_bustin_out_a",
    "electronics_computer_cheap_screen_spore_a",
    "electronics_computer_cheap_screen_startup",
    "electronics_console",
    "electronics_dancing_element_particle",
    "electronics_metal_detector",
    "electronics_phone_wall_standard",
    "electronics_phone_wall_standard_screen",
    "electronics_stereo",
    "electronics_stereo_01",
    "electronics_stereo_02",
    "electronics_stereo_jukebox_biker",
    "electronics_tv_channel_food_01",
    "electronics_tv_channel_news_01",
    "electronics_tv_channel_sports_01",
    "electronics_tv_plasma_1x3",
    "electronics_vending_drink_full",
    "electronics_vending_drink_redbullcan",
    "electronics_vending_drink_redbullcan_02",
    "electronics_vending_drink_redbullcan_cas",
    "electronics_vending_drink_redbullcan_cas_02",
    "electronics_vending_hotdog",
    "electronics_vending_icecream",
    "electronics_vending_icecream_cone_tile",
    "electronics_vending_redbull_panels",
    "electronics_vending_redbull_panels_02",
    "electronics_vending_redbull_stand",
    "electronics_vending_redbull_stand_02",
    "electronics_vending_snack",
    "electronicsPinballPetCrazy",
    "electronicsPinballPetCrazyScoreboard1",
    "electronicsPinballPetcrazyscoreboard2",
    "electronicsPinballPetcrazyscoreboard3",
    "espresso_grill",
    "fabric_velvet_red",
    "farm_patchworkcrops",
    "fence_deadwood",
    "fence_endcap",
    "fence_japanese",
    "fence_white_picket",
    "fenceExpensive",
    "final-floor-tile-46",
    "final_adult_male_nude",
    "fireplace_expensive",
    "fireplace_expensive_tile",
    "fireplace_expensive_tile_dirty",
    "fireplace_log",
    "fireplaceMissionLog",
    "fireplaceMissionMarbleClean",
    "fireplaceMissionMarbleDirty",
    "fireplaceMissionMarbleLight",
    "fireplaceMissionWood",
    "flamingo",
    "floor_ac_woodpanel",
    "floor_ac_woodtiles",
    "floor_allien_orange",
    "floor_bio_01",
    "floor_bio_02",
    "floor_bio_03",
    "floor_bio_04",
    "floor_browncrosstile",
    "floor_browntile",
    "floor_carpet_blue",
    "floor_carpet_orange",
    "floor_carpet_standard",
    "floor_carpethotel",
    "floor_cement_roundstones",
    "floor_cement_sharpstones",
    "floor_checker_02",
    "floor_clubredwood",
    "floor_clubredwoodwhite",
    "floor_clubzigzag",
    "floor_derelict_02",
    "floor_diagredtile",
    "floor_dimondlighttile",
    "floor_dirt",
    "floor_dirt_1",
    "floor_dirt_yellow",
    "floor_dirtroad_edge_02",
    "floor_distressedboards",
    "floor_editoggler",
    "floor_grass_yellow",
    "floor_groovy_pat01",
    "floor_house_02",
    "floor_house_03",
    "floor_jap_01",
    "floor_kitchentilequad",
    "floor_lightbricks",
    "floor_loftcoralsandstone",
    "floor_lofteuropebeigtileplain",
    "floor_malltilebeigecenterdiagonal",
    "floor_marbleandrattandiagonal",
    "floor_mission_blackwhite",
    "floor_mission_flowertile",
    "floor_petthemed_cattile",
    "floor_petthemed_dogbonetileorange",
    "floor_petthemed_dogbonetilepurple",
    "floor_petthemed_pawtile",
    "floor_plaza_redtile",
    "floor_plazatile_bone_grey",
    "floor_plazatile_bone_tan",
    "floor_salvagedwood",
    "floor_sidewalk_venice",
    "floor_stainedconcrete_gold",
    "floor_stone_blueorange",
    "floor_stone_pink",
    "floor_tile_blue",
    "floor_tile_blueflower",
    "floor_tile_bluetriangle",
    "floor_tile_invisible",
    "floor_tile_orangewhite",
    "floor_tile_street_edge",
    "floor_tile_street_middle_left",
    "floor_tile_street_middle_right",
    "floor_trailer_carpet",
    "floor_wood_deck",
    "floor_wood_light",
    "floor_wood_medbrwndeck",
    "floor_wood_red",
    "floor_woodpanel_dark",
    "floor_woodpanel_rustichardwood",
    "floortile_apt3_linoleum_diagonal",
    "floortile_apt3_linoleum_punk",
    "floortile_apt_granite_checker",
    "floortile_biker_concrete",
    "floortile_biodome_grate",
    "floortile_cozmo_carpet_pattern",
    "floortile_cozmo_wood_bamboo",
    "floortile_foundry_tile_metal",
    "floortile_global_sidewalk",
    "floortile_global_sidewalk_grooves",
    "floortile_rooftop_concrete",
    "floortile_subway_runner_cross",
    "flowertulips",
    "food_espresso_machine",
    "food_object_bowl",
    "food_object_bowl_stuff",
    "food_object_bread",
    "food_object_burrito",
    "food_object_casserole",
    "food_object_pizza",
    "food_object_roast",
    "food_object_rotten",
    "food_object_salad",
    "food_object_sandwich",
    "foodpetbowlbowl",
    "foodpetbowlbowl_expensive",
    "foodpetbowlbowl_moderate",
    "foodpetbowlsaucer",
    "forest_mountain_catbuttes",
    "forest_mountain_catbuttes_cats",
    "forest_rock",
    "forest_tree_cut",
    "g_bladder_icon",
    "g_bladderjob_icon",
    "game_airhockey",
    "game_checker_board",
    "game_drivingrange",
    "game_foosball_field",
    "game_pinball_machine",
    "game_pingpong_paddle",
    "game_slot_machine",
    "gardengnome",
    "gate_petproofinterior_materials",
    "global_appliance_retro",
    "global_bamboo_dead",
    "global_bamboo_top",
    "global_bedsheet_atomic_set1",
    "global_bedsheet_petcrazy_1",
    "global_bedsheet_petcrazy_2",
    "global_bedsheet_pillow_set",
    "global_bedsheet_pillow_set_atomic1",
    "global_bedsheet_pillow_set_paws1",
    "global_brass",
    "global_brass_solid",
    "global_cashregister",
    "global_copper",
    "global_cushion_fabric_cream",
    "global_decorative_japanese02",
    "global_formica_red",
    "global_formica_white",
    "global_foundry_warning_label",
    "global_foundry_welded_metal",
    "global_fridge_bits",
    "global_glass_frosted",
    "global_glass_redlight_scroll",
    "global_gold",
    "global_gold_pat_3",
    "global_gold_plating",
    "global_gold_reflective",
    "global_metal_diamondgraphite",
    "global_metal_shiny",
    "global_metal_stainless_01",
    "global_molding_metal_brushed",
    "global_molding_wood_white",
    "global_park_bench",
    "global_picframemtl",
    "global_plastic_black",
    "global_plastic_blue_speckle",
    "global_plastic_cream",
    "global_plastic_lightblue_speckle",
    "global_plastic_red_speckle",
    "global_plastic_white",
    "global_quilted_metal",
    "global_rebar",
    "global_shower_door",
    "global_stainless_solid",
    "global_stainless_steel_pets",
    "global_stone_tilable",
    "global_velvet_wiseguys",
    "global_vinyl_rib",
    "global_warning_stripe",
    "global_wood_black",
    "global_wood_brown",
    "global_wood_cherry",
    "global_wood_light",
    "global_wood_panel",
    "global_wood_red",
    "global_wood_slats",
    "gradient_straight",
    "gravel_64",
    "hamster_wheel_frame",
    "hamster_wheel_main",
    "hamster_wheel_spokes_n_components",
    "house01_wallpanel",
    "house_load0",
    "house_load1",
    "house_load2",
    "house_load3",
    "house_load4",
    "house_load5",
    "house_load6",
    "house_load7",
    "icon_cash",
    "japanese_hottub_stone",
    "jewel_blue",
    "job_ferret_box",
    "job_fireworks",
    "k_bodnhead_cbeige",
    "k_bodnhead_cblack",
    "k_bodnhead_cblue",
    "k_bodnhead_cdarkbrown",
    "k_bodnhead_cgolden",
    "k_bodnhead_cgreen",
    "k_bodnhead_clightbrown",
    "k_bodnhead_corange",
    "k_bodnhead_cpink",
    "k_bodnhead_cpurple",
    "k_bodnhead_cred",
    "k_bodnhead_csilver",
    "k_bodnhead_cwhite",
    "k_bodnhead_cyellow",
    "kicktail_metal_02",
    "labrador_fur",
    "lake_bottom",
    "lamp_floor_gasheater",
    "lamp_floor_tiki",
    "lamp_floor_tiki_shade",
    "lamp_wall_modern",
    "lampAntiqueShade",
    "lampAtomicBase",
    "lampAtomicBulb",
    "lampFloorAntiqueBase",
    "lampFloorPetCrazy",
    "lampFloorPetCrazyShade",
    "lampMissionLeadedglass",
    "lampMissionLeadedglassOn",
    "lampMissionWood",
    "lampTableAntiqueBase",
    "lampTablePetCrazy",
    "lampTablePetCrazyShade",
    "lampWallAntiqueBase",
    "lampWallAntiqueShade",
    "lampWallAtomicBase",
    "lampWallPetCrazy",
    "largerockwall_base",
    "largerockwall_base_more",
    "largerockwall_base_vertcracks",
    "largerockwall_justtile",
    "loading_subway_map",
    "loveseat_groovy_cushion",
    "loveseat_groovy_frame",
    "mailbox",
    "mailbox_colonial",
    "maple_door_frame",
    "marbles_biege_green",
    "marbles_biege_green_dirty",
    "mesa_concrete",
    "mesa_goodtop",
    "mesa_massive",
    "mesa_road",
    "mesa_rockwall_mid02",
    "metal",
    "metal_burnt",
    "metal_galvanized",
    "metal_panel02",
    "metal_panel03",
    "metal_rusty_gradiant",
    "microwave_expensive",
    "mini_pp_top",
    "mini_pp_wall",
    "mintchip",
    "misc_movieprops",
    "narcisco_floor_mirror",
    "neon_blue",
    "neon_green",
    "neon_lightblue",
    "neon_red",
    "neon_white",
    "neon_white_dim",
    "neon_yellow",
    "new_cursor_03",
    "new_cursor_03_buildmode",
    "new_cursor_buymode",
    "no_stray_pets",
    "nothing",
    "numica_counter_face",
    "numica_counter_side",
    "numica_counter_top",
    "outdoor_trash_can",
    "performance_stage_blue",
    "performance_stage_wood",
    "pet_bed_basket",
    "pet_bed_house_atomic",
    "pet_bed_house_cheap",
    "pet_bed_house_moderate",
    "pet_bone_dog",
    "pet_bone_nylabone",
    "pet_bone_rope",
    "pet_cat_condo_awesome",
    "pet_cat_condo_moderate",
    "pet_cat_litter_box_cheap",
    "pet_cat_litter_box_expensive",
    "pet_cat_litter_box_moderate",
    "pet_cat_litter_box_moderate_chrome",
    "pet_cat_litter_box_sand",
    "pet_cat_litter_box_sand_dirty",
    "pet_catcondo_cheap",
    "pet_petbedhouseexpensive",
    "pet_petbedhouseexpensive_curtains",
    "pet_rope_tug_of_war",
    "pet_saucer_cream",
    "pet_scratchingpost",
    "pet_scratchingpost_expensive",
    "pet_scratchingpost_expensive_fish",
    "pet_smoothie",
    "pet_toy_ball_tennis",
    "pet_toy_chew_squeaky_hotdog",
    "pet_toy_chewtoy_pbob",
    "pet_toy_chewtoy_squeaky_newspaper",
    "pet_toy_chewtoy_squeaky_shoe",
    "pet_toy_fish_dangle",
    "pet_toy_flyingdisk",
    "pet_toy_mouse_dangle",
    "pet_toy_not_aerobie",
    "pet_toy_plastic_ball",
    "pet_toy_rubber_pull",
    "pet_toy_spider_dangle",
    "pet_toy_stick",
    "pet_toy_stretchy_mailman",
    "pet_toy_stuffed_animal",
    "pet_toy_tragicclown",
    "pet_toychewblue",
    "pet_treadmill_off",
    "pet_treadmill_on",
    "pet_treat_ball",
    "pet_treat_chicken_croissant",
    "pet_treat_cupcake",
    "pet_treat_dumpling",
    "pet_treats_alien",
    "pet_treats_all",
    "petBedBasketExpensiveClean",
    "petBedBasketExpensiveDirty",
    "petBedBasketModerateClean",
    "petBedBasketModerateDirty",
    "petfoodbox_mammal",
    "pettoycatnip",
    "pettoycatnipbird",
    "pettoycatnipbunny",
    "pettoycatnipcarrot",
    "pettoycatnipdebris_surface",
    "pettoycatnipsack",
    "pillar_small_green",
    "placeholder",
    "plant_floor_planter_romantic",
    "plant_floor_rubber_tree",
    "plant_tree_saguro",
    "plantFloorOrchidDead",
    "plantFloorOrchidHealthy",
    "plantFloorOrchidPot",
    "plate_of_food(empty)",
    "plate_of_food(full)",
    "plate_of_food(half-empty)",
    "plaza_atm_main",
    "plaza_bench_frame",
    "plaza_brickwall",
    "plaza_cart_coffee",
    "plaza_cart_coffee_avgroof",
    "plaza_cart_coffee_cup",
    "plaza_cart_coffee_tiffany",
    "plaza_cart_smoothie_bluetiles",
    "plaza_cart_smoothie_bluetiles_small",
    "plaza_cart_smoothie_roof",
    "plaza_cart_smoothie_sign",
    "plaza_cart_smoothie_tiles",
    "plaza_cart_smoothie_tiles_small",
    "plaza_cart_smoothie_top_orange",
    "plaza_cart_smoothie_top_pink",
    "plaza_cart_smoothie_top_purple",
    "plaza_cart_smoothie_top_yellow",
    "plaza_central_fountain_fish",
    "plaza_central_fountain_main",
    "plaza_centralstatue_gold",
    "plaza_drinkingfountain",
    "plaza_drinkingfountain_chrome",
    "plaza_emporium_interior",
    "plaza_emporium_tile_1",
    "plaza_emporium_tile_1c",
    "plaza_emporium_tile_2b",
    "plaza_emporium_tile_2c",
    "plaza_emporium_tile_2d",
    "plaza_emporium_tile_3b",
    "plaza_emporium_tile_base",
    "plaza_emporium_wetfood",
    "plaza_icecream_cone",
    "plaza_icecreamcart_sign",
    "plaza_pet_emporium_sign",
    "plaza_pet_emporium_signglow",
    "plaza_pet_kennel_interior_wall",
    "plaza_pet_kennel_sign",
    "plaza_pet_salon_logoglow",
    "plaza_pet_salon_materials",
    "plaza_pet_salon_sign",
    "plaza_pet_store_bakery",
    "plaza_pet_store_bakery_hextile",
    "plaza_pet_store_bakery_logoglow",
    "plaza_pet_store_bakery_sign",
    "plaza_pet_store_purveyor_door",
    "plaza_pet_store_purveyor_logoglow",
    "plaza_pet_store_purveyor_sign",
    "plaza_pet_store_purveyor_stucco",
    "plaza_pet_store_purveyor_stucco_white",
    "plaza_pet_store_purveyor_trim",
    "plaza_pet_store_purveyor_trim_2",
    "plaza_pet_store_toy_base",
    "plaza_pet_store_toy_counter",
    "plaza_pet_store_toy_logoglow",
    "plaza_pet_store_toy_roof",
    "plaza_pet_store_toy_sign",
    "plaza_phonebooth",
    "plaza_sign",
    "plumbing_bathtub_hitech_jets",
    "plumbing_bathtub_ornate",
    "plumbing_fountain_love_main",
    "plumbing_hottub_hearts",
    "plumbing_hydrant_redandwhite",
    "plumbing_sink_counter_retro",
    "plumbing_sink_floor_porcelain",
    "plumbing_toilet_cheap",
    "plumbing_toilet_retro",
    "powersocials_chugalug_can",
    "powersocials_xamshot_camera",
    "prop_banana",
    "prop_baseball_bat",
    "prop_basket_weave",
    "prop_bbq_spatula",
    "prop_billiards",
    "prop_bills",
    "prop_blastikiss_breath_spray",
    "prop_bone",
    "prop_book",
    "prop_bottle",
    "prop_bug",
    "prop_carddeck",
    "prop_cat_catnip",
    "prop_cat_fish_skeleton",
    "prop_cat_fish_whole",
    "prop_coin",
    "prop_colacan",
    "prop_critter_blue",
    "prop_critter_red",
    "prop_daisy",
    "prop_dollar",
    "prop_donut",
    "prop_dustpan",
    "prop_dustpanash",
    "prop_electronics_vending_hotdog_object",
    "prop_energycan",
    "prop_espresso_cup",
    "prop_extinguisher",
    "prop_firecrackers",
    "prop_fish_opaque",
    "prop_gamble_dice",
    "prop_game_drivingrange_ball_propa",
    "prop_game_drivingrange_club_left_hand",
    "prop_gift_box",
    "prop_glove",
    "prop_gnome_tools",
    "prop_hackysack",
    "prop_handbroom",
    "prop_handheld_game",
    "prop_hobostick",
    "prop_hoverboard",
    "prop_job_fashion_photo",
    "prop_job_ferret",
    "prop_job_sushi_whizz_plate",
    "prop_juggle",
    "prop_knife",
    "prop_magazine",
    "prop_microphone",
    "prop_money",
    "prop_monkey_arm",
    "prop_mop",
    "prop_newspaper",
    "prop_nightstick",
    "prop_paintbrush_art",
    "prop_piano_mallet",
    "prop_pinecone",
    "prop_plunger",
    "prop_portrait_awful",
    "prop_portrait_great",
    "prop_powerchord_grate",
    "prop_powerchord_guitar_left_handprop1",
    "prop_powerchord_speaker",
    "prop_powerchord_speakerfelt",
    "prop_reaper_scythe",
    "prop_redbullcan",
    "prop_redbullcan_02",
    "prop_remote",
    "prop_repogun",
    "prop_ringbox",
    "prop_robothand",
    "prop_sandwich",
    "prop_screwdriver",
    "prop_scrubbrush",
    "prop_sexymask",
    "prop_skateboard",
    "prop_socials_stinkbomb",
    "prop_sock",
    "prop_sockpuppet",
    "prop_soda_can",
    "prop_sofa_monkey_paw",
    "prop_softball",
    "prop_spit",
    "prop_sponge",
    "prop_spoon",
    "prop_spraycan",
    "prop_strobe",
    "prop_surfboards",
    "prop_towelwhip",
    "prop_trashbag",
    "prop_tumbler",
    "prop_underwear",
    "prop_urbz",
    "prop_utensils",
    "prop_vacuum",
    "prop_violin",
    "prop_wateringcan",
    "prop_wine_bottle",
    "prop_wrench",
    "prop_xam",
    "props_wire_brush_marshmellow",
    "puppykittenballoon_balloon",
    "puppykittenballoon_box",
    "puppykittenbasket_basket",
    "redbull_small",
    "redbull_small_02",
    "reflection_glass",
    "reflection_stainless_steel",
    "refrigerator_black_diamond",
    "rep_01",
    "rep_02",
    "rep_03",
    "rep_04",
    "rep_05",
    "ringmaker",
    "river_opaque",
    "rockcliff_island",
    "rockcliff_river",
    "rocks_lake_shore",
    "roof_02_jap",
    "roof_jap",
    "roof_tile_brown",
    "roof_tile_brown_1",
    "roof_tile_green",
    "roof_tile_white",
    "roof_yellow",
    "roofing_flat01",
    "rug_expensive",
    "sandybank",
    "sausagepackage",
    "scarecrow",
    "scenery_floor_roofvent",
    "scenery_neon_walkway",
    "scenery_tree_shrub_bark",
    "sceneryFruitTreeFruitDirt",
    "sceneryFruitTreeLevel2",
    "sceneryFruitTreeLevel3",
    "sceneryVegetableGardenDirt",
    "sceneryVegetableGardenLevel1",
    "sceneryVegetableGardenLevel3",
    "sculptureFloorAtomic",
    "sculptureFloorPetCrazy1",
    "sculptureFloorPetCrazy2",
    "sculptureWallAtomic",
    "sculptureWallPetCrazy1",
    "sculptureWallPetCrazy2",
    "seating_chair_adirondack",
    "seating_chair_stationary_eames",
    "seating_couch_chia_fruit",
    "seating_couch_hostel",
    "seating_sofa_modern_green",
    "seating_sofa_worn",
    "seating_swing_love_heart",
    "seatingAntiqueWood",
    "seatingChairComfyAntique",
    "seatingChairComfyPetCrazy",
    "seatingChairScootableAntique",
    "seatingChairScootableBlueAndWhite",
    "seatingChairScootableOutdoor",
    "seatingChairScootablePetCrazy",
    "seatingLoveseatAntiqueFabric",
    "seatingLoveseatAtomic",
    "seatingLoveseatPetCrazy",
    "seatingMissionCushions",
    "seatingMissionWood",
    "seatingSofaAntiqueFabric",
    "seatingSofaAtomic",
    "seatingSofaAtomicPillow",
    "seatingsofapetcrazy",
    "shared_marble_white",
    "shared_pettoycatdancer",
    "shared_plaza_centralstatue_inside",
    "shared_plaza_centralstatue_petcolor",
    "shared_plaza_centralstatue_petday",
    "shore_grass_sand",
    "shore_rocks",
    "shower_curtain",
    "shower_towel",
    "sink_bowl_copper",
    "skill_creative_painting1",
    "skill_creative_painting2",
    "skill_creative_painting3",
    "skill_creative_painting_canvas",
    "skill_creative_piano_upright",
    "skill_mechanical_invention_bench",
    "skill_mental_head",
    "skill_mirror_all_02",
    "skill_physical_punching_bag",
    "skill_physical_weight_bench_machine",
    "skydome_gradient",
    "skydome_gradient_02",
    "slotmachine",
    "snpc_paperboy",
    "solid_color_blue",
    "sportbikengine",
    "stereo_boombox",
    "stereo_expensive",
    "stereo_expensive_on",
    "stereo_expensive_speaker",
    "stereo_expensive_wood",
    "stone_bridge_main",
    "stone_bridge_rail",
    "stone_wall",
    "stone_wall02",
    "strawberry",
    "street_lane_01_house",
    "street_lane_03_house",
    "street_stripe",
    "subway_map_brick",
    "swing_regular",
    "table_desk_normal_2x1",
    "table_dining_expensive",
    "table_dining_expensive_metal",
    "table_japanese_flower",
    "table_legs_dark",
    "tableDeskAntique",
    "tableDiningAntique",
    "tableDiningChairScootablePetA",
    "tableDiningMission",
    "tableDiningOutdoor",
    "tableDiningPetCrazyTop",
    "tableEndAntique",
    "tableEndMission",
    "tableEndPetCrazy",
    "taxi_headlights",
    "telescopes",
    "templateareastoavoid",
    "terrain_cliff_stone05",
    "terrain_cliff_stone06",
    "terrain_desert_hills",
    "terrain_grass_noise_detail",
    "terrain_gravel",
    "terrain_ground_grassground",
    "terrain_ground_jap_noise",
    "terrain_hills_groundgrass",
    "terrain_jap_stone",
    "terrain_jap_stone_01",
    "terrain_jap_stone_alone",
    "terrain_mesa_low_01",
    "Terrain_Mesa_Low_02",
    "terrain_mesa_low_03",
    "terrain_mesa_low_04",
    "terrain_mesa_noise_detail",
    "terrain_mom_house_detail",
    "terrain_road_gravel",
    "terrain_rock_inner_128",
    "terrain_rockwall_base_128",
    "tiled_counter_face",
    "tiled_counter_side",
    "tiled_counter_top",
    "toilet_bronze",
    "tombstone",
    "towngrowth_brick",
    "towngrowth_roof",
    "towngrowth_wood",
    "tractorrusted",
    "train_tracks",
    "trampoline_pad",
    "trampoline_stripes",
    "transition_cas_wardrobe",
    "transition_cas_wardrobe_02",
    "transition_cas_wardrobe_03",
    "transitionCASdresserPetCrazy",
    "trash_ash_search",
    "trashcan_bottomless",
    "tree_alien",
    "tree_birch_bark",
    "tree_dirt",
    "tree_maple_bark",
    "tree_rose_bark",
    "tree_willow_bark",
    "trottco_27_inch_color_television-(screen)-00",
    "turd_water",
    "tutorial_rep_icon",
    "tv_cheap",
    "tv_cheap_wood",
    "tv_normal",
    "tv_screen_search",
    "twitchomatic_skill_zonebar",
    "ui_creditscreen",
    "van_windshield",
    "vehicle_honda_civic",
    "vehicle_honda_element_small",
    "vehicle_honda_s2000",
    "vehicle_japanese_compact_silver",
    "vehicle_verizonvan",
    "vehicle_verizonvan_02",
    "vehicle_windshield",
    "verizon_logolong",
    "w_bodnhead_cblack",
    "w_bodnhead_cblue",
    "w_bodnhead_cbluegrey",
    "w_bodnhead_ccream",
    "w_bodnhead_cdkbrown",
    "w_bodnhead_cdkgrey",
    "w_bodnhead_cgolden",
    "w_bodnhead_cgreen",
    "w_bodnhead_cmedbrown",
    "w_bodnhead_cpink",
    "w_bodnhead_cpurple",
    "w_bodnhead_cred",
    "w_bodnhead_credkisspoint",
    "w_bodnhead_csilver",
    "w_bodnhead_cwhite",
    "wall_atomicrustic_left",
    "wall_atomicrustic_right",
    "wall_brick_qb1_grey",
    "wall_brick_red",
    "wall_brick_vertical_concrete_base",
    "wall_cement_blue",
    "wall_cement_white",
    "wall_curtain_red",
    "wall_exterior_redwoodshingle",
    "wall_fashion_white",
    "wall_floral_euroblue",
    "wall_flw_left",
    "wall_flw_middle",
    "wall_flw_middleup",
    "wall_flw_right",
    "wall_flw_stainedglass",
    "wall_jap_stone",
    "wall_jap_stone_orange",
    "wall_mission_blackwhite",
    "wall_paintburgundy_k2crech2_md",
    "wall_paintburntorange_k2crech2_lt",
    "wall_paintjewelspotlightgreen",
    "wall_panel_angledclap_lightwood",
    "wall_panel_horizontalwood_teak",
    "wall_panel_woodmissionleft_brown",
    "wall_panel_woodmissionleftcenter_brown",
    "wall_panel_woodmissionrightcenter_brown",
    "wall_panelwood_birch",
    "wall_panelwood_inlaytraditional2",
    "wall_paper_bakeryblue_stripesolid",
    "wall_paper_bearhunter_stripesolid",
    "wall_paper_brownleaf",
    "wall_paper_storyborderfloral_6",
    "wall_paperdamask_wdouble_md",
    "wall_pet_kennel",
    "wall_pettheme_bonebrick",
    "wall_pettheme_catnosecurtain",
    "wall_pettheme_catnosecurtain_2",
    "wall_pettheme_paintdogbones",
    "wall_petthemed_paintpawflower_brown",
    "wall_petthemed_paintpawflower_white",
    "wall_pouredstucco_tan",
    "wall_stone_brown",
    "wall_stone_grey",
    "wall_stone_greystonefence",
    "wall_stone_stackedstone",
    "wall_stucco_red_1",
    "wall_tiki_mask",
    "wall_wallpapermedford",
    "wall_wood_blue",
    "wall_wood_concrete",
    "wall_wood_darkbrown_vertical",
    "wall_wood_darkbrown_vertical_1",
    "wall_wood_lightbrown_fence",
    "wall_wood_lightbrown_vertical_1",
    "wallpaper_derelict_02",
    "wallpaper_plain_blue",
    "wallpaper_plain_white",
    "wallpaper_wood_planks_vertical_1",
    "walls_allien_03",
    "walls_allien_steelout",
    "walls_allien_trailer",
    "walls_allien_trailerbedroom",
    "walls_cliff_01",
    "walls_cliff_02",
    "walls_funky_02",
    "walls_groovy_pat03",
    "walls_house_02",
    "walls_house_06",
    "walls_house_09",
    "walls_industrial_concrete",
    "walls_industrial_concretetile_blue",
    "walls_industrial_grayconcrete",
    "walls_jap_02",
    "walls_jap_bath",
    "walls_jap_out",
    "walls_jap_paper",
    "walls_jap_wood",
    "walls_jap_woodrail",
    "walls_mesa_03",
    "walls_mesa_gallery",
    "walls_mesa_out_02",
    "walls_paint_woodgreen",
    "walls_residental_brick_red",
    "walls_residental_metal_white",
    "walls_residental_paint_cream",
    "walls_residental_paint_offwhite",
    "walls_residental_paper_blue",
    "walls_residental_tile_blue",
    "walls_residental_wood_panel",
    "walls_residental_wood_red",
    "walls_residental_wood_stripe",
    "walls_residental_wood_white",
    "walls_residential_drkbrwnbase",
    "walls_residential_greydiamondbath",
    "walls_residential_lightgreywainscot",
    "walls_residential_tilehalfdarkblue",
    "walls_residential_whiteclapboard",
    "walls_residential_woodpanel01",
    "walls_residential_woodshingle",
    "walls_stone_blueorange",
    "walls_stone_white",
    "walls_west_bath",
    "walls_west_bed",
    "walls_west_hotel",
    "walls_west_out01",
    "walls_west_out02",
    "walls_west_rest",
    "walnut_door",
    "wheel_truck",
    "white",
    "window_alien_pane_full",
    "window_expensive_full",
    "window_expensive_full_marble",
    "window_four_pane_mix",
    "window_full_pane_shoji",
    "window_porthole",
    "WindowAntique1",
    "windowAntique2",
    "windowAtomicFrameBlack",
    "windowPaneHalfMissionLeadedglass",
    "windowPetCrazyFrame",
    "windowWoodMission",
    "windsor_door",
    "wire_brush_marshmellow",
    "wood01",
    "wood02",
    "wood_board",
    "wood_bundle",
    "wood_crate",
    "wood_crate_dirty",
    "wood_distressed_01",
    "wood_fence_rough",
    "wood_generic_worn_no_shadows",
    "wood_groovy",
    "wood_groovy2",
    "wood_groovy3",
    "wood_jap",
    "wood_pallet",
    "wood_slat01",
    "wood_violin",
    "woods_trees_a",
    "wrappers",
];
