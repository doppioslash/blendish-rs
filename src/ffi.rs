
use libc::{c_float, c_int, c_char};
use libc::{c_uint, c_ushort, c_uchar, c_void};


use nanovg::{Ctx, Color};


/// describes the theme used to draw a single widget or widget box;
/// these values correspond to the same values that can be retrieved from
/// the Theme panel in the Blender preferences
#[repr(C)]
pub struct BNDwidgetTheme {
    /// color of widget box outline
    pub outlineColor: Color,
    /// color of widget item (meaning changes depending on class)
    pub itemColor: Color,
    /// fill color of widget box
    pub innerColor: Color,
    /// fill color of widget box when active
    pub innerSelectedColor: Color,
    /// color of text label
    pub textColor: Color,
    /// color of text label when active
    pub textSelectedColor: Color,
    /// delta modifier for upper part of gradient (-100 to 100)
    pub shadeTop: i32,
    /// delta modifier for lower part of gradient (-100 to 100)
    pub shadeDown: i32,
}

/// describes the theme used to draw widgets
#[repr(C)]
pub struct BNDtheme {
    /// the background color of panels and windows
    pub backgroundColor: Color,
    /// theme for labels
    pub regularTheme: BNDwidgetTheme,
    /// theme for tool buttons
    pub toolTheme: BNDwidgetTheme,
    /// theme for radio buttons
    pub radioTheme: BNDwidgetTheme,
    /// theme for text fields
    pub textFieldTheme: BNDwidgetTheme,
    /// theme for option buttons (checkboxes)
    pub optionTheme: BNDwidgetTheme,
    /// theme for choice buttons (comboboxes)
    /// Blender calls them "menu buttons"
    pub choiceTheme: BNDwidgetTheme,
    /// theme for number fields
    pub numberFieldTheme: BNDwidgetTheme,
    /// theme for slider controls
    pub sliderTheme: BNDwidgetTheme,
    /// theme for scrollbars
    pub scrollBarTheme: BNDwidgetTheme,
    /// theme for tooltips
    pub tooltipTheme: BNDwidgetTheme,
    /// theme for menu backgrounds
    pub menuTheme: BNDwidgetTheme,
    /// theme for menu items
    pub menuItemTheme: BNDwidgetTheme,
}


pub type NVGwinding = c_uint;
pub static NVG_CCW: c_uint = 1;

/// how text on a control is aligned
pub type BNDtextAlignment = c_uint;
    pub static BND_LEFT: c_uint = 0;
    pub static BND_CENTER: c_uint = 1;

/// states altering the styling of a widget
pub type BNDwidgetState = c_uint;
    /// not interacting
    pub static BND_DEFAULT: c_uint = 0;
    /// the mouse is hovering over the control
    pub static BND_HOVER: c_uint = 1;
    /// the widget is activated (pressed) or in an active state (toggled)
    pub static BND_ACTIVE: c_uint = 2;

/// flags indicating which corners are sharp (for grouping widgets)
//pub type BNDcornerFlags = c_int;
pub type BNDcornerFlags = c_uint;
    // all corners are round
    pub static BND_CORNER_NONE: c_uint = 0;
    // sharp top left corner
    pub static BND_CORNER_TOP_LEFT: c_uint = 1;
    // sharp top right corner
    pub static BND_CORNER_TOP_RIGHT: c_uint = 2;
    // sharp bottom right corner
    pub static BND_CORNER_DOWN_RIGHT: c_uint = 4;
    // sharp bottom left corner
    pub static BND_CORNER_DOWN_LEFT: c_uint = 8;
    // all corners are sharp;
    // you can invert a set of flags using ^= BND_CORNER_ALL
    pub static BND_CORNER_ALL: c_uint = 0xF;
    // top border is sharp
    pub static BND_CORNER_TOP: c_uint = 3;
    // bottom border is sharp
    pub static BND_CORNER_DOWN: c_uint = 0xC;
    // left border is sharp
    pub static BND_CORNER_LEFT: c_uint = 9;
    // right border is sharp
    pub static BND_CORNER_RIGHT: c_uint = 6;


// build an icon ID from two coordinates into the icon sheet, where
// (0,0) designates the upper-leftmost icon, (1,0) the one right next to it,
// and so on.
//#define BND_ICONID(x,y) ((x)|((y)<<8))
pub fn BND_ICONID(x: u8, y: u8) -> u16 { x as u16 | (y as u16 << 8) }

