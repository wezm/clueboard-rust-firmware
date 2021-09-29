use keyberon::action::Action::NoOp;
use keyberon::action::{Action, SequenceEvent};
use keyberon::key_code::KeyCode::*;

type ClueboardLayer = &'static [&'static [Action]];

#[allow(unused)]
enum Layer {
    BaseLayer = 0,
    FunctionLayer,
    MacroLayer,
}

/* Clueboard matrix layout
 * ,-----------------------------------------------------------.  ,---.
 * | 00| 01| 02| 03| 04| 05| 06| 07| 50| 51| 52| 53| 54|   56  |  | 57|
 * |-----------------------------------------------------------|  |---|
 * |   10| 11| 12| 13| 14| 15| 16| 17| 60| 61| 62| 63| 64|   65|  | 67|
 * |-----------------------------------------------------------|  `---'
 * |    20| 21| 22| 23| 24| 25| 26| 27| 70| 71| 72| 73|      75|
 * |--------------------------------------------------------------.
 * |  30    | 32| 33| 34| 35| 36| 37| 80| 81| 82| 83|      85  |86|
 * |------------------------------------------------------------------.
 * |  40| 41|  42|        45|       46|   90|  92|  93|  94| 95|96| 97|
 * `------------------------------------------------------------------'
 * ,-----------------------------------------------------------.  ,---.
 * |  `|  1|  2|  3|  4|  5|  6|  7|  8|  9|  0|  -|  =|Backsp |  |Ins|
 * |-----------------------------------------------------------|  |---|
 * |Tab  |  Q|  W|  E|  R|  T|  Y|  U|  I|  O|  P|  [|  ]|    \|  |Del|
 * |-----------------------------------------------------------|  `---'
 * |Caps  |  A|  S|  D|  F|  G|  H|  J|  k|  L|  ;|  '|Enter   |
 * |--------------------------------------------------------------.
 * |Shift   |  Z|  X|  C|  V|  B|  N|  M|  ,|  .|  /|    Shift| Up|
 * |------------------------------------------------------------------.
 * |Ctrl|Alt|Gui |     Space|  Space|Gui |Alt |Fn  |Ctrl|Left|Down|Rgt|
 * `------------------------------------------------------------------'
 */

// Re-map a layer in keyboard order to matrix order
macro_rules! layer {
    (
    $k00:expr, $k01:expr, $k02:expr, $k03:expr, $k04:expr, $k05:expr, $k06:expr, $k07:expr, $k50:expr, $k51:expr, $k52:expr, $k53:expr, $k54:expr, $k55:expr, $k57:expr,
    $k10:expr, $k11:expr, $k12:expr, $k13:expr, $k14:expr, $k15:expr, $k16:expr, $k17:expr, $k60:expr, $k61:expr, $k62:expr, $k63:expr, $k64:expr, $k65:expr, $k67:expr,
    $k20:expr, $k21:expr, $k22:expr, $k23:expr, $k24:expr, $k25:expr, $k26:expr, $k27:expr, $k70:expr, $k71:expr, $k72:expr, $k73:expr, $k75:expr,
    $k30:expr, $k32:expr, $k33:expr, $k34:expr, $k35:expr, $k36:expr, $k37:expr, $k80:expr, $k81:expr, $k82:expr, $k83:expr, $k85:expr, $k86:expr,
    $k40:expr, $k41:expr, $k42:expr, $k45:expr, $k46:expr, $k90:expr, $k92:expr, $k93:expr, $k94:expr, $k95:expr, $k96:expr, $k97:expr
    ) => {
        &[
            &[$k00, $k01, $k02, $k03, $k04, $k05, $k06, $k07],
            &[$k10, $k11, $k12, $k13, $k14, $k15, $k16, $k17],
            &[$k20, $k21, $k22, $k23, $k24, $k25, $k26, $k27],
            &[$k30, NoOp, $k32, $k33, $k34, $k35, $k36, $k37],
            &[$k40, $k41, $k42, NoOp, NoOp, $k45, $k46, NoOp],
            &[$k50, $k51, $k52, $k53, $k54, $k55, NoOp, $k57],
            &[$k60, $k61, $k62, $k63, $k64, $k65, NoOp, $k67],
            &[$k70, $k71, $k72, $k73, NoOp, $k75, NoOp, NoOp],
            &[$k80, $k81, $k82, $k83, NoOp, $k85, $k86, NoOp],
            &[$k90, NoOp, $k92, $k93, $k94, $k95, $k96, $k97],
        ]
    };
}

