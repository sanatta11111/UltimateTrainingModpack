#![allow(non_snake_case)]
extern crate byteflags;
extern crate num_derive;

use serde::{Deserialize, Serialize};

pub mod options;
pub use options::*;
pub mod files;
pub use files::*;
pub mod config;
pub use config::*;

use training_mod_sync::*;
use training_mod_tui::SubMenuType::*;
pub use training_mod_tui::*;

pub const TOGGLE_MAX: u8 = 5;

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TrainingModpackMenu {
    pub aerial_delay: Delay,
    pub air_dodge_dir: Direction,
    pub attack_angle: AttackAngle,
    pub buff_state: BuffOption,
    pub character_item: CharacterItem,
    pub clatter_strength: ClatterFrequency,
    pub crouch: OnOff,
    pub di_state: Direction,
    pub falling_aerials: BoolFlag,
    pub fast_fall_delay: Delay,
    pub fast_fall: BoolFlag,
    pub follow_up: Action,
    pub frame_advantage: OnOff,
    pub full_hop: BoolFlag,
    pub hitbox_vis: OnOff,
    pub input_display: InputDisplay,
    pub input_display_status: OnOff,
    pub hud: OnOff,
    pub input_delay: Delay,
    pub ledge_delay: LongDelay,
    pub ledge_state: LedgeOption,
    pub mash_state: Action,
    pub mash_triggers: MashTrigger,
    pub miss_tech_state: MissTechFlags,
    pub oos_offset: Delay,
    pub pummel_delay: MedDelay,
    pub reaction_time: Delay,
    pub save_damage_cpu: SaveDamage,
    pub save_damage_limits_cpu: DamagePercent,
    pub save_damage_player: SaveDamage,
    pub save_damage_limits_player: DamagePercent,
    pub save_state_autoload: OnOff,
    pub save_state_enable: OnOff,
    pub save_state_slot: SaveStateSlot,
    pub randomize_slots: SaveStateSlot,
    pub save_state_mirroring: SaveStateMirroring,
    pub save_state_playback: PlaybackSlot,
    pub sdi_state: Direction,
    pub sdi_strength: SdiFrequency,
    pub shield_state: Shield,
    pub shield_tilt: Direction,
    pub stage_hazards: OnOff,
    pub tech_state: TechFlags,
    pub throw_delay: MedDelay,
    pub throw_state: ThrowOption,
    pub ledge_neutral_override: Action,
    pub ledge_roll_override: Action,
    pub ledge_jump_override: Action,
    pub ledge_attack_override: Action,
    pub tech_action_override: Action,
    pub clatter_override: Action,
    pub tumble_override: Action,
    pub hitstun_override: Action,
    pub parry_override: Action,
    pub shieldstun_override: Action,
    pub footstool_override: Action,
    pub landing_override: Action,
    pub trump_override: Action,
    pub recording_slot: RecordSlot,
    pub record_trigger: RecordTrigger,
    pub recording_duration: RecordingDuration,
    pub playback_button_slots: PlaybackSlot,
    pub hitstun_playback: HitstunPlayback,
    pub playback_mash: OnOff,
    pub playback_loop: OnOff,
    pub menu_open_start_press: OnOff,
    pub save_state_save: ButtonConfig,
    pub save_state_load: ButtonConfig,
    pub input_record: ButtonConfig,
    pub input_playback: ButtonConfig,
    pub recording_crop: OnOff,
    pub stale_dodges: OnOff,
    pub tech_hide: OnOff,
    pub update_policy: UpdatePolicy,
    pub lra_reset: OnOff,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuJsonStruct {
    pub menu: TrainingModpackMenu,
    pub defaults_menu: TrainingModpackMenu,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}

pub static BASE_MENU: TrainingModpackMenu = TrainingModpackMenu {
    aerial_delay: Delay::empty(),
    air_dodge_dir: Direction::empty(),
    attack_angle: AttackAngle::empty(),
    buff_state: BuffOption::empty(),
    character_item: CharacterItem::NONE,
    clatter_strength: ClatterFrequency::NONE,
    crouch: OnOff::OFF,
    di_state: Direction::empty(),
    falling_aerials: BoolFlag::FALSE,
    fast_fall_delay: Delay::empty(),
    fast_fall: BoolFlag::FALSE,
    follow_up: Action::empty(),
    frame_advantage: OnOff::OFF,
    full_hop: BoolFlag::TRUE,
    hitbox_vis: OnOff::OFF,
    input_display: InputDisplay::SMASH,
    input_display_status: OnOff::OFF,
    hud: OnOff::ON,
    input_delay: Delay::D0,
    ledge_delay: LongDelay::empty(),
    ledge_state: LedgeOption::default(),
    mash_state: Action::empty(),
    mash_triggers: MashTrigger::default(),
    miss_tech_state: MissTechFlags::all(),
    oos_offset: Delay::empty(),
    pummel_delay: MedDelay::empty(),
    reaction_time: Delay::empty(),
    save_damage_cpu: SaveDamage::DEFAULT,
    save_damage_limits_cpu: DamagePercent::default(),
    save_damage_player: SaveDamage::DEFAULT,
    save_damage_limits_player: DamagePercent::default(),
    save_state_autoload: OnOff::OFF,
    save_state_enable: OnOff::ON,
    save_state_slot: SaveStateSlot::S1,
    randomize_slots: SaveStateSlot::empty(),
    save_state_mirroring: SaveStateMirroring::NONE,
    save_state_playback: PlaybackSlot::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiFrequency::NONE,
    shield_state: Shield::NONE,
    shield_tilt: Direction::empty(),
    stage_hazards: OnOff::OFF,
    tech_state: TechFlags::all(),
    throw_delay: MedDelay::empty(),
    throw_state: ThrowOption::NONE,
    ledge_neutral_override: Action::empty(),
    ledge_roll_override: Action::empty(),
    ledge_jump_override: Action::empty(),
    ledge_attack_override: Action::empty(),
    tech_action_override: Action::empty(),
    clatter_override: Action::empty(),
    tumble_override: Action::empty(),
    hitstun_override: Action::empty(),
    parry_override: Action::empty(),
    shieldstun_override: Action::empty(),
    footstool_override: Action::empty(),
    landing_override: Action::empty(),
    trump_override: Action::empty(),
    recording_slot: RecordSlot::S1,
    recording_duration: RecordingDuration::F150,
    record_trigger: RecordTrigger::COMMAND,
    playback_button_slots: PlaybackSlot::S1,
    hitstun_playback: HitstunPlayback::HITSTUN,
    playback_mash: OnOff::ON,
    playback_loop: OnOff::OFF,
    menu_open_start_press: OnOff::ON,
    save_state_save: ButtonConfig {
        ZL: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    save_state_load: ButtonConfig {
        ZL: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    input_record: ButtonConfig {
        ZR: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    input_playback: ButtonConfig {
        ZR: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    recording_crop: OnOff::ON,
    stale_dodges: OnOff::ON,
    tech_hide: OnOff::OFF,
    update_policy: UpdatePolicy::default(),
    lra_reset: OnOff::ON,
};

pub static DEFAULTS_MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);
pub static MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);

pub unsafe fn create_app<'a>() -> App<'a> {
    let mut overall_menu = App::new();

    // --- 反撃設定 (Mash Tab) ---
    let mut mash_tab_submenus: Vec<SubMenu> = Vec::new();
    mash_tab_submenus.push(Action::to_submenu(
        "反撃行動設定",
        "mash_state",
        "硬直終了後、最速で実行するアクションを選択します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Action::to_submenu(
        "追撃設定",
        "follow_up",
        "反撃行動の後に続けて行うアクションを設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MashTrigger::to_submenu(
        "反撃のきっかけ",
        "mash_triggers",
        "CPUが反撃を開始するタイミング（ガード後、ヒット後など）を設定します",
        ToggleMultiple,
        false,
    ));
    mash_tab_submenus.push(AttackAngle::to_submenu(
        "攻撃シフト入力",
        "attack_angle",
        "強攻撃などの上下シフトを設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(ThrowOption::to_submenu(
        "投げ方向",
        "throw_state",
        "掴んだ後の投げ方向を選択します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "投げ遅延",
        "throw_delay",
        "掴んでから投げるまでのフレーム数を遅らせます",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "つかみ打撃遅延",
        "pummel_delay",
        "掴んでから打撃を入れるまでの待ち時間を設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "空中攻撃降下",
        "falling_aerials",
        "空中攻撃を「昇り」で出すか「降り」で出すか設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "大ジャンプ設定",
        "full_hop",
        "CPUが大ジャンプを行うか小ジャンプを行うかを選択します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "空中攻撃遅延",
        "aerial_delay",
        "空中攻撃を出すタイミングを何フレーム遅らせるか設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "急降下設定",
        "fast_fall",
        "CPUがジャンプ中に急降下を入れるかどうかを設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "急降下遅延",
        "fast_fall_delay",
        "急降下を入れるまでのフレーム数を設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "ガード継続数",
        "oos_offset",
        "何回ガードしてから反撃行動に移るかを設定します",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "反応速度",
        "reaction_time",
        "ガード解除など、反撃に移るまでの猶予フレームを設定します",
        ToggleMultiple,
        true,
    ));
    let mash_tab = Tab {
        id: "mash",
        title: "反撃・コンボ設定",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, mash_tab_submenus),
    };
    overall_menu.tabs.push(mash_tab);

    // --- 状況別上書き (Mash Override Tab) ---
    let mut override_tab_submenus: Vec<SubMenu> = Vec::new();
    override_tab_submenus.push(Action::to_submenu(
        "崖あがり(その場)",
        "ledge_neutral_override",
        "その場あがり後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖あがり(回避)",
        "ledge_roll_override",
        "回避あがり後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖あがり(ジャンプ)",
        "ledge_jump_override",
        "ジャンプあがり後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖あがり(攻撃)",
        "ledge_attack_override",
        "攻撃あがり後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "受け身行動後",
        "tech_action_override",
        "各種受け身をとった後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "もがき・埋まり脱出後",
        "clatter_override",
        "掴みや埋まり状態から脱出した後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "ダウン回復後",
        "tumble_override",
        "ダウン状態から復帰した直後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "吹っ飛び硬直終了後",
        "hitstun_override",
        "吹っ飛び硬直が解けた直後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "ジャストガード後",
        "parry_override",
        "ジャストガード成功後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "シールド硬直終了後",
        "shieldstun_override",
        "シールド硬直解除後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "踏みつけ後",
        "footstool_override",
        "踏みつけられた後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "着地時",
        "landing_override",
        "着地した瞬間の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖奪われ後",
        "trump_override",
        "崖を奪われた後の行動を上書きします",
        ToggleMultiple,
        true,
    ));
    let override_tab = Tab {
        id: "override",
        title: "状況別上書き",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            override_tab_submenus,
        ),
    };
    overall_menu.tabs.push(override_tab);

    // --- 防御設定 (Defensive Tab) ---
    let mut defensive_tab_submenus: Vec<SubMenu> = Vec::new();
    defensive_tab_submenus.push(Direction::to_submenu(
        "空中回避方向",
        "air_dodge_dir",
        "移動空中回避を行う方向を指定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "DI (ベクトル変更)",
        "di_state",
        "吹っ飛び時のベクトル変更方向を指定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "SDI (ずらし)",
        "sdi_state",
        "ヒットストップ中のずらし方向を指定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(SdiFrequency::to_submenu(
        "ずらし強度",
        "sdi_strength",
        "ずらし入力の頻度・強さを設定します",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(ClatterFrequency::to_submenu(
        "レバガチャ強度",
        "clatter_strength",
        "掴みや埋まりからの脱出速度を設定します",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(LedgeOption::to_submenu(
        "崖あがり方法",
        "ledge_state",
        "崖に掴まった際にとる行動を選択します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(LongDelay::to_submenu(
        "崖あがり遅延",
        "ledge_delay",
        "崖をあがるまでの待ち時間を設定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(TechFlags::to_submenu(
        "受け身設定",
        "tech_state",
        "地面や壁に叩きつけられた時の受け身方法を設定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(MissTechFlags::to_submenu(
        "起き上がり設定",
        "miss_tech_state",
        "受け身に失敗した後の起き上がり行動を設定します",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Shield::to_submenu(
        "シールド設定",
        "shield_state",
        "CPUのシールド動作を設定します",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "シールドシフト",
        "shield_tilt",
        "シールドを傾ける方向を指定します",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu(
        "しゃがみ待機",
        "crouch",
        "地上でしゃがみ状態を維持するかどうかを設定します",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu(
        "回避劣化", 
        "stale_dodges", 
        "連続使用による回避の無敵減少を再現するか設定します\n(注意: 本来のゲーム内では不可能なコンボが繋がる場合があります)", 
        ToggleSingle, 
        false
    ));
    defensive_tab_submenus.push(OnOff::to_submenu(
        "受け身演出の非表示", 
        "tech_hide", 
        "反応練習のため、受け身開始から7F以降の演出やエフェクトを非表示にします", 
        ToggleSingle, 
        false
    ));
    let defensive_tab = Tab {
        id: "defensive",
        title: "防御設定",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            defensive_tab_submenus,
        ),
    };
    overall_menu.tabs.push(defensive_tab);

    // --- 入力の記録・再生 (Input Recording Tab) ---
    let mut input_recording_tab_submenus: Vec<SubMenu> = Vec::new();
    input_recording_tab_submenus.push(RecordSlot::to_submenu(
        "記録スロット",
        "recording_slot",
        "操作を記録するスロットを選択します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(RecordTrigger::to_submenu(
        "記録開始トリガー",
        "record_trigger",
        "記録を開始するタイミング（コマンド入力時、またはロード時）を設定します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(RecordingDuration::to_submenu(
        "記録フレーム数",
        "recording_duration",
        "操作を記録する長さをフレーム単位で設定します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "記録末尾のカット",
        "recording_crop",
        "記録終了間際の無入力（ニュートラル）フレームを自動で削除します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "再生用ボタンスロット",
        "playback_button_slots",
        "ボタンコンビネーションで再生するスロットを選択します",
        ToggleMultiple,
        true,
    ));
    input_recording_tab_submenus.push(HitstunPlayback::to_submenu(
        "反撃時の再生タイミング",
        "hitstun_playback",
        "吹っ飛び硬直などから反撃（Mash）する際、どのタイミングで再生を開始するか設定します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "ロード時再生スロット",
        "save_state_playback",
        "保存状態をロードした際に自動再生するスロットを選択します",
        ToggleMultiple,
        true,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "反撃時の再生中断",
        "playback_mash",
        "反撃（Mash）が発生した際、再生中の入力を強制終了します",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "ループ再生",
        "playback_loop",
        "再生終了後、自動的に最初から繰り返し再生します",
        ToggleSingle,
        false,
    ));
    let input_tab = Tab {
        id: "input",
        title: "操作の記録・再生",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            input_recording_tab_submenus,
        ),
    };
    overall_menu.tabs.push(input_tab);

    // --- ボタン設定 (Button Tab) ---
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(OnOff::to_submenu(
        "Startでメニューを開く", 
        "menu_open_start_press", 
        "Start長押しまたはMinusでMODメニューを開きます（通常のポーズはStart短押し）\n※十字キー上保持＋Bは常に使用可能です", 
        ToggleSingle, 
        false
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "保存コマンド",
        "save_state_save",
        "状態を保存するためのボタンの組み合わせを設定します",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "ロードコマンド",
        "save_state_load",
        "保存した状態を読み込むためのボタンの組み合わせを設定します",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "記録開始コマンド",
        "input_record",
        "操作の記録を開始するためのボタンの組み合わせを設定します",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "再生開始コマンド",
        "input_playback",
        "記録した操作を再生するためのボタンの組み合わせを設定します",
        ToggleMultiple,
        false,
    ));
    let button_tab = Tab {
        id: "button",
        title: "ボタン設定",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus,
        ),
    };
    overall_menu.tabs.push(button_tab);

    // --- 保存・ロード設定 (Save State Tab) ---
    let mut save_state_tab_submenus: Vec<SubMenu> = Vec::new();
    save_state_tab_submenus.push(SaveStateMirroring::to_submenu(
        "左右反転ロード",
        "save_state_mirroring",
        "ロード時、ステージ中央を軸にして左右の位置を入れ替えます",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "オートロード",
        "save_state_autoload",
        "撃墜時に自動で保存状態をロードします",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "ダメージ保存 (CPU)",
        "save_damage_cpu",
        "CPUのダメージ値を保存・復元するか設定します",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "ランダムダメージ範囲 (CPU)",
        "save_damage_limits_cpu",
        "ロード時にCPUに付与されるランダムダメージの幅を設定します",
        Slider,
        false,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "ダメージ保存 (プレイヤー)",
        "save_damage_player",
        "プレイヤーのダメージ値を保存・復元するか設定します",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "ランダムダメージ範囲 (プレイヤー)",
        "save_damage_limits_player",
        "ロード時にプレイヤーに付与されるランダムダメージの幅を設定します",
        Slider,
        false,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "保存機能の有効化",
        "save_state_enable",
        "保存・ロード機能を有効にします（初期設定: シールド＋下アピで保存、上アピでロード）",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "保存スロット",
        "save_state_slot",
        "使用する保存スロットを切り替えます",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "ランダムスロット",
        "randomize_slots",
        "ロード時にランダムで選ばれるスロットの候補を選択します",
        ToggleMultiple,
        true,
    ));
    save_state_tab_submenus.push(CharacterItem::to_submenu(
        "所持アイテム",
        "character_item",
        "ロード時にプレイヤーが所持するアイテムを設定します",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(BuffOption::to_submenu(
        "強化状態 (バフ)",
        "buff_state",
        "ロード時に各ファイターへ付与するバフを選択します",
        ToggleMultiple,
        false,
    ));
    let save_state_tab = Tab {
        id: "save_state",
        title: "保存・ロード",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            save_state_tab_submenus,
        ),
    };
    overall_menu.tabs.push(save_state_tab);

    // --- その他設定 (Miscellaneous Tab) ---
    let mut misc_tab_submenus: Vec<SubMenu> = Vec::new();
    misc_tab_submenus.push(OnOff::to_submenu(
        "有利フレーム表示", 
        "frame_advantage", 
        "プレイヤーとCPUの行動可能になるまでの差を表示します\n※CPUが反撃（Mash）設定になっていない場合のみ機能します", 
        ToggleSingle, 
        false
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "攻撃判定の可視化",
        "hitbox_vis",
        "攻撃判定（ヒットボックス）を表示します（一部のエフェクトは隠れます）",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(InputDisplay::to_submenu(
        "キーディスプレイ",
        "input_display",
        "画面左側に操作ログを表示します",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "ログを状態でグループ化",
        "input_display_status",
        "入力ログを発生時のアクション（Status）ごとに整理して表示します",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(Delay::to_submenu(
        "入力遅延 (ラグ再現)",
        "input_delay",
        "プレイヤーの入力に擬似的な遅延を設定します",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "ステージギミック",
        "stage_hazards",
        "ステージギミックの有無を切り替えます",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "HUD表示",
        "hud",
        "％やストック、タイマーなどのUI表示を切り替えます",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(UpdatePolicy::to_submenu(
        "自動アップデート",
        "update_policy",
        "MODの自動更新設定（実機のみ）",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "L+R+Aでリセット",
        "lra_reset",
        "L+R+A同時押しでトレーニングモードをリセットします",
        ToggleSingle,
        false,
    ));
    let misc_tab = Tab {
        id: "misc",
        title: "その他設定",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, misc_tab_submenus),
    };
    overall_menu.tabs.push(misc_tab);

    if overall_menu.tabs.get_selected().is_none() {
        overall_menu.tabs.state.select(Some(0));
    }

    overall_menu
}