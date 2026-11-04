//! Matchers: words.

use super::super::engine::Lexer;
use crate::classes::{is_letter, is_mark, is_number, is_word};
use crate::rules::Chars;

/// The plain words of `KeywordsAt16`'s third alternative.
const KEYWORDS_AT_16: &[&[u8]] = &[
    b"deinit",
    b"didSet",
    b"func",
    b"set",
    b"subscript",
    b"willSet",
];

/// The words of `BuiltinTypesBuiltinClassType`'s regex.
const BUILTIN_TYPES_BUILTIN_CLASS_TYPE: &[&[u8]] = &[
    b"AnyGenerator",
    b"ManagedBuffer",
    b"ManagedProtoBuffer",
    b"NonObjectiveCBase",
];

/// The words of `BuiltinTypesBuiltinEnumTypeAt2`'s regex.
const BUILTIN_TYPES_BUILTIN_ENUM_TYPE_AT2: &[&[u8]] = &[
    b"AncestorRepresentation",
    b"Bit",
    b"DictionaryIndexRepresentation",
    b"DictionaryIteratorRepresentation",
    b"DisplayStyle",
    b"FloatingPointClassification",
    b"FloatingPointRoundingRule",
    b"FloatingPointSign",
    b"ImplicitlyUnwrappedOptional",
    b"MemoryLayout",
    b"Never",
    b"Optional",
    b"PlaygroundQuickLook",
    b"Representation",
    b"SetIndexRepresentation",
    b"SetIteratorRepresentation",
    b"UnicodeDecodingResult",
];

/// The words of `BuiltinTypesBuiltinEnumTypeAt3`'s regex.
const BUILTIN_TYPES_BUILTIN_ENUM_TYPE_AT3: &[&[u8]] = &[b"MirrorDisposition", b"QuickLookObject"];

/// The words of `BuiltinTypesBuiltinProtocolTypeAt1`'s regex.
const BUILTIN_TYPES_BUILTIN_PROTOCOL_TYPE_AT1: &[&[u8]] = &[
    b"AbsoluteValuable",
    b"AnyObject",
    b"BidirectionalCollection",
    b"BidirectionalIndexable",
    b"BinaryFloatingPoint",
    b"BitwiseOperations",
    b"CVarArg",
    b"Collection",
    b"Comparable",
    b"CustomDebugStringConvertible",
    b"CustomLeafReflectable",
    b"CustomPlaygroundQuickLookable",
    b"CustomReflectable",
    b"CustomStringConvertible",
    b"Equatable",
    b"Error",
    b"ExpressibleByArrayLiteral",
    b"ExpressibleByBooleanLiteral",
    b"ExpressibleByDictionaryLiteral",
    b"ExpressibleByExtendedGraphemeClusterLiteral",
    b"ExpressibleByFloatLiteral",
    b"ExpressibleByIntegerLiteral",
    b"ExpressibleByNilLiteral",
    b"ExpressibleByStringInterpolation",
    b"ExpressibleByStringLiteral",
    b"ExpressibleByUnicodeScalarLiteral",
    b"FloatingPoint",
    b"Hashable",
    b"Indexable",
    b"IndexableBase",
    b"Integer",
    b"IntegerArithmetic",
    b"IteratorProtocol",
    b"LazyCollectionProtocol",
    b"LazySequenceProtocol",
    b"LosslessStringConvertible",
    b"MirrorPath",
    b"MutableCollection",
    b"MutableIndexable",
    b"OptionSet",
    b"RandomAccessCollection",
    b"RandomAccessIndexable",
    b"RangeReplaceableCollection",
    b"RangeReplaceableIndexable",
    b"RawRepresentable",
    b"Sequence",
    b"SetAlgebra",
    b"SignedInteger",
    b"SignedNumber",
    b"Streamable",
    b"Strideable",
    b"TextOutputStream",
    b"UnicodeCodec",
    b"UnsignedInteger",
];

/// The words of `BuiltinTypesBuiltinProtocolTypeAt2`'s regex.
const BUILTIN_TYPES_BUILTIN_PROTOCOL_TYPE_AT2: &[&[u8]] = &[
    b"AnyCollectionType",
    b"ArrayLiteralConvertible",
    b"BidirectionalIndexType",
    b"BitwiseOperationsType",
    b"BooleanLiteralConvertible",
    b"BooleanType",
    b"CVarArgType",
    b"CollectionType",
    b"DictionaryLiteralConvertible",
    b"ErrorType",
    b"ExtendedGraphemeClusterLiteralConvertible",
    b"ExtensibleCollectionType",
    b"FloatLiteralConvertible",
    b"FloatingPointType",
    b"ForwardIndexType",
    b"GeneratorType",
    b"IntegerArithmeticType",
    b"IntegerLiteralConvertible",
    b"IntegerType",
    b"IntervalType",
    b"MirrorPathType",
    b"MirrorType",
    b"MutableCollectionType",
    b"MutableSliceable",
    b"NilLiteralConvertible",
    b"OptionSetType",
    b"OutputStreamType",
    b"RandomAccessIndexType",
    b"RangeReplaceableCollectionType",
    b"SequenceType",
    b"SetAlgebraType",
    b"SignedIntegerType",
    b"SignedNumberType",
    b"SinkType",
    b"Sliceable",
    b"StringInterpolationConvertible",
    b"StringLiteralConvertible",
    b"UnicodeCodecType",
    b"UnicodeScalarLiteralConvertible",
    b"UnsignedIntegerType",
];

