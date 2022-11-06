// This file was automatically generated by a Python script.
// It requires the `requests` package to run.

pub enum INPUT {
    PropPointer,
    PropDirect,
    PropButtonpad,
    PropSemiMt,
    PropTopbuttonpad,
    PropPointingStick,
    PropAccelerometer,
    PropMax,
}

pub enum EV {
    Syn,
    Key,
    Rel,
    Abs,
    Msc,
    Sw,
    Led,
    Snd,
    Rep,
    Ff,
    Pwr,
    FfStatus,
    Max,
}

pub enum SYN {
    Report,
    Config,
    MtReport,
    Dropped,
    Max,
}

pub enum KEY {
    Esc,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    Leftbrace,
    Rightbrace,
    Enter,
    Leftctrl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Apostrophe,
    Grave,
    Leftshift,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    Rightshift,
    Kpasterisk,
    Leftalt,
    Space,
    Capslock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    Numlock,
    Scrolllock,
    Kp7,
    Kp8,
    Kp9,
    Kpminus,
    Kp4,
    Kp5,
    Kp6,
    Kpplus,
    Kp1,
    Kp2,
    Kp3,
    Kp0,
    Kpdot,
    Zenkakuhankaku,
    Key102nd,
    F11,
    F12,
    Ro,
    Katakana,
    Hiragana,
    Henkan,
    Katakanahiragana,
    Muhenkan,
    Kpjpcomma,
    Kpenter,
    Rightctrl,
    Kpslash,
    Sysrq,
    Rightalt,
    Linefeed,
    Home,
    Up,
    Pageup,
    Left,
    Right,
    End,
    Down,
    Pagedown,
    Insert,
    Delete,
    Macro,
    Mute,
    Volumedown,
    Volumeup,
    Power,
    Kpequal,
    Kpplusminus,
    Pause,
    Scale,
    Kpcomma,
    Hangeul,
    Hanja,
    Yen,
    Leftmeta,
    Rightmeta,
    Compose,
    Stop,
    Again,
    Props,
    Undo,
    Front,
    Copy,
    Open,
    Paste,
    Find,
    Cut,
    Help,
    Menu,
    Calc,
    Setup,
    Sleep,
    Wakeup,
    File,
    Sendfile,
    Deletefile,
    Xfer,
    Prog1,
    Prog2,
    Www,
    Msdos,
    Coffee,
    RotateDisplay,
    Cyclewindows,
    Mail,
    Bookmarks,
    Computer,
    Back,
    Forward,
    Closecd,
    Ejectcd,
    Ejectclosecd,
    Nextsong,
    Playpause,
    Previoussong,
    Stopcd,
    Record,
    Rewind,
    Phone,
    Iso,
    Config,
    Homepage,
    Refresh,
    Exit,
    Move,
    Edit,
    Scrollup,
    Scrolldown,
    Kpleftparen,
    Kprightparen,
    New,
    Redo,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Playcd,
    Pausecd,
    Prog3,
    Prog4,
    Dashboard,
    Suspend,
    Close,
    Play,
    Fastforward,
    Bassboost,
    Print,
    Hp,
    Camera,
    Sound,
    Question,
    Email,
    Chat,
    Search,
    Connect,
    Finance,
    Sport,
    Shop,
    Alterase,
    Cancel,
    Brightnessdown,
    Brightnessup,
    Media,
    Switchvideomode,
    Kbdillumtoggle,
    Kbdillumdown,
    Kbdillumup,
    Send,
    Reply,
    Forwardmail,
    Save,
    Documents,
    Battery,
    Bluetooth,
    Wlan,
    Uwb,
    Unknown,
    VideoNext,
    VideoPrev,
    BrightnessCycle,
    BrightnessAuto,
    DisplayOff,
    Wwan,
    Rfkill,
    Micmute,
    Ok,
    Select,
    Goto,
    Clear,
    Power2,
    Option,
    Info,
    Time,
    Vendor,
    Archive,
    Program,
    Channel,
    Favorites,
    Epg,
    Pvr,
    Mhp,
    Language,
    Title,
    Subtitle,
    Angle,
    FullScreen,
    Mode,
    Keyboard,
    AspectRatio,
    Pc,
    Tv,
    Tv2,
    Vcr,
    Vcr2,
    Sat,
    Sat2,
    Cd,
    Tape,
    Radio,
    Tuner,
    Player,
    Text,
    Dvd,
    Aux,
    Mp3,
    Audio,
    Video,
    Directory,
    List,
    Memo,
    Calendar,
    Red,
    Green,
    Yellow,
    Blue,
    Channelup,
    Channeldown,
    First,
    Last,
    Ab,
    Next,
    Restart,
    Slow,
    Shuffle,
    Break,
    Previous,
    Digits,
    Teen,
    Twen,
    Videophone,
    Games,
    Zoomin,
    Zoomout,
    Zoomreset,
    Wordprocessor,
    Editor,
    Spreadsheet,
    Graphicseditor,
    Presentation,
    Database,
    News,
    Voicemail,
    Addressbook,
    Messenger,
    Displaytoggle,
    Spellcheck,
    Logoff,
    Dollar,
    Euro,
    Frameback,
    Frameforward,
    ContextMenu,
    MediaRepeat,
    Key10channelsup,
    Key10channelsdown,
    Images,
    DelEol,
    DelEos,
    InsLine,
    DelLine,
    Fn,
    FnEsc,
    FnF1,
    FnF2,
    FnF3,
    FnF4,
    FnF5,
    FnF6,
    FnF7,
    FnF8,
    FnF9,
    FnF10,
    FnF11,
    FnF12,
    Fn1,
    Fn2,
    FnD,
    FnE,
    FnF,
    FnS,
    FnB,
    BrlDot1,
    BrlDot2,
    BrlDot3,
    BrlDot4,
    BrlDot5,
    BrlDot6,
    BrlDot7,
    BrlDot8,
    BrlDot9,
    BrlDot10,
    Numeric0,
    Numeric1,
    Numeric2,
    Numeric3,
    Numeric4,
    Numeric5,
    Numeric6,
    Numeric7,
    Numeric8,
    Numeric9,
    NumericStar,
    NumericPound,
    NumericA,
    NumericB,
    NumericC,
    NumericD,
    CameraFocus,
    WpsButton,
    TouchpadToggle,
    TouchpadOn,
    TouchpadOff,
    CameraZoomin,
    CameraZoomout,
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    AttendantOn,
    AttendantOff,
    AttendantToggle,
    LightsToggle,
    AlsToggle,
    RotateLockToggle,
    Buttonconfig,
    Taskmanager,
    Journal,
    Controlpanel,
    Appselect,
    Screensaver,
    Voicecommand,
    Assistant,
    KbdLayoutNext,
    BrightnessMin,
    BrightnessMax,
    KbdinputassistPrev,
    KbdinputassistNext,
    KbdinputassistPrevgroup,
    KbdinputassistNextgroup,
    KbdinputassistAccept,
    KbdinputassistCancel,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
    RootMenu,
    MediaTopMenu,
    Numeric11,
    Numeric12,
    AudioDesc,
    Key3dMode,
    NextFavorite,
    StopRecord,
    PauseRecord,
    Vod,
    Unmute,
    Fastreverse,
    Slowreverse,
    Data,
    OnscreenKeyboard,
    PrivacyScreenToggle,
    SelectiveScreenshot,
    Macro1,
    Macro2,
    Macro3,
    Macro4,
    Macro5,
    Macro6,
    Macro7,
    Macro8,
    Macro9,
    Macro10,
    Macro11,
    Macro12,
    Macro13,
    Macro14,
    Macro15,
    Macro16,
    Macro17,
    Macro18,
    Macro19,
    Macro20,
    Macro21,
    Macro22,
    Macro23,
    Macro24,
    Macro25,
    Macro26,
    Macro27,
    Macro28,
    Macro29,
    Macro30,
    MacroRecordStart,
    MacroRecordStop,
    MacroPresetCycle,
    MacroPreset1,
    MacroPreset2,
    MacroPreset3,
    KbdLcdMenu1,
    KbdLcdMenu2,
    KbdLcdMenu3,
    KbdLcdMenu4,
    KbdLcdMenu5,
    Max,
}