#[rustfmt::skip]
pub(crate) static BASE_LAYER: ClueboardLayer = layer!(
KC_ESC,  KC_1,   KC_2,   KC_3,   KC_4,   KC_5,   KC_6,   KC_7,   KC_8,   KC_9,    KC_0,    KC_MINS, KC_EQL,  KC_BSPC,                 KC_PGUP,
KC_TAB,  KC_Q,   KC_W,   KC_E,   KC_R,   KC_T,   KC_Y,   KC_U,   KC_I,   KC_O,    KC_P,    KC_LBRC, KC_RBRC, KC_BSLS,                 KC_PGDN,
KC_LCTL, KC_A,   KC_S,   KC_D,   KC_F,   KC_G,   KC_H,   KC_J,   KC_K,   KC_L,    KC_SCLN, KC_QUOT,          KC_ENT,
KC_LSFT,         KC_Z,   KC_X,   KC_C,   KC_V,   KC_B,   KC_N,   KC_M,   KC_COMM, KC_DOT,  KC_SLSH,          KC_RSFT,         KC_UP,
MO_FL,   KC_LALT,KC_LGUI,                KC_SPC, KC_SPC,                          KC_NO  , KC_RGUI, MO_ML,   KC_APP , KC_LEFT,KC_DOWN,KC_RGHT);

#[rustfmt::skip]
pub(crate) static FUNCTION_LAYER: ClueboardLayer = layer!(
KC_GRV,  KC_F1,  KC_F2,  KC_F3,  KC_F4,  KC_F5,  KC_F6,  KC_F7,  KC_F8,  KC_F9,   KC_F10,  KC_F11,  KC_F12,  KC_DEL,                 KC_VOLU,
______,  ______, ______, ______, ______, ______, ______, ______, ______, KC_MPRV, KC_MPLY, KC_MNXT, KC_MUTE, KC_INS,                 KC_VOLD,
______,  ______, ______, ______, ______, ______, KC_LEFT,KC_DOWN,KC_UP  ,KC_RGHT, ______,  ______,           ______,
______,          ______, ______, ______, ______, ______, ______, ______, ______,  ______,  ______,           ______,         KC_PGUP,
______,  ______, ______,                 ______, ______,                          ______,  ______,  ______,  ______, KC_HOME,KC_PGDN,KC_END);

#[rustfmt::skip]
pub(crate) static MACRO_LAYER: ClueboardLayer = layer!(
______,  ______, EMAIL,  ______, ______, ______, ______, ______, ______, ______,  ______,  ______,  ______,  KC_PRN,                 ______,
______,  ______, FNAME,  ______, ______, ______, ______, UNAME,  ______, ______,  PHONE,   ______,  ______,  ______,                 ______,
______,  ADDR,   ______, ______, ______, ______, ______, ______, ______, ______,  ______,  ______,           ______,
______,          ______, ______, ______, ______, TOWN,   ______, LNAME,  ______,  ______,  ______,           ______,         ______,
______,  ______, ______,                 ______, ______,                          ______,  ______,  ______,  ______, ______, ______, ______);

// Map keyberon Actions to QMK key codes
// https://docs.qmk.fm/#/keycodes_basic
const ______: Action = Action::Trans;
const KC_NO: Action = Action::NoOp;

const KC_GRV: Action = Action::KeyCode(Grave);
const KC_F1: Action = Action::KeyCode(F1);
const KC_F2: Action = Action::KeyCode(F2);
const KC_F3: Action = Action::KeyCode(F3);
const KC_F4: Action = Action::KeyCode(F4);
const KC_F5: Action = Action::KeyCode(F5);
const KC_F6: Action = Action::KeyCode(F6);
const KC_F7: Action = Action::KeyCode(F7);
const KC_F8: Action = Action::KeyCode(F8);
const KC_F9: Action = Action::KeyCode(F9);
const KC_F10: Action = Action::KeyCode(F10);
const KC_F11: Action = Action::KeyCode(F11);
const KC_F12: Action = Action::KeyCode(F12);
const KC_INS: Action = Action::KeyCode(Insert);
const KC_DEL: Action = Action::KeyCode(Delete);

const KC_VOLU: Action = Action::KeyCode(VolUp);
const KC_VOLD: Action = Action::KeyCode(VolDown);
const KC_MUTE: Action = Action::KeyCode(Mute);
const KC_MPRV: Action = Action::KeyCode(MediaPreviousSong);
const KC_MPLY: Action = Action::KeyCode(MediaPlayPause);
const KC_MNXT: Action = Action::KeyCode(MediaNextSong);

