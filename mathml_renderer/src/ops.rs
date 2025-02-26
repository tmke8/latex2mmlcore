#[cfg(feature = "serde")]
use serde::Serialize;

use crate::attribute::Stretchy;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(transparent)]
pub struct Op(char);

impl Op {
    #[inline(always)]
    pub const fn as_char(&self) -> char {
        self.0
    }
}

impl From<Op> for char {
    #[inline]
    fn from(op: Op) -> Self {
        op.0
    }
}

impl From<&Op> for char {
    #[inline]
    fn from(op: &Op) -> Self {
        op.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(transparent)]
pub struct Rel(char);

impl Rel {
    #[inline(always)]
    pub const fn as_op(&self) -> Op {
        Op(self.0)
    }
}

impl From<Rel> for Op {
    #[inline(always)]
    fn from(op: Rel) -> Self {
        Op(op.0)
    }
}

impl From<&Rel> for Op {
    #[inline(always)]
    fn from(op: &Rel) -> Self {
        Op(op.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(transparent)]
pub struct Bin(char);

impl Bin {
    #[inline(always)]
    pub const fn as_op(&self) -> Op {
        Op(self.0)
    }
}

impl From<Bin> for Op {
    #[inline]
    fn from(op: Bin) -> Self {
        Op(op.0)
    }
}

impl From<&Bin> for Op {
    #[inline]
    fn from(op: &Bin) -> Self {
        Op(op.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(transparent)]
pub struct Big(char);

impl From<Big> for Op {
    #[inline]
    fn from(op: Big) -> Self {
        Op(op.0)
    }
}

impl From<&Big> for Op {
    #[inline]
    fn from(op: &Big) -> Self {
        Op(op.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ParenOp(char, bool, Stretchy);

impl ParenOp {
    /// The parenthesis behaves like a normal identifier
    /// (which is different from an operator with reduced spacing!)
    #[inline]
    pub fn ordinary_spacing(&self) -> bool {
        self.1
    }
    #[inline]
    pub fn stretchy(&self) -> Stretchy {
        self.2
    }
}

impl From<&ParenOp> for char {
    #[inline]
    fn from(op: &ParenOp) -> Self {
        op.0
    }
}

//
// Unicode Block: Basic Latin
//
pub const NULL: &ParenOp = &ParenOp('\u{0}', false, Stretchy::Always);
pub const EXCLAMATION_MARK: Rel = Rel('!');
// pub const APOSTROPHE: Op = Op('\'');
pub const LEFT_PARENTHESIS: &ParenOp = &ParenOp('(', false, Stretchy::Always);
pub const RIGHT_PARENTHESIS: &ParenOp = &ParenOp(')', false, Stretchy::Always);
// pub const ASTERISK: Op = Op('*');
pub const PLUS_SIGN: Bin = Bin('+');
pub const COMMA: Rel = Rel(',');
pub const FULL_STOP: char = '.'; // not treated as operator
pub const SOLIDUS: &ParenOp = &ParenOp('/', true, Stretchy::Never);
pub const COLON: Rel = Rel(':');
pub const SEMICOLON: Rel = Rel(';');
// pub const LESS_THAN_SIGN: Op = Op('<');
pub const EQUALS_SIGN: Rel = Rel('=');
// pub const GREATER_THAN_SIGN: Op = Op('>');
// pub const QUESTION_MARK: Op = Op('?');
pub const LEFT_SQUARE_BRACKET: &ParenOp = &ParenOp('[', false, Stretchy::Always);
pub const REVERSE_SOLIDUS: &ParenOp = &ParenOp('\\', true, Stretchy::Never);
pub const RIGHT_SQUARE_BRACKET: &ParenOp = &ParenOp(']', false, Stretchy::Always);
pub const CIRCUMFLEX_ACCENT: Op = Op('^');
pub const LOW_LINE: Op = Op('_');
pub const GRAVE_ACCENT: Op = Op('`');
pub const LEFT_CURLY_BRACKET: &ParenOp = &ParenOp('{', false, Stretchy::Always);
pub const VERTICAL_LINE: &ParenOp = &ParenOp('|', true, Stretchy::PrePostfix);
pub const RIGHT_CURLY_BRACKET: &ParenOp = &ParenOp('}', false, Stretchy::Always);
pub const TILDE: Op = Op('~');

//
// Unicode Block: Latin-1 Supplement
//
pub const DIAERESIS: Op = Op('¨');
pub const NOT_SIGN: Rel = Rel('¬');
pub const MACRON: Op = Op('¯');
pub const PLUS_MINUS_SIGN: Bin = Bin('±');
pub const ACUTE_ACCENT: Op = Op('´');
pub const MIDDLE_DOT: Bin = Bin('·');
pub const MULTIPLICATION_SIGN: Bin = Bin('×');
pub const DIVISION_SIGN: Bin = Bin('÷');

//
// Unicode Block: Spacing Modifier Letters
//
pub const CARON: Op = Op('ˇ');
pub const BREVE: Op = Op('˘');
pub const DOT_ABOVE: Op = Op('˙');

//
// Unicode Block: General Punctuation
//
pub const DOUBLE_VERTICAL_LINE: &ParenOp = &ParenOp('‖', true, Stretchy::PrePostfix);
pub const HORIZONTAL_ELLIPSIS: Rel = Rel('…');
pub const PRIME: Rel = Rel('′');
pub const DOUBLE_PRIME: Rel = Rel('″');
pub const TRIPLE_PRIME: Rel = Rel('‴');
pub const REVERSED_PRIME: Rel = Rel('‵');
pub const REVERSED_DOUBLE_PRIME: Rel = Rel('‶');
pub const REVERSED_TRIPLE_PRIME: Rel = Rel('‷');
// pub const CARET: Op = Op('‸');
// pub const SINGLE_LEFT_POINTING_ANGLE_QUOTATION_MARK: Op = Op('‹');
// pub const SINGLE_RIGHT_POINTING_ANGLE_QUOTATION_MARK: Op = Op('›');
// pub const REFERENCE_MARK: Op = Op('※');
// pub const DOUBLE_EXCLAMATION_MARK: Op = Op('‼');
// pub const INTERROBANG: Op = Op('‽');
pub const OVERLINE: Rel = Rel('‾');

pub const QUADRUPLE_PRIME: Rel = Rel('⁗');

//
// Unicode Block: Arrows
//
pub const LEFTWARDS_ARROW: Rel = Rel('←');
pub const UPWARDS_ARROW: &ParenOp = &ParenOp('↑', false, Stretchy::Inconsistent);
pub const RIGHTWARDS_ARROW: Rel = Rel('→');
pub const DOWNWARDS_ARROW: &ParenOp = &ParenOp('↓', false, Stretchy::Inconsistent);
pub const LEFT_RIGHT_ARROW: Rel = Rel('↔');
pub const UP_DOWN_ARROW: &ParenOp = &ParenOp('↕', false, Stretchy::Inconsistent);
pub const NORTH_WEST_ARROW: Rel = Rel('↖');
pub const NORTH_EAST_ARROW: Rel = Rel('↗');
pub const SOUTH_EAST_ARROW: Rel = Rel('↘');
pub const SOUTH_WEST_ARROW: Rel = Rel('↙');
pub const LEFTWARDS_ARROW_WITH_STROKE: Rel = Rel('↚');
pub const RIGHTWARDS_ARROW_WITH_STROKE: Rel = Rel('↛');
// pub const LEFTWARDS_WAVE_ARROW: Op = Op('↜');
// pub const RIGHTWARDS_WAVE_ARROW: Op = Op('↝');
// pub const LEFTWARDS_TWO_HEADED_ARROW: Op = Op('↞');
// pub const UPWARDS_TWO_HEADED_ARROW: Op = Op('↟');
// pub const RIGHTWARDS_TWO_HEADED_ARROW: Op = Op('↠');
// pub const DOWNWARDS_TWO_HEADED_ARROW: Op = Op('↡');
pub const LEFTWARDS_ARROW_WITH_TAIL: Rel = Rel('↢');
pub const RIGHTWARDS_ARROW_WITH_TAIL: Rel = Rel('↣');
// pub const LEFTWARDS_ARROW_FROM_BAR: Op = Op('↤');
// pub const UPWARDS_ARROW_FROM_BAR: Op = Op('↥');
pub const RIGHTWARDS_ARROW_FROM_BAR: Rel = Rel('↦');
// pub const DOWNWARDS_ARROW_FROM_BAR: Op = Op('↧');
// pub const UP_DOWN_ARROW_WITH_BASE: Op = Op('↨');
pub const LEFTWARDS_ARROW_WITH_HOOK: Rel = Rel('↩');
pub const RIGHTWARDS_ARROW_WITH_HOOK: Rel = Rel('↪');
pub const LEFTWARDS_ARROW_WITH_LOOP: Rel = Rel('↫');
pub const RIGHTWARDS_ARROW_WITH_LOOP: Rel = Rel('↬');
pub const LEFT_RIGHT_WAVE_ARROW: Rel = Rel('↭');
pub const LEFT_RIGHT_ARROW_WITH_STROKE: Rel = Rel('↮');
pub const DOWNWARDS_ZIGZAG_ARROW: Rel = Rel('↯');
pub const UPWARDS_ARROW_WITH_TIP_LEFTWARDS: Rel = Rel('↰');
pub const UPWARDS_ARROW_WITH_TIP_RIGHTWARDS: Rel = Rel('↱');
// pub const DOWNWARDS_ARROW_WITH_TIP_LEFTWARDS: Op = Op('↲');
// pub const DOWNWARDS_ARROW_WITH_TIP_RIGHTWARDS: Op = Op('↳');
// pub const RIGHTWARDS_ARROW_WITH_CORNER_DOWNWARDS: Op = Op('↴');
// pub const DOWNWARDS_ARROW_WITH_CORNER_LEFTWARDS: Op = Op('↵');
pub const ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW: Rel = Rel('↶');
pub const CLOCKWISE_TOP_SEMICIRCLE_ARROW: Rel = Rel('↷');
// pub const NORTH_WEST_ARROW_TO_LONG_BAR: Op = Op('↸');
// pub const LEFTWARDS_ARROW_TO_BAR_OVER_RIGHTWARDS_ARROW_TO_BAR: Op = Op('↹');
pub const ANTICLOCKWISE_OPEN_CIRCLE_ARROW: Rel = Rel('↺');
pub const CLOCKWISE_OPEN_CIRCLE_ARROW: Rel = Rel('↻');
pub const LEFTWARDS_HARPOON_WITH_BARB_UPWARDS: Rel = Rel('↼');
pub const LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS: Rel = Rel('↽');
pub const UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS: Rel = Rel('↾');
pub const UPWARDS_HARPOON_WITH_BARB_LEFTWARDS: Rel = Rel('↿');
pub const RIGHTWARDS_HARPOON_WITH_BARB_UPWARDS: Rel = Rel('⇀');
pub const RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS: Rel = Rel('⇁');
pub const DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS: Rel = Rel('⇂');
pub const DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS: Rel = Rel('⇃');
pub const RIGHTWARDS_ARROW_OVER_LEFTWARDS_ARROW: Rel = Rel('⇄');
// pub const UPWARDS_ARROW_LEFTWARDS_OF_DOWNWARDS_ARROW: Op = Op('⇅');
pub const LEFTWARDS_ARROW_OVER_RIGHTWARDS_ARROW: Rel = Rel('⇆');
pub const LEFTWARDS_PAIRED_ARROWS: Rel = Rel('⇇');
pub const UPWARDS_PAIRED_ARROWS: Rel = Rel('⇈');
pub const RIGHTWARDS_PAIRED_ARROWS: Rel = Rel('⇉');
pub const DOWNWARDS_PAIRED_ARROWS: Rel = Rel('⇊');
pub const LEFTWARDS_HARPOON_OVER_RIGHTWARDS_HARPOON: Rel = Rel('⇋');
pub const RIGHTWARDS_HARPOON_OVER_LEFTWARDS_HARPOON: Rel = Rel('⇌');
pub const LEFTWARDS_DOUBLE_ARROW_WITH_STROKE: Rel = Rel('⇍');
pub const LEFT_RIGHT_DOUBLE_ARROW_WITH_STROKE: Rel = Rel('⇎');
pub const RIGHTWARDS_DOUBLE_ARROW_WITH_STROKE: Rel = Rel('⇏');
pub const LEFTWARDS_DOUBLE_ARROW: Rel = Rel('⇐');
pub const UPWARDS_DOUBLE_ARROW: &ParenOp = &ParenOp('⇑', false, Stretchy::Inconsistent);
pub const RIGHTWARDS_DOUBLE_ARROW: Rel = Rel('⇒');
pub const DOWNWARDS_DOUBLE_ARROW: &ParenOp = &ParenOp('⇓', false, Stretchy::Inconsistent);
pub const LEFT_RIGHT_DOUBLE_ARROW: Rel = Rel('⇔');
pub const UP_DOWN_DOUBLE_ARROW: &ParenOp = &ParenOp('⇕', false, Stretchy::Inconsistent);
// pub const NORTH_WEST_DOUBLE_ARROW: Op = Op('⇖');
// pub const NORTH_EAST_DOUBLE_ARROW: Op = Op('⇗');
// pub const SOUTH_EAST_DOUBLE_ARROW: Op = Op('⇘');
// pub const SOUTH_WEST_DOUBLE_ARROW: Op = Op('⇙');
pub const LEFTWARDS_TRIPLE_ARROW: Rel = Rel('⇚');
pub const RIGHTWARDS_TRIPLE_ARROW: Rel = Rel('⇛');
// pub const LEFTWARDS_SQUIGGLE_ARROW: Op = Op('⇜');
pub const RIGHTWARDS_SQUIGGLE_ARROW: Rel = Rel('⇝');
// pub const UPWARDS_ARROW_WITH_DOUBLE_STROKE: Op = Op('⇞');
// pub const DOWNWARDS_ARROW_WITH_DOUBLE_STROKE: Op = Op('⇟');
// pub const LEFTWARDS_DASHED_ARROW: Op = Op('⇠');
// pub const UPWARDS_DASHED_ARROW: Op = Op('⇡');
// pub const RIGHTWARDS_DASHED_ARROW: Op = Op('⇢');
// pub const DOWNWARDS_DASHED_ARROW: Op = Op('⇣');
// pub const LEFTWARDS_ARROW_TO_BAR: Op = Op('⇤');
// pub const RIGHTWARDS_ARROW_TO_BAR: Op = Op('⇥');
// pub const LEFTWARDS_WHITE_ARROW: Op = Op('⇦');
// pub const UPWARDS_WHITE_ARROW: Op = Op('⇧');
// pub const RIGHTWARDS_WHITE_ARROW: Op = Op('⇨');
// pub const DOWNWARDS_WHITE_ARROW: Op = Op('⇩');
// pub const UPWARDS_WHITE_ARROW_FROM_BAR: Op = Op('⇪');
// pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL: Op = Op('⇫');
// pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_HORIZONTAL_BAR: Op = Op('⇬');
// pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_VERTICAL_BAR: Op = Op('⇭');
// pub const UPWARDS_WHITE_DOUBLE_ARROW: Op = Op('⇮');
// pub const UPWARDS_WHITE_DOUBLE_ARROW_ON_PEDESTAL: Op = Op('⇯');
// pub const RIGHTWARDS_WHITE_ARROW_FROM_WALL: Op = Op('⇰');
// pub const NORTH_WEST_ARROW_TO_CORNER: Op = Op('⇱');
// pub const SOUTH_EAST_ARROW_TO_CORNER: Op = Op('⇲');
// pub const UP_DOWN_WHITE_ARROW: Op = Op('⇳');
// pub const RIGHT_ARROW_WITH_SMALL_CIRCLE: Op = Op('⇴');
// pub const DOWNWARDS_ARROW_LEFTWARDS_OF_UPWARDS_ARROW: Op = Op('⇵');
// pub const THREE_RIGHTWARDS_ARROWS: Op = Op('⇶');
// pub const LEFTWARDS_ARROW_WITH_VERTICAL_STROKE: Op = Op('⇷');
// pub const RIGHTWARDS_ARROW_WITH_VERTICAL_STROKE: Op = Op('⇸');
// pub const LEFT_RIGHT_ARROW_WITH_VERTICAL_STROKE: Op = Op('⇹');
// pub const LEFTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE: Op = Op('⇺');
// pub const RIGHTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE: Op = Op('⇻');
// pub const LEFT_RIGHT_ARROW_WITH_DOUBLE_VERTICAL_STROKE: Op = Op('⇼');
// pub const LEFTWARDS_OPEN_HEADED_ARROW: Op = Op('⇽');
// pub const RIGHTWARDS_OPEN_HEADED_ARROW: Op = Op('⇾');
// pub const LEFT_RIGHT_OPEN_HEADED_ARROW: Op = Op('⇿');

//
// Unicode Block: Mathematical Operators
//
pub const FOR_ALL: Rel = Rel('∀');
pub const COMPLEMENT: char = '∁'; // not treated as operator
pub const PARTIAL_DIFFERENTIAL: char = '∂'; // not treated as operator
pub const THERE_EXISTS: Rel = Rel('∃');
pub const THERE_DOES_NOT_EXIST: Rel = Rel('∄');
pub const EMPTY_SET: char = '∅';
// pub const INCREMENT: Op = Op('∆');
pub const NABLA: char = '∇'; // not treated as operator
pub const ELEMENT_OF: Rel = Rel('∈');
pub const NOT_AN_ELEMENT_OF: Rel = Rel('∉');
// pub const SMALL_ELEMENT_OF: Op = Op('∊');
pub const CONTAINS_AS_MEMBER: Rel = Rel('∋');
// pub const DOES_NOT_CONTAIN_AS_MEMBER: Op = Op('∌');
pub const SMALL_CONTAINS_AS_MEMBER: Rel = Rel('∍');
// pub const END_OF_PROOF: Op = Op('∎');
pub const N_ARY_PRODUCT: Big = Big('∏');
pub const N_ARY_COPRODUCT: Big = Big('∐');
pub const N_ARY_SUMMATION: Big = Big('∑');
pub const MINUS_SIGN: Bin = Bin('−');
pub const MINUS_OR_PLUS_SIGN: Bin = Bin('∓');
pub const DOT_PLUS: Bin = Bin('∔');
// pub const DIVISION_SLASH: Op = Op('∕');
pub const SET_MINUS: Rel = Rel('∖');
pub const ASTERISK_OPERATOR: Rel = Rel('∗');
pub const RING_OPERATOR: Rel = Rel('∘');
pub const BULLET_OPERATOR: Rel = Rel('∙');
// pub const SQUARE_ROOT: Op = Op('√');
// pub const CUBE_ROOT: Op = Op('∛');
// pub const FOURTH_ROOT: Op = Op('∜');
pub const PROPORTIONAL_TO: Rel = Rel('∝');
pub const INFINITY: char = '∞';
// pub const RIGHT_ANGLE: Op = Op('∟');
pub const ANGLE: char = '∠';
pub const MEASURED_ANGLE: char = '∡';
pub const SPHERICAL_ANGLE: char = '∢';
pub const DIVIDES: Rel = Rel('∣');
pub const DOES_NOT_DIVIDE: Rel = Rel('∤');
pub const PARALLEL_TO: Rel = Rel('∥');
pub const NOT_PARALLEL_TO: Rel = Rel('∦');
pub const LOGICAL_AND: Rel = Rel('∧');
pub const LOGICAL_OR: Rel = Rel('∨');
pub const INTERSECTION: Rel = Rel('∩');
pub const UNION: Rel = Rel('∪');
pub const INTEGRAL: Big = Big('∫');
pub const DOUBLE_INTEGRAL: Big = Big('∬');
pub const TRIPLE_INTEGRAL: Big = Big('∭');
pub const CONTOUR_INTEGRAL: Big = Big('∮');
pub const SURFACE_INTEGRAL: Big = Big('∯');
pub const VOLUME_INTEGRAL: Big = Big('∰');
pub const CLOCKWISE_INTEGRAL: Big = Big('∱');
pub const CLOCKWISE_CONTOUR_INTEGRAL: Big = Big('∲');
pub const ANTICLOCKWISE_CONTOUR_INTEGRAL: Big = Big('∳');
pub const THEREFORE: Rel = Rel('∴');
pub const BECAUSE: Rel = Rel('∵');
// pub const RATIO: Op = Op('∶');
pub const PROPORTION: Rel = Rel('∷');
// pub const DOT_MINUS: Op = Op('∸');
pub const EXCESS: Rel = Rel('∹');
pub const GEOMETRIC_PROPORTION: Rel = Rel('∺');
pub const HOMOTHETIC: Rel = Rel('∻');
pub const TILDE_OPERATOR: Rel = Rel('∼');
pub const REVERSED_TILDE: Rel = Rel('∽');
// pub const INVERTED_LAZY_S: Op = Op('∾');
// pub const SINE_WAVE: Op = Op('∿');
pub const WREATH_PRODUCT: Rel = Rel('≀');
pub const NOT_TILDE: Rel = Rel('≁');
pub const MINUS_TILDE: Rel = Rel('≂');
pub const ASYMPTOTICALLY_EQUAL_TO: Rel = Rel('≃');
pub const NOT_ASYMPTOTICALLY_EQUAL_TO: Rel = Rel('≄');
pub const APPROXIMATELY_EQUAL_TO: Rel = Rel('≅');
// pub const APPROXIMATELY_BUT_NOT_ACTUALLY_EQUAL_TO: Op = Op('≆');
// pub const NEITHER_APPROXIMATELY_NOR_ACTUALLY_EQUAL_TO: Op = Op('≇');
pub const ALMOST_EQUAL_TO: Rel = Rel('≈');
pub const NOT_ALMOST_EQUAL_TO: Rel = Rel('≉');
pub const ALMOST_EQUAL_OR_EQUAL_TO: Rel = Rel('≊');
// pub const TRIPLE_TILDE: Op = Op('≋');
// pub const ALL_EQUAL_TO: Op = Op('≌');
pub const EQUIVALENT_TO: Rel = Rel('≍');
pub const GEOMETRICALLY_EQUIVALENT_TO: Rel = Rel('≎');
pub const DIFFERENCE_BETWEEN: Rel = Rel('≏');
pub const APPROACHES_THE_LIMIT: Rel = Rel('≐');
pub const GEOMETRICALLY_EQUAL_TO: Rel = Rel('≑');
pub const APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF: Rel = Rel('≒');
pub const IMAGE_OF_OR_APPROXIMATELY_EQUAL_TO: Rel = Rel('≓');
pub const COLON_EQUALS: Rel = Rel('≔');
pub const EQUALS_COLON: Rel = Rel('≕');
pub const RING_IN_EQUAL_TO: Rel = Rel('≖');
pub const RING_EQUAL_TO: Rel = Rel('≗');
pub const CORRESPONDS_TO: Rel = Rel('≘');
pub const ESTIMATES: Rel = Rel('≙');
pub const EQUIANGULAR_TO: Rel = Rel('≚');
pub const STAR_EQUALS: Rel = Rel('≛');
pub const DELTA_EQUAL_TO: Rel = Rel('≜');
pub const EQUAL_TO_BY_DEFINITION: Rel = Rel('≝');
pub const MEASURED_BY: Rel = Rel('≞');
pub const QUESTIONED_EQUAL_TO: Rel = Rel('≟');
pub const NOT_EQUAL_TO: Rel = Rel('≠');
pub const IDENTICAL_TO: Rel = Rel('≡');
pub const NOT_IDENTICAL_TO: Rel = Rel('≢');
// pub const STRICTLY_EQUIVALENT_TO: Op = Op('≣');
pub const LESS_THAN_OR_EQUAL_TO: Rel = Rel('≤');
pub const GREATER_THAN_OR_EQUAL_TO: Rel = Rel('≥');
pub const LESS_THAN_OVER_EQUAL_TO: Rel = Rel('≦');
pub const GREATER_THAN_OVER_EQUAL_TO: Rel = Rel('≧');
pub const LESS_THAN_BUT_NOT_EQUAL_TO: Rel = Rel('≨');
pub const GREATER_THAN_BUT_NOT_EQUAL_TO: Rel = Rel('≩');
pub const MUCH_LESS_THAN: Rel = Rel('≪');
pub const MUCH_GREATER_THAN: Rel = Rel('≫');
pub const BETWEEN: Rel = Rel('≬');
// pub const NOT_EQUIVALENT_TO: Op = Op('≭');
pub const NOT_LESS_THAN: Rel = Rel('≮');
pub const NOT_GREATER_THAN: Rel = Rel('≯');
pub const NEITHER_LESS_THAN_NOR_EQUAL_TO: Rel = Rel('≰');
pub const NEITHER_GREATER_THAN_NOR_EQUAL_TO: Rel = Rel('≱');
pub const LESS_THAN_OR_EQUIVALENT_TO: Rel = Rel('≲');
pub const GREATER_THAN_OR_EQUIVALENT_TO: Rel = Rel('≳');
pub const NEITHER_LESS_THAN_NOR_EQUIVALENT_TO: Rel = Rel('≴');
pub const NEITHER_GREATER_THAN_NOR_EQUIVALENT_TO: Rel = Rel('≵');
pub const LESS_THAN_OR_GREATER_THAN: Rel = Rel('≶');
pub const GREATER_THAN_OR_LESS_THAN: Rel = Rel('≷');
pub const NEITHER_LESS_THAN_NOR_GREATER_THAN: Rel = Rel('≸');
pub const NEITHER_GREATER_THAN_NOR_LESS_THAN: Rel = Rel('≹');
pub const PRECEDES: Rel = Rel('≺');
pub const SUCCEEDS: Rel = Rel('≻');
pub const PRECEDES_OR_EQUAL_TO: Rel = Rel('≼');
pub const SUCCEEDS_OR_EQUAL_TO: Rel = Rel('≽');
pub const PRECEDES_OR_EQUIVALENT_TO: Rel = Rel('≾');
pub const SUCCEEDS_OR_EQUIVALENT_TO: Rel = Rel('≿');
pub const DOES_NOT_PRECEDE: Rel = Rel('⊀');
pub const DOES_NOT_SUCCEED: Rel = Rel('⊁');
pub const SUBSET_OF: Rel = Rel('⊂');
pub const SUPERSET_OF: Rel = Rel('⊃');
pub const NOT_A_SUBSET_OF: Rel = Rel('⊄');
pub const NOT_A_SUPERSET_OF: Rel = Rel('⊅');
pub const SUBSET_OF_OR_EQUAL_TO: Rel = Rel('⊆');
pub const SUPERSET_OF_OR_EQUAL_TO: Rel = Rel('⊇');
pub const NEITHER_A_SUBSET_OF_NOR_EQUAL_TO: Rel = Rel('⊈');
pub const NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO: Rel = Rel('⊉');
pub const SUBSET_OF_WITH_NOT_EQUAL_TO: Rel = Rel('⊊');
pub const SUPERSET_OF_WITH_NOT_EQUAL_TO: Rel = Rel('⊋');
// pub const MULTISET: Op = Op('⊌');
// pub const MULTISET_MULTIPLICATION: Op = Op('⊍');
pub const MULTISET_UNION: Rel = Rel('⊎');
pub const SQUARE_IMAGE_OF: Rel = Rel('⊏');
pub const SQUARE_ORIGINAL_OF: Rel = Rel('⊐');
pub const SQUARE_IMAGE_OF_OR_EQUAL_TO: Rel = Rel('⊑');
pub const SQUARE_ORIGINAL_OF_OR_EQUAL_TO: Rel = Rel('⊒');
pub const SQUARE_CAP: Rel = Rel('⊓');
pub const SQUARE_CUP: Rel = Rel('⊔');
pub const CIRCLED_PLUS: Rel = Rel('⊕');
pub const CIRCLED_MINUS: Rel = Rel('⊖');
pub const CIRCLED_TIMES: Rel = Rel('⊗');
pub const CIRCLED_DIVISION_SLASH: Rel = Rel('⊘');
pub const CIRCLED_DOT_OPERATOR: Rel = Rel('⊙');
pub const CIRCLED_RING_OPERATOR: Rel = Rel('⊚');
pub const CIRCLED_ASTERISK_OPERATOR: Rel = Rel('⊛');
// pub const CIRCLED_EQUALS: Op = Op('⊜');
pub const CIRCLED_DASH: Rel = Rel('⊝');
pub const SQUARED_PLUS: Rel = Rel('⊞');
pub const SQUARED_MINUS: Rel = Rel('⊟');
pub const SQUARED_TIMES: Rel = Rel('⊠');
pub const SQUARED_DOT_OPERATOR: Rel = Rel('⊡');
pub const RIGHT_TACK: Rel = Rel('⊢');
pub const LEFT_TACK: Rel = Rel('⊣');
pub const DOWN_TACK: char = '⊤';
pub const UP_TACK: char = '⊥';
// pub const ASSERTION: Op = Op('⊦');
// pub const MODELS: Op = Op('⊧');
pub const TRUE: Rel = Rel('⊨');
pub const FORCES: Rel = Rel('⊩');
pub const TRIPLE_VERTICAL_BAR_RIGHT_TURNSTILE: Rel = Rel('⊪');
pub const DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE: Rel = Rel('⊫');
pub const DOES_NOT_PROVE: Rel = Rel('⊬');
pub const NOT_TRUE: Rel = Rel('⊭');
pub const DOES_NOT_FORCE: Rel = Rel('⊮');
pub const NEGATED_DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE: Rel = Rel('⊯');
// pub const PRECEDES_UNDER_RELATION: Op = Op('⊰');
// pub const SUCCEEDS_UNDER_RELATION: Op = Op('⊱');
pub const NORMAL_SUBGROUP_OF: Rel = Rel('⊲');
pub const CONTAINS_AS_NORMAL_SUBGROUP: Rel = Rel('⊳');
pub const NORMAL_SUBGROUP_OF_OR_EQUAL_TO: Rel = Rel('⊴');
pub const CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO: Rel = Rel('⊵');
// pub const ORIGINAL_OF: Op = Op('⊶');
// pub const IMAGE_OF: Op = Op('⊷');
pub const MULTIMAP: Rel = Rel('⊸');
// pub const HERMITIAN_CONJUGATE_MATRIX: Op = Op('⊹');
pub const INTERCALATE: Rel = Rel('⊺');
pub const XOR: Rel = Rel('⊻');
pub const NAND: Rel = Rel('⊼');
// pub const NOR: Op = Op('⊽');
// pub const RIGHT_ANGLE_WITH_ARC: Op = Op('⊾');
// pub const RIGHT_TRIANGLE: Op = Op('⊿');
pub const N_ARY_LOGICAL_AND: Big = Big('⋀');
pub const N_ARY_LOGICAL_OR: Big = Big('⋁');
pub const N_ARY_INTERSECTION: Big = Big('⋂');
pub const N_ARY_UNION: Big = Big('⋃');
pub const DIAMOND_OPERATOR: Rel = Rel('⋄');
// pub const DOT_OPERATOR: Op = Op('⋅');
pub const STAR_OPERATOR: Rel = Rel('⋆');
pub const DIVISION_TIMES: Rel = Rel('⋇');
pub const BOWTIE: Rel = Rel('⋈');
pub const LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT: Rel = Rel('⋉');
pub const RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT: Rel = Rel('⋊');
pub const LEFT_SEMIDIRECT_PRODUCT: Rel = Rel('⋋');
pub const RIGHT_SEMIDIRECT_PRODUCT: Rel = Rel('⋌');
pub const REVERSED_TILDE_EQUALS: Rel = Rel('⋍');
pub const CURLY_LOGICAL_OR: Rel = Rel('⋎');
pub const CURLY_LOGICAL_AND: Rel = Rel('⋏');
pub const DOUBLE_SUBSET: Rel = Rel('⋐');
pub const DOUBLE_SUPERSET: Rel = Rel('⋑');
pub const DOUBLE_INTERSECTION: Rel = Rel('⋒');
pub const DOUBLE_UNION: Rel = Rel('⋓');
pub const PITCHFORK: Rel = Rel('⋔');
// pub const EQUAL_AND_PARALLEL_TO: Op = Op('⋕');
pub const LESS_THAN_WITH_DOT: Rel = Rel('⋖');
// pub const GREATER_THAN_WITH_DOT: Op = Op('⋗');
pub const VERY_MUCH_LESS_THAN: Rel = Rel('⋘');
// pub const VERY_MUCH_GREATER_THAN: Op = Op('⋙');
pub const LESS_THAN_EQUAL_TO_OR_GREATER_THAN: Rel = Rel('⋚');
// pub const GREATER_THAN_EQUAL_TO_OR_LESS_THAN: Op = Op('⋛');
// pub const EQUAL_TO_OR_LESS_THAN: Op = Op('⋜');
// pub const EQUAL_TO_OR_GREATER_THAN: Op = Op('⋝');
pub const EQUAL_TO_OR_PRECEDES: Rel = Rel('⋞');
pub const EQUAL_TO_OR_SUCCEEDS: Rel = Rel('⋟');
pub const DOES_NOT_PRECEDE_OR_EQUAL: Rel = Rel('⋠');
pub const DOES_NOT_SUCCEED_OR_EQUAL: Rel = Rel('⋡');
// pub const NOT_SQUARE_IMAGE_OF_OR_EQUAL_TO: Op = Op('⋢');
// pub const NOT_SQUARE_ORIGINAL_OF_OR_EQUAL_TO: Op = Op('⋣');
// pub const SQUARE_IMAGE_OF_OR_NOT_EQUAL_TO: Op = Op('⋤');
// pub const SQUARE_ORIGINAL_OF_OR_NOT_EQUAL_TO: Op = Op('⋥');
// pub const LESS_THAN_BUT_NOT_EQUIVALENT_TO: Op = Op('⋦');
// pub const GREATER_THAN_BUT_NOT_EQUIVALENT_TO: Op = Op('⋧');
pub const PRECEDES_BUT_NOT_EQUIVALENT_TO: Rel = Rel('⋨');
pub const SUCCEEDS_BUT_NOT_EQUIVALENT_TO: Rel = Rel('⋩');
// pub const NOT_NORMAL_SUBGROUP_OF: Op = Op('⋪');
// pub const DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP: Op = Op('⋫');
// pub const NOT_NORMAL_SUBGROUP_OF_OR_EQUAL_TO: Op = Op('⋬');
// pub const DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP_OR_EQUAL: Op = Op('⋭');
pub const VERTICAL_ELLIPSIS: Rel = Rel('⋮');
// pub const MIDLINE_HORIZONTAL_ELLIPSIS: Rel = Rel('⋯');
// pub const UP_RIGHT_DIAGONAL_ELLIPSIS: Op = Op('⋰');
pub const DOWN_RIGHT_DIAGONAL_ELLIPSIS: Rel = Rel('⋱');
// pub const ELEMENT_OF_WITH_LONG_HORIZONTAL_STROKE: Op = Op('⋲');
// pub const ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: Op = Op('⋳');
// pub const SMALL_ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: Op = Op('⋴');
// pub const ELEMENT_OF_WITH_DOT_ABOVE: Op = Op('⋵');
// pub const ELEMENT_OF_WITH_OVERBAR: Op = Op('⋶');
// pub const SMALL_ELEMENT_OF_WITH_OVERBAR: Op = Op('⋷');
// pub const ELEMENT_OF_WITH_UNDERBAR: Op = Op('⋸');
// pub const ELEMENT_OF_WITH_TWO_HORIZONTAL_STROKES: Op = Op('⋹');
// pub const CONTAINS_WITH_LONG_HORIZONTAL_STROKE: Op = Op('⋺');
// pub const CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: Op = Op('⋻');
// pub const SMALL_CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: Op = Op('⋼');
// pub const CONTAINS_WITH_OVERBAR: Op = Op('⋽');
// pub const SMALL_CONTAINS_WITH_OVERBAR: Op = Op('⋾');
// pub const Z_NOTATION_BAG_MEMBERSHIP: Op = Op('⋿');

//
// Unicode Block: Miscellaneous Technical
//
pub const LEFT_CEILING: &ParenOp = &ParenOp('⌈', false, Stretchy::Always);
pub const RIGHT_CEILING: &ParenOp = &ParenOp('⌉', false, Stretchy::Always);
pub const LEFT_FLOOR: &ParenOp = &ParenOp('⌊', false, Stretchy::Always);
pub const RIGHT_FLOOR: &ParenOp = &ParenOp('⌋', false, Stretchy::Always);
pub const TOP_LEFT_CORNER: char = '⌜';
pub const TOP_RIGHT_CORNER: char = '⌝';
pub const BOTTOM_LEFT_CORNER: char = '⌞';
pub const BOTTOM_RIGHT_CORNER: char = '⌟';
pub const FROWN: Rel = Rel('⌢');
pub const SMILE: Rel = Rel('⌣');
pub const TOP_SQUARE_BRACKET: Op = Op('⎴');
pub const BOTTOM_SQUARE_BRACKET: Op = Op('⎵');
pub const TOP_PARENTHESIS: Op = Op('⏜');
pub const BOTTOM_PARENTHESIS: Op = Op('⏝');
pub const TOP_CURLY_BRACKET: Op = Op('⏞');
pub const BOTTOM_CURLY_BRACKET: Op = Op('⏟');

//
// Unicode Block: Enclosed Alphanumerics
//
pub const CIRCLED_LATIN_CAPITAL_LETTER_R: char = 'Ⓡ'; // not treated as operator
pub const CIRCLED_LATIN_CAPITAL_LETTER_S: char = 'Ⓢ'; // not treated as operator

//
// Unicode Block: Geometric Shapes
//
pub const BLACK_SQUARE: char = '■';
pub const BLACK_UP_POINTING_TRIANGLE: char = '▲';
pub const BLACK_RIGHT_POINTING_TRIANGLE: char = '▶';
pub const BLACK_DOWN_POINTING_TRIANGLE: char = '▼';
pub const BLACK_LEFT_POINTING_TRIANGLE: char = '◀';

pub const WHITE_UP_POINTING_TRIANGLE: char = '△';
pub const WHITE_RIGHT_POINTING_TRIANGLE: char = '▷';
pub const WHITE_DOWN_POINTING_TRIANGLE: char = '▽';
pub const WHITE_LEFT_POINTING_TRIANGLE: char = '◁';

pub const LARGE_CIRCLE: char = '◯';

//
// Unicode Block: Miscellaneous Symbols
//
pub const BLACK_STAR: char = '★';

//
// Unicode Block: Miscellaneous Mathematical Symbols-A
//
pub const PERPENDICULAR: Rel = Rel('⟂');
pub const MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET: &ParenOp = &ParenOp('⟦', false, Stretchy::Always);
pub const MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET: &ParenOp =
    &ParenOp('⟧', false, Stretchy::Always);
pub const MATHEMATICAL_LEFT_ANGLE_BRACKET: &ParenOp = &ParenOp('⟨', false, Stretchy::Always);
pub const MATHEMATICAL_RIGHT_ANGLE_BRACKET: &ParenOp = &ParenOp('⟩', false, Stretchy::Always);
pub const MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS: &ParenOp =
    &ParenOp('⟮', false, Stretchy::Always);
pub const MATHEMATICAL_RIGHT_FLATTENED_PARENTHESIS: &ParenOp =
    &ParenOp('⟯', false, Stretchy::Always);

//
// Unicode Block: Supplemental Arrows-A
//
pub const LONG_LEFTWARDS_ARROW: Rel = Rel('⟵');
pub const LONG_RIGHTWARDS_ARROW: Rel = Rel('⟶');
pub const LONG_LEFT_RIGHT_ARROW: Rel = Rel('⟷');
pub const LONG_LEFTWARDS_DOUBLE_ARROW: Rel = Rel('⟸');
pub const LONG_RIGHTWARDS_DOUBLE_ARROW: Rel = Rel('⟹');
pub const LONG_LEFT_RIGHT_DOUBLE_ARROW: Rel = Rel('⟺');
// pub const LONG_LEFTWARDS_ARROW_FROM_BAR: Op = Op('⟻');
pub const LONG_RIGHTWARDS_ARROW_FROM_BAR: Rel = Rel('⟼');

//
// Unicode Block: Supplemental Arrows-B
//
pub const LEFTWARDS_ARROW_TAIL: Rel = Rel('⤙');
pub const RIGHTWARDS_ARROW_TAIL: Rel = Rel('⤚');

//
// Unicode Block: Miscellaneous Mathematical Symbols-B
//
pub const LEFT_WHITE_CURLY_BRACKET: &ParenOp = &ParenOp('⦃', false, Stretchy::Always);
pub const RIGHT_WHITE_CURLY_BRACKET: &ParenOp = &ParenOp('⦄', false, Stretchy::Always);
// pub const LEFT_WHITE_PARENTHESIS: &ParenOp = &ParenOp('⦅', false, Stretchy::Always);
// pub const RIGHT_WHITE_PARENTHESIS: &ParenOp = &ParenOp('⦆', false, Stretchy::Always);
pub const Z_NOTATION_LEFT_IMAGE_BRACKET: &ParenOp = &ParenOp('⦇', false, Stretchy::Always);
pub const Z_NOTATION_RIGHT_IMAGE_BRACKET: &ParenOp = &ParenOp('⦈', false, Stretchy::Always);
pub const Z_NOTATION_LEFT_BINDING_BRACKET: &ParenOp = &ParenOp('⦉', false, Stretchy::Always);
pub const Z_NOTATION_RIGHT_BINDING_BRACKET: &ParenOp = &ParenOp('⦊', false, Stretchy::Always);

pub const SQUARED_RISING_DIAGONAL_SLASH: Rel = Rel('⧄');
pub const SQUARED_FALLING_DIAGONAL_SLASH: Rel = Rel('⧅');
pub const SQUARED_SQUARE: Rel = Rel('⧈');
pub const BLACK_LOZENGE: char = '⧫';

//
// Unicode Block: Supplemental Mathematical Operators
//
pub const N_ARY_CIRCLED_DOT_OPERATOR: Big = Big('⨀');
pub const N_ARY_CIRCLED_PLUS_OPERATOR: Big = Big('⨁');
pub const N_ARY_CIRCLED_TIMES_OPERATOR: Big = Big('⨂');
pub const N_ARY_UNION_OPERATOR_WITH_DOT: Big = Big('⨃');
pub const N_ARY_UNION_OPERATOR_WITH_PLUS: Big = Big('⨄');
pub const N_ARY_SQUARE_INTERSECTION_OPERATOR: Big = Big('⨅');
pub const N_ARY_SQUARE_UNION_OPERATOR: Big = Big('⨆');
pub const TWO_LOGICAL_AND_OPERATOR: Big = Big('⨇');
pub const TWO_LOGICAL_OR_OPERATOR: Big = Big('⨈');
pub const N_ARY_TIMES_OPERATOR: Big = Big('⨉');
// pub const MODULO_TWO_SUM: Op = Op('⨊');
pub const SUMMATION_WITH_INTEGRAL: Big = Big('⨋');
pub const QUADRUPLE_INTEGRAL_OPERATOR: Big = Big('⨌');
pub const FINITE_PARTL_INTEGRAL: Big = Big('⨍');
pub const INTEGRAL_WITH_DOUBLE_STROKE: Big = Big('⨎');
pub const INTEGRAL_AVERAGE_WITH_SLASH: Big = Big('⨏');
pub const CIRCULATION_FUNCTION: Big = Big('⨐');
pub const ANTICLOCKWISE_INTEGRATION: Big = Big('⨑');
// pub const LINE_INTEGRATION_WITH_RECTANGULAR_PATH_AROUND_POLE: Op = Op('⨒');
// pub const LINE_INTEGRATION_WITH_SEMICIRCULAR_PATH_AROUND_POLE: Op = Op('⨓');
// pub const LINE_INTEGRATION_NOT_INCLUDING_THE_POLE: Op = Op('⨔');
// pub const INTEGRAL_AROUND_A_POINT_OPERATOR: Op = Op('⨕');
// pub const QUATERNION_INTEGRAL_OPERATOR: Op = Op('⨖');
// pub const INTEGRAL_WITH_LEFTWARDS_ARROW_WITH_HOOK: Op = Op('⨗');
// pub const INTEGRAL_WITH_TIMES_SIGN: Op = Op('⨘');
// pub const INTEGRAL_WITH_INTERSECTION: Op = Op('⨙');
// pub const INTEGRAL_WITH_UNION: Op = Op('⨚');
// pub const INTEGRAL_WITH_OVERBAR: Op = Op('⨛');
// pub const INTEGRAL_WITH_UNDERBAR: Op = Op('⨜');
// pub const JOIN: Op = Op('⨝');
// pub const LARGE_LEFT_TRIANGLE_OPERATOR: Op = Op('⨞');
pub const Z_NOTATION_SCHEMA_COMPOSITION: Rel = Rel('⨟');
// pub const Z_NOTATION_SCHEMA_PIPING: Op = Op('⨠');
// pub const Z_NOTATION_SCHEMA_PROJECTION: Op = Op('⨡');
// pub const PLUS_SIGN_WITH_SMALL_CIRCLE_ABOVE: Op = Op('⨢');
// pub const PLUS_SIGN_WITH_CIRCUMFLEX_ACCENT_ABOVE: Op = Op('⨣');
// pub const PLUS_SIGN_WITH_TILDE_ABOVE: Op = Op('⨤');
// pub const PLUS_SIGN_WITH_DOT_BELOW: Op = Op('⨥');
// pub const PLUS_SIGN_WITH_TILDE_BELOW: Op = Op('⨦');
// pub const PLUS_SIGN_WITH_SUBSCRIPT_TWO: Op = Op('⨧');
// pub const PLUS_SIGN_WITH_BLACK_TRIANGLE: Op = Op('⨨');
// pub const MINUS_SIGN_WITH_COMMA_ABOVE: Op = Op('⨩');
// pub const MINUS_SIGN_WITH_DOT_BELOW: Op = Op('⨪');
// pub const MINUS_SIGN_WITH_FALLING_DOTS: Op = Op('⨫');
// pub const MINUS_SIGN_WITH_RISING_DOTS: Op = Op('⨬');
// pub const PLUS_SIGN_IN_LEFT_HALF_CIRCLE: Op = Op('⨭');
// pub const PLUS_SIGN_IN_RIGHT_HALF_CIRCLE: Op = Op('⨮');
// pub const VECTOR_OR_CROSS_PRODUCT: Op = Op('⨯');
// pub const MULTIPLICATION_SIGN_WITH_DOT_ABOVE: Op = Op('⨰');
// pub const MULTIPLICATION_SIGN_WITH_UNDERBAR: Op = Op('⨱');
// pub const SEMIDIRECT_PRODUCT_WITH_BOTTOM_CLOSED: Op = Op('⨲');
// pub const SMASH_PRODUCT: Op = Op('⨳');
// pub const MULTIPLICATION_SIGN_IN_LEFT_HALF_CIRCLE: Op = Op('⨴');
// pub const MULTIPLICATION_SIGN_IN_RIGHT_HALF_CIRCLE: Op = Op('⨵');
// pub const CIRCLED_MULTIPLICATION_SIGN_WITH_CIRCUMFLEX_ACCENT: Op = Op('⨶');
// pub const MULTIPLICATION_SIGN_IN_DOUBLE_CIRCLE: Op = Op('⨷');
// pub const CIRCLED_DIVISION_SIGN: Op = Op('⨸');
// pub const PLUS_SIGN_IN_TRIANGLE: Op = Op('⨹');
// pub const MINUS_SIGN_IN_TRIANGLE: Op = Op('⨺');
// pub const MULTIPLICATION_SIGN_IN_TRIANGLE: Op = Op('⨻');
// pub const INTERIOR_PRODUCT: Op = Op('⨼');
// pub const RIGHTHAND_INTERIOR_PRODUCT: Op = Op('⨽');
// pub const Z_NOTATION_RELATIONAL_COMPOSITION: Op = Op('⨾');
pub const AMALGAMATION_OR_COPRODUCT: Rel = Rel('⨿');
// pub const INTERSECTION_WITH_DOT: Op = Op('⩀');
// pub const UNION_WITH_MINUS_SIGN: Op = Op('⩁');
// pub const UNION_WITH_OVERBAR: Op = Op('⩂');
// pub const INTERSECTION_WITH_OVERBAR: Op = Op('⩃');
// pub const INTERSECTION_WITH_LOGICAL_AND: Op = Op('⩄');
// pub const UNION_WITH_LOGICAL_OR: Op = Op('⩅');
// pub const UNION_ABOVE_INTERSECTION: Op = Op('⩆');
// pub const INTERSECTION_ABOVE_UNION: Op = Op('⩇');
// pub const UNION_ABOVE_BAR_ABOVE_INTERSECTION: Op = Op('⩈');
// pub const INTERSECTION_ABOVE_BAR_ABOVE_UNION: Op = Op('⩉');
// pub const UNION_BESIDE_AND_JOINED_WITH_UNION: Op = Op('⩊');
// pub const INTERSECTION_BESIDE_AND_JOINED_WITH_INTERSECTION: Op = Op('⩋');
// pub const CLOSED_UNION_WITH_SERIFS: Op = Op('⩌');
// pub const CLOSED_INTERSECTION_WITH_SERIFS: Op = Op('⩍');
// pub const DOUBLE_SQUARE_INTERSECTION: Op = Op('⩎');
// pub const DOUBLE_SQUARE_UNION: Op = Op('⩏');
// pub const CLOSED_UNION_WITH_SERIFS_AND_SMASH_PRODUCT: Op = Op('⩐');
// pub const LOGICAL_AND_WITH_DOT_ABOVE: Op = Op('⩑');
// pub const LOGICAL_OR_WITH_DOT_ABOVE: Op = Op('⩒');
// pub const DOUBLE_LOGICAL_AND: Op = Op('⩓');
// pub const DOUBLE_LOGICAL_OR: Op = Op('⩔');
// pub const TWO_INTERSECTING_LOGICAL_AND: Op = Op('⩕');
// pub const TWO_INTERSECTING_LOGICAL_OR: Op = Op('⩖');
// pub const SLOPING_LARGE_OR: Op = Op('⩗');
// pub const SLOPING_LARGE_AND: Op = Op('⩘');
// pub const LOGICAL_OR_OVERLAPPING_LOGICAL_AND: Op = Op('⩙');
// pub const LOGICAL_AND_WITH_MIDDLE_STEM: Op = Op('⩚');
// pub const LOGICAL_OR_WITH_MIDDLE_STEM: Op = Op('⩛');
// pub const LOGICAL_AND_WITH_HORIZONTAL_DASH: Op = Op('⩜');
// pub const LOGICAL_OR_WITH_HORIZONTAL_DASH: Op = Op('⩝');
pub const LOGICAL_AND_WITH_DOUBLE_OVERBAR: Rel = Rel('⩞');
// pub const LOGICAL_AND_WITH_UNDERBAR: Op = Op('⩟');
// pub const LOGICAL_AND_WITH_DOUBLE_UNDERBAR: Op = Op('⩠');
// pub const SMALL_VEE_WITH_UNDERBAR: Op = Op('⩡');
// pub const LOGICAL_OR_WITH_DOUBLE_OVERBAR: Op = Op('⩢');
// pub const LOGICAL_OR_WITH_DOUBLE_UNDERBAR: Op = Op('⩣');
// pub const Z_NOTATION_DOMAIN_ANTIRESTRICTION: Op = Op('⩤');
// pub const Z_NOTATION_RANGE_ANTIRESTRICTION: Op = Op('⩥');
pub const EQUALS_SIGN_WITH_DOT_BELOW: Rel = Rel('⩦');
// pub const IDENTICAL_WITH_DOT_ABOVE: Op = Op('⩧');
// pub const TRIPLE_HORIZONTAL_BAR_WITH_DOUBLE_VERTICAL_STROKE: Op = Op('⩨');
// pub const TRIPLE_HORIZONTAL_BAR_WITH_TRIPLE_VERTICAL_STROKE: Op = Op('⩩');
// pub const TILDE_OPERATOR_WITH_DOT_ABOVE: Op = Op('⩪');
// pub const TILDE_OPERATOR_WITH_RISING_DOTS: Op = Op('⩫');
// pub const SIMILAR_MINUS_SIMILAR: Op = Op('⩬');
// pub const CONGRUENT_WITH_DOT_ABOVE: Op = Op('⩭');
// pub const EQUALS_WITH_ASTERISK: Op = Op('⩮');
// pub const ALMOST_EQUAL_TO_WITH_CIRCUMFLEX_ACCENT: Op = Op('⩯');
// pub const APPROXIMATELY_EQUAL_OR_EQUAL_TO: Op = Op('⩰');
// pub const EQUALS_SIGN_ABOVE_PLUS_SIGN: Op = Op('⩱');
// pub const PLUS_SIGN_ABOVE_EQUALS_SIGN: Op = Op('⩲');
// pub const EQUALS_SIGN_ABOVE_TILDE_OPERATOR: Op = Op('⩳');
// pub const DOUBLE_COLON_EQUAL: Op = Op('⩴');
// pub const TWO_CONSECUTIVE_EQUALS_SIGNS: Op = Op('⩵');
// pub const THREE_CONSECUTIVE_EQUALS_SIGNS: Op = Op('⩶');
// pub const EQUALS_SIGN_WITH_TWO_DOTS_ABOVE_AND_TWO_DOTS_BELOW: Op = Op('⩷');
// pub const EQUIVALENT_WITH_FOUR_DOTS_ABOVE: Op = Op('⩸');
// pub const LESS_THAN_WITH_CIRCLE_INSIDE: Op = Op('⩹');
// pub const GREATER_THAN_WITH_CIRCLE_INSIDE: Op = Op('⩺');
// pub const LESS_THAN_WITH_QUESTION_MARK_ABOVE: Op = Op('⩻');
// pub const GREATER_THAN_WITH_QUESTION_MARK_ABOVE: Op = Op('⩼');
pub const LESS_THAN_OR_SLANTED_EQUAL_TO: Rel = Rel('⩽');
pub const GREATER_THAN_OR_SLANTED_EQUAL_TO: Rel = Rel('⩾');
// pub const LESS_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE: Op = Op('⩿');
// pub const GREATER_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE: Op = Op('⪀');
// pub const LESS_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE: Op = Op('⪁');
// pub const GREATER_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE: Op = Op('⪂');
// pub const LESS_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_RIGHT: Op = Op('⪃');
// pub const GREATER_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_LEFT: Op = Op('⪄');
pub const LESS_THAN_OR_APPROXIMATE: Rel = Rel('⪅');
pub const GREATER_THAN_OR_APPROXIMATE: Rel = Rel('⪆');
pub const LESS_THAN_AND_SINGLE_LINE_NOT_EQUAL_TO: Rel = Rel('⪇');
pub const GREATER_THAN_AND_SINGLE_LINE_NOT_EQUAL_TO: Rel = Rel('⪈');
// pub const LESS_THAN_AND_NOT_APPROXIMATE: Op = Op('⪉');
// pub const GREATER_THAN_AND_NOT_APPROXIMATE: Op = Op('⪊');
pub const LESS_THAN_ABOVE_DOUBLE_LINE_EQUAL_ABOVE_GREATER_THAN: Rel = Rel('⪋');
// pub const GREATER_THAN_ABOVE_DOUBLE_LINE_EQUAL_ABOVE_LESS_THAN: Op = Op('⪌');
// pub const LESS_THAN_ABOVE_SIMILAR_OR_EQUAL: Op = Op('⪍');
// pub const GREATER_THAN_ABOVE_SIMILAR_OR_EQUAL: Op = Op('⪎');
// pub const LESS_THAN_ABOVE_SIMILAR_ABOVE_GREATER_THAN: Op = Op('⪏');
// pub const GREATER_THAN_ABOVE_SIMILAR_ABOVE_LESS_THAN: Op = Op('⪐');
// pub const LESS_THAN_ABOVE_GREATER_THAN_ABOVE_DOUBLE_LINE_EQUAL: Op = Op('⪑');
// pub const GREATER_THAN_ABOVE_LESS_THAN_ABOVE_DOUBLE_LINE_EQUAL: Op = Op('⪒');
// pub const LESS_THAN_ABOVE_SLANTED_EQUAL_ABOVE_GREATER_THAN_ABOVE_SLANTED_EQUAL: Op = Op('⪓');
// pub const GREATER_THAN_ABOVE_SLANTED_EQUAL_ABOVE_LESS_THAN_ABOVE_SLANTED_EQUAL: Op = Op('⪔');
pub const SLANTED_EQUAL_TO_OR_LESS_THAN: Rel = Rel('⪕');
pub const SLANTED_EQUAL_TO_OR_GREATER_THAN: Rel = Rel('⪖');
// pub const SLANTED_EQUAL_TO_OR_LESS_THAN_WITH_DOT_INSIDE: Op = Op('⪗');
// pub const SLANTED_EQUAL_TO_OR_GREATER_THAN_WITH_DOT_INSIDE: Op = Op('⪘');
// pub const DOUBLE_LINE_EQUAL_TO_OR_LESS_THAN: Op = Op('⪙');
// pub const DOUBLE_LINE_EQUAL_TO_OR_GREATER_THAN: Op = Op('⪚');
// pub const DOUBLE_LINE_SLANTED_EQUAL_TO_OR_LESS_THAN: Op = Op('⪛');
// pub const DOUBLE_LINE_SLANTED_EQUAL_TO_OR_GREATER_THAN: Op = Op('⪜');
// pub const SIMILAR_OR_LESS_THAN: Op = Op('⪝');
// pub const SIMILAR_OR_GREATER_THAN: Op = Op('⪞');
// pub const SIMILAR_ABOVE_LESS_THAN_ABOVE_EQUALS_SIGN: Op = Op('⪟');
// pub const SIMILAR_ABOVE_GREATER_THAN_ABOVE_EQUALS_SIGN: Op = Op('⪠');
// pub const DOUBLE_NESTED_LESS_THAN: Op = Op('⪡');
// pub const DOUBLE_NESTED_GREATER_THAN: Op = Op('⪢');
// pub const DOUBLE_NESTED_LESS_THAN_WITH_UNDERBAR: Op = Op('⪣');
// pub const GREATER_THAN_OVERLAPPING_LESS_THAN: Op = Op('⪤');
// pub const GREATER_THAN_BESIDE_LESS_THAN: Op = Op('⪥');
// pub const LESS_THAN_CLOSED_BY_CURVE: Op = Op('⪦');
// pub const GREATER_THAN_CLOSED_BY_CURVE: Op = Op('⪧');
// pub const LESS_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL: Op = Op('⪨');
// pub const GREATER_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL: Op = Op('⪩');
// pub const SMALLER_THAN: Op = Op('⪪');
// pub const LARGER_THAN: Op = Op('⪫');
// pub const SMALLER_THAN_OR_EQUAL_TO: Op = Op('⪬');
// pub const LARGER_THAN_OR_EQUAL_TO: Op = Op('⪭');
// pub const EQUALS_SIGN_WITH_BUMPY_ABOVE: Op = Op('⪮');
pub const PRECEDES_ABOVE_SINGLE_LINE_EQUALS_SIGN: Rel = Rel('⪯');
pub const SUCCEEDS_ABOVE_SINGLE_LINE_EQUALS_SIGN: Rel = Rel('⪰');
// pub const PRECEDES_ABOVE_SINGLE_LINE_NOT_EQUAL_TO: Op = Op('⪱');
// pub const SUCCEEDS_ABOVE_SINGLE_LINE_NOT_EQUAL_TO: Op = Op('⪲');
// pub const PRECEDES_ABOVE_EQUALS_SIGN: Op = Op('⪳');
// pub const SUCCEEDS_ABOVE_EQUALS_SIGN: Op = Op('⪴');
pub const PRECEDES_ABOVE_NOT_EQUAL_TO: Rel = Rel('⪵');
pub const SUCCEEDS_ABOVE_NOT_EQUAL_TO: Rel = Rel('⪶');
pub const PRECEDES_ABOVE_ALMOST_EQUAL_TO: Rel = Rel('⪷');
pub const SUCCEEDS_ABOVE_ALMOST_EQUAL_TO: Rel = Rel('⪸');
pub const PRECEDES_ABOVE_NOT_ALMOST_EQUAL_TO: Rel = Rel('⪹');
pub const SUCCEEDS_ABOVE_NOT_ALMOST_EQUAL_TO: Rel = Rel('⪺');
// pub const DOUBLE_PRECEDES: Op = Op('⪻');
// pub const DOUBLE_SUCCEEDS: Op = Op('⪼');
// pub const SUBSET_WITH_DOT: Op = Op('⪽');
// pub const SUPERSET_WITH_DOT: Op = Op('⪾');
// pub const SUBSET_WITH_PLUS_SIGN_BELOW: Op = Op('⪿');
// pub const SUPERSET_WITH_PLUS_SIGN_BELOW: Op = Op('⫀');
// pub const SUBSET_WITH_MULTIPLICATION_SIGN_BELOW: Op = Op('⫁');
// pub const SUPERSET_WITH_MULTIPLICATION_SIGN_BELOW: Op = Op('⫂');
// pub const SUBSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE: Op = Op('⫃');
// pub const SUPERSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE: Op = Op('⫄');
// pub const SUBSET_OF_ABOVE_EQUALS_SIGN: Op = Op('⫅');
// pub const SUPERSET_OF_ABOVE_EQUALS_SIGN: Op = Op('⫆');
// pub const SUBSET_OF_ABOVE_TILDE_OPERATOR: Op = Op('⫇');
// pub const SUPERSET_OF_ABOVE_TILDE_OPERATOR: Op = Op('⫈');
// pub const SUBSET_OF_ABOVE_ALMOST_EQUAL_TO: Op = Op('⫉');
// pub const SUPERSET_OF_ABOVE_ALMOST_EQUAL_TO: Op = Op('⫊');
pub const SUBSET_OF_ABOVE_NOT_EQUAL_TO: Rel = Rel('⫋');
pub const SUPERSET_OF_ABOVE_NOT_EQUAL_TO: Rel = Rel('⫌');

//
// Unicode Block: Small Form Variants
//
pub const SMALL_REVERSE_SOLIDUS: Rel = Rel('﹨');