/// default widget height
pub static BND_WIDGET_HEIGHT: c_uint = 21;
/// default toolbutton width (if icon only)
pub static BND_TOOL_WIDTH: c_uint = 20;

/// width of vertical scrollbar
pub static BND_SCROLLBAR_WIDTH: c_uint = 13;
/// height of horizontal scrollbar
pub static BND_SCROLLBAR_HEIGHT: c_uint = 14;

/// default vertical spacing
pub static BND_VSPACING: c_uint = 1;
/// default vertical spacing between groups
pub static BND_VSPACING_GROUP: c_uint = 8;
/// default horizontal spacing
pub static BND_HSPACING: c_uint = 8;

/// alpha of disabled widget groups
/// can be used in conjunction with nvgGlobalAlpha()
pub static BND_DISABLED_ALPHA: c_float = 0.5;


// default text size
pub static BND_LABEL_FONT_SIZE: c_float = 13.0;

// default text padding in inner box
pub static BND_PAD_LEFT: c_uint = 8;
pub static BND_PAD_RIGHT: c_uint = 8;

// label: value separator string
pub static BND_LABEL_SEPARATOR: &'static str = ": ";

// alpha intensity of transparent items (0xa4)
pub static BND_TRANSPARENT_ALPHA: c_float = 0.643;

// shade intensity of beveled panels (expressed in percentage, -100..100)
pub static BND_BEVEL_SHADE: c_int = 30;
// shade intensity of beveled insets
pub static BND_INSET_BEVEL_SHADE: c_int = 30;
// shade intensity of hovered inner boxes
pub static BND_HOVER_SHADE: c_int = 15;

// width of icon sheet
pub static BND_ICON_SHEET_WIDTH: c_uint = 602;
// height of icon sheet
pub static BND_ICON_SHEET_HEIGHT: c_uint = 640;
// gridsize of icon sheet in both dimensions
pub static BND_ICON_SHEET_GRID: c_uint = 21;
// offset of first icon tile relative to left border
pub static BND_ICON_SHEET_OFFSET_X: c_uint = 5;
// offset of first icon tile relative to top border
pub static BND_ICON_SHEET_OFFSET_Y: c_uint = 10;
// resolution of single icon
pub static BND_ICON_SHEET_RES: c_uint = 16;

// size of number field arrow
pub static BND_NUMBER_ARROW_SIZE: c_float = 4.0;

//// default text color
// TODO fix Color so that it can be struct-initialized
//pub static BND_COLOR_TEXT: Color = rgba_f( 0.0, 0.0, 0.0, 1.0);
//// default highlighted text color
//pub static BND_COLOR_TEXT_SELECTED: Color = rgba_f( 1.0, 1.0, 1.0, 1.0);

// radius of tool button
pub static BND_TOOL_RADIUS: c_float = 4.0;

// radius of option button
pub static BND_OPTION_RADIUS: c_float = 4.0;
// width of option button checkbox
pub static BND_OPTION_WIDTH: c_float = 14.0;
// height of option button checkbox
pub static BND_OPTION_HEIGHT: c_float = 15.0;

// radius of text field
pub static BND_TEXT_RADIUS: c_float = 4.0;

// radius of number button
pub static BND_NUMBER_RADIUS: c_float = 10.0;

// radius of menu popup
pub static BND_MENU_RADIUS: c_float = 3.0;
// feather of menu popup shadow
pub static BND_SHADOW_FEATHER: c_float = 12.0;
// alpha of menu popup shadow
pub static BND_SHADOW_ALPHA: c_float = 0.5;

// radius of scrollbar
pub static BND_SCROLLBAR_RADIUS: c_float = 7.0;
// shade intensity of active scrollbar (percentage delta, -100..100)
pub static BND_SCROLLBAR_ACTIVE_SHADE: c_int = 15;

// max glyphs for position testing
pub static BND_MAX_GLYPHS: c_uint = 1024;

// text distance from bottom
pub static BND_TEXT_PAD_DOWN: c_uint = 7;


