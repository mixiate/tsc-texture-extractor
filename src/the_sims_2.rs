fn convert(bytes: &[u8]) -> image::RgbaImage {
    let bytes = &bytes[16..];
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let width = usize::from(u16::from_le_bytes(
        bytes[null_position + 21..null_position + 23].try_into().unwrap(),
    ));
    let height = usize::from(u16::from_le_bytes(
        bytes[null_position + 23..null_position + 25].try_into().unwrap(),
    ));

    let flags = bytes[null_position + 13];
    let flags2 = bytes[null_position + 29];
    let flags3 = bytes[null_position + 31];

    if flags & 0b1000_0000 == 0 && flags2 & 0b0000_0001 == 0 {
        crate::xbox::decode_bc1(&bytes[null_position + 37..], width, height)
    } else if flags & 0b1000_0000 == 0 && flags & 0b0100_0000 == 0 && flags2 & 0b0000_0001 > 0 {
        crate::xbox::decode_bc2(&bytes[null_position + 37..], width, height)
    } else if flags & 0b1000_0000 > 0 && flags3 & 0b0001_0000 != 0 {
        crate::xbox::decode_rgb5(&bytes[null_position + 33..], width, height)
    } else if flags & 0b1000_0000 > 0 && flags2 & 0b0000_0001 != 0 {
        let palette = &bytes[bytes.len() - 1024..];
        crate::xbox::decode_c8(&bytes[null_position + 37..], width, height, palette)
    } else {
        crate::xbox::decode_rgba8(&bytes[null_position + 37..], width, height)
    }
}

pub fn extract_textures(textures_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let textures = std::fs::read(textures_path).unwrap();

    let file_list = crate::arc::list_files(&textures, crate::Endianness::Little);

    for (name, _, bytes) in file_list {
        let image = convert(bytes);
        crate::save_texture(image, &name, output_path, SPECULAR_FILE_NAMES.contains(&name.as_str()));
    }
}