/// The words of `BuiltinTypesBuiltinStructTypeAt0`'s regex.
const BUILTIN_TYPES_BUILTIN_STRUCT_TYPE_AT0: &[&[u8]] = &[
    b"AnyBidirectionalCollection",
    b"AnyCollection",
    b"AnyGenerator",
    b"AnyHashable",
    b"AnyIndex",
    b"AnyIterator",
    b"AnyRandomAccessCollection",
    b"AnySequence",
    b"Array",
    b"ArraySlice",
    b"AutoreleasingUnsafeMutablePointer",
    b"BidirectionalSlice",
    b"Bool",
    b"COpaquePointer",
    b"CVaListPointer",
    b"Character",
    b"CharacterView",
    b"ClosedInterval",
    b"ClosedRange",
    b"ClosedRangeIndex",
    b"ClosedRangeIterator",
    b"CollectionOfOne",
    b"ContiguousArray",
    b"CountableClosedRange",
    b"CountableRange",
    b"DefaultBidirectionalIndices",
    b"DefaultIndices",
    b"DefaultRandomAccessIndices",
    b"Dictionary",
    b"DictionaryIndex",
    b"DictionaryIterator",
    b"DictionaryLiteral",
    b"Double",
    b"EmptyCollection",
    b"EmptyGenerator",
    b"EmptyIterator",
    b"EnumerateGenerator",
    b"EnumerateSequence",
    b"EnumeratedIterator",
    b"EnumeratedSequence",
    b"FlattenBidirectionalCollection",
    b"FlattenBidirectionalCollectionIndex",
    b"FlattenCollection",
    b"FlattenCollectionIndex",
    b"FlattenGenerator",
    b"FlattenIterator",
    b"FlattenSequence",
    b"Float",
    b"Float80",
    b"GeneratorOfOne",
    b"GeneratorSequence",
    b"HalfOpenInterval",
    b"Index",
    b"IndexingGenerator",
    b"IndexingIterator",
    b"Indices",
    b"Int",
    b"Int16",
    b"Int32",
    b"Int64",
    b"Int8",
    b"Iterator",
    b"IteratorOverOne",
    b"IteratorSequence",
    b"JoinGenerator",
    b"JoinedIterator",
    b"JoinedSequence",
    b"LazyBidirectionalCollection",
    b"LazyCollection",
    b"LazyFilterBidirectionalCollection",
    b"LazyFilterCollection",
    b"LazyFilterGenerator",
    b"LazyFilterIndex",
    b"LazyFilterIterator",
    b"LazyFilterSequence",
    b"LazyMapBidirectionalCollection",
    b"LazyMapCollection",
    b"LazyMapGenerator",
    b"LazyMapIterator",
    b"LazyMapRandomAccessCollection",
    b"LazyMapSequence",
    b"LazyRandomAccessCollection",
    b"LazySequence",
    b"LegacyChildren",
    b"ManagedBufferPointer",
    b"Mirror",
    b"MutableBidirectionalSlice",
    b"MutableRandomAccessSlice",
    b"MutableRangeReplaceableBidirectionalSlice",
    b"MutableRangeReplaceableRandomAccessSlice",
    b"MutableRangeReplaceableSlice",
    b"MutableSlice",
    b"ObjectIdentifier",
    b"OpaquePointer",
    b"PermutationGenerator",
    b"RandomAccessSlice",
    b"Range",
    b"RangeGenerator",
    b"RangeReplaceableBidirectionalSlice",
    b"RangeReplaceableRandomAccessSlice",
    b"RangeReplaceableSlice",
    b"Repeat",
    b"Repeated",
    b"ReversedCollection",
    b"ReversedIndex",
    b"ReversedRandomAccessCollection",
    b"ReversedRandomAccessIndex",
    b"Set",
    b"SetIndex",
    b"SetIterator",
    b"Slice",
    b"StaticString",
    b"StrideThrough",
    b"StrideThroughGenerator",
    b"StrideThroughIterator",
    b"StrideTo",
    b"StrideToGenerator",
    b"StrideToIterator",
    b"String",
    b"UInt",
    b"UInt16",
    b"UInt32",
    b"UInt64",
    b"UInt8",
    b"UTF16",
    b"UTF16View",
    b"UTF32",
    b"UTF8",
    b"UTF8View",
    b"UnfoldSequence",
    b"UnicodeScalar",
    b"UnicodeScalarView",
    b"Unmanaged",
    b"UnsafeBufferPointer",
    b"UnsafeBufferPointerGenerator",
    b"UnsafeBufferPointerIterator",
    b"UnsafeMutableBufferPointer",
    b"UnsafeMutablePointer",
    b"UnsafeMutableRawPointer",
    b"UnsafePointer",
    b"UnsafeRawPointer",
    b"Zip2Iterator",
    b"Zip2Sequence",
];

/// The words of `BuiltinTypesBuiltinStructTypeAt1`'s regex.
const BUILTIN_TYPES_BUILTIN_STRUCT_TYPE_AT1: &[&[u8]] = &[
    b"AnyBidirectionalIndex",
    b"AnyForwardCollection",
    b"AnyForwardIndex",
    b"AnyRandomAccessIndex",
    b"DictionaryGenerator",
    b"FilterCollection",
    b"FilterCollectionIndex",
    b"FilterGenerator",
    b"FilterSequence",
    b"LazyForwardCollection",
    b"MapCollection",
    b"MapGenerator",
    b"MapSequence",
    b"RawByte",
    b"ReverseCollection",
    b"ReverseIndex",
    b"ReverseRandomAccessCollection",
    b"ReverseRandomAccessIndex",
    b"SetGenerator",
    b"SinkOf",
    b"Zip2Generator",
];

/// The words of `BuiltinTypesBuiltinTypealiasAt0`'s regex.
const BUILTIN_TYPES_BUILTIN_TYPEALIAS_AT0: &[&[u8]] = &[
    b"AnyClass",
    b"Base",
    b"BooleanLiteralType",
    b"Buffer",
    b"CBool",
    b"CChar",
    b"CChar16",
    b"CChar32",
    b"CDouble",
    b"CFloat",
    b"CInt",
    b"CLong",
    b"CLongLong",
    b"CShort",
    b"CSignedChar",
    b"CUnsignedChar",
    b"CUnsignedInt",
    b"CUnsignedLong",
    b"CUnsignedLongLong",
    b"CUnsignedShort",
    b"CWideChar",
    b"Child",
    b"Children",
    b"CodeUnit",
    b"Distance",
    b"Element",
    b"Elements",
    b"Exponent",
    b"ExtendedGraphemeClusterLiteralType",
    b"ExtendedGraphemeClusterType",
    b"Float32",
    b"Float64",
    b"FloatLiteralType",
    b"Index",
    b"IndexDistance",
    b"Indices",
    b"IntMax",
    b"IntegerLiteralType",
    b"Iterator",
    b"Key",
    b"NativeBuffer",
    b"RawExponent",
    b"RawSignificand",
    b"RawValue",
    b"Storage",
    b"Stream1",
    b"Stream2",
    b"Stride",
    b"StringLiteralType",
    b"SubSequence",
    b"UIntMax",
    b"UTF16Index",
    b"UTF16View",
    b"UTF8Index",
    b"UnfoldFirstSequence",
    b"UnicodeScalarIndex",
    b"UnicodeScalarLiteralType",
    b"UnicodeScalarType",
    b"UnicodeScalarView",
    b"Value",
    b"Void",
];

/// The words of `BuiltinTypesBuiltinTypealiasAt1`'s regex.
const BUILTIN_TYPES_BUILTIN_TYPEALIAS_AT1: &[&[u8]] =
    &[b"Generator", b"PlaygroundQuickLook", b"UWord", b"Word"];

/// The words of `BuiltinFunctionsAt0`'s regex.
const BUILTIN_FUNCTIONS_AT0: &[&[u8]] = &[
    b"contains",
    b"filter",
    b"first",
    b"flatMap",
    b"forEach",
    b"index",
    b"map",
    b"max",
    b"min",
    b"partition",
    b"sort",
    b"sorted",
    b"split",
    b"withCString",
    b"withMutableCharacters",
    b"withUTF8Buffer",
    b"withUnsafeBufferPointer",
    b"withUnsafeMutableBufferPointer",
    b"withUnsafeMutablePointerToElements",
    b"withUnsafeMutablePointerToHeader",
    b"withUnsafeMutablePointers",
];