const KC_ESC: Action = Action::KeyCode(Escape);
const KC_1: Action = Action::KeyCode(Kb1);
const KC_2: Action = Action::KeyCode(Kb2);
const KC_3: Action = Action::KeyCode(Kb3);
const KC_4: Action = Action::KeyCode(Kb4);
const KC_5: Action = Action::KeyCode(Kb5);
const KC_6: Action = Action::KeyCode(Kb6);
const KC_7: Action = Action::KeyCode(Kb7);
const KC_8: Action = Action::KeyCode(Kb8);
const KC_9: Action = Action::KeyCode(Kb9);
const KC_0: Action = Action::KeyCode(Kb0);
const KC_MINS: Action = Action::KeyCode(Minus);
const KC_EQL: Action = Action::KeyCode(Equal);
const KC_BSPC: Action = Action::KeyCode(BSpace);
const KC_PRN: Action = Action::KeyCode(PScreen);

const KC_HOME: Action = Action::KeyCode(Home);
const KC_END: Action = Action::KeyCode(End);
const KC_PGUP: Action = Action::KeyCode(PgUp);
const KC_PGDN: Action = Action::KeyCode(PgDown);

const KC_TAB: Action = Action::KeyCode(Tab);
const KC_Q: Action = Action::KeyCode(Q);
const KC_W: Action = Action::KeyCode(W);
const KC_E: Action = Action::KeyCode(E);
const KC_R: Action = Action::KeyCode(R);
const KC_T: Action = Action::KeyCode(T);
const KC_Y: Action = Action::KeyCode(Y);
const KC_U: Action = Action::KeyCode(U);
const KC_I: Action = Action::KeyCode(I);
const KC_O: Action = Action::KeyCode(O);
const KC_P: Action = Action::KeyCode(P);
const KC_LBRC: Action = Action::KeyCode(LBracket);
const KC_RBRC: Action = Action::KeyCode(RBracket);
const KC_BSLS: Action = Action::KeyCode(Bslash);

const KC_LCTL: Action = Action::KeyCode(LCtrl);
const KC_A: Action = Action::KeyCode(A);
const KC_S: Action = Action::KeyCode(S);
const KC_D: Action = Action::KeyCode(D);
const KC_F: Action = Action::KeyCode(F);
const KC_G: Action = Action::KeyCode(G);
const KC_H: Action = Action::KeyCode(H);
const KC_J: Action = Action::KeyCode(J);
const KC_K: Action = Action::KeyCode(K);
const KC_L: Action = Action::KeyCode(L);
const KC_SCLN: Action = Action::KeyCode(SColon);
const KC_QUOT: Action = Action::KeyCode(Quote);
const KC_ENT: Action = Action::KeyCode(Enter);

const KC_LSFT: Action = Action::KeyCode(LShift);
const KC_Z: Action = Action::KeyCode(Z);
const KC_X: Action = Action::KeyCode(X);
const KC_C: Action = Action::KeyCode(C);
const KC_V: Action = Action::KeyCode(V);
const KC_B: Action = Action::KeyCode(B);
const KC_N: Action = Action::KeyCode(N);
const KC_M: Action = Action::KeyCode(M);
const KC_COMM: Action = Action::KeyCode(Comma);
const KC_DOT: Action = Action::KeyCode(Dot);
const KC_SLSH: Action = Action::KeyCode(Slash);
const KC_RSFT: Action = Action::KeyCode(RShift);
const KC_UP: Action = Action::KeyCode(Up);

const KC_LALT: Action = Action::KeyCode(LAlt);
const KC_LGUI: Action = Action::KeyCode(LGui);
const KC_SPC: Action = Action::KeyCode(Space);
// const KC_RALT: Action = Action::KeyCode(RAlt);
const KC_RGUI: Action = Action::KeyCode(RGui);
const KC_APP: Action = Action::KeyCode(Application);
const KC_LEFT: Action = Action::KeyCode(Left);
const KC_DOWN: Action = Action::KeyCode(Down);
const KC_RGHT: Action = Action::KeyCode(Right);

const MO_FL: Action = Action::Layer(Layer::FunctionLayer as usize);
const MO_ML: Action = Action::Layer(Layer::MacroLayer as usize);

// Contains macro definitions generated by build.rs
include!(concat!(env!("OUT_DIR"), "/macros.rs"));