////////////////////////////////////////////////////////////////////////////////
//
//// set the current theme all widgets will be drawn with.
//// the default Blender 2.6 theme is set by default.
//void bndSetTheme(BNDtheme theme);
//
//// Returns the currently set theme
//const BNDtheme *bndGetTheme();
//
//// designates an image handle as returned by nvgCreateImage*() as the themes'
//// icon sheet. The icon sheet format must be compatible to Blender 2.6's icon
//// sheet; the order of icons does not matter.
//// A valid icon sheet is e.g. shown at
//// http://wiki.blender.org/index.php/Dev:2.5/Doc/How_to/Add_an_icon
//void bndSetIconImage(int image);
//
//// designates an image handle as returned by nvgCreateFont*() as the themes'
//// UI font. Blender's original UI font Droid Sans is perfectly suited and
//// available here:
//// https://svn.blender.org/svnroot/bf-blender/trunk/blender/release/datafiles/fonts/
//void bndSetFont(int font);
//
////////////////////////////////////////////////////////////////////////////////

extern {

    // High Level Functions
    // --------------------
    // Use these functions to draw themed widgets with your NVGcontext.

    // Draw a label with its lower left origin at (x,y) and size of (w,h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndLabel(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, iconid: c_int, label: *const c_char);

    // Draw a tool button  with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndToolButton(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        iconid: c_int, label: *const c_char);

    // Draw a radio button with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndRadioButton(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        iconid: c_int, label: *const c_char);

    // Draw a text field with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if text is not NULL, text will be printed to the widget
    // cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
    // cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
    // if cend < cbegin, then no caret will be drawn
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndTextField(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        iconid: c_int, text: *const c_char, cbegin: c_int, cend: c_int);

    // Draw an option button with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndOptionButton(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
        label: *const c_char);

    // Draw a choice button with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndChoiceButton(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        iconid: c_int, label: *const c_char);

    // Draw a number field with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndNumberField(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        label: *const c_char, value: *const c_char);

    // Draw slider control with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // progress must be in the range 0..1 and controls the size of the slider bar
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndSlider(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
        progress: c_float, label: *const c_char, value: *const c_char);

    // Draw scrollbar with its lower left origin at (x,y) and size of (w,h),
    // where state denotes the widgets current UI state.
    // offset is in the range 0..1 and controls the position of the scroll handle
    // size is in the range 0..1 and controls the size of the scroll handle
    // horizontal widget looks best when height is BND_SCROLLBAR_HEIGHT,
    // vertical looks best when width is BND_SCROLLBAR_WIDTH
    fn bndScrollBar(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
        offset: c_float, size: c_float);

    // Draw a menu background with its lower left origin at (x,y) and size of (w,h),
    // where flags is one or multiple flags from BNDcornerFlags.
    fn bndMenuBackground(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int);

    // Draw a menu label with its lower left origin at (x,y) and size of (w,h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndMenuLabel(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, iconid: c_int, label: *const c_char);

    // Draw a menu item with its lower left origin at (x,y) and size of (w,h),
    // where state denotes the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is BND_WIDGET_HEIGHT
    fn bndMenuItem(ctx: &Ctx,
        x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
        iconid: c_int, label: *const c_char);

    // Draw a tooltip background with its lower left origin at (x,y) and size of (w,h)
    fn bndTooltipBackground(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float);

    ////////////////////////////////////////////////////////////////////////////////

    // Estimator Functions
    // -------------------
    // Use these functions to estimate sizes for widgets with your NVGcontext.

    // returns the ideal width for a label with given icon and text
    fn bndLabelWidth(ctx: &Ctx, iconid: c_int, label: *const c_char) -> c_float;

    ////////////////////////////////////////////////////////////////////////////////


    // Low Level Functions
    // -------------------
    // these are part of the implementation detail and can be used to theme
    // new kinds of controls in a similar fashion.

    // make color transparent using the default alpha value
    fn bndTransparent(color: Color) -> Color;

    // offset a color by a given integer delta in the range -100 to 100
    fn bndOffsetColor(color: Color, delta: c_int) -> Color;

    // assigns radius r to the four entries of array radiuses depending on whether
    // the corner is marked as sharp or not; see BNDcornerFlags for possible
    // flag values.
    fn bndSelectCorners(radiuses: *const c_float, r: c_float, flags: c_int);

    // computes the upper and lower gradient colors for the inner box from a widget
    // theme and the widgets state. If flipActive is set and the state is
    // BND_ACTIVE, the upper and lower colors will be swapped.
    fn bndInnerColors(shade_top: *const Color, shade_down: *const Color,
        theme: *const BNDwidgetTheme, state: BNDwidgetState, flipActive: c_int);

    // computes the text color for a widget label from a widget theme and the
    // widgets state.
    fn bndTextColor(theme: *const BNDwidgetTheme, state: BNDwidgetState) -> Color;

    // computes the bounds of the scrollbar handle from the scrollbar size
    // and the handles offset and size.
    // offset is in the range 0..1 and defines the position of the scroll handle
    // size is in the range 0..1 and defines the size of the scroll handle
    fn bndScrollHandleRect(x: *const c_float, y: *const c_float, w: *const c_float, h: *const c_float,
        offset: c_float, size: c_float);

    // Add a rounded box path at position (x,y) with size (w,h) and a separate
    // radius for each corner listed in clockwise order, so that cr0 = top left,
    // cr1 = top right, cr2 = bottom right, cr3 = bottom left;
    // this is a low level drawing function: the path must be stroked or filled
    // to become visible.
    fn bndRoundedBox(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float);

    // Draw a flat panel without any decorations at position (x,y) with size (w,h)
    // and fills it with backgroundColor
    fn bndBackground(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float);

    // Draw a beveled border at position (x,y) with size (w,h) shaded with
    // lighter and darker versions of backgroundColor
    fn bndBevel(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float);

    // Draw a lower inset for a rounded box at position (x,y) with size (w,h)
    // that gives the impression the surface has been pushed in.
    // cr2 and cr3 contain the radiuses of the bottom right and bottom left
    // corners of the rounded box.
    fn bndBevelInset(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        cr2: c_float, cr3: c_float);

    // Draw an icon with (x,y) as its upper left coordinate; the iconid selects
    // the icon from the sheet; use the BND_ICONID macro to build icon IDs.
    fn bndIcon(ctx: &Ctx, x: c_float, y: c_float, iconid: c_int);

    // Draw a drop shadow around the rounded box at (x,y) with size (w,h) and
    // radius r, with feather as its maximum range in pixels.
    // No shadow will be painted inside the rounded box.
    fn bndDropShadow(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        r: c_float, feather: c_float, alpha: c_float);

    // Draw the inner part of a widget box, with a gradient from shade_top to
    // shade_down. If h>w, the gradient will be horizontal instead of
    // vertical.
    fn bndInnerBox(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float,
        shade_top: Color, shade_down: Color);

    // Draw the outline part of a widget box with the given color
    fn bndOutlineBox(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float, color: Color);

    // Draw an optional icon specified by <iconid> and an optional label with
    // given alignment (BNDtextAlignment), fontsize and color within a widget box.
    // if iconid is >= 0, an icon will be drawn and the labels remaining space
    // will be adjusted.
    // if label is not NULL, it will be drawn with the specified alignment, fontsize
    // and color.
    // if value is not NULL, label and value will be drawn with a ":" separator
    // inbetween.
    fn bndIconLabelValue(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        iconid: c_int, color: Color, align: c_int, fontsize: c_float, label: *const c_char,
        value: *const c_char);

    // Draw an optional icon specified by <iconid>, an optional label and
    // a caret with given fontsize and color within a widget box.
    // if iconid is >= 0, an icon will be drawn and the labels remaining space
    // will be adjusted.
    // if label is not NULL, it will be drawn with the specified alignment, fontsize
    // and color.
    // cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
    // cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
    // if cend < cbegin, then no caret will be drawn
    fn bndIconLabelCaret(ctx: &Ctx, x: c_float, y: c_float, w: c_float, h: c_float,
        iconid: c_int, color: Color, fontsize: c_float, label: *const c_char,
        caretcolor: Color, cbegin: c_int, cend: c_int);

    // Draw a checkmark for an option box with the given upper left coordinates
    // (ox,oy) with the specified color.
    fn bndCheck(ctx: &Ctx, ox: c_float, oy: c_float, color: Color);

    // Draw a horizontal arrow for a number field with its center at (x,y) and
    // size s; if s is negative, the arrow points to the left.
    fn bndArrow(ctx: &Ctx, x: c_float, y: c_float, s: c_float, color: Color);

    // Draw an up/down arrow for a choice box with its center at (x,y) and size s
    fn bndUpDownArrow(ctx: &Ctx, x: c_float, y: c_float, s: c_float, color: Color);
}