/// The words of `BuiltinFunctionsAt1`'s regex.
const BUILTIN_FUNCTIONS_AT1: &[&[u8]] = &[
    b"abs",
    b"add",
    b"addProduct",
    b"addWithOverflow",
    b"adding",
    b"addingProduct",
    b"advanced",
    b"advancedBy",
    b"alloc",
    b"allocate",
    b"append",
    b"appendContentsOf",
    b"assign",
    b"assignBackwardFrom",
    b"assignFrom",
    b"assumingMemoryBound",
    b"autorelease",
    b"bindMemory",
    b"clamp",
    b"clamped",
    b"copy",
    b"copyBytes",
    b"countByEnumerating",
    b"create",
    b"dealloc",
    b"deallocate",
    b"deallocateCapacity",
    b"decode",
    b"decodeCString",
    b"deinitialize",
    b"descendant",
    b"destroy",
    b"distance",
    b"distanceTo",
    b"divide",
    b"divideWithOverflow",
    b"divided",
    b"dropFirst",
    b"dropLast",
    b"elementsEqual",
    b"encode",
    b"enumerate",
    b"enumerated",
    b"escape",
    b"escaped",
    b"exclusiveOr",
    b"exclusiveOrInPlace",
    b"flatten",
    b"formIndex",
    b"formIntersection",
    b"formRemainder",
    b"formSquareRoot",
    b"formSymmetricDifference",
    b"formTruncatingRemainder",
    b"formUnion",
    b"fromCString",
    b"fromCStringRepairingIllFormedUTF8",
    b"fromOpaque",
    b"generate",
    b"getElement",
    b"getObjects",
    b"hasPrefix",
    b"hasSuffix",
    b"holdsUniqueOrPinnedReference",
    b"holdsUniqueReference",
    b"indexForKey",
    b"indexOf",
    b"initialize",
    b"initializeFrom",
    b"initializeMemory",
    b"insert",
    b"insertContentsOf",
    b"intersect",
    b"intersectInPlace",
    b"intersection",
    b"isContinuation",
    b"isDisjoint",
    b"isDisjointWith",
    b"isEqual",
    b"isLeadSurrogate",
    b"isLess",
    b"isLessThanOrEqualTo",
    b"isMutableAndUniquelyReferenced",
    b"isMutableAndUniquelyReferencedOrPinned",
    b"isStrictSubset",
    b"isStrictSubsetOf",
    b"isStrictSuperset",
    b"isStrictSupersetOf",
    b"isSubset",
    b"isSubsetOf",
    b"isSuperset",
    b"isSupersetOf",
    b"isTotallyOrdered",
    b"isTrailSurrogate",
    b"isUniqueReference",
    b"isUniquelyReferenced",
    b"isUniquelyReferencedOrPinned",
    b"joinWithSeparator",
    b"joined",
    b"leadSurrogate",
    b"lexicographicalCompare",
    b"lexicographicallyPrecedes",
    b"load",
    b"lowercased",
    b"makeDescription",
    b"makeIterator",
    b"maxElement",
    b"maximum",
    b"maximumMagnitude",
    b"measure",
    b"minElement",
    b"minimum",
    b"minimumMagnitude",
    b"move",
    b"moveAssign",
    b"moveAssignFrom",
    b"moveInitialize",
    b"moveInitializeBackwardFrom",
    b"moveInitializeFrom",
    b"moveInitializeMemory",
    b"multiplied",
    b"multiply",
    b"multiplyWithOverflow",
    b"negate",
    b"negated",
    b"next",
    b"objectAt",
    b"overlaps",
    b"passRetained",
    b"passUnretained",
    b"popFirst",
    b"popLast",
    b"predecessor",
    b"prefix",
    b"reduce",
    b"release",
    b"remainder",
    b"remainderWithOverflow",
    b"remove",
    b"removeAll",
    b"removeAtIndex",
    b"removeFirst",
    b"removeLast",
    b"removeRange",
    b"removeSubrange",
    b"removeValue",
    b"removeValueForKey",
    b"replace",
    b"replaceRange",
    b"replaceSubrange",
    b"requestNativeBuffer",
    b"requestUniqueMutableBackingBuffer",
    b"reserveCapacity",
    b"retain",
    b"reverse",
    b"reversed",
    b"round",
    b"rounded",
    b"samePosition",
    b"sortInPlace",
    b"squareRoot",
    b"starts",
    b"storeBytes",
    b"stride",
    b"subtract",
    b"subtractInPlace",
    b"subtractWithOverflow",
    b"subtracting",
    b"successor",
    b"suffix",
    b"symmetricDifference",
    b"takeRetainedValue",
    b"takeUnretainedValue",
    b"toIntMax",
    b"toOpaque",
    b"toUIntMax",
    b"trailSurrogate",
    b"transcodedLength",
    b"truncatingRemainder",
    b"underestimateCount",
    b"union",
    b"unionInPlace",
    b"unwrappedOrError",
    b"update",
    b"updateValue",
    b"uppercased",
    b"width",
    b"withMemoryRebound",
    b"write",
    b"writeTo",
];

/// The words of `BuiltinFunctionsAt2`'s regex.
const BUILTIN_FUNCTIONS_AT2: &[&[u8]] = &[
    b"characterAtIndex",
    b"copyWithZone",
    b"countByEnumeratingWithState",
    b"customMirror",
    b"customPlaygroundQuickLook",
    b"getMirror",
    b"isASCII",
    b"isEmptyInput",
    b"join",
    b"keyEnumerator",
    b"length",
    b"member",
    b"moveInitializeAssignFrom",
    b"nextObject",
    b"objectAtIndex",
    b"objectEnumerator",
    b"objectForKey",
    b"put",
    b"samePositionIn",
    b"startsWith",
    b"superclassMirror",
    b"withUnsafeMutablePointerToValue",
];

/// The words of `BuiltinGlobalFunctionsAt1`'s regex.
const BUILTIN_GLOBAL_FUNCTIONS_AT1: &[&[u8]] = &[b"anyGenerator", b"autoreleasepool"];

/// The words of `BuiltinGlobalFunctionsAt2`'s regex.
const BUILTIN_GLOBAL_FUNCTIONS_AT2: &[&[u8]] = &[
    b"abs",
    b"alignof",
    b"alignofValue",
    b"assert",
    b"assertionFailure",
    b"debugPrint",
    b"dump",
    b"fatalError",
    b"getVaList",
    b"isKnownUniquelyReferenced",
    b"isUniquelyReferenced",
    b"isUniquelyReferencedNonObjC",
    b"max",
    b"min",
    b"numericCast",
    b"precondition",
    b"preconditionFailure",
    b"print",
    b"readLine",
    b"repeatElement",
    b"sequence",
    b"sizeof",
    b"sizeofValue",
    b"stride",
    b"strideof",
    b"strideofValue",
    b"swap",
    b"transcode",
    b"unsafeAddress",
    b"unsafeAddressOf",
    b"unsafeBitCast",
    b"unsafeDowncast",
    b"unsafeUnwrap",
    b"withExtendedLifetime",
    b"withUnsafeMutablePointer",
    b"withUnsafePointer",
    b"withVaList",
    b"zip",
];

