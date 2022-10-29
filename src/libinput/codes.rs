// This file was automatically generated by a Python script.

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static!{
    static ref INPUT_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "PROP_POINTER"),
        (1, "PROP_DIRECT"),
        (2, "PROP_BUTTONPAD"),
        (3, "PROP_SEMI_MT"),
        (4, "PROP_TOPBUTTONPAD"),
        (5, "PROP_POINTING_STICK"),
        (6, "PROP_ACCELEROMETER"),
        (31, "PROP_MAX"),
    ]);
    static ref EV_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "SYN"),
        (1, "KEY"),
        (2, "REL"),
        (3, "ABS"),
        (4, "MSC"),
        (5, "SW"),
        (17, "LED"),
        (18, "SND"),
        (20, "REP"),
        (21, "FF"),
        (22, "PWR"),
        (23, "FF_STATUS"),
        (31, "MAX"),
    ]);
    static ref SYN_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "REPORT"),
        (1, "CONFIG"),
        (2, "MT_REPORT"),
        (3, "DROPPED"),
        (15, "MAX"),
    ]);
    static ref KEY_DICT: HashMap<u16, &'static str> = HashMap::from([
        (1, "ESC"),
        (2, "1"),
        (3, "2"),
        (4, "3"),
        (5, "4"),
        (6, "5"),
        (7, "6"),
        (8, "7"),
        (9, "8"),
        (10, "9"),
        (11, "0"),
        (12, "MINUS"),
        (13, "EQUAL"),
        (14, "BACKSPACE"),
        (15, "TAB"),
        (16, "Q"),
        (17, "W"),
        (18, "E"),
        (19, "R"),
        (20, "T"),
        (21, "Y"),
        (22, "U"),
        (23, "I"),
        (24, "O"),
        (25, "P"),
        (26, "LEFTBRACE"),
        (27, "RIGHTBRACE"),
        (28, "ENTER"),
        (29, "LEFTCTRL"),
        (30, "A"),
        (31, "S"),
        (32, "D"),
        (33, "F"),
        (34, "G"),
        (35, "H"),
        (36, "J"),
        (37, "K"),
        (38, "L"),
        (39, "SEMICOLON"),
        (40, "APOSTROPHE"),
        (41, "GRAVE"),
        (42, "LEFTSHIFT"),
        (43, "BACKSLASH"),
        (44, "Z"),
        (45, "X"),
        (46, "C"),
        (47, "V"),
        (48, "B"),
        (49, "N"),
        (50, "M"),
        (51, "COMMA"),
        (52, "DOT"),
        (53, "SLASH"),
        (54, "RIGHTSHIFT"),
        (55, "KPASTERISK"),
        (56, "LEFTALT"),
        (57, "SPACE"),
        (58, "CAPSLOCK"),
        (59, "F1"),
        (60, "F2"),
        (61, "F3"),
        (62, "F4"),
        (63, "F5"),
        (64, "F6"),
        (65, "F7"),
        (66, "F8"),
        (67, "F9"),
        (68, "F10"),
        (69, "NUMLOCK"),
        (70, "SCROLLLOCK"),
        (71, "KP7"),
        (72, "KP8"),
        (73, "KP9"),
        (74, "KPMINUS"),
        (75, "KP4"),
        (76, "KP5"),
        (77, "KP6"),
        (78, "KPPLUS"),
        (79, "KP1"),
        (80, "KP2"),
        (81, "KP3"),
        (82, "KP0"),
        (83, "KPDOT"),
        (85, "ZENKAKUHANKAKU"),
        (86, "102ND"),
        (87, "F11"),
        (88, "F12"),
        (89, "RO"),
        (90, "KATAKANA"),
        (91, "HIRAGANA"),
        (92, "HENKAN"),
        (93, "KATAKANAHIRAGANA"),
        (94, "MUHENKAN"),
        (95, "KPJPCOMMA"),
        (96, "KPENTER"),
        (97, "RIGHTCTRL"),
        (98, "KPSLASH"),
        (99, "SYSRQ"),
        (100, "RIGHTALT"),
        (101, "LINEFEED"),
        (102, "HOME"),
        (103, "UP"),
        (104, "PAGEUP"),
        (105, "LEFT"),
        (106, "RIGHT"),
        (107, "END"),
        (108, "DOWN"),
        (109, "PAGEDOWN"),
        (110, "INSERT"),
        (111, "DELETE"),
        (112, "MACRO"),
        (113, "MUTE"),
        (114, "VOLUMEDOWN"),
        (115, "VOLUMEUP"),
        (116, "POWER"),
        (117, "KPEQUAL"),
        (118, "KPPLUSMINUS"),
        (119, "PAUSE"),
        (120, "SCALE"),
        (121, "KPCOMMA"),
        (122, "HANGEUL"),
        (123, "HANJA"),
        (124, "YEN"),
        (125, "LEFTMETA"),
        (126, "RIGHTMETA"),
        (127, "COMPOSE"),
        (128, "STOP"),
        (129, "AGAIN"),
        (130, "PROPS"),
        (131, "UNDO"),
        (132, "FRONT"),
        (133, "COPY"),
        (134, "OPEN"),
        (135, "PASTE"),
        (136, "FIND"),
        (137, "CUT"),
        (138, "HELP"),
        (139, "MENU"),
        (140, "CALC"),
        (141, "SETUP"),
        (142, "SLEEP"),
        (143, "WAKEUP"),
        (144, "FILE"),
        (145, "SENDFILE"),
        (146, "DELETEFILE"),
        (147, "XFER"),
        (148, "PROG1"),
        (149, "PROG2"),
        (150, "WWW"),
        (151, "MSDOS"),
        (152, "COFFEE"),
        (153, "ROTATE_DISPLAY"),
        (154, "CYCLEWINDOWS"),
        (155, "MAIL"),
        (156, "BOOKMARKS"),
        (157, "COMPUTER"),
        (158, "BACK"),
        (159, "FORWARD"),
        (160, "CLOSECD"),
        (161, "EJECTCD"),
        (162, "EJECTCLOSECD"),
        (163, "NEXTSONG"),
        (164, "PLAYPAUSE"),
        (165, "PREVIOUSSONG"),
        (166, "STOPCD"),
        (167, "RECORD"),
        (168, "REWIND"),
        (169, "PHONE"),
        (170, "ISO"),
        (171, "CONFIG"),
        (172, "HOMEPAGE"),
        (173, "REFRESH"),
        (174, "EXIT"),
        (175, "MOVE"),
        (176, "EDIT"),
        (177, "SCROLLUP"),
        (178, "SCROLLDOWN"),
        (179, "KPLEFTPAREN"),
        (180, "KPRIGHTPAREN"),
        (181, "NEW"),
        (182, "REDO"),
        (183, "F13"),
        (184, "F14"),
        (185, "F15"),
        (186, "F16"),
        (187, "F17"),
        (188, "F18"),
        (189, "F19"),
        (190, "F20"),
        (191, "F21"),
        (192, "F22"),
        (193, "F23"),
        (194, "F24"),
        (200, "PLAYCD"),
        (201, "PAUSECD"),
        (202, "PROG3"),
        (203, "PROG4"),
        (204, "DASHBOARD"),
        (205, "SUSPEND"),
        (206, "CLOSE"),
        (207, "PLAY"),
        (208, "FASTFORWARD"),
        (209, "BASSBOOST"),
        (210, "PRINT"),
        (211, "HP"),
        (212, "CAMERA"),
        (213, "SOUND"),
        (214, "QUESTION"),
        (215, "EMAIL"),
        (216, "CHAT"),
        (217, "SEARCH"),
        (218, "CONNECT"),
        (219, "FINANCE"),
        (220, "SPORT"),
        (221, "SHOP"),
        (222, "ALTERASE"),
        (223, "CANCEL"),
        (224, "BRIGHTNESSDOWN"),
        (225, "BRIGHTNESSUP"),
        (226, "MEDIA"),
        (227, "SWITCHVIDEOMODE"),
        (228, "KBDILLUMTOGGLE"),
        (229, "KBDILLUMDOWN"),
        (230, "KBDILLUMUP"),
        (231, "SEND"),
        (232, "REPLY"),
        (233, "FORWARDMAIL"),
        (234, "SAVE"),
        (235, "DOCUMENTS"),
        (236, "BATTERY"),
        (237, "BLUETOOTH"),
        (238, "WLAN"),
        (239, "UWB"),
        (240, "UNKNOWN"),
        (241, "VIDEO_NEXT"),
        (242, "VIDEO_PREV"),
        (243, "BRIGHTNESS_CYCLE"),
        (244, "BRIGHTNESS_AUTO"),
        (245, "DISPLAY_OFF"),
        (246, "WWAN"),
        (247, "RFKILL"),
        (248, "MICMUTE"),
        (352, "OK"),
        (353, "SELECT"),
        (354, "GOTO"),
        (355, "CLEAR"),
        (356, "POWER2"),
        (357, "OPTION"),
        (358, "INFO"),
        (359, "TIME"),
        (360, "VENDOR"),
        (361, "ARCHIVE"),
        (362, "PROGRAM"),
        (363, "CHANNEL"),
        (364, "FAVORITES"),
        (365, "EPG"),
        (366, "PVR"),
        (367, "MHP"),
        (368, "LANGUAGE"),
        (369, "TITLE"),
        (370, "SUBTITLE"),
        (371, "ANGLE"),
        (372, "FULL_SCREEN"),
        (373, "MODE"),
        (374, "KEYBOARD"),
        (375, "ASPECT_RATIO"),
        (376, "PC"),
        (377, "TV"),
        (378, "TV2"),
        (379, "VCR"),
        (380, "VCR2"),
        (381, "SAT"),
        (382, "SAT2"),
        (383, "CD"),
        (384, "TAPE"),
        (385, "RADIO"),
        (386, "TUNER"),
        (387, "PLAYER"),
        (388, "TEXT"),
        (389, "DVD"),
        (390, "AUX"),
        (391, "MP3"),
        (392, "AUDIO"),
        (393, "VIDEO"),
        (394, "DIRECTORY"),
        (395, "LIST"),
        (396, "MEMO"),
        (397, "CALENDAR"),
        (398, "RED"),
        (399, "GREEN"),
        (400, "YELLOW"),
        (401, "BLUE"),
        (402, "CHANNELUP"),
        (403, "CHANNELDOWN"),
        (404, "FIRST"),
        (405, "LAST"),
        (406, "AB"),
        (407, "NEXT"),
        (408, "RESTART"),
        (409, "SLOW"),
        (410, "SHUFFLE"),
        (411, "BREAK"),
        (412, "PREVIOUS"),
        (413, "DIGITS"),
        (414, "TEEN"),
        (415, "TWEN"),
        (416, "VIDEOPHONE"),
        (417, "GAMES"),
        (418, "ZOOMIN"),
        (419, "ZOOMOUT"),
        (420, "ZOOMRESET"),
        (421, "WORDPROCESSOR"),
        (422, "EDITOR"),
        (423, "SPREADSHEET"),
        (424, "GRAPHICSEDITOR"),
        (425, "PRESENTATION"),
        (426, "DATABASE"),
        (427, "NEWS"),
        (428, "VOICEMAIL"),
        (429, "ADDRESSBOOK"),
        (430, "MESSENGER"),
        (431, "DISPLAYTOGGLE"),
        (432, "SPELLCHECK"),
        (433, "LOGOFF"),
        (434, "DOLLAR"),
        (435, "EURO"),
        (436, "FRAMEBACK"),
        (437, "FRAMEFORWARD"),
        (438, "CONTEXT_MENU"),
        (439, "MEDIA_REPEAT"),
        (440, "10CHANNELSUP"),
        (441, "10CHANNELSDOWN"),
        (442, "IMAGES"),
        (448, "DEL_EOL"),
        (449, "DEL_EOS"),
        (450, "INS_LINE"),
        (451, "DEL_LINE"),
        (464, "FN"),
        (465, "FN_ESC"),
        (466, "FN_F1"),
        (467, "FN_F2"),
        (468, "FN_F3"),
        (469, "FN_F4"),
        (470, "FN_F5"),
        (471, "FN_F6"),
        (472, "FN_F7"),
        (473, "FN_F8"),
        (474, "FN_F9"),
        (475, "FN_F10"),
        (476, "FN_F11"),
        (477, "FN_F12"),
        (478, "FN_1"),
        (479, "FN_2"),
        (480, "FN_D"),
        (481, "FN_E"),
        (482, "FN_F"),
        (483, "FN_S"),
        (484, "FN_B"),
        (497, "BRL_DOT1"),
        (498, "BRL_DOT2"),
        (499, "BRL_DOT3"),
        (500, "BRL_DOT4"),
        (501, "BRL_DOT5"),
        (502, "BRL_DOT6"),
        (503, "BRL_DOT7"),
        (504, "BRL_DOT8"),
        (505, "BRL_DOT9"),
        (506, "BRL_DOT10"),
        (512, "NUMERIC_0"),
        (513, "NUMERIC_1"),
        (514, "NUMERIC_2"),
        (515, "NUMERIC_3"),
        (516, "NUMERIC_4"),
        (517, "NUMERIC_5"),
        (518, "NUMERIC_6"),
        (519, "NUMERIC_7"),
        (520, "NUMERIC_8"),
        (521, "NUMERIC_9"),
        (522, "NUMERIC_STAR"),
        (523, "NUMERIC_POUND"),
        (524, "NUMERIC_A"),
        (525, "NUMERIC_B"),
        (526, "NUMERIC_C"),
        (527, "NUMERIC_D"),
        (528, "CAMERA_FOCUS"),
        (529, "WPS_BUTTON"),
        (530, "TOUCHPAD_TOGGLE"),
        (531, "TOUCHPAD_ON"),
        (532, "TOUCHPAD_OFF"),
        (533, "CAMERA_ZOOMIN"),
        (534, "CAMERA_ZOOMOUT"),
        (535, "CAMERA_UP"),
        (536, "CAMERA_DOWN"),
        (537, "CAMERA_LEFT"),
        (538, "CAMERA_RIGHT"),
        (539, "ATTENDANT_ON"),
        (540, "ATTENDANT_OFF"),
        (541, "ATTENDANT_TOGGLE"),
        (542, "LIGHTS_TOGGLE"),
        (560, "ALS_TOGGLE"),
        (561, "ROTATE_LOCK_TOGGLE"),
        (576, "BUTTONCONFIG"),
        (577, "TASKMANAGER"),
        (578, "JOURNAL"),
        (579, "CONTROLPANEL"),
        (580, "APPSELECT"),
        (581, "SCREENSAVER"),
        (582, "VOICECOMMAND"),
        (583, "ASSISTANT"),
        (584, "KBD_LAYOUT_NEXT"),
        (592, "BRIGHTNESS_MIN"),
        (593, "BRIGHTNESS_MAX"),
        (608, "KBDINPUTASSIST_PREV"),
        (609, "KBDINPUTASSIST_NEXT"),
        (610, "KBDINPUTASSIST_PREVGROUP"),
        (611, "KBDINPUTASSIST_NEXTGROUP"),
        (612, "KBDINPUTASSIST_ACCEPT"),
        (613, "KBDINPUTASSIST_CANCEL"),
        (614, "RIGHT_UP"),
        (615, "RIGHT_DOWN"),
        (616, "LEFT_UP"),
        (617, "LEFT_DOWN"),
        (618, "ROOT_MENU"),
        (619, "MEDIA_TOP_MENU"),
        (620, "NUMERIC_11"),
        (621, "NUMERIC_12"),
        (622, "AUDIO_DESC"),
        (623, "3D_MODE"),
        (624, "NEXT_FAVORITE"),
        (625, "STOP_RECORD"),
        (626, "PAUSE_RECORD"),
        (627, "VOD"),
        (628, "UNMUTE"),
        (629, "FASTREVERSE"),
        (630, "SLOWREVERSE"),
        (631, "DATA"),
        (632, "ONSCREEN_KEYBOARD"),
        (633, "PRIVACY_SCREEN_TOGGLE"),
        (634, "SELECTIVE_SCREENSHOT"),
        (656, "MACRO1"),
        (657, "MACRO2"),
        (658, "MACRO3"),
        (659, "MACRO4"),
        (660, "MACRO5"),
        (661, "MACRO6"),
        (662, "MACRO7"),
        (663, "MACRO8"),
        (664, "MACRO9"),
        (665, "MACRO10"),
        (666, "MACRO11"),
        (667, "MACRO12"),
        (668, "MACRO13"),
        (669, "MACRO14"),
        (670, "MACRO15"),
        (671, "MACRO16"),
        (672, "MACRO17"),
        (673, "MACRO18"),
        (674, "MACRO19"),
        (675, "MACRO20"),
        (676, "MACRO21"),
        (677, "MACRO22"),
        (678, "MACRO23"),
        (679, "MACRO24"),
        (680, "MACRO25"),
        (681, "MACRO26"),
        (682, "MACRO27"),
        (683, "MACRO28"),
        (684, "MACRO29"),
        (685, "MACRO30"),
        (688, "MACRO_RECORD_START"),
        (689, "MACRO_RECORD_STOP"),
        (690, "MACRO_PRESET_CYCLE"),
        (691, "MACRO_PRESET1"),
        (692, "MACRO_PRESET2"),
        (693, "MACRO_PRESET3"),
        (696, "KBD_LCD_MENU1"),
        (697, "KBD_LCD_MENU2"),
        (698, "KBD_LCD_MENU3"),
        (699, "KBD_LCD_MENU4"),
        (700, "KBD_LCD_MENU5"),
        (767, "MAX"),
    ]);
    static ref BTN_DICT: HashMap<u16, &'static str> = HashMap::from([
        (256, "0"),
        (257, "1"),
        (258, "2"),
        (259, "3"),
        (260, "4"),
        (261, "5"),
        (262, "6"),
        (263, "7"),
        (264, "8"),
        (265, "9"),
        (272, "LEFT"),
        (273, "RIGHT"),
        (274, "MIDDLE"),
        (275, "SIDE"),
        (276, "EXTRA"),
        (277, "FORWARD"),
        (278, "BACK"),
        (279, "TASK"),
        (288, "TRIGGER"),
        (289, "THUMB"),
        (290, "THUMB2"),
        (291, "TOP"),
        (292, "TOP2"),
        (293, "PINKIE"),
        (294, "BASE"),
        (295, "BASE2"),
        (296, "BASE3"),
        (297, "BASE4"),
        (298, "BASE5"),
        (299, "BASE6"),
        (303, "DEAD"),
        (304, "SOUTH"),
        (305, "EAST"),
        (306, "C"),
        (307, "NORTH"),
        (308, "WEST"),
        (309, "Z"),
        (310, "TL"),
        (311, "TR"),
        (312, "TL2"),
        (313, "TR2"),
        (314, "SELECT"),
        (315, "START"),
        (316, "MODE"),
        (317, "THUMBL"),
        (318, "THUMBR"),
        (320, "TOOL_PEN"),
        (321, "TOOL_RUBBER"),
        (322, "TOOL_BRUSH"),
        (323, "TOOL_PENCIL"),
        (324, "TOOL_AIRBRUSH"),
        (325, "TOOL_FINGER"),
        (326, "TOOL_MOUSE"),
        (327, "TOOL_LENS"),
        (328, "TOOL_QUINTTAP"),
        (329, "STYLUS3"),
        (330, "TOUCH"),
        (331, "STYLUS"),
        (332, "STYLUS2"),
        (333, "TOOL_DOUBLETAP"),
        (334, "TOOL_TRIPLETAP"),
        (335, "TOOL_QUADTAP"),
        (336, "GEAR_DOWN"),
        (337, "GEAR_UP"),
        (544, "DPAD_UP"),
        (545, "DPAD_DOWN"),
        (546, "DPAD_LEFT"),
        (547, "DPAD_RIGHT"),
        (704, "TRIGGER_HAPPY1"),
        (705, "TRIGGER_HAPPY2"),
        (706, "TRIGGER_HAPPY3"),
        (707, "TRIGGER_HAPPY4"),
        (708, "TRIGGER_HAPPY5"),
        (709, "TRIGGER_HAPPY6"),
        (710, "TRIGGER_HAPPY7"),
        (711, "TRIGGER_HAPPY8"),
        (712, "TRIGGER_HAPPY9"),
        (713, "TRIGGER_HAPPY10"),
        (714, "TRIGGER_HAPPY11"),
        (715, "TRIGGER_HAPPY12"),
        (716, "TRIGGER_HAPPY13"),
        (717, "TRIGGER_HAPPY14"),
        (718, "TRIGGER_HAPPY15"),
        (719, "TRIGGER_HAPPY16"),
        (720, "TRIGGER_HAPPY17"),
        (721, "TRIGGER_HAPPY18"),
        (722, "TRIGGER_HAPPY19"),
        (723, "TRIGGER_HAPPY20"),
        (724, "TRIGGER_HAPPY21"),
        (725, "TRIGGER_HAPPY22"),
        (726, "TRIGGER_HAPPY23"),
        (727, "TRIGGER_HAPPY24"),
        (728, "TRIGGER_HAPPY25"),
        (729, "TRIGGER_HAPPY26"),
        (730, "TRIGGER_HAPPY27"),
        (731, "TRIGGER_HAPPY28"),
        (732, "TRIGGER_HAPPY29"),
        (733, "TRIGGER_HAPPY30"),
        (734, "TRIGGER_HAPPY31"),
        (735, "TRIGGER_HAPPY32"),
        (736, "TRIGGER_HAPPY33"),
        (737, "TRIGGER_HAPPY34"),
        (738, "TRIGGER_HAPPY35"),
        (739, "TRIGGER_HAPPY36"),
        (740, "TRIGGER_HAPPY37"),
        (741, "TRIGGER_HAPPY38"),
        (742, "TRIGGER_HAPPY39"),
        (743, "TRIGGER_HAPPY40"),
    ]);
    static ref REL_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "X"),
        (1, "Y"),
        (2, "Z"),
        (3, "RX"),
        (4, "RY"),
        (5, "RZ"),
        (6, "HWHEEL"),
        (7, "DIAL"),
        (8, "WHEEL"),
        (9, "MISC"),
        (11, "WHEEL_HI_RES"),
        (12, "HWHEEL_HI_RES"),
        (15, "MAX"),
    ]);
    static ref ABS_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "X"),
        (1, "Y"),
        (2, "Z"),
        (3, "RX"),
        (4, "RY"),
        (5, "RZ"),
        (6, "THROTTLE"),
        (7, "RUDDER"),
        (8, "WHEEL"),
        (9, "GAS"),
        (10, "BRAKE"),
        (16, "HAT0X"),
        (17, "HAT0Y"),
        (18, "HAT1X"),
        (19, "HAT1Y"),
        (20, "HAT2X"),
        (21, "HAT2Y"),
        (22, "HAT3X"),
        (23, "HAT3Y"),
        (24, "PRESSURE"),
        (25, "DISTANCE"),
        (26, "TILT_X"),
        (27, "TILT_Y"),
        (28, "TOOL_WIDTH"),
        (32, "VOLUME"),
        (40, "MISC"),
        (47, "MT_SLOT"),
        (48, "MT_TOUCH_MAJOR"),
        (49, "MT_TOUCH_MINOR"),
        (50, "MT_WIDTH_MAJOR"),
        (51, "MT_WIDTH_MINOR"),
        (52, "MT_ORIENTATION"),
        (53, "MT_POSITION_X"),
        (54, "MT_POSITION_Y"),
        (55, "MT_TOOL_TYPE"),
        (56, "MT_BLOB_ID"),
        (57, "MT_TRACKING_ID"),
        (58, "MT_PRESSURE"),
        (59, "MT_DISTANCE"),
        (60, "MT_TOOL_X"),
        (61, "MT_TOOL_Y"),
        (63, "MAX"),
    ]);
    static ref SW_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "LID"),
        (1, "TABLET_MODE"),
        (2, "HEADPHONE_INSERT"),
        (3, "RFKILL_ALL"),
        (4, "MICROPHONE_INSERT"),
        (5, "DOCK"),
        (6, "LINEOUT_INSERT"),
        (7, "JACK_PHYSICAL_INSERT"),
        (8, "VIDEOOUT_INSERT"),
        (9, "CAMERA_LENS_COVER"),
        (10, "KEYPAD_SLIDE"),
        (11, "FRONT_PROXIMITY"),
        (12, "ROTATE_LOCK"),
        (13, "LINEIN_INSERT"),
        (14, "MUTE_DEVICE"),
        (15, "PEN_INSERTED"),
        (16, "MAX"),
    ]);
    static ref MSC_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "SERIAL"),
        (1, "PULSELED"),
        (2, "GESTURE"),
        (3, "RAW"),
        (4, "SCAN"),
        (5, "TIMESTAMP"),
        (7, "MAX"),
    ]);
    static ref LED_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "NUML"),
        (1, "CAPSL"),
        (2, "SCROLLL"),
        (3, "COMPOSE"),
        (4, "KANA"),
        (5, "SLEEP"),
        (6, "SUSPEND"),
        (7, "MUTE"),
        (8, "MISC"),
        (9, "MAIL"),
        (10, "CHARGING"),
        (15, "MAX"),
    ]);
    static ref REP_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "DELAY"),
        (1, "MAX"),
    ]);
    static ref SND_DICT: HashMap<u16, &'static str> = HashMap::from([
        (0, "CLICK"),
        (1, "BELL"),
        (2, "TONE"),
        (7, "MAX"),
    ]);
}