pub enum BTN {
    Btn0,
    Btn1,
    Btn2,
    Btn3,
    Btn4,
    Btn5,
    Btn6,
    Btn7,
    Btn8,
    Btn9,
    Left,
    Right,
    Middle,
    Side,
    Extra,
    Forward,
    Back,
    Task,
    Trigger,
    Thumb,
    Thumb2,
    Top,
    Top2,
    Pinkie,
    Base,
    Base2,
    Base3,
    Base4,
    Base5,
    Base6,
    Dead,
    South,
    East,
    C,
    North,
    West,
    Z,
    Tl,
    Tr,
    Tl2,
    Tr2,
    Select,
    Start,
    Mode,
    Thumbl,
    Thumbr,
    ToolPen,
    ToolRubber,
    ToolBrush,
    ToolPencil,
    ToolAirbrush,
    ToolFinger,
    ToolMouse,
    ToolLens,
    ToolQuinttap,
    Stylus3,
    Touch,
    Stylus,
    Stylus2,
    ToolDoubletap,
    ToolTripletap,
    ToolQuadtap,
    GearDown,
    GearUp,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    TriggerHappy1,
    TriggerHappy2,
    TriggerHappy3,
    TriggerHappy4,
    TriggerHappy5,
    TriggerHappy6,
    TriggerHappy7,
    TriggerHappy8,
    TriggerHappy9,
    TriggerHappy10,
    TriggerHappy11,
    TriggerHappy12,
    TriggerHappy13,
    TriggerHappy14,
    TriggerHappy15,
    TriggerHappy16,
    TriggerHappy17,
    TriggerHappy18,
    TriggerHappy19,
    TriggerHappy20,
    TriggerHappy21,
    TriggerHappy22,
    TriggerHappy23,
    TriggerHappy24,
    TriggerHappy25,
    TriggerHappy26,
    TriggerHappy27,
    TriggerHappy28,
    TriggerHappy29,
    TriggerHappy30,
    TriggerHappy31,
    TriggerHappy32,
    TriggerHappy33,
    TriggerHappy34,
    TriggerHappy35,
    TriggerHappy36,
    TriggerHappy37,
    TriggerHappy38,
    TriggerHappy39,
    TriggerHappy40,
}