/// The words of `BuiltinGlobalFunctionsAt3`'s regex.
const BUILTIN_GLOBAL_FUNCTIONS_AT3: &[&[u8]] = &[
    b"advance",
    b"distance",
    b"dropFirst",
    b"dropLast",
    b"extend",
    b"insert",
    b"join",
    b"lazy",
    b"overlaps",
    b"prefix",
    b"reflect",
    b"removeAll",
    b"removeAtIndex",
    b"removeLast",
    b"removeRange",
    b"sort",
    b"splice",
    b"split",
    b"suffix",
    b"withUnsafeMutablePointers",
    b"withUnsafePointers",
];

/// The words of `BuiltinPropertiesAt1`'s regex.
const BUILTIN_PROPERTIES_AT1: &[&[u8]] = &[
    b"alignment",
    b"allZeros",
    b"allocatedElementCount",
    b"array",
    b"arrayPropertyIsNativeTypeChecked",
    b"base",
    b"baseAddress",
    b"bigEndian",
    b"binade",
    b"bitPattern",
    b"buffer",
    b"byteSize",
    b"byteSwapped",
    b"capacity",
    b"characters",
    b"count",
    b"countTrailingZeros",
    b"customMirror",
    b"customPlaygroundQuickLook",
    b"debugDescription",
    b"description",
    b"elements",
    b"endIndex",
    b"exponent",
    b"exponentBitCount",
    b"exponentBitPattern",
    b"first",
    b"firstElementAddress",
    b"firstElementAddressIfContiguous",
    b"floatingPointClass",
    b"greatestFiniteMagnitude",
    b"hasPointerRepresentation",
    b"hashValue",
    b"header",
    b"identity",
    b"indices",
    b"infinity",
    b"isASCII",
    b"isCanonical",
    b"isEmpty",
    b"isFinite",
    b"isInfinite",
    b"isNaN",
    b"isNormal",
    b"isSignMinus",
    b"isSignaling",
    b"isSignalingNaN",
    b"isSubnormal",
    b"isZero",
    b"keys",
    b"last",
    b"lazy",
    b"leastNonzeroMagnitude",
    b"leastNormalMagnitude",
    b"littleEndian",
    b"lowercaseString",
    b"max",
    b"memory",
    b"min",
    b"nan",
    b"nativeOwner",
    b"nextDown",
    b"nextUp",
    b"nulTerminatedUTF8",
    b"owner",
    b"pi",
    b"pointee",
    b"quietNaN",
    b"radix",
    b"rawValue",
    b"sign",
    b"signBitIndex",
    b"signalingNaN",
    b"significand",
    b"significandBitCount",
    b"significandBitPattern",
    b"significandWidth",
    b"size",
    b"startIndex",
    b"stride",
    b"stringValue",
    b"subscriptBaseAddress",
    b"summary",
    b"superclassMirror",
    b"uintValue",
    b"ulp",
    b"ulpOfOne",
    b"underestimatedCount",
    b"unicodeScalar",
    b"unicodeScalars",
    b"unsafelyUnwrapped",
    b"uppercaseString",
    b"utf16",
    b"utf8",
    b"utf8CString",
    b"utf8CodeUnitCount",
    b"utf8Start",
    b"value",
    b"values",
];

/// The words of `BuiltinPropertiesAt2`'s regex.
const BUILTIN_PROPERTIES_AT2: &[&[u8]] = &[
    b"boolValue",
    b"disposition",
    b"end",
    b"objectIdentifier",
    b"quickLookObject",
    b"start",
    b"valueType",
];

/// The words of `BuiltinPropertiesAt3`'s regex.
const BUILTIN_PROPERTIES_AT3: &[&[u8]] = &[
    b"attributedString",
    b"awayFromZero",
    b"bezierPath",
    b"bool",
    b"collection",
    b"color",
    b"customized",
    b"dictionary",
    b"double",
    b"down",
    b"emptyInput",
    b"error",
    b"float",
    b"generated",
    b"image",
    b"int",
    b"minus",
    b"negativeInfinity",
    b"negativeNormal",
    b"negativeSubnormal",
    b"negativeZero",
    b"none",
    b"optional",
    b"plus",
    b"point",
    b"positiveInfinity",
    b"positiveNormal",
    b"positiveSubnormal",
    b"positiveZero",
    b"quietNaN",
    b"range",
    b"rectangle",
    b"scalarValue",
    b"set",
    b"signalingNaN",
    b"size",
    b"some",
    b"sound",
    b"sprite",
    b"suppressed",
    b"text",
    b"toNearestOrAwayFromZero",
    b"toNearestOrEven",
    b"towardZero",
    b"tuple",
    b"uInt",
    b"up",
    b"url",
    b"view",
];

/// The words of `KeywordsAt0`'s regex.
const KEYWORDS_AT0: &[&[u8]] = &[
    b"case",
    b"default",
    b"else",
    b"fallthrough",
    b"guard",
    b"if",
    b"switch",
    b"where",
];

/// The words of `KeywordsAt1`'s regex.
const KEYWORDS_AT1: &[&[u8]] = &[b"break", b"continue", b"fallthrough", b"return", b"yield"];

/// The words of `KeywordsAt2`'s regex.
const KEYWORDS_AT2: &[&[u8]] = &[b"each", b"for", b"in", b"while"];

/// The words of `KeywordsAt12`'s regex.
const KEYWORDS_AT12: &[&[u8]] = &[b"associatedtype", b"operator", b"typealias"];

/// The words of `KeywordsAt14`'s regex.
const KEYWORDS_AT14: &[&[u8]] = &[
    b"borrowing",
    b"consuming",
    b"convenience",
    b"distributed",
    b"dynamic",
    b"final",
    b"indirect",
    b"infix",
    b"inout",
    b"lazy",
    b"mutating",
    b"nonmutating",
    b"optional",
    b"override",
    b"postfix",
    b"prefix",
    b"required",
    b"static",
];

/// The words of `KeywordsAt18`'s regex.
const KEYWORDS_AT18: &[&[u8]] = &[
    b"fileprivate",
    b"internal",
    b"open",
    b"package",
    b"private",
    b"public",
];

/// The words of `KeywordsAt21`'s regex.
const KEYWORDS_AT21: &[&[u8]] = &[b"Self", b"self", b"super"];

