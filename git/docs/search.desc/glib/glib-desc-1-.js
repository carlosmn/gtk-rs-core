searchState.loadedDescShard("glib", 1, "An invalid <code>Type</code> used as error return value in some …\nThe fundamental type corresponding to C <code>long</code>\nThe fundamental type from which all objects are derived\nThe fundamental type from which all <code>GParamSpec</code> types are …\nThe fundamental type corresponding to a pointer\nThe target of a Pointer\nA GLib pointer\nThe fundamental type corresponding to <code>String</code>\nTypes that are supported by GLib dynamic typing.\nA GLib or GLib-based library type\nThe fundamental type corresponding to <code>u32</code>\nThe fundamental type corresponding to <code>u64</code>\nThe fundamental type corresponding to <code>u8</code>\nThe fundamental type corresponding to the unit type <code>()</code>\nThe fundamental type corresponding to C <code>unsigned long</code>\nThe fundamental type of GVariant\nEnsures that the type has been registered with the type …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks that the type is not <code>INVALID</code>\nReturns the type identifier of <code>Self</code>.\nA <code>Value</code> containing another <code>Value</code>.\nValue type checker.\nTrait to retrieve the contained value from a <code>Value</code>.\nTrait for types that implement <code>FromValue</code> and are Optional.\nGeneric <code>Value</code> type checker for types.\nGeneric <code>Value</code> type checker for optional types.\nAn error returned from the <code>get</code> function on a <code>Value</code> for …\nA version of <code>Value</code> for storing <code>Send</code> types, that implements …\nConverts to <code>SendValue</code>.\nTrait to convert a value to a <code>Value</code>.\nTrait to convert an <code>Option</code> to a <code>Value</code> for optional types.\nType to get the <code>Type</code> from.\nA generic value capable of carrying various types.\nA type that can be stored in <code>Value</code>s.\nTrait for <code>Value</code> type checkers.\nAn error returned from the <code>get</code> function on a <code>Value</code> for …\nAn error returned from the <code>get</code> function on a <code>Value</code> for …\nA type that can be stored in <code>Value</code>s and is optional.\nWrapped <code>Value</code> type checker for optional types.\nCreates a new <code>Value</code> that is initialized for a given …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBorrows the underlying C value.\nBorrows the underlying C value.\nBorrows the underlying C value mutably.\nBorrows the underlying C value mutably.\nCreates a new <code>Value</code> that is initialized with <code>type_</code>.\nCreates a new <code>Value</code> that is initialized with <code>type_</code>.\nGet the contained value from a <code>Value</code>.\nTries to get a value of type <code>T</code>.\nTries to get a value of an owned type <code>T</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes <code>Value</code> and returns the corresponding <code>GValue</code>.\nConsumes <code>SendValue</code> and returns the corresponding <code>GValue</code>.\nReturns <code>true</code> if the type of the value corresponds to <code>T</code> or …\nReturns <code>true</code> if the type of the value corresponds to <code>type_</code> …\nReturns a <code>SendValue</code> clone of <code>self</code>.\nConvert a value to a <code>Value</code>.\nConvert an <code>Option</code> to a <code>Value</code>.\nTries to transform the value into a value of the target …\nTries to transform the value into a value of the target …\nReturns the type of the value.\nReturns whether <code>Value</code>s of type <code>src</code> can be transformed to …\nReturns the type identifier of <code>self</code>.\nA Dictionary entry.\nWrapper type for fixed size type arrays.\nTrait for fixed size variant types.\nExtracts a value.\nA wrapper type around <code>Variant</code> handles.\nA wrapper type around <code>Variant</code> object paths.\nA wrapper type around <code>Variant</code> signatures.\nReturns <code>VariantType</code> of <code>Self</code>.\nConverts to <code>Variant</code>.\nA generic immutable value capable of carrying various …\nAn error returned from the <code>try_get</code> function on a <code>Variant</code> …\nCreates a new Variant array from a fixed array.\nCreates a new Variant array from children.\nCreates a new Variant array from children with the …\nCreate an iterator over borrowed strings from a GVariant …\nExtract the value of a maybe Variant.\nReturn the inner pointer to the underlying C value.\nUnboxes self.\nReturns a copy of the variant in the opposite endianness.\nRead a child item out of a container <code>Variant</code> instance.\nReads a child item out of a container <code>Variant</code> instance.\nReturns the classification of the variant.\nReturns the serialized form of a GVariant instance.\nReturns the serialized form of a GVariant instance.\nTries to extract a <code>&amp;[T]</code> from a variant of array type with …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a new serialized-mode GVariant instance.\nConstructs a new serialized-mode GVariant instance.\nConstructs a new serialized-mode GVariant instance with a …\nConstructs a new serialized-mode GVariant instance with a …\nConstructs a new serialized-mode GVariant instance.\nConstructs a new serialized-mode GVariant instance.\nConstructs a new serialized-mode GVariant instance with a …\nConstructs a new serialized-mode GVariant instance with a …\nCreates a new dictionary entry Variant.\nBorrows the underlying C value.\nCreates a new maybe Variant.\nCreates a new maybe Variant with Nothing.\nCreates a new maybe Variant from a child.\nTries to extract a value.\nBoxes value.\nTries to extract a value of type <code>T</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if the type of the value corresponds to <code>T</code>.\nReturn whether this Variant is a container type.\nReturn whether this Variant is in normal form.\nReturn whether input string is a valid …\nReturn whether input string is a valid …\nReturns <code>true</code> if the type of the value corresponds to <code>type_</code>.\nCreate an iterator over items in the variant.\nDetermines the number of children in a container GVariant …\nReturns a copy of the variant in normal form.\nParses a GVariant from the text representation produced by …\nPretty-print the contents of this variant in a …\nReturns the size of serialized form of a GVariant instance.\nReturns the <code>VariantType</code> corresponding to <code>Self</code>.\nStores the serialized form of a GVariant instance into the …\nTries to extract a <code>&amp;str</code>.\nReturns a <code>Variant</code> clone of <code>self</code>.\nTry to read a child item out of a container <code>Variant</code> …\nTry to read a child item out of a container <code>Variant</code> …\nTries to extract a value of type <code>T</code>.\nCreates a new Variant tuple from children.\nReturns the type of the value.")