// Autogenerated file. To regenerate, please run `cargo run --bin generate-syntax`.
use core::fmt;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SyntaxKind {
    Trivia,
    ExprList,
    Arg,
    ArgClauseNamed,
    ArgClauseUnnamed,
    ArgClauseFieldInitShorthand,
    ExprFieldInitShorthand,
    ArgList,
    ExprMissing,
    PathSegmentSimple,
    OptionTerminalColonColonEmpty,
    PathSegmentWithGenericArgs,
    ExprPath,
    ExprParenthesized,
    ExprUnary,
    ExprBinary,
    ExprListParenthesized,
    ExprFunctionCall,
    ArgListParenthesized,
    OptionArgListParenthesizedEmpty,
    ExprStructCtorCall,
    ExprBlock,
    ExprMatch,
    MatchArms,
    MatchArm,
    ExprIf,
    ExprLoop,
    ElseClause,
    OptionElseClauseEmpty,
    ExprErrorPropagate,
    ExprIndexed,
    ExprInlineMacro,
    StructArgExpr,
    OptionStructArgExprEmpty,
    StructArgSingle,
    StructArgTail,
    StructArgList,
    ArgListBraced,
    ExprListBracketed,
    ExprListBraced,
    PatternIdentifier,
    PatternStruct,
    PatternStructParamList,
    PatternTuple,
    PatternList,
    PatternStructParamWithExpr,
    PatternEnum,
    PatternEnumInnerPattern,
    OptionPatternEnumInnerPatternEmpty,
    TypeClause,
    OptionTypeClauseEmpty,
    ReturnTypeClause,
    OptionReturnTypeClauseEmpty,
    StatementList,
    StatementMissing,
    StatementLet,
    OptionTerminalSemicolonEmpty,
    StatementExpr,
    StatementContinue,
    ExprClause,
    OptionExprClauseEmpty,
    StatementReturn,
    StatementBreak,
    Param,
    ModifierList,
    ParamList,
    ImplicitsClause,
    ImplicitsList,
    OptionImplicitsClauseEmpty,
    OptionTerminalNoPanicEmpty,
    FunctionSignature,
    Member,
    MemberList,
    Variant,
    VariantList,
    ItemList,
    ItemMissing,
    Attribute,
    AttributeList,
    ItemModule,
    ModuleBody,
    FunctionDeclaration,
    ItemConstant,
    FunctionWithBody,
    ItemExternFunction,
    ItemExternType,
    ItemTrait,
    TraitBody,
    TraitItemList,
    TraitItemMissing,
    TraitItemFunction,
    ItemImpl,
    ImplBody,
    ImplItemList,
    ImplItemMissing,
    ItemImplAlias,
    ItemStruct,
    ItemEnum,
    ItemTypeAlias,
    ItemUse,
    UsePathLeaf,
    UsePathSingle,
    UsePathMulti,
    UsePathList,
    AliasClause,
    OptionAliasClauseEmpty,
    GenericArgExpr,
    GenericArgs,
    GenericArgList,
    OptionWrappedGenericParamListEmpty,
    WrappedGenericParamList,
    GenericParamList,
    GenericParamType,
    GenericParamConst,
    GenericParamImpl,
    TokenIdentifier,
    TerminalIdentifier,
    TokenLiteralNumber,
    TerminalLiteralNumber,
    TokenShortString,
    TerminalShortString,
    TokenAs,
    TerminalAs,
    TokenConst,
    TerminalConst,
    TokenElse,
    TerminalElse,
    TokenEnum,
    TerminalEnum,
    TokenExtern,
    TerminalExtern,
    TokenFalse,
    TerminalFalse,
    TokenFunction,
    TerminalFunction,
    TokenIf,
    TerminalIf,
    TokenLoop,
    TerminalLoop,
    TokenImpl,
    TerminalImpl,
    TokenImplicits,
    TerminalImplicits,
    TokenLet,
    TerminalLet,
    TokenMatch,
    TerminalMatch,
    TokenModule,
    TerminalModule,
    TokenMut,
    TerminalMut,
    TokenNoPanic,
    TerminalNoPanic,
    TokenOf,
    TerminalOf,
    TokenRef,
    TerminalRef,
    TokenContinue,
    TerminalContinue,
    TokenReturn,
    TerminalReturn,
    TokenBreak,
    TerminalBreak,
    TokenStruct,
    TerminalStruct,
    TokenTrait,
    TerminalTrait,
    TokenTrue,
    TerminalTrue,
    TokenType,
    TerminalType,
    TokenUse,
    TerminalUse,
    TokenAnd,
    TerminalAnd,
    TokenAndAnd,
    TerminalAndAnd,
    TokenArrow,
    TerminalArrow,
    TokenAt,
    TerminalAt,
    TokenBadCharacters,
    TerminalBadCharacters,
    TokenColon,
    TerminalColon,
    TokenColonColon,
    TerminalColonColon,
    TokenComma,
    TerminalComma,
    TokenDiv,
    TerminalDiv,
    TokenDivEq,
    TerminalDivEq,
    TokenDot,
    TerminalDot,
    TokenDotDot,
    TerminalDotDot,
    TokenEndOfFile,
    TerminalEndOfFile,
    TokenEq,
    TerminalEq,
    TokenEqEq,
    TerminalEqEq,
    TokenGE,
    TerminalGE,
    TokenGT,
    TerminalGT,
    TokenHash,
    TerminalHash,
    TokenLBrace,
    TerminalLBrace,
    TokenLBrack,
    TerminalLBrack,
    TokenLE,
    TerminalLE,
    TokenLParen,
    TerminalLParen,
    TokenLT,
    TerminalLT,
    TokenMatchArrow,
    TerminalMatchArrow,
    TokenMinus,
    TerminalMinus,
    TokenMinusEq,
    TerminalMinusEq,
    TokenMod,
    TerminalMod,
    TokenModEq,
    TerminalModEq,
    TokenMul,
    TerminalMul,
    TokenMulEq,
    TerminalMulEq,
    TokenNeq,
    TerminalNeq,
    TokenNot,
    TerminalNot,
    TokenBitNot,
    TerminalBitNot,
    TokenOr,
    TerminalOr,
    TokenOrOr,
    TerminalOrOr,
    TokenPlus,
    TerminalPlus,
    TokenPlusEq,
    TerminalPlusEq,
    TokenQuestionMark,
    TerminalQuestionMark,
    TokenRBrace,
    TerminalRBrace,
    TokenRBrack,
    TerminalRBrack,
    TokenRParen,
    TerminalRParen,
    TokenSemicolon,
    TerminalSemicolon,
    TokenUnderscore,
    TerminalUnderscore,
    TokenXor,
    TerminalXor,
    SyntaxFile,
    TokenSingleLineComment,
    TokenWhitespace,
    TokenNewline,
    TokenMissing,
    TokenSkipped,
}
impl SyntaxKind {
    pub fn is_token(&self) -> bool {
        matches!(
            *self,
            SyntaxKind::TokenIdentifier
                | SyntaxKind::TokenLiteralNumber
                | SyntaxKind::TokenShortString
                | SyntaxKind::TokenAs
                | SyntaxKind::TokenConst
                | SyntaxKind::TokenElse
                | SyntaxKind::TokenEnum
                | SyntaxKind::TokenExtern
                | SyntaxKind::TokenFalse
                | SyntaxKind::TokenFunction
                | SyntaxKind::TokenIf
                | SyntaxKind::TokenLoop
                | SyntaxKind::TokenImpl
                | SyntaxKind::TokenImplicits
                | SyntaxKind::TokenLet
                | SyntaxKind::TokenMatch
                | SyntaxKind::TokenModule
                | SyntaxKind::TokenMut
                | SyntaxKind::TokenNoPanic
                | SyntaxKind::TokenOf
                | SyntaxKind::TokenRef
                | SyntaxKind::TokenContinue
                | SyntaxKind::TokenReturn
                | SyntaxKind::TokenBreak
                | SyntaxKind::TokenStruct
                | SyntaxKind::TokenTrait
                | SyntaxKind::TokenTrue
                | SyntaxKind::TokenType
                | SyntaxKind::TokenUse
                | SyntaxKind::TokenAnd
                | SyntaxKind::TokenAndAnd
                | SyntaxKind::TokenArrow
                | SyntaxKind::TokenAt
                | SyntaxKind::TokenBadCharacters
                | SyntaxKind::TokenColon
                | SyntaxKind::TokenColonColon
                | SyntaxKind::TokenComma
                | SyntaxKind::TokenDiv
                | SyntaxKind::TokenDivEq
                | SyntaxKind::TokenDot
                | SyntaxKind::TokenDotDot
                | SyntaxKind::TokenEndOfFile
                | SyntaxKind::TokenEq
                | SyntaxKind::TokenEqEq
                | SyntaxKind::TokenGE
                | SyntaxKind::TokenGT
                | SyntaxKind::TokenHash
                | SyntaxKind::TokenLBrace
                | SyntaxKind::TokenLBrack
                | SyntaxKind::TokenLE
                | SyntaxKind::TokenLParen
                | SyntaxKind::TokenLT
                | SyntaxKind::TokenMatchArrow
                | SyntaxKind::TokenMinus
                | SyntaxKind::TokenMinusEq
                | SyntaxKind::TokenMod
                | SyntaxKind::TokenModEq
                | SyntaxKind::TokenMul
                | SyntaxKind::TokenMulEq
                | SyntaxKind::TokenNeq
                | SyntaxKind::TokenNot
                | SyntaxKind::TokenBitNot
                | SyntaxKind::TokenOr
                | SyntaxKind::TokenOrOr
                | SyntaxKind::TokenPlus
                | SyntaxKind::TokenPlusEq
                | SyntaxKind::TokenQuestionMark
                | SyntaxKind::TokenRBrace
                | SyntaxKind::TokenRBrack
                | SyntaxKind::TokenRParen
                | SyntaxKind::TokenSemicolon
                | SyntaxKind::TokenUnderscore
                | SyntaxKind::TokenXor
                | SyntaxKind::TokenSingleLineComment
                | SyntaxKind::TokenWhitespace
                | SyntaxKind::TokenNewline
                | SyntaxKind::TokenMissing
                | SyntaxKind::TokenSkipped
        )
    }
    pub fn is_terminal(&self) -> bool {
        matches!(
            *self,
            SyntaxKind::TerminalIdentifier
                | SyntaxKind::TerminalLiteralNumber
                | SyntaxKind::TerminalShortString
                | SyntaxKind::TerminalAs
                | SyntaxKind::TerminalConst
                | SyntaxKind::TerminalElse
                | SyntaxKind::TerminalEnum
                | SyntaxKind::TerminalExtern
                | SyntaxKind::TerminalFalse
                | SyntaxKind::TerminalFunction
                | SyntaxKind::TerminalIf
                | SyntaxKind::TerminalLoop
                | SyntaxKind::TerminalImpl
                | SyntaxKind::TerminalImplicits
                | SyntaxKind::TerminalLet
                | SyntaxKind::TerminalMatch
                | SyntaxKind::TerminalModule
                | SyntaxKind::TerminalMut
                | SyntaxKind::TerminalNoPanic
                | SyntaxKind::TerminalOf
                | SyntaxKind::TerminalRef
                | SyntaxKind::TerminalContinue
                | SyntaxKind::TerminalReturn
                | SyntaxKind::TerminalBreak
                | SyntaxKind::TerminalStruct
                | SyntaxKind::TerminalTrait
                | SyntaxKind::TerminalTrue
                | SyntaxKind::TerminalType
                | SyntaxKind::TerminalUse
                | SyntaxKind::TerminalAnd
                | SyntaxKind::TerminalAndAnd
                | SyntaxKind::TerminalArrow
                | SyntaxKind::TerminalAt
                | SyntaxKind::TerminalBadCharacters
                | SyntaxKind::TerminalColon
                | SyntaxKind::TerminalColonColon
                | SyntaxKind::TerminalComma
                | SyntaxKind::TerminalDiv
                | SyntaxKind::TerminalDivEq
                | SyntaxKind::TerminalDot
                | SyntaxKind::TerminalDotDot
                | SyntaxKind::TerminalEndOfFile
                | SyntaxKind::TerminalEq
                | SyntaxKind::TerminalEqEq
                | SyntaxKind::TerminalGE
                | SyntaxKind::TerminalGT
                | SyntaxKind::TerminalHash
                | SyntaxKind::TerminalLBrace
                | SyntaxKind::TerminalLBrack
                | SyntaxKind::TerminalLE
                | SyntaxKind::TerminalLParen
                | SyntaxKind::TerminalLT
                | SyntaxKind::TerminalMatchArrow
                | SyntaxKind::TerminalMinus
                | SyntaxKind::TerminalMinusEq
                | SyntaxKind::TerminalMod
                | SyntaxKind::TerminalModEq
                | SyntaxKind::TerminalMul
                | SyntaxKind::TerminalMulEq
                | SyntaxKind::TerminalNeq
                | SyntaxKind::TerminalNot
                | SyntaxKind::TerminalBitNot
                | SyntaxKind::TerminalOr
                | SyntaxKind::TerminalOrOr
                | SyntaxKind::TerminalPlus
                | SyntaxKind::TerminalPlusEq
                | SyntaxKind::TerminalQuestionMark
                | SyntaxKind::TerminalRBrace
                | SyntaxKind::TerminalRBrack
                | SyntaxKind::TerminalRParen
                | SyntaxKind::TerminalSemicolon
                | SyntaxKind::TerminalUnderscore
                | SyntaxKind::TerminalXor
        )
    }
    pub fn is_keyword_token(&self) -> bool {
        matches!(
            *self,
            SyntaxKind::TokenAs
                | SyntaxKind::TokenConst
                | SyntaxKind::TokenElse
                | SyntaxKind::TokenEnum
                | SyntaxKind::TokenExtern
                | SyntaxKind::TokenFalse
                | SyntaxKind::TokenFunction
                | SyntaxKind::TokenIf
                | SyntaxKind::TokenLoop
                | SyntaxKind::TokenImpl
                | SyntaxKind::TokenImplicits
                | SyntaxKind::TokenLet
                | SyntaxKind::TokenMatch
                | SyntaxKind::TokenModule
                | SyntaxKind::TokenMut
                | SyntaxKind::TokenNoPanic
                | SyntaxKind::TokenOf
                | SyntaxKind::TokenRef
                | SyntaxKind::TokenContinue
                | SyntaxKind::TokenReturn
                | SyntaxKind::TokenBreak
                | SyntaxKind::TokenStruct
                | SyntaxKind::TokenTrait
                | SyntaxKind::TokenTrue
                | SyntaxKind::TokenType
                | SyntaxKind::TokenUse
        )
    }
    pub fn is_keyword_terminal(&self) -> bool {
        matches!(
            *self,
            SyntaxKind::TerminalAs
                | SyntaxKind::TerminalConst
                | SyntaxKind::TerminalElse
                | SyntaxKind::TerminalEnum
                | SyntaxKind::TerminalExtern
                | SyntaxKind::TerminalFalse
                | SyntaxKind::TerminalFunction
                | SyntaxKind::TerminalIf
                | SyntaxKind::TerminalLoop
                | SyntaxKind::TerminalImpl
                | SyntaxKind::TerminalImplicits
                | SyntaxKind::TerminalLet
                | SyntaxKind::TerminalMatch
                | SyntaxKind::TerminalModule
                | SyntaxKind::TerminalMut
                | SyntaxKind::TerminalNoPanic
                | SyntaxKind::TerminalOf
                | SyntaxKind::TerminalRef
                | SyntaxKind::TerminalContinue
                | SyntaxKind::TerminalReturn
                | SyntaxKind::TerminalBreak
                | SyntaxKind::TerminalStruct
                | SyntaxKind::TerminalTrait
                | SyntaxKind::TerminalTrue
                | SyntaxKind::TerminalType
                | SyntaxKind::TerminalUse
        )
    }
}
impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