pub enum REL {
    X,
    Y,
    Z,
    Rx,
    Ry,
    Rz,
    Hwheel,
    Dial,
    Wheel,
    Misc,
    WheelHiRes,
    HwheelHiRes,
    Max,
}

pub enum ABS {
    X,
    Y,
    Z,
    Rx,
    Ry,
    Rz,
    Throttle,
    Rudder,
    Wheel,
    Gas,
    Brake,
    Hat0x,
    Hat0y,
    Hat1x,
    Hat1y,
    Hat2x,
    Hat2y,
    Hat3x,
    Hat3y,
    Pressure,
    Distance,
    TiltX,
    TiltY,
    ToolWidth,
    Volume,
    Misc,
    MtSlot,
    MtTouchMajor,
    MtTouchMinor,
    MtWidthMajor,
    MtWidthMinor,
    MtOrientation,
    MtPositionX,
    MtPositionY,
    MtToolType,
    MtBlobId,
    MtTrackingId,
    MtPressure,
    MtDistance,
    MtToolX,
    MtToolY,
    Max,
}

pub enum SW {
    Lid,
    TabletMode,
    HeadphoneInsert,
    RfkillAll,
    MicrophoneInsert,
    Dock,
    LineoutInsert,
    JackPhysicalInsert,
    VideooutInsert,
    CameraLensCover,
    KeypadSlide,
    FrontProximity,
    RotateLock,
    LineinInsert,
    MuteDevice,
    PenInserted,
    Max,
}

pub enum MSC {
    Serial,
    Pulseled,
    Gesture,
    Raw,
    Scan,
    Timestamp,
    Max,
}

pub enum LED {
    Numl,
    Capsl,
    Scrolll,
    Compose,
    Kana,
    Sleep,
    Suspend,
    Mute,
    Misc,
    Mail,
    Charging,
    Max,
}

pub enum REP {
    Delay,
    Max,
}

pub enum SND {
    Click,
    Bell,
    Tone,
    Max,
}