const SPECULAR_FILE_NAMES: [&str; 1159] = [
    "_garbage",
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
    "am_hh_dummy",
    "am_sl_furcoat_ln_blue",
    "appliance_bar",
    "appliance_blender_cheap_main",
    "appliance_blender_expensive_main",
    "appliance_bubbleblower",
    "appliance_counterfeit",
    "appliance_dishwasher_traditional",
    "appliance_fire_extinguisher_panel",
    "appliance_food_processor_groove",
    "appliance_fridge_retro",
    "appliance_grill_sausage",
    "appliance_keg_ice",
    "appliance_keg_pineapple",
    "appliance_keg_pineapple_skin",
    "appliance_modern_fridge_detail",
    "appliance_refrigerater_cheap",
    "appliance_stove_alien",
    "appliance_stove_bbq_charcoal",
    "appliance_stove_modern_body",
    "appliance_stove_retro",
    "appliance_stove_x_stainless",
    "appliance_vacuum_dc",
    "aquarium_floor_02_parts",
    "art_alien",
    "art_alien_space",
    "art_floor_machine_claw",
    "art_floor_sculpture_contemporary_metal_frame",
    "art_floor_sculpture_contemporary_patina_grn_art",
    "art_floor_sculpture_contemporary_peened_metal",
    "art_painting_back_2",
    "art_painting_landscape_01",
    "art_painting_landscape_02",
    "art_picture_01",
    "art_picture_02",
    "art_plant_wild",
    "art_sculpture_birdbath_main",
    "art_sculpture_brain_jar",
    "art_sculpture_x_parrot",
    "art_trim_01_sm",
    "art_trim_02_sm",
    "art_wall_back_02",
    "art_wall_back_canvas",
    "art_wall_contemp_large",
    "art_wall_contemp_small",
    "art_wall_firearm",
    "art_wall_frame_fancy",
    "art_wall_frame_metal",
    "art_wall_frame_wood",
    "art_wall_music_studio",
    "art_wall_painting_01",
    "art_wall_painting_impressionism",
    "art_wall_painting_jaydeLichtenstein",
    "art_wall_painting_plumbobwarhol",
    "art_wall_poster_kungfubunny",
    "art_wall_poster_punk",
    "art_wall_poster_reggae",
    "art_wall_poster_willwright",
    "art_wall_romantic_heart",
    "art_wall_romantic_heart_back",
    "art_wall_romantic_large",
    "art_wall_romantic_large_frame",
    "art_wall_romantic_large_gilding",
    "art_wall_skull",
    "bamboo_soap",
    "basketballcourt",
    "bathtub_oriental_brown",
    "beach_house_trim",
    "bed_double_groovy",
    "bed_double_japanese",
    "bed_double_japanese_wood",
    "bed_modern_comforter",
    "bed_padded_comforter",
    "bed_padded_comforter_pink",
    "bed_single_cot",
    "bed_single_cot_wood",
    "bed_single_double_brass",
    "bed_single_tent_sheet",
    "bedsheet_pillow_set_pink",
    "beejaphone_guitar",
    "billboard_verizon02_white",
    "billboard_verizon02_white_02",
    "biodome_hextile",
    "biodome_oxytank",
    "biodome_platform_sidewall",
    "biodome_submarine",
    "bookcase_books",
    "bookshelf_comic_covers",
    "bookshelf_comics_humidor",
    "bookshelf_expensive",
    "bridge_metal",
    "bridge_sides",
    "building_02",
    "building_06",
    "building_08",
    "building_40",
    "building_apartment_01_roof",
    "building_apartment_01_wall",
    "cabinet_armoire_punk_base",
    "cabinet_armoire_punk_door",
    "caf_backdrop_matte",
    "caf_camera",
    "caf_column_wood_steel",
    "caf_floor_lights",
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
    "cas_floor_wideoakplanks",
    "cas_lightning_grad",
    "cas_loveseat_beige",
    "cas_loveseat_tan",
    "cas_mirror_stainless",
    "cas_pictureframe_frames",
    "cas_pictureframe_matte",
    "cas_pictureframe_photos",
    "cas_placeholder",
    "cas_wall_brick_chimney_red",
    "chair_comfyegg_blue",
    "chair_dining_hostel",
    "chair_dining_outdoors",
    "chair_expensive",
    "chair_living_groovy",
    "chair_music_studio",
    "chopper_paint_blue",
    "chrome_tool",
    "clogged_water",
    "club_interior_speaker",
    "concrete_46_floor",
    "condo_concrete",
    "condo_concrete_a",
    "condo_concrete_base",
    "condo_door_main",
    "condo_door_main_a",
    "condo_door_red_a",
    "condo_fence_a",
    "condo_flowers",
    "condo_innerlot_stucco",
    "condo_innerlot_woodsiding",
    "condo_metal_roofing",
    "condo_metal_roofing_a",
    "condo_metal_swatch",
    "condo_metal_swatch_a",
    "condo_mulch",
    "condo_roof_sand",
    "condo_roof_sand_a",
    "condo_stucco_01",
    "condo_stucco_01_a",
    "condo_stucco_01_and_base",
    "condo_stucco_01_and_base_a",
    "condo_stucco_03_a",
    "condo_wood_deck",
    "condo_wood_deck_a",
    "condo_woodsiding_01_a",
    "condo_woodsiding_02",
    "condo_woodsiding_02_a",
    "condo_woodsiding_03_and_base",
    "condo_woodsiding_03_and_base_a",
    "coral06",
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
    "cozmo_brass",
    "cozmo_door_glass_brass",
    "cozmo_lamp_wall_sconce",
    "cozmo_metal_02",
    "cozmo_toilet_drain",
    "cozmo_wood_01",
    "cozmo_wood_02",
    "cozmo_wood_03",
    "cozmo_wood_molding_01",
    "dark_wood",
    "dark_wood_edged",
    "dirt_01",
    "door_alien_airlock",
    "door_barred",
    "door_expensive_fancy",
    "door_expensive_plain_metal1",
    "door_expensive_plain_metal2",
    "door_fancy_inlay_body",
    "door_garage",
    "door_garage_blue",
    "door_garage_yellow",
    "door_grey_hostel",
    "door_hostel_white",
    "door_metal_smallwindow",
    "door_nauti",
    "door_nauti_fancy",
    "door_painted_plain",
    "door_painted_plain_detail",
    "door_plain",
    "door_solid_screen",
    "door_solid_screen_cloth",
    "door_solid_screen_cloth_02",
    "door_western_grey",
    "door_western_plain",
    "door_western_plain_wood",
    "door_western_saloon",
    "doors_house_white",
    "drag_bike_green",
    "dresser_groovy_drawers",
    "dresser_groovy_wood",
    "drink_bar_saloon",
    "drinkbar_biker_bottle",
    "drinkbar_biker_drinkglass",
    "drinkbar_biker_shaker",
    "drinkbar_biker_tray",
    "drinkbar_contemporary",
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
    "electronics_arcade_love",
    "electronics_arcade_love_inuse_loop1",
    "electronics_arcade_love_inuse_loop2",
    "electronics_arcade_love_on_loop1",
    "electronics_arcade_people_invaders",
    "electronics_arcade_regular",
    "electronics_arcade_scifi",
    "electronics_arcade_scifi_inuse_loop1",
    "electronics_arcade_scifi_inuse_loop2",
    "electronics_arcade_scifi_on_loop1",
    "electronics_arcade_scifi_ship",
    "electronics_arcade_screen",
    "electronics_computer_alien",
    "electronics_computer_cheap_main",
    "electronics_computer_cheap_screen_blog_a",
    "electronics_computer_cheap_screen_sims2_a",
    "electronics_computer_cheap_screen_sims_bustin_out_a",
    "electronics_computer_cheap_screen_spore_a",
    "electronics_computer_cheap_screen_startup",
    "electronics_computer_expensive_main",
    "electronics_console",
    "electronics_dancing_element_particle",
    "electronics_expensive_stereo",
    "electronics_massage_table",
    "electronics_metal_detector",
    "electronics_phone_wall_standard",
    "electronics_phone_wall_standard_screen",
    "electronics_stereo",
    "electronics_stereo_01",
    "electronics_stereo_02",
    "electronics_stereo_dj_booth",
    "electronics_stereo_jukebox_biker",
    "electronics_tv_channel_food_01",
    "electronics_tv_channel_horror_01",
    "electronics_tv_channel_music_01",
    "electronics_tv_channel_news_01",
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
    "espresso_grill",
    "eyetoy_billboard_01",
    "eyetoy_default_slot_01",
    "eyetoy_default_slot_02",
    "eyetoy_default_slot_03",
    "eyetoy_default_slot_04",
    "eyetoy_default_slot_05",
    "eyetoy_funframe_01",
    "eyetoy_funframe_02",
    "eyetoy_funframe_03",
    "eyetoy_funframe_04",
    "eyetoy_funframe_05",
    "eyetoy_funframe_06",
    "eyetoy_funframe_07",
    "eyetoy_funframe_08",
    "eyetoy_funframe_09",
    "eyetoy_funframe_10",
    "eyetoy_funframe_11",
    "eyetoy_funframe_12",
    "fabric_velvet_red",
    "fe_black",
    "fe_floor_tile_street_edge",
    "fence_brick_concrete",
    "fence_deadwood",
    "fence_endcap",
    "fence_japanese",
    "fence_white_picket",
    "fiberglass_blue",
    "final-floor-tile-46",
    "final_adult_male_nude",
    "fireplace_expensive",
    "fireplace_expensive_tile",
    "fireplace_expensive_tile_dirty",
    "fireplace_log",
    "floor_allien_orange",
    "floor_allien_steel",
    "floor_bio_01",
    "floor_bio_02",
    "floor_bio_03",
    "floor_bio_04",
    "floor_carpet_blue",
    "floor_carpet_brown",
    "floor_carpet_brownline",
    "floor_carpet_brownlineflip",
    "floor_carpet_green",
    "floor_carpet_orange",
    "floor_carpet_orangeline",
    "floor_carpet_orangelineflip",
    "floor_carpet_red",
    "floor_concrete_grey",
    "floor_derelict_02",
    "floor_dirt",
    "floor_dirt_yellow",
    "floor_dirtroad",
    "floor_dirtroad_edge_02",
    "floor_editoggler",
    "floor_funky_01",
    "floor_grass_warmgreen",
    "floor_grass_yellow",
    "floor_groovy_pat01",
    "floor_groovy_pat02",
    "floor_hostel",
    "floor_house_01",
    "floor_house_02",
    "floor_house_03",
    "floor_house_04",
    "floor_house_05",
    "floor_house_06",
    "floor_jap_01",
    "floor_jap_02",
    "floor_jap_03",
    "floor_marble_blue",
    "floor_neon_blue",
    "floor_sidewalk_venice",
    "floor_stone_blueorange",
    "floor_stone_gray",
    "floor_stone_mex",
    "floor_stone_pink",
    "floor_tile_4wide",
    "floor_tile_blue",
    "floor_tile_gold",
    "floor_tile_green",
    "floor_tile_invisible",
    "floor_tile_street_edge",
    "floor_tile_street_middle_left",
    "floor_tile_street_middle_right",
    "floor_trailer_bath",
    "floor_trailer_carpet",
    "floor_wood_deck",
    "floor_wood_light",
    "floor_wood_light01",
    "floor_wood_medbrwndeck",
    "floor_wood_red",
    "floortile_apt3_linoleum_diagonal",
    "floortile_apt3_linoleum_punk",
    "floortile_apt_granite_checker",
    "floortile_biker_concrete",
    "floortile_cozmo_carpet_pattern",
    "floortile_cozmo_wood_bamboo",
    "floortile_global_sidewalk",
    "floortile_global_sidewalk_grooves",
    "floortile_subway_runner_cross",
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
    "furniture_costume_trunk_drawers",
    "furniture_costume_trunk_main",
    "g_bladder_icon",
    "g_bladderjob_icon",
    "game_air_hockey",
    "game_airhockey",
    "game_bottle",
    "game_checker_board",
    "game_drivingrange",
    "game_foosball_field",
    "game_foosball_players",
    "game_pinball_machine",
    "game_pingpong_paddle",
    "game_slot_machine",
    "genie_head",
    "genie_jewelpanel",
    "girder",
    "global_appliance_retro",
    "global_bamboo_dead",
    "global_bamboo_top",
    "global_bedsheet_pillow_set",
    "global_brass",
    "global_brass_solid",
    "global_chrome",
    "global_concrete_trim",
    "global_copper",
    "global_cushion_canvas_blue",
    "global_cushion_fabric_cream",
    "global_cushion_fabric_hippy",
    "global_cushion_leather_dark",
    "global_cushion_leather_old",
    "global_cushion_leather_red",
    "global_cushion_pattern_grn",
    "global_decorative_japanese02",
    "global_formica_red",
    "global_formica_white",
    "global_foundry_warning_label",
    "global_foundry_welded_metal",
    "global_fridge_bits",
    "global_glass_red_dark",
    "global_glass_redlight_scroll",
    "global_gold_plating",
    "global_gold_reflective",
    "global_marble",
    "global_metal_shiny",
    "global_metal_siding02",
    "global_metal_stainless_01",
    "global_metalrust02",
    "global_molding_metal_brushed",
    "global_molding_wood_white",
    "global_picframemtl",
    "global_plastic_black",
    "global_plastic_blue_speckle",
    "global_plastic_cream",
    "global_plastic_lightblue_speckle",
    "global_plastic_red_speckle",
    "global_rebar",
    "global_shower_door",
    "global_stainless_solid",
    "global_stone_tilable",
    "global_velvet_wiseguys",
    "global_vinyl_rib",
    "global_warning_stripe",
    "global_wood_black",
    "global_wood_brown",
    "global_wood_cherry",
    "global_wood_dark",
    "global_wood_light",
    "global_wood_neon",
    "global_wood_panel",
    "global_wood_red",
    "global_wood_slats",
    "gravel_64",
    "greek_female",
    "hamster_wheel_frame",
    "hamster_wheel_main",
    "hamster_wheel_spokes_n_components",
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
    "japanese_wall_art_1",
    "japanese_wall_art_2",
    "jewel_blue",
    "job_ferret_box",
    "job_fireworks",
    "kicktail_metal_02",
    "lamp_dance_lights",
    "lamp_floor_02",
    "lamp_floor_boom",
    "lamp_floor_expensive",
    "lamp_floor_gasheater",
    "lamp_floor_groovy_off",
    "lamp_floor_groovy_on",
    "lamp_floor_multi_metal",
    "lamp_floor_multi_off",
    "lamp_floor_multi_on",
    "lamp_floor_music_studio",
    "lamp_floor_tiki",
    "lamp_floor_tiki_shade",
    "lamp_table_expensive1",
    "lamp_table_expensive1_off",
    "lamp_table_expensive2",
    "lamp_table_pineapple_lava",
    "lamp_table_pineapple_lava_shade",
    "lamp_table_saloon",
    "lamp_table_saloon_shade",
    "lamp_wall_01",
    "lamp_wall_expensive1",
    "lamp_wall_expensive2",
    "lamp_wall_modern",
    "lamp_wall_saloon",
    "lamp_wall_sconce_blue",
    "largerockwall_base",
    "largerockwall_base_more",
    "largerockwall_base_vertcracks",
    "largerockwall_justtile",
    "leaves_street",
    "leaves_walk",
    "lighthouse_stripe",
    "lighting_floor_studio_lights",
    "lighting_table_asian_lamp_screen_shoj_lit",
    "lighting_table_central_asian2_base_green",
    "lighting_table_central_asian2_shade_off",
    "lighting_table_central_asian2_shade_on",
    "loading_subway_map",
    "loveseat_groovy_cushion",
    "loveseat_groovy_frame",
    "mailbox",
    "mailbox_colonial",
    "map_1",
    "map_2",
    "map_edge_beach_water",
    "map_edge_grass_beach",
    "map_hills_01",
    "map_hills_02",
    "map_hills_03",
    "map_lot12",
    "map_lot5",
    "map_lot6",
    "map_lot7",
    "map_lot8",
    "map_lot9",
    "map_street_lane_01_house",
    "map_terrain_desert_stone",
    "maple_door_frame",
    "marbles_biege_green",
    "marbles_biege_green_dirty",
    "massage_cushion",
    "menubevel b - r",
    "mesa_concrete",
    "mesa_goodtop",
    "mesa_massive",
    "mesa_road",
    "mesa_rockwall_mid02",
    "metal",
    "metal_burnt",
    "metal_panel02",
    "metal_rusty_gradiant",
    "microwave_expensive",
    "misc_allien_platform",
    "misc_allien_ship",
    "misc_barrier_yellow",
    "misc_cactus",
    "misc_glow",
    "misc_golfcart",
    "misc_house_small",
    "misc_movieprops",
    "misc_painting_western",
    "misc_rail_gray",
    "misc_shops",
    "misc_sidewalkcurb_grey",
    "misc_storetop_blue",
    "misc_storetop_red",
    "misc_wood_deckrailing",
    "misc_wood_planks",
    "molding_maple",
    "narcisco_floor_mirror",
    "neon_blue",
    "neon_green",
    "neon_lightblue",
    "neon_white",
    "neon_white_dim",
    "neon_yellow",
    "new_cursor_03",
    "new_cursor_03_buildmode",
    "new_cursor_buymode",
    "nothing",
    "numica_counter_face",
    "numica_counter_side",
    "numica_counter_top",
    "outdoor_trash_can",
    "performance_stage_blue",
    "performance_stage_wood",
    "pillar_small_green",
    "pinball_machine_shark",
    "pinball_machine_ufo",
    "placeholder",
    "plant_floor_alien",
    "plant_floor_planter_cactus",
    "plant_floor_planter_romantic",
    "plant_floor_rubber_tree",
    "plant_palmetto",
    "plant_tree_saguro",
    "plate_of_food(empty)",
    "plate_of_food(full)",
    "plate_of_food(half-empty)",
    "plumbing_alien_tank_grate",
    "plumbing_antigrav_hose",
    "plumbing_antigrav_spa",
    "plumbing_bathtub_hitech_jets",
    "plumbing_bathtub_ornate",
    "plumbing_fountain_love_cupid",
    "plumbing_fountain_love_main",
    "plumbing_hottub_hearts",
    "plumbing_hottub_heartshaped",
    "plumbing_hydrant_redandwhite",
    "plumbing_shower_outdoor",
    "plumbing_shower_sonic",
    "plumbing_sink_counter_retro",
    "plumbing_sink_floor_porcelain",
    "plumbing_toilet_alien",
    "plumbing_toilet_cheap",
    "plumbing_toilet_industrial",
    "plumbing_toilet_outhouse",
    "plumbing_toilet_retro",
    "plumming_sink_floor_public_drain",
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
    "prop_foamhand",
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
    "redbull_small",
    "redbull_small_02",
    "reflection_stainless_steel",
    "refrigerator_black_diamond",
    "refrigerator_replicator",
    "rep_01",
    "rep_01_shader_poster",
    "rep_02",
    "rep_02_shader_floor_ad",
    "rep_02_shader_wall_ad",
    "rep_03",
    "rep_03_shader_floor_ad_02",
    "rep_03_shader_graffiti",
    "rep_03_shader_wall_ad_02",
    "rep_03_shader_wall_machine",
    "rep_04",
    "rep_04_shader_painting",
    "rep_04_shader_tv",
    "rep_04_verizon",
    "rep_05",
    "rep_05_shader_billboard",
    "rep_05_shader_honda",
    "rep_05_shader_jumbotron",
    "rep_05_shader_verizon",
    "ringmaker",
    "rock_cliff_sims2",
    "rock_cliff_sims2_gray",
    "roof_01_jap",
    "roof_02_jap",
    "roof_black",
    "roof_blue",
    "roof_jap",
    "roof_red",
    "roof_shingle_bluegrey",
    "roof_shingle_greengrey",
    "roof_terracottatile01",
    "roof_tile_brown",
    "roof_tile_brown_1",
    "roof_tile_green",
    "roof_tile_white",
    "roof_yellow",
    "roofing_flat01",
    "rug_expensive",
    "rug_modern_new",
    "rug_outdoor_blanket",
    "rug_persian_genie",
    "rug_persian_genie_feet",
    "satin_fringe",
    "satin_fringe_red",
    "sausagepackage",
    "scenery_neon_walkway",
    "scenery_sewer_grate",
    "scenery_tree_boxwoods",
    "scenery_tree_shrub_bark",
    "sculpture_chimes",
    "sculpture_japanese",
    "seating_2x1_bench_public",
    "seating_chair_adirondack",
    "seating_chair_comfy_western",
    "seating_chair_dining_saloon",
    "seating_chair_stationary_eames",
    "seating_couch_chia_3x1",
    "seating_couch_chia_3x1_grass",
    "seating_couch_chia_3x1_grass_dead",
    "seating_couch_chia_3x1_growth",
    "seating_couch_chia_decay",
    "seating_couch_chia_fruit",
    "seating_couch_expensive",
    "seating_couch_hostel",
    "seating_couch_wicker",
    "seating_couch_wicker_yellow",
    "seating_sofa_chair_x_cream_action_queue",
    "seating_sofa_couch_x_cream_action_queue",
    "seating_sofa_modern_green",
    "seating_sofa_worn",
    "seating_swing_love_heart",
    "see_me_feel_me_pinball_machine",
    "ship_allien",
    "shower_curtain",
    "shower_towel",
    "sidewalk_curb_grey",
    "sidewalk_safety",
    "sidewalk_tile_01",
    "sidewalk_turf",
    "siding_blue",
    "sink_bowl_copper",
    "skill_creative_painting1",
    "skill_creative_painting2",
    "skill_creative_painting3",
    "skill_creative_painting_canvas",
    "skill_creative_piano_player",
    "skill_creative_piano_player_roll",
    "skill_creative_piano_upright",
    "skill_mechanical_invention_bench",
    "skill_mental_chess_alien",
    "skill_mental_head",
    "skill_mirror_all_02",
    "skill_physical_punching_bag",
    "skill_physical_weight_bench_machine",
    "skydome_gradient",
    "skydome_gradient_02",
    "skydome_gradient_yacht",
    "slotmachine",
    "snpc_paperboy",
    "sportbikengine",
    "stairs",
    "stereo_boombox",
    "street_gray",
    "street_lane_01_biodome",
    "street_lane_01_house",
    "street_lane_01_yacht",
    "street_lane_03_house",
    "street_lane_04",
    "street_stripe",
    "sub_color",
    "subway_map_brick",
    "swing_regular",
    "table_coffee_groovy",
    "table_coffee_groovy_02",
    "table_desk_normal_2x1",
    "table_dining_expensive",
    "table_dining_expensive_metal",
    "table_dining_groovy",
    "table_dining_groovy_base",
    "table_dining_hostel",
    "table_dining_japanese",
    "table_dining_japanese_lacquer",
    "table_dining_japanese_lacquer_lines",
    "table_end_plastic_white",
    "table_japanese_flower",
    "table_legs_dark",
    "table_outdoors",
    "table_saloon_round",
    "table_teppan_3x2",
    "table_utility_alien",
    "taxi_headlights",
    "telescopes",
    "television_cheap_stand",
    "tenniscourt",
    "terrain_biodome_cliff01",
    "terrain_biodome_cliff02",
    "terrain_biodome_cliff03",
    "terrain_cliff_stone05",
    "terrain_cliff_stone06",
    "terrain_desert_hills",
    "terrain_desert_hole",
    "terrain_desert_hole_01",
    "terrain_desert_stone",
    "terrain_grass_noise_detail",
    "terrain_gravel",
    "terrain_ground_grassdryground",
    "terrain_ground_grassground",
    "terrain_ground_grassyellow",
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
    "terrain_road_yellow",
    "terrain_rock_inner_128",
    "terrain_rockwall_base_128",
    "terrain_tanbark",
    "tile_blue_tilable",
    "tile_mosaic_quarter",
    "tile_redblue_tilable",
    "tile_table_sides",
    "tiled_counter_face",
    "tiled_counter_side",
    "tiled_counter_top",
    "toilet_bronze",
    "toilet_teal",
    "tombstone",
    "trafficlight_dark",
    "train_tracks",
    "trampoline_pad",
    "trampoline_stripes",
    "transition_cas_wardrobe",
    "transition_cas_wardrobe_02",
    "transition_cas_wardrobe_03",
    "trash_ash_search",
    "trashcan_bottomless",
    "tree_alien",
    "tree_birch_02_branch",
    "tree_birch_bark",
    "tree_joshua_bark",
    "tree_maple_bark",
    "tree_rose_bark",
    "tree_willow_bark",
    "trottco_27_inch_color_television-(screen)-00",
    "turd_water",
    "tutorial_rep_icon",
    "tv_normal",
    "tv_screen_search",
    "twitchomatic_skill_zonebar",
    "ui_creditscreen",
    "ui_mainmenuscreen",
    "ui_online_conn",
    "ui_screen_gradient",
    "ui_start_screen",
    "vehicle_honda_civic",
    "vehicle_honda_element_small",
    "vehicle_honda_s2000",
    "vehicle_japanese_compact_silver",
    "vehicle_verizonvan",
    "vehicle_verizonvan_02",
    "vehicle_windshield",
    "verizon_logolong",
    "wall_blueconcrete",
    "wall_brick_red",
    "wall_brick_vertical_concrete_base",
    "wall_cement_blue",
    "wall_cement_white",
    "wall_fashion_white",
    "wall_hostel_brick",
    "wall_hostel_redbrick_browntrim",
    "wall_jap_stone",
    "wall_jap_stone_orange",
    "wall_metal_cargo_white",
    "wall_metal_cargo_white01",
    "wall_shingle_greytrim",
    "wall_stucco_red",
    "wall_stucco_red_1",
    "wall_tile_bluewood",
    "wall_tile_yellowwood",
    "wall_window_halfheight_condo",
    "wall_window_halfheight_condo_a",
    "wall_wood_blue",
    "wall_wood_blue_vertical",
    "wall_wood_concrete",
    "wall_wood_darkbrown_vertical",
    "wall_wood_darkbrown_vertical_1",
    "wall_wood_green_vertical",
    "wall_wood_grey_vertical",
    "wall_wood_lightbrown_fence",
    "wall_wood_lightbrown_vertical",
    "wall_wood_lightbrown_vertical_1",
    "wall_wood_planks_vertical",
    "wall_wood_purple01_vertical",
    "wall_wood_purple_vertical",
    "wall_wood_vertical_base",
    "wall_wood_vertical_base01",
    "wall_yellowconcrete",
    "wallpaper_derelict_02",
    "wallpaper_wood_planks_vertical",
    "wallpaper_wood_planks_vertical_1",
    "walls_allien_01",
    "walls_allien_02",
    "walls_allien_03",
    "walls_allien_bath",
    "walls_allien_steel",
    "walls_allien_steel_tr",
    "walls_allien_steelbedroom",
    "walls_allien_steelout",
    "walls_allien_trailer",
    "walls_allien_trailerbath",
    "walls_allien_trailerbedroom",
    "walls_allien_trailerout",
    "walls_bio_01",
    "walls_bio_02",
    "walls_bio_03",
    "walls_bio_04",
    "walls_cliff_01",
    "walls_cliff_02",
    "walls_down",
    "walls_funky_01",
    "walls_funky_02",
    "walls_funky_03",
    "walls_garden_concrete",
    "walls_groovy_pat01",
    "walls_groovy_pat02",
    "walls_groovy_pat03",
    "walls_house_02",
    "walls_house_05",
    "walls_house_06",
    "walls_house_08",
    "walls_house_09",
    "walls_house_10",
    "walls_industrial_concrete",
    "walls_industrial_concrete_blue",
    "walls_industrial_concrete_purple",
    "walls_industrial_concretetile_blue",
    "walls_industrial_concwall01",
    "walls_industrial_grayconcrete",
    "walls_jap_01",
    "walls_jap_02",
    "walls_jap_bath",
    "walls_jap_out",
    "walls_jap_paper",
    "walls_jap_paperlines",
    "walls_jap_wood",
    "walls_jap_woodrail",
    "walls_marble_blue",
    "walls_mesa_01",
    "walls_mesa_02",
    "walls_mesa_03",
    "walls_mesa_gallery",
    "walls_mesa_out_01",
    "walls_mesa_out_02",
    "walls_mesa_studio",
    "walls_paint_redstripe",
    "walls_paint_redwhitewood",
    "walls_paint_woodgreen",
    "walls_paint_woodorange",
    "walls_residental_brick_red",
    "walls_residental_metal_white",
    "walls_residental_paint_cream",
    "walls_residental_paint_creamstripe",
    "walls_residental_paint_offwhite",
    "walls_residental_paint_yellow",
    "walls_residental_paper_blue",
    "walls_residental_tile_blue",
    "walls_residental_tileblue",
    "walls_residental_wood_panel",
    "walls_residental_wood_pannel02",
    "walls_residental_wood_pannel03",
    "walls_residental_wood_red",
    "walls_residental_wood_squares",
    "walls_residental_wood_stripe",
    "walls_residental_wood_white",
    "walls_residential_drkbrwnbase",
    "walls_residential_greydiamondbath",
    "walls_residential_lightgreywainscot",
    "walls_residential_pinebase",
    "walls_residential_rndbluetile",
    "walls_residential_sidingwhite",
    "walls_residential_stuccowoodbrd",
    "walls_residential_stucpanel_orange",
    "walls_residential_taupebasebeige",
    "walls_residential_tilehalfdarkblue",
    "walls_residential_tilehalfsagetop",
    "walls_residential_whiteclapboard",
    "walls_residential_woodhorsiding",
    "walls_residential_woodpanel01",
    "walls_residential_woodshingle",
    "walls_residential_woodshinglesiding",
    "walls_residential_woodslat01",
    "walls_stone_blueorange",
    "walls_stone_white",
    "walls_west_bath",
    "walls_west_bed",
    "walls_west_hotel",
    "walls_west_out01",
    "walls_west_out02",
    "walls_west_rest",
    "walnut_door",
    "white",
    "window_alien_pane_full",
    "window_expensive_full",
    "window_expensive_full_marble",
    "window_four_pane_mix",
    "window_full_pane_shoji",
    "window_porthole",
    "window_western_fake",
    "windows_inside_04",
    "windsor_door",
    "wire_brush_marshmellow",
    "wood01",
    "wood02",
    "wood03",
    "wood_blond",
    "wood_board",
    "wood_bundle",
    "wood_crate",
    "wood_crate_dirty",
    "wood_distressed_01",
    "wood_generic_worn_edges",
    "wood_generic_worn_no_shadows",
    "wood_groovy",
    "wood_groovy2",
    "wood_groovy3",
    "wood_jap",
    "wood_light",
    "wood_pallet",
    "wood_slat01",
    "wood_violin",
    "wrappers",
    "yacht",
    "yacht_details",
    "yacht_filler",
];
