IDL文法
=======

## regex
|key        | reg                                                                           |
|:----------|:------------------------------------------------------------------------------|
|integer    | /-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/                                   |
|float      | /-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/ |
|identifier | /_?[A-Za-z][0-9A-Z_a-z-]*/                                                    |
|string     | /"[^"]*"/                                                                     |
|whitespace | /[\t\n\r ]+/                                                                  |
|comment    | /\/\/.*|\/\*(.|\n)*?\*\//                                                     |
|other      | /[^\t\n\r 0-9A-Za-z]/                                                         |

## 文法

```
Definitions ::
    ExtendedAttributeList Definition Definitions
    ε

Definition ::
    CallbackOrInterface
    Namespace
    Partial
    Dictionary
    Enum
    Typedef
    ImplementsStatement

CallbackOrInterface ::
    "callback" CallbackRestOrInterface
    Interface

ArgumentNameKeyword ::
    attribute
    callback
    const
    deleter
    dictionary
    enum
    getter
    implements
    inherit
    interface
    iterable
    legacycaller
    maplike
    namespace
    partial
    required
    serializer
    setlike
    setter
    static
    stringifier
    typedef
    unrestricted

CallbackRestOrInterface ::
    CallbackRest
    Interface

Interface ::
    interface identifier Inheritance { InterfaceMembers } ;

Partial ::
    partial PartialDefinition

PartialDefinition ::
    PartialInterface
    PartialDictionary
    Namespace

PartialInterface ::
    interface identifier { InterfaceMembers } ;

InterfaceMembers ::
    ExtendedAttributeList InterfaceMember InterfaceMembers
    ε

InterfaceMember ::
    Const
    Operation
    Serializer
    Stringifier
    StaticMember
    Iterable
    ReadOnlyMember
    ReadWriteAttribute
    ReadWriteMaplike
    ReadWriteSetlike

Inheritance ::
    : identifier
    ε

Const ::
    const ConstType identifier = ConstValue ;

ConstValue ::
    BooleanLiteral
    FloatLiteral
    integer
    null

BooleanLiteral ::
    true
    false

FloatLiteral ::
    float
    -Infinity
    Infinity
    NaN

ConstType ::
    PrimitiveType Null
    identifier Null

ReadOnlyMember ::
    readonly ReadOnlyMemberRest

ReadOnlyMemberRest ::
    AttributeRest
    ReadWriteMaplike
    ReadWriteSetlike

ReadWriteAttribute ::
    inherit ReadOnly AttributeRest
    AttributeRest

AttributeRest ::
    attribute Type AttributeName ;

AttributeName ::
    AttributeNameKeyword
    identifier

AttributeNameKeyword ::
    required

Inherit ::
    inherit
    ε

ReadOnly ::
    readonly
    ε

DefaultValue ::
    ConstValue
    string
    [ ]

Operation ::
    ReturnType OperationRest
    SpecialOperation

SpecialOperation ::
    Special Specials ReturnType OperationRest

Specials ::
    Special Specials
    ε

Special ::
    getter
    setter
    deleter
    legacycaller

OperationRest ::
    OptionalIdentifier ( ArgumentList ) ;

OptionalIdentifier ::
    identifier
    ε

ArgumentList ::
    Argument Arguments
    ε

Arguments ::
    , Argument Arguments
    ε

Argument ::
    ExtendedAttributeList OptionalOrRequiredArgument

OptionalOrRequiredArgument ::
    optional Type ArgumentName Default
    Type Ellipsis ArgumentName

ArgumentName ::
    ArgumentNameKeyword
    identifier

Ellipsis ::
    ...
    ε

ReturnType ::
    Type
    void

Stringifier ::
    stringifier StringifierRest

StringifierRest ::
    ReadOnly AttributeRest
    ReturnType OperationRest
    ;

Serializer ::
    serializer SerializerRest

SerializerRest ::
    OperationRest
    = SerializationPattern ;
    ;

SerializationPattern ::
    { SerializationPatternMap }
    [ SerializationPatternList ]
    identifier

SerializationPatternMap ::
    getter
    inherit Identifiers
    identifier Identifiers
    ε

SerializationPatternList ::
    getter
    identifier Identifiers
    ε

Identifiers ::
    , identifier Identifiers
    ε

StaticMember ::
    static StaticMemberRest

StaticMemberRest ::
    ReadOnly AttributeRest
    ReturnType OperationRest

Iterable ::
    iterable < Type OptionalType > ;

OptionalType ::
    , Type
    ε

ReadWriteMaplike ::
    MaplikeRest

MaplikeRest ::
    maplike < Type , Type > ;

ReadWriteSetlike ::
    SetlikeRest

SetlikeRest ::
    setlike < Type > ;

Namespace ::
    namespace identifier { NamespaceMembers } ;

NamespaceMembers ::
    ExtendedAttributeList NamespaceMember NamespaceMembers
    ε

NamespaceMember ::
    ReturnType OperationRest
    readonly AttributeRest

Dictionary ::
    dictionary identifier Inheritance { DictionaryMembers } ;

DictionaryMembers ::
    ExtendedAttributeList DictionaryMember DictionaryMembers
    ε

DictionaryMember ::
    Required Type identifier Default ;

Required ::
    required
    ε

PartialDictionary ::
    dictionary identifier { DictionaryMembers } ;

Default ::
    = DefaultValue
    ε

Enum ::
    enum identifier { EnumValueList } ;

EnumValueList ::
    string EnumValueListComma

EnumValueListComma ::
    , EnumValueListString
    ε

EnumValueListString ::
    string EnumValueListComma
    ε

CallbackRest ::
    identifier = ReturnType ( ArgumentList ) ;

Typedef ::
    typedef Type identifier ;

ImplementsStatement ::
    identifier implements identifier ;

Type ::
    SingleType
    UnionType Null

SingleType ::
    NonAnyType
    any

UnionType ::
    ( UnionMemberType or UnionMemberType UnionMemberTypes )

UnionMemberType ::
    NonAnyType
    UnionType Null

UnionMemberTypes ::
    or UnionMemberType UnionMemberTypes
    ε

NonAnyType ::
    PromiseType ε
    PrimitiveType Null
    StringType Null
    identifier Null
    sequence < Type > Null
    object Null
    Error Null
    DOMException Null
    BufferRelatedType Null
    FrozenArray < Type > Null
    RecordType Null

PrimitiveType ::
    UnsignedIntegerType
    UnrestrictedFloatType
    boolean
    byte
    octet

UnrestrictedFloatType ::
    unrestricted FloatType
    FloatType

FloatType ::
    float
    double

UnsignedIntegerType ::
    unsigned IntegerType
    IntegerType

IntegerType ::
    short
    long OptionalLong

OptionalLong ::
    long
    ε

StringType ::
    ByteString
    DOMString
    USVString

PromiseType ::
    Promise < ReturnType >

RecordType ::
    record < StringType , Type >

Null ::
    ?
    ε

BufferRelatedType ::
    ArrayBuffer
    DataView
    Int8Array
    Int16Array
    Int32Array
    Uint8Array
    Uint16Array
    Uint32Array
    Uint8ClampedArray
    Float32Array
    Float64Array

ExtendedAttributeList ::
    [ ExtendedAttribute ExtendedAttributes ]
    ε

ExtendedAttributes ::
    , ExtendedAttribute ExtendedAttributes
    ε

ExtendedAttribute ::
    ( ExtendedAttributeInner ) ExtendedAttributeRest
    [ ExtendedAttributeInner ] ExtendedAttributeRest
    { ExtendedAttributeInner } ExtendedAttributeRest
    Other ExtendedAttributeRest

ExtendedAttributeRest ::
    ExtendedAttribute
    ε

ExtendedAttributeInner ::
    ( ExtendedAttributeInner ) ExtendedAttributeInner
    [ ExtendedAttributeInner ] ExtendedAttributeInner
    { ExtendedAttributeInner } ExtendedAttributeInner
    OtherOrComma ExtendedAttributeInner
    ε

Other ::
    integer
    float
    identifier
    string
    other
    -
    -Infinity
    .
    ...
    :
    ;
    <
    =
    >
    ?
    ByteString
    DOMString
    FrozenArray
    Infinity
    NaN
    USVString
    any
    boolean
    byte
    double
    false
    float
    long
    null
    object
    octet
    or
    optional
    sequence
    short
    true
    unsigned
    void
    ArgumentNameKeyword
    BufferRelatedType

OtherOrComma ::
    Other
    ,

IdentifierList ::
    identifier Identifiers

ExtendedAttributeNoArgs ::
    identifier

ExtendedAttributeArgList ::
    identifier ( ArgumentList )

ExtendedAttributeIdent ::
    identifier = identifier

ExtendedAttributeIdentList ::
    identifier = ( IdentifierList )

ExtendedAttributeNamedArgList ::
    identifier = identifier ( ArgumentList )
```