impl Lexer<'_> {
    /// `\S+`, for `Nested0At0`, `Nested1At3`.
    pub(in crate::swift) fn rx_nested_0_at_0(&mut self, p: usize) -> Option<usize> {
        let mut i = p;
        while i < self.lim() && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        self.spend(i.saturating_sub(p));
        (i > p).then_some(i)
    }

    /// `(file)\s*(:)\s*(?=")`, for `Nested1At0`.
    pub(in crate::swift) fn rx_nested_1_at_0(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"file")?;
        let colon = self.space_end(end);
        if self.at(colon) != b':' {
            return None;
        }
        let after = self.space_end(colon + 1);
        if self.at(after) != b'"' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, colon, colon + 1);
        Some(after)
    }

    /// `(?!\G)`, for `Nested1At0End`, `CommentsAt5End`, `AttributesAt0At1End`,
    /// `AttributesAt0At2End`, `DeclarationsPrecedencegroupEnd`,
    /// `DeclarationsInheritanceClauseInheritedTypeEnd`,
    /// `DeclarationsTypeIdentifierAt0End`,
    /// `DeclarationsTypeEnumRawValueAssignmentEnd`, `OperatorsAt1End`,
    /// `DeclarationsTypealiasAt0End`.
    pub(in crate::swift) fn rx_nested_1_at_0_end(&mut self, p: usize) -> Option<usize> {
        (!(self.g && p == self.pos)).then_some(p)
    }

    /// `\b(Managed((?:|Proto)Buffer)|NonObjectiveCBase|AnyGenerator)\b`, for
    /// `BuiltinTypesBuiltinClassType`.
    pub(in crate::swift) fn rx_builtin_types_builtin_class_type(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_CLASS_TYPE)
    }

    /// `\b(?:CommandLine|Process(?=\.))\b`, for
    /// `BuiltinTypesBuiltinEnumTypeAt0`.
    pub(in crate::swift) fn rx_builtin_types_builtin_enum_type_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        (word == b"CommandLine" || (word == b"Process" && self.at(end) == b'.')).then_some(end)
    }

    /// `\bNever\b`, for `BuiltinTypesBuiltinEnumTypeAt1`.
    pub(in crate::swift) fn rx_builtin_types_builtin_enum_type_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"Never")
    }

    /// `\b(?:ImplicitlyUnwrappedOptional|Representation|MemoryLayout|FloatingPointClassification|SetIndexRepresentation|SetIteratorRepresentation|FloatingPointRoundingRule|UnicodeDecodingResult|Optional|DictionaryIndexRepresentation|AncestorRepresentation|DisplayStyle|PlaygroundQuickLook|Never|FloatingPointSign|Bit|DictionaryIteratorRepresentation)\b`, for `BuiltinTypesBuiltinEnumTypeAt2`.
    pub(in crate::swift) fn rx_builtin_types_builtin_enum_type_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_ENUM_TYPE_AT2)
    }

    /// `\b(?:MirrorDisposition|QuickLookObject)\b`, for
    /// `BuiltinTypesBuiltinEnumTypeAt3`.
    pub(in crate::swift) fn rx_builtin_types_builtin_enum_type_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_ENUM_TYPE_AT3)
    }

    /// `\bActor\b`, for `BuiltinTypesBuiltinProtocolTypeAt0`.
    pub(in crate::swift) fn rx_builtin_types_builtin_protocol_type_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"Actor")
    }

    /// `\b(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinTypesBuiltinProtocolTypeAt1`.
    pub(in crate::swift) fn rx_builtin_types_builtin_protocol_type_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_PROTOCOL_TYPE_AT1)
    }

    /// `\b(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinTypesBuiltinProtocolTypeAt2`.
    pub(in crate::swift) fn rx_builtin_types_builtin_protocol_type_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_PROTOCOL_TYPE_AT2)
    }

    /// `\b(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinTypesBuiltinStructTypeAt0`.
    pub(in crate::swift) fn rx_builtin_types_builtin_struct_type_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_STRUCT_TYPE_AT0)
    }

    /// `\b(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinTypesBuiltinStructTypeAt1`.
    pub(in crate::swift) fn rx_builtin_types_builtin_struct_type_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_STRUCT_TYPE_AT1)
    }

    /// `\b(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinTypesBuiltinTypealiasAt0`.
    pub(in crate::swift) fn rx_builtin_types_builtin_typealias_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_TYPEALIAS_AT0)
    }

    /// `\b(?:Generator|PlaygroundQuickLook|UWord|Word)\b`, for
    /// `BuiltinTypesBuiltinTypealiasAt1`.
    pub(in crate::swift) fn rx_builtin_types_builtin_typealias_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.listed(p, BUILTIN_TYPES_BUILTIN_TYPEALIAS_AT1)
    }

    /// `\bAny\b`, for `BuiltinTypesAt5`.
    pub(in crate::swift) fn rx_builtin_types_at_5(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"Any")
    }

    /// `(?<=[!?])[!?]+`, for `Nested2At0`.
    pub(in crate::swift) fn rx_nested_2_at_0(&mut self, p: usize) -> Option<usize> {
        if !(self.prev_is(p, b'!') || self.prev_is(p, b'?')) {
            return None;
        }
        let end = self.skip(p, |b| matches!(b, b'!' | b'?'));
        (end > p).then_some(end)
    }

    /// `\.`, for `Nested3At0`, `DeclarationsImportAt0At3`.
    pub(in crate::swift) fn rx_nested_3_at_0(&mut self, p: usize) -> Option<usize> {
        (p < self.lim() && self.at(p) == b'.').then_some(p + 1)
    }

    /// `\G(\+\+|--)$`, for `Nested7At0`, `Nested8At0`.
    pub(in crate::swift) fn rx_nested_7_at_0(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"++", b"--"], b"")
    }

    /// `\G([-+])$`, for `Nested7At1`.
    pub(in crate::swift) fn rx_nested_7_at_1(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"-", b"+"], b"")
    }

    /// `\G!$`, for `Nested7At2`, `Nested8At1`.
    pub(in crate::swift) fn rx_nested_7_at_2(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"!"], b"")
    }

    /// `\G~$`, for `Nested7At3`.
    pub(in crate::swift) fn rx_nested_7_at_3(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"~"], b"")
    }

    /// `.+`, for `Nested7At4`, `Nested8At2`, `Nested9At11`, `Nested10At0`,
    /// `Nested11At0`, `Nested12At1`.
    pub(in crate::swift) fn rx_nested_7_at_4(&mut self, p: usize) -> Option<usize> {
        (p < self.lim()).then_some(self.lim())
    }

    /// `\G=$`, for `Nested9At0`.
    pub(in crate::swift) fn rx_nested_9_at_0(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"="], b"")
    }

    /// `\G([-%*+/]|<<|>>|[\&^|]|&&|\|\|)=$`, for `Nested9At1`.
    pub(in crate::swift) fn rx_nested_9_at_1(&mut self, p: usize) -> Option<usize> {
        self.only(
            p,
            &[
                b"-", b"%", b"*", b"+", b"/", b"<<", b">>", b"&", b"^", b"|", b"&&", b"||",
            ],
            b"=",
        )
    }

    /// `\G([-*+/])$`, for `Nested9At2`.
    pub(in crate::swift) fn rx_nested_9_at_2(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"-", b"*", b"+", b"/"], b"")
    }

    /// `\G&([-*+])$`, for `Nested9At3`.
    pub(in crate::swift) fn rx_nested_9_at_3(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"&-", b"&*", b"&+"], b"")
    }

    /// `\G%$`, for `Nested9At4`.
    pub(in crate::swift) fn rx_nested_9_at_4(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"%"], b"")
    }

    /// `\G(==|!=|[<>]|>=|<=|~=)$`, for `Nested9At5`.
    pub(in crate::swift) fn rx_nested_9_at_5(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"==", b"!=", b"<", b">", b">=", b"<=", b"~="], b"")
    }

    /// `\G\?\?$`, for `Nested9At6`.
    pub(in crate::swift) fn rx_nested_9_at_6(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"??"], b"")
    }

    /// `\G(&&|\|\|)$`, for `Nested9At7`.
    pub(in crate::swift) fn rx_nested_9_at_7(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"&&", b"||"], b"")
    }

    /// `\G([\&^|]|<<|>>)$`, for `Nested9At8`.
    pub(in crate::swift) fn rx_nested_9_at_8(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"&", b"^", b"|", b"<<", b">>"], b"")
    }

    /// `\G([!=]==)$`, for `Nested9At9`.
    pub(in crate::swift) fn rx_nested_9_at_9(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"!==", b"==="], b"")
    }

    /// `\G\?$`, for `Nested9At10`.
    pub(in crate::swift) fn rx_nested_9_at_10(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"?"], b"")
    }

    /// `\G\.\.[.<]$`, for `Nested12At0`.
    pub(in crate::swift) fn rx_nested_12_at_0(&mut self, p: usize) -> Option<usize> {
        self.only(p, &[b"...", b"..<"], b"")
    }

    /// `(?<=\.)(?:…)`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinFunctionsAt0`.
    pub(in crate::swift) fn rx_builtin_functions_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.listed_run(p, BUILTIN_FUNCTIONS_AT0)?;
        self.ahead_open(end, b"({").then_some(end)
    }

    /// `(?<=\.)(?:…)`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinFunctionsAt1`.
    pub(in crate::swift) fn rx_builtin_functions_at_1(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.listed_run(p, BUILTIN_FUNCTIONS_AT1)?;
        self.ahead_open(end, b"(").then_some(end)
    }

    /// `(?<=\.)(?:…)(?=\s*\()`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinFunctionsAt2`.
    pub(in crate::swift) fn rx_builtin_functions_at_2(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.listed_run(p, BUILTIN_FUNCTIONS_AT2)?;
        self.ahead_open(end, b"(").then_some(end)
    }

    /// `\b(type)(\()\s*(of)(:)`, for `BuiltinGlobalFunctionsAt0`.
    pub(in crate::swift) fn rx_builtin_global_functions_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, b"type")?;
        let open = self.lit(word, b"(")?;
        let of = self.space_end(open);
        let of_end = self.lit(of, b"of")?;
        let colon = self.lit(of_end, b":")?;
        self.cap(1, p, word);
        self.cap(2, word, open);
        self.cap(3, of, of_end);
        self.cap(4, of_end, colon);
        Some(colon)
    }

    /// `\ba(?:nyGenerator|utoreleasepool)(?=\s*[({])\b`, for
    /// `BuiltinGlobalFunctionsAt1`.
    pub(in crate::swift) fn rx_builtin_global_functions_at_1(&mut self, p: usize) -> Option<usize> {
        let end = self.listed(p, BUILTIN_GLOBAL_FUNCTIONS_AT1)?;
        self.ahead_open(end, b"({").then_some(end)
    }

    /// `\b(?:…)`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinGlobalFunctionsAt2`.
    pub(in crate::swift) fn rx_builtin_global_functions_at_2(&mut self, p: usize) -> Option<usize> {
        let end = self.listed(p, BUILTIN_GLOBAL_FUNCTIONS_AT2)?;
        self.ahead_open(end, b"(").then_some(end)
    }

    /// `\b(?:…)(?=\s*\()`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinGlobalFunctionsAt3`.
    pub(in crate::swift) fn rx_builtin_global_functions_at_3(&mut self, p: usize) -> Option<usize> {
        let end = self.listed(p, BUILTIN_GLOBAL_FUNCTIONS_AT3)?;
        self.ahead_open(end, b"(").then_some(end)
    }

    /// `(?<=(?:^|\W)(?:Process\.|CommandLine\.))(arguments|argc|unsafeArgv)`,
    /// for `BuiltinPropertiesAt0`.
    pub(in crate::swift) fn rx_builtin_properties_at_0(&mut self, p: usize) -> Option<usize> {
        let after = |owner: &[u8]| {
            p >= owner.len()
                && self.text(p - owner.len(), p) == owner
                && match p - owner.len() {
                    0 => true,
                    start => self.char_before(start).is_some_and(|c| !is_word(c)),
                }
        };
        if !(after(b"Process.") || after(b"CommandLine.")) {
            return None;
        }
        [&b"arguments"[..], b"argc", b"unsafeArgv"]
            .into_iter()
            .find_map(|word| self.lit(p, word))
    }

    /// `(?<=\.)(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinPropertiesAt1`.
    pub(in crate::swift) fn rx_builtin_properties_at_1(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        self.listed_run(p, BUILTIN_PROPERTIES_AT1)
    }

    /// `(?<=\.)(?:
    /// boolValue|disposition|end|objectIdentifier|quickLookObject|start|valueType)\
    /// b`, for `BuiltinPropertiesAt2`.
    pub(in crate::swift) fn rx_builtin_properties_at_2(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        self.listed_run(p, BUILTIN_PROPERTIES_AT2)
    }

    /// `(?<=\.)(?:…)\b`, a long alternation that the word list of this
    /// matcher spells out, for `BuiltinPropertiesAt3`.
    pub(in crate::swift) fn rx_builtin_properties_at_3(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        self.listed_run(p, BUILTIN_PROPERTIES_AT3)
    }

    /// `` (?<q>`?)(?!_:)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>): ``, for
    /// `Nested13At0`.
    pub(in crate::swift) fn rx_nested_13_at_0(&mut self, p: usize) -> Option<usize> {
        let quote = usize::from(self.at(p) == b'`');
        let start = p + quote;
        if self.starts(start, b"_:") {
            return None;
        }
        let (first, mut i) = self.char_at(start)?;
        if !(first == '_' || is_letter(first)) {
            return None;
        }
        while let Some((c, end)) = self.char_at(i)
            && (c == '_' || is_letter(c) || is_number(c) || is_mark(c))
        {
            i = end;
        }
        self.spend(i - start);
        if quote == 1 && self.at(i) != b'`' {
            return None;
        }
        if self.at(i + quote) != b':' {
            return None;
        }
        self.cap(1, p, start);
        self.cap(2, i, i + quote);
        Some(i + quote + 1)
    }

    /// `(?<!\.)\b(?:if|else|guard|where|switch|case|default|fallthrough)\b`,
    /// for `KeywordsAt0`.
    pub(in crate::swift) fn rx_keywords_at_0(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT0)
    }

    /// `(?<!\.)\b(?:continue|break|fallthrough|return|yield)\b`, for
    /// `KeywordsAt1`.
    pub(in crate::swift) fn rx_keywords_at_1(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT1)
    }

    /// `(?<!\.)\b(?:while|for|in|each)\b`, for `KeywordsAt2`.
    pub(in crate::swift) fn rx_keywords_at_2(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT2)
    }

    /// `` \bany\b(?=\s*`?[_\p{L}]) ``, for `KeywordsAt4`.
    pub(in crate::swift) fn rx_keywords_at_4(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"any")?;
        self.ahead_ident(self.space_end(end)).then_some(end)
    }

    /// `(?<!\.)\b(repeat)\b(\s*)`, for `KeywordsAt5`.
    pub(in crate::swift) fn rx_keywords_at_5(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.keyword(p, b"repeat")?;
        self.cap(1, p, end);
        let after = self.space_end(end);
        self.cap(2, end, after);
        Some(after)
    }

    /// `(?<!\.)\bdefer\b`, for `KeywordsAt6`.
    pub(in crate::swift) fn rx_keywords_at_6(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.keyword(p, b"defer")
    }

    /// `(?<!\.)\b(?:(await\s+try)|(await))\b`, for `KeywordsAt7`.
    pub(in crate::swift) fn rx_keywords_at_7(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"await")?;
        let blank = self.space_end(end);
        if blank > end
            && let Some(word) = self.lit(blank, b"try")
            && self.word_at(word).is_none()
        {
            self.cap(1, p, word);
            return Some(word);
        }
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(2, p, end);
        Some(end)
    }

    /// `(?<!\.)\b(?:catch|throw|try)\b|\btry[!?]\B`, for `KeywordsAt8`.
    pub(in crate::swift) fn rx_keywords_at_8(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            const WORDS: &[&[u8]] = &[b"catch", b"throw", b"try"];
            if let Some(end) = self.listed(p, WORDS) {
                return Some(end);
            }
        }
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"try")?;
        (matches!(self.at(end), b'!' | b'?') && !self.boundary(end + 1)).then_some(end + 1)
    }

    /// `(?<!\.)\b(?:|re)throws\b`, for `KeywordsAt9`.
    pub(in crate::swift) fn rx_keywords_at_9(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, &[b"rethrows", b"throws"])
    }

    /// `(?<!\.)\b(do)\b(\s*)`, for `KeywordsAt10`.
    pub(in crate::swift) fn rx_keywords_at_10(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.keyword(p, b"do")?;
        self.cap(1, p, end);
        let after = self.space_end(end);
        self.cap(2, end, after);
        Some(after)
    }

    /// `(?<!\.)\b(?:(async)\s+)?(let|var)\b`, for `KeywordsAt11`.
    pub(in crate::swift) fn rx_keywords_at_11(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        if let Some(word) = self.lit(p, b"async") {
            let start = self.space_end(word);
            if start > word
                && let Some(end) = self.let_or_var(start)
            {
                self.cap(1, p, word);
                self.cap(2, start, end);
                return Some(end);
            }
        }
        let end = self.let_or_var(p)?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `(?<!\.)\b(?:associatedtype|operator|typealias)\b`, for `KeywordsAt12`.
    pub(in crate::swift) fn rx_keywords_at_12(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT12)
    }

    /// `` (?<!\.)\b(class)\b(?=\s*`?[_\p{L}]) ``, for `KeywordsAt13Class`.
    pub(in crate::swift) fn rx_keywords_at_13_class(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"class")
    }

    /// `` (?<!\.)\b(enum)\b(?=\s*`?[_\p{L}]) ``, for `KeywordsAt13Enum`.
    pub(in crate::swift) fn rx_keywords_at_13_enum(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"enum")
    }

    /// `` (?<!\.)\b(extension)\b(?=\s*`?[_\p{L}]) ``, for
    /// `KeywordsAt13Extension`.
    pub(in crate::swift) fn rx_keywords_at_13_extension(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"extension")
    }

    /// `` (?<!\.)\b(precedencegroup)\b(?=\s*`?[_\p{L}]) ``, for
    /// `KeywordsAt13Precedencegroup`.
    pub(in crate::swift) fn rx_keywords_at_13_precedencegroup(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.declaration_keyword(p, b"precedencegroup")
    }

    /// `` (?<!\.)\b(protocol)\b(?=\s*`?[_\p{L}]) ``, for
    /// `KeywordsAt13Protocol`.
    pub(in crate::swift) fn rx_keywords_at_13_protocol(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"protocol")
    }

    /// `` (?<!\.)\b(struct)\b(?=\s*`?[_\p{L}]) ``, for `KeywordsAt13Struct`.
    pub(in crate::swift) fn rx_keywords_at_13_struct(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"struct")
    }

    /// `` (?<!\.)\b(actor)\b(?=\s*`?[_\p{L}]) ``, for `KeywordsAt13Actor`.
    pub(in crate::swift) fn rx_keywords_at_13_actor(&mut self, p: usize) -> Option<usize> {
        self.declaration_keyword(p, b"actor")
    }

    /// `(?<!\.)\b(?:
    /// inout|static|final|lazy|mutating|nonmutating|optional|indirect|required|override|dynamic|convenience|infix|prefix|postfix|distributed|borrowing|consuming)\
    /// b`, for `KeywordsAt14`.
    pub(in crate::swift) fn rx_keywords_at_14(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT14)
    }

    /// `(?<!\.)\bnonisolated(?:\(nonsending\)|\b)`, for `KeywordsAt15`.
    pub(in crate::swift) fn rx_keywords_at_15(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"nonisolated")?;
        if let Some(group) = self.lit(end, b"(nonsending)") {
            return Some(group);
        }
        self.word_at(end).is_none().then_some(end)
    }

    /// `\binit[!?]|\binit\b|(?<!\.)\b(?:
    /// func|deinit|subscript|didSet|set|willSet|yielding\s+borrow|yielding\
    /// s+mutate)\b`, for `KeywordsAt16`.
    pub(in crate::swift) fn rx_keywords_at_16(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p)
            && let Some(end) = self.lit(p, b"init")
        {
            if matches!(self.at(end), b'!' | b'?') {
                return Some(end + 1);
            }
            if self.word_at(end).is_none() {
                return Some(end);
            }
        }
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        if word == b"yielding" {
            let start = self.space_end(end);
            if start == end {
                return None;
            }
            return [&b"borrow"[..], b"mutate"]
                .into_iter()
                .filter_map(|tail| self.lit(start, tail))
                .find(|&tail_end| self.word_at(tail_end).is_none());
        }
        KEYWORDS_AT_16.binary_search(&word).is_ok().then_some(end)
    }

    /// `\b(get)(?:\s+(throws\s+async)|\s+(async)(?:\s+(throws))?)?\b`, for
    /// `KeywordsAt17`.
    pub(in crate::swift) fn rx_keywords_at_17(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let get = self.lit(p, b"get")?;
        let blank = self.space_end(get);
        if blank > get {
            if let Some(throws) = self.lit(blank, b"throws") {
                let start = self.space_end(throws);
                if start > throws
                    && let Some(end) = self.lit(start, b"async")
                    && self.word_at(end).is_none()
                {
                    self.cap(1, p, get);
                    self.cap(2, blank, end);
                    return Some(end);
                }
            }
            if let Some(word) = self.lit(blank, b"async") {
                let start = self.space_end(word);
                if start > word
                    && let Some(end) = self.lit(start, b"throws")
                    && self.word_at(end).is_none()
                {
                    self.cap(1, p, get);
                    self.cap(3, blank, word);
                    self.cap(4, start, end);
                    return Some(end);
                }
                if self.word_at(word).is_none() {
                    self.cap(1, p, get);
                    self.cap(3, blank, word);
                    return Some(word);
                }
            }
        }
        if self.word_at(get).is_some() {
            return None;
        }
        self.cap(1, p, get);
        Some(get)
    }

    /// `(?<!\.)\b(?:fileprivate|private|internal|public|open|package)\b`, for
    /// `KeywordsAt18`.
    pub(in crate::swift) fn rx_keywords_at_18(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT18)
    }

    /// `(?<!\.)\bunowned\((?:|un)safe\)|(?<!\.)\b(?:weak|unowned)\b`, for
    /// `KeywordsAt19`.
    pub(in crate::swift) fn rx_keywords_at_19(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        if let Some(open) = self.lit(p, b"unowned(") {
            let end = self
                .lit(open, b"safe)")
                .or_else(|| self.lit(open, b"unsafe)"));
            if end.is_some() {
                return end;
            }
        }
        self.listed(p, &[b"unowned", b"weak"])
    }

    /// `(?<=\.)(?:(dynamicType|self)|(Protocol|Type))\b`, for `KeywordsAt20`.
    pub(in crate::swift) fn rx_keywords_at_20(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.word_end(p);
        match self.text(p, end) {
            b"dynamicType" | b"self" => self.cap(1, p, end),
            b"Protocol" | b"Type" => self.cap(2, p, end),
            _ => return None,
        }
        Some(end)
    }

    /// `(?<!\.)\b(?:super|self|Self)\b`, for `KeywordsAt21`.
    pub(in crate::swift) fn rx_keywords_at_21(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.listed(p, KEYWORDS_AT21)
    }

    /// `(?:\B#(?:file|filePath|fileID|line|column|function|dsohandle|isolation)|\b__(?:FILE|LINE|COLUMN|FUNCTION|DSO_HANDLE)__)\b`, for `KeywordsAt22`.
    pub(in crate::swift) fn rx_keywords_at_22(&mut self, p: usize) -> Option<usize> {
        const HASHED: &[&[u8]] = &[
            b"column",
            b"dsohandle",
            b"file",
            b"fileID",
            b"filePath",
            b"function",
            b"isolation",
            b"line",
        ];
        const DUNDER: &[&[u8]] = &[
            b"__COLUMN__",
            b"__DSO_HANDLE__",
            b"__FILE__",
            b"__FUNCTION__",
            b"__LINE__",
        ];
        if self.at(p) == b'#' {
            if self.boundary(p) {
                return None;
            }
            let end = self.word_end(p + 1);
            return HASHED
                .binary_search(&self.text(p + 1, end))
                .is_ok()
                .then_some(end);
        }
        self.listed(p, DUNDER)
    }

    /// `(?<!\.)\bimport\b`, for `KeywordsAt23`.
    pub(in crate::swift) fn rx_keywords_at_23(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.keyword(p, b"import")
    }

    /// `` (?<!\.)\bconsume(?=\s+`?[_\p{L}]) ``, for `KeywordsAt24`.
    pub(in crate::swift) fn rx_keywords_at_24(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"consume")?;
        (self.space_end(end) > end && self.ahead_ident(self.space_end(end))).then_some(end)
    }

    /// `` (?<!\.)\bcopy(?=\s+`?[_\p{L}]) ``, for `KeywordsAt25`.
    pub(in crate::swift) fn rx_keywords_at_25(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"copy")?;
        (self.space_end(end) > end && self.ahead_ident(self.space_end(end))).then_some(end)
    }

    /// `\brethrows\b`, for `Nested15At0`.
    pub(in crate::swift) fn rx_nested_15_at_0(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"rethrows")
    }

    /// `(line)\s*(:)\s*([0-9]+)`, for `Nested1At1`.
    pub(in crate::swift) fn rx_nested_1_at_1(&mut self, p: usize) -> Option<usize> {
        let word = self.lit(p, b"line")?;
        let colon = self.space_end(word);
        if self.at(colon) != b':' {
            return None;
        }
        let start = self.space_end(colon + 1);
        let end = self.skip(start, |b| b.is_ascii_digit());
        if end == start {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, colon, colon + 1);
        self.cap(3, start, end);
        Some(end)
    }
}

impl Lexer<'_> {
    /// `\b` and then one of `list`, as the whole word.
    /// `list` is sorted.
    /// Every word of a list is a run of word characters.
    /// The word that ends at a `\b` is therefore the whole run.
    fn listed(&self, p: usize, list: &[&[u8]]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.listed_run(p, list)
    }

    /// One of `list` as the whole run of word characters at `p`, where the
    /// character before `p` is already known not to be one.
    fn listed_run(&self, p: usize, list: &[&[u8]]) -> Option<usize> {
        let end = self.word_end(p);
        list.binary_search(&self.text(p, end))
            .is_ok()
            .then_some(end)
    }

    /// `(?=\s*[...])`, where `opens` lists the accepted bytes.
    fn ahead_open(&self, i: usize, opens: &[u8]) -> bool {
        opens.contains(&self.at(self.space_end(i)))
    }

    /// `` `?[_\p{L}] `` at `i`, after the blanks the caller skipped.
    fn ahead_ident(&self, i: usize) -> bool {
        let start = i + usize::from(self.at(i) == b'`');
        self.char_at(start)
            .is_some_and(|(c, _)| c == '_' || is_letter(c))
    }

    /// `` (?<!\.)\bWORD\b(?=\s*`?[_\p{L}]) ``.
    fn declaration_keyword(&self, p: usize, word: &[u8]) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.keyword(p, word)?;
        self.ahead_ident(self.space_end(end)).then_some(end)
    }

    /// `(let|var)\b`.
    fn let_or_var(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"let").or_else(|| self.lit(p, b"var"))?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\G(op|…)tail$` for the nested patterns that tokenize a whole
    /// operator: the operator is the capture's entire text.
    fn only(&self, p: usize, ops: &[&[u8]], tail: &[u8]) -> Option<usize> {
        if !(self.g && p == self.pos) {
            return None;
        }
        ops.iter().find_map(|op| {
            let end = self.lit(p, op)? + tail.len();
            self.lit(end - tail.len(), tail)?;
            self.at_eol(end).then_some(end)
        })
    }
}
