#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListAllocateBufferAndSerialize(sourcecollection: *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes: *mut u32, ptargetbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListAllocateBufferAndSerialize(sourcecollection : *const SENSOR_COLLECTION_LIST, ptargetbuffersizeinbytes : *mut u32, ptargetbuffer : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListAllocateBufferAndSerialize(core::mem::transmute(sourcecollection), ptargetbuffersizeinbytes as _, ptargetbuffer as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListCopyAndMarshall(target: *mut SENSOR_COLLECTION_LIST, source: *const SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListCopyAndMarshall(target : *mut SENSOR_COLLECTION_LIST, source : *const SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListCopyAndMarshall(core::mem::transmute(target), core::mem::transmute(source)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListDeserializeFromBuffer(sourcebuffer: &[u8], targetcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListDeserializeFromBuffer(sourcebuffersizeinbytes : u32, sourcebuffer : *const u8, targetcollection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListDeserializeFromBuffer(sourcebuffer.len().try_into().unwrap(), core::mem::transmute(sourcebuffer.as_ptr()), core::mem::transmute(targetcollection)) }
}
#[inline]
pub unsafe fn CollectionsListGetFillableCount(buffersizebytes: u32) -> u32 {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetFillableCount(buffersizebytes : u32) -> u32);
    unsafe { CollectionsListGetFillableCount(buffersizebytes) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetMarshalledSize(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    unsafe { CollectionsListGetMarshalledSize(core::mem::transmute(collection)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListGetMarshalledSizeWithoutSerialization(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetMarshalledSizeWithoutSerialization(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    unsafe { CollectionsListGetMarshalledSizeWithoutSerialization(core::mem::transmute(collection)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListGetSerializedSize(collection: *const SENSOR_COLLECTION_LIST) -> u32 {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListGetSerializedSize(collection : *const SENSOR_COLLECTION_LIST) -> u32);
    unsafe { CollectionsListGetSerializedSize(core::mem::transmute(collection)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListMarshall(target: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListMarshall(target : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListMarshall(core::mem::transmute(target)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListSerializeToBuffer(sourcecollection: *const SENSOR_COLLECTION_LIST, targetbuffer: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListSerializeToBuffer(sourcecollection : *const SENSOR_COLLECTION_LIST, targetbuffersizeinbytes : u32, targetbuffer : *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListSerializeToBuffer(core::mem::transmute(sourcecollection), targetbuffer.len().try_into().unwrap(), core::mem::transmute(targetbuffer.as_ptr())) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds: *const SENSOR_COLLECTION_LIST, pcollection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListSortSubscribedActivitiesByConfidence(thresholds : *const SENSOR_COLLECTION_LIST, pcollection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListSortSubscribedActivitiesByConfidence(core::mem::transmute(thresholds), core::mem::transmute(pcollection)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CollectionsListUpdateMarshalledPointer(collection: *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn CollectionsListUpdateMarshalledPointer(collection : *mut SENSOR_COLLECTION_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { CollectionsListUpdateMarshalledPointer(core::mem::transmute(collection)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn EvaluateActivityThresholds(newsample: *const SENSOR_COLLECTION_LIST, oldsample: *const SENSOR_COLLECTION_LIST, thresholds: *const SENSOR_COLLECTION_LIST) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn EvaluateActivityThresholds(newsample : *const SENSOR_COLLECTION_LIST, oldsample : *const SENSOR_COLLECTION_LIST, thresholds : *const SENSOR_COLLECTION_LIST) -> bool);
    unsafe { EvaluateActivityThresholds(core::mem::transmute(newsample), core::mem::transmute(oldsample), core::mem::transmute(thresholds)) }
}
#[inline]
pub unsafe fn GetPerformanceTime(timems: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn GetPerformanceTime(timems : *mut u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetPerformanceTime(timems as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSIDArray(members: &[windows_core::GUID]) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    windows_link::link!("sensorsutilsv2.dll" "system" fn InitPropVariantFromCLSIDArray(members : *const windows_core::GUID, size : u32, ppropvar : *mut super::super::System::Com::StructuredStorage:: PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromCLSIDArray(core::mem::transmute(members.as_ptr()), members.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn InitPropVariantFromFloat(fltval: f32) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
    windows_link::link!("sensorsutilsv2.dll" "system" fn InitPropVariantFromFloat(fltval : f32, ppropvar : *mut super::super::System::Com::StructuredStorage:: PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromFloat(fltval, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn IsCollectionListSame(lista: *const SENSOR_COLLECTION_LIST, listb: *const SENSOR_COLLECTION_LIST) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn IsCollectionListSame(lista : *const SENSOR_COLLECTION_LIST, listb : *const SENSOR_COLLECTION_LIST) -> bool);
    unsafe { IsCollectionListSame(core::mem::transmute(lista), core::mem::transmute(listb)) }
}
#[inline]
pub unsafe fn IsGUIDPresentInList(guidarray: &[windows_core::GUID], guidelem: *const windows_core::GUID) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn IsGUIDPresentInList(guidarray : *const windows_core::GUID, arraylength : u32, guidelem : *const windows_core::GUID) -> bool);
    unsafe { IsGUIDPresentInList(core::mem::transmute(guidarray.as_ptr()), guidarray.len().try_into().unwrap(), guidelem) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn IsKeyPresentInCollectionList(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn IsKeyPresentInCollectionList(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY) -> bool);
    unsafe { IsKeyPresentInCollectionList(core::mem::transmute(plist), pkey) }
}
#[inline]
pub unsafe fn IsKeyPresentInPropertyList(plist: *const SENSOR_PROPERTY_LIST, pkey: *const super::super::Foundation::PROPERTYKEY) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn IsKeyPresentInPropertyList(plist : *const SENSOR_PROPERTY_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY) -> bool);
    unsafe { IsKeyPresentInPropertyList(plist, pkey) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn IsSensorSubscribed(subscriptionlist: *const SENSOR_COLLECTION_LIST, currenttype: windows_core::GUID) -> bool {
    windows_link::link!("sensorsutilsv2.dll" "system" fn IsSensorSubscribed(subscriptionlist : *const SENSOR_COLLECTION_LIST, currenttype : windows_core::GUID) -> bool);
    unsafe { IsSensorSubscribed(core::mem::transmute(subscriptionlist), core::mem::transmute(currenttype)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetBool(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut windows_core::BOOL) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetBool(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut windows_core::BOOL) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetBool(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetDouble(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut f64) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetDouble(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut f64) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetDouble(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetFileTime(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetFileTime(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetFileTime(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetFloat(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut f32) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetFloat(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut f32) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetFloat(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetGuid(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut windows_core::GUID) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetGuid(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut windows_core::GUID) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetGuid(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt32(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut i32) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetInt32(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut i32) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetInt32(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetInt64(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut i64) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetInt64(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthInt64(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, occurrence: u32, pretvalue: *mut i64) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthInt64(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, occurrence : u32, pretvalue : *mut i64) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetNthInt64(core::mem::transmute(plist), pkey, occurrence, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, occurrence: u32, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthUlong(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, occurrence : u32, pretvalue : *mut u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetNthUlong(core::mem::transmute(plist), pkey, occurrence, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetNthUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, occurrence: u32, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetNthUshort(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, occurrence : u32, pretvalue : *mut u16) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetNthUshort(core::mem::transmute(plist), pkey, occurrence, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetPropVariant(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, typecheck: bool, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetPropVariant(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, typecheck : bool, pvalue : *mut super::super::System::Com::StructuredStorage:: PROPVARIANT) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetPropVariant(core::mem::transmute(plist), pkey, typecheck, core::mem::transmute(pvalue)) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetUlong(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetUlong(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetUlong(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeyGetUshort(plist: *const SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, pretvalue: *mut u16) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeyGetUshort(plist : *const SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, pretvalue : *mut u16) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeyGetUshort(core::mem::transmute(plist), pkey, pretvalue as _) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropKeyFindKeySetPropVariant(plist: *mut SENSOR_COLLECTION_LIST, pkey: *const super::super::Foundation::PROPERTYKEY, typecheck: bool, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropKeyFindKeySetPropVariant(plist : *mut SENSOR_COLLECTION_LIST, pkey : *const super::super::Foundation:: PROPERTYKEY, typecheck : bool, pvalue : *const super::super::System::Com::StructuredStorage:: PROPVARIANT) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropKeyFindKeySetPropVariant(core::mem::transmute(plist), pkey, typecheck, core::mem::transmute(pvalue)) }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropVariantGetInformation(propvariantvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propvariantoffset: Option<*mut u32>, propvariantsize: Option<*mut u32>, propvariantpointer: Option<*mut *mut core::ffi::c_void>, remappedtype: Option<*mut super::Properties::DEVPROPTYPE>) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropVariantGetInformation(propvariantvalue : *const super::super::System::Com::StructuredStorage:: PROPVARIANT, propvariantoffset : *mut u32, propvariantsize : *mut u32, propvariantpointer : *mut *mut core::ffi::c_void, remappedtype : *mut super::Properties:: DEVPROPTYPE) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropVariantGetInformation(core::mem::transmute(propvariantvalue), propvariantoffset.unwrap_or(core::mem::zeroed()) as _, propvariantsize.unwrap_or(core::mem::zeroed()) as _, propvariantpointer.unwrap_or(core::mem::zeroed()) as _, remappedtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PropertiesListCopy(target: *mut SENSOR_PROPERTY_LIST, source: *const SENSOR_PROPERTY_LIST) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropertiesListCopy(target : *mut SENSOR_PROPERTY_LIST, source : *const SENSOR_PROPERTY_LIST) -> super::super::Foundation:: NTSTATUS);
    unsafe { PropertiesListCopy(target as _, source) }
}
#[inline]
pub unsafe fn PropertiesListGetFillableCount(buffersizebytes: u32) -> u32 {
    windows_link::link!("sensorsutilsv2.dll" "system" fn PropertiesListGetFillableCount(buffersizebytes : u32) -> u32);
    unsafe { PropertiesListGetFillableCount(buffersizebytes) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SensorCollectionGetAt(index: u32, psensorslist: *const SENSOR_COLLECTION_LIST, pkey: Option<*mut super::super::Foundation::PROPERTYKEY>, pvalue: Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn SensorCollectionGetAt(index : u32, psensorslist : *const SENSOR_COLLECTION_LIST, pkey : *mut super::super::Foundation:: PROPERTYKEY, pvalue : *mut super::super::System::Com::StructuredStorage:: PROPVARIANT) -> super::super::Foundation:: NTSTATUS);
    unsafe { SensorCollectionGetAt(index, core::mem::transmute(psensorslist), pkey.unwrap_or(core::mem::zeroed()) as _, pvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SerializationBufferAllocate(sizeinbytes: u32, pbuffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_link::link!("sensorsutilsv2.dll" "system" fn SerializationBufferAllocate(sizeinbytes : u32, pbuffer : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { SerializationBufferAllocate(sizeinbytes, pbuffer as _) }
}
#[inline]
pub unsafe fn SerializationBufferFree(buffer: Option<*const u8>) {
    windows_link::link!("sensorsutilsv2.dll" "system" fn SerializationBufferFree(buffer : *const u8));
    unsafe { SerializationBufferFree(buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTIVITY_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTIVITY_STATE_COUNT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AXIS(pub i32);
pub const AXIS_MAX: AXIS = AXIS(3i32);
pub const AXIS_X: AXIS = AXIS(0i32);
pub const AXIS_Y: AXIS = AXIS(1i32);
pub const AXIS_Z: AXIS = AXIS(2i32);
pub const ActivityStateCount: ACTIVITY_STATE_COUNT = ACTIVITY_STATE_COUNT(8i32);
pub const ActivityState_Biking: ACTIVITY_STATE = ACTIVITY_STATE(64i32);
pub const ActivityState_Fidgeting: ACTIVITY_STATE = ACTIVITY_STATE(4i32);
pub const ActivityState_Force_Dword: ACTIVITY_STATE = ACTIVITY_STATE(-1i32);
pub const ActivityState_Idle: ACTIVITY_STATE = ACTIVITY_STATE(128i32);
pub const ActivityState_InVehicle: ACTIVITY_STATE = ACTIVITY_STATE(32i32);
pub const ActivityState_Max: ACTIVITY_STATE = ACTIVITY_STATE(256i32);
pub const ActivityState_Running: ACTIVITY_STATE = ACTIVITY_STATE(16i32);
pub const ActivityState_Stationary: ACTIVITY_STATE = ACTIVITY_STATE(2i32);
pub const ActivityState_Unknown: ACTIVITY_STATE = ACTIVITY_STATE(1i32);
pub const ActivityState_Walking: ACTIVITY_STATE = ACTIVITY_STATE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ELEVATION_CHANGE_MODE(pub i32);
pub const ElevationChangeMode_Elevator: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(1i32);
pub const ElevationChangeMode_Force_Dword: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(-1i32);
pub const ElevationChangeMode_Max: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(3i32);
pub const ElevationChangeMode_Stepping: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(2i32);
pub const ElevationChangeMode_Unknown: ELEVATION_CHANGE_MODE = ELEVATION_CHANGE_MODE(0i32);
pub const GNSS_CLEAR_ALL_ASSISTANCE_DATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_SENSOR: windows_core::GUID = windows_core::GUID::from_u128(0xba1bb692_9b7a_4833_9a1e_525ed134e7e2);
pub const GUID_SensorCategory_All: windows_core::GUID = windows_core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const GUID_SensorCategory_Biometric: windows_core::GUID = windows_core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const GUID_SensorCategory_Electrical: windows_core::GUID = windows_core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const GUID_SensorCategory_Environmental: windows_core::GUID = windows_core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const GUID_SensorCategory_Light: windows_core::GUID = windows_core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const GUID_SensorCategory_Location: windows_core::GUID = windows_core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const GUID_SensorCategory_Mechanical: windows_core::GUID = windows_core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const GUID_SensorCategory_Motion: windows_core::GUID = windows_core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const GUID_SensorCategory_Orientation: windows_core::GUID = windows_core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const GUID_SensorCategory_Other: windows_core::GUID = windows_core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const GUID_SensorCategory_PersonalActivity: windows_core::GUID = windows_core::GUID::from_u128(0xf1609081_1e12_412b_a14d_cbb0e95bd2e5);
pub const GUID_SensorCategory_Scanner: windows_core::GUID = windows_core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const GUID_SensorCategory_Unsupported: windows_core::GUID = windows_core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
pub const GUID_SensorType_Accelerometer3D: windows_core::GUID = windows_core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const GUID_SensorType_ActivityDetection: windows_core::GUID = windows_core::GUID::from_u128(0x9d9e0118_1807_4f2e_96e4_2ce57142e196);
pub const GUID_SensorType_AmbientLight: windows_core::GUID = windows_core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const GUID_SensorType_Barometer: windows_core::GUID = windows_core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const GUID_SensorType_Custom: windows_core::GUID = windows_core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const GUID_SensorType_FloorElevation: windows_core::GUID = windows_core::GUID::from_u128(0xade4987f_7ac4_4dfa_9722_0a027181c747);
pub const GUID_SensorType_GeomagneticOrientation: windows_core::GUID = windows_core::GUID::from_u128(0xe77195f8_2d1f_4823_971b_1c4467556c9d);
pub const GUID_SensorType_GravityVector: windows_core::GUID = windows_core::GUID::from_u128(0x03b52c73_bb76_463f_9524_38de76eb700b);
pub const GUID_SensorType_Gyrometer3D: windows_core::GUID = windows_core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const GUID_SensorType_HingeAngle: windows_core::GUID = windows_core::GUID::from_u128(0x82358065_f4c4_4da1_b272_13c23332a207);
pub const GUID_SensorType_Humidity: windows_core::GUID = windows_core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const GUID_SensorType_LinearAccelerometer: windows_core::GUID = windows_core::GUID::from_u128(0x038b0283_97b4_41c8_bc24_5ff1aa48fec7);
pub const GUID_SensorType_Magnetometer3D: windows_core::GUID = windows_core::GUID::from_u128(0x55e5effb_15c7_40df_8698_a84b7c863c53);
pub const GUID_SensorType_Orientation: windows_core::GUID = windows_core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const GUID_SensorType_Pedometer: windows_core::GUID = windows_core::GUID::from_u128(0xb19f89af_e3eb_444b_8dea_202575a71599);
pub const GUID_SensorType_Proximity: windows_core::GUID = windows_core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const GUID_SensorType_RelativeOrientation: windows_core::GUID = windows_core::GUID::from_u128(0x40993b51_4706_44dc_98d5_c920c037ffab);
pub const GUID_SensorType_SimpleDeviceOrientation: windows_core::GUID = windows_core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const GUID_SensorType_Temperature: windows_core::GUID = windows_core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUMAN_PRESENCE_DETECTION_TYPE_COUNT(pub i32);
pub const HumanPresenceDetectionTypeCount: HUMAN_PRESENCE_DETECTION_TYPE_COUNT = HUMAN_PRESENCE_DETECTION_TYPE_COUNT(4i32);
pub const HumanPresenceDetectionType_AudioBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(8i32);
pub const HumanPresenceDetectionType_FacialBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(4i32);
pub const HumanPresenceDetectionType_Force_Dword: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(-1i32);
pub const HumanPresenceDetectionType_Undefined: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(0i32);
pub const HumanPresenceDetectionType_VendorDefinedBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(2i32);
pub const HumanPresenceDetectionType_VendorDefinedNonBiometric: HUMAN_PRESENCE_DETECTION_TYPE = HUMAN_PRESENCE_DETECTION_TYPE(1i32);
windows_core::imp::define_interface!(ILocationPermissions, ILocationPermissions_Vtbl, 0xd5fb0a7f_e74e_44f5_8e02_4806863a274f);
windows_core::imp::interface_hierarchy!(ILocationPermissions, windows_core::IUnknown);
impl ILocationPermissions {
    pub unsafe fn GetGlobalLocationPermission(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalLocationPermission)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckLocationCapability(&self, dwclientthreadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckLocationCapability)(windows_core::Interface::as_raw(self), dwclientthreadid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPermissions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGlobalLocationPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CheckLocationCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ILocationPermissions_Impl: windows_core::IUnknownImpl {
    fn GetGlobalLocationPermission(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CheckLocationCapability(&self, dwclientthreadid: u32) -> windows_core::Result<()>;
}
impl ILocationPermissions_Vtbl {
    pub const fn new<Identity: ILocationPermissions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGlobalLocationPermission<Identity: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationPermissions_Impl::GetGlobalLocationPermission(this) {
                    Ok(ok__) => {
                        pfenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckLocationCapability<Identity: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientthreadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationPermissions_Impl::CheckLocationCapability(this, core::mem::transmute_copy(&dwclientthreadid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGlobalLocationPermission: GetGlobalLocationPermission::<Identity, OFFSET>,
            CheckLocationCapability: CheckLocationCapability::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationPermissions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILocationPermissions {}
windows_core::imp::define_interface!(ISensor, ISensor_Vtbl, 0x5fa08f80_2657_458e_af75_46f73fa6ac5c);
windows_core::imp::interface_hierarchy!(ISensor, windows_core::IUnknown);
impl ISensor {
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetProperties<P0>(&self, pkeys: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSupportedDataFields(&self) -> windows_core::Result<super::PortableDevices::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedDataFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn SetProperties<P0>(&self, pproperties: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), pproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SupportsDataField(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsDataField)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<SensorState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetData(&self) -> windows_core::Result<ISensorDataReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SupportsEvent(&self, eventguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsEvent)(windows_core::Interface::as_raw(self), eventguid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventInterest(&self, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEventInterest)(windows_core::Interface::as_raw(self), ppvalues as _, pcount as _).ok() }
    }
    pub unsafe fn SetEventInterest(&self, pvalues: Option<&[windows_core::GUID]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventInterest)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensorEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSupportedDataFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSupportedDataFields: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    SetProperties: usize,
    pub SupportsDataField: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorState) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISensor_Impl: windows_core::IUnknownImpl {
    fn GetID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetCategory(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetProperties(&self, pkeys: windows_core::Ref<'_, super::PortableDevices::IPortableDeviceKeyCollection>) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn GetSupportedDataFields(&self) -> windows_core::Result<super::PortableDevices::IPortableDeviceKeyCollection>;
    fn SetProperties(&self, pproperties: windows_core::Ref<'_, super::PortableDevices::IPortableDeviceValues>) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn SupportsDataField(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetState(&self) -> windows_core::Result<SensorState>;
    fn GetData(&self) -> windows_core::Result<ISensorDataReport>;
    fn SupportsEvent(&self, eventguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetEventInterest(&self, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::Result<()>;
    fn SetEventInterest(&self, pvalues: *const windows_core::GUID, count: u32) -> windows_core::Result<()>;
    fn SetEventSink(&self, pevents: windows_core::Ref<'_, ISensorEvents>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISensor_Vtbl {
    pub const fn new<Identity: ISensor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetID<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetID(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensorcategory: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetCategory(this) {
                    Ok(ok__) => {
                        psensorcategory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensortype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetType(this) {
                    Ok(ok__) => {
                        psensortype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetFriendlyName(this) {
                    Ok(ok__) => {
                        pfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetProperty(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetProperties(this, core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedDataFields<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdatafields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetSupportedDataFields(this) {
                    Ok(ok__) => {
                        ppdatafields.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperties<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SetProperties(this, core::mem::transmute_copy(&pproperties)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportsDataField<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SupportsDataField(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pissupported.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetData<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdatareport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetData(this) {
                    Ok(ok__) => {
                        ppdatareport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportsEvent<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventguid: *const windows_core::GUID, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SupportsEvent(this, core::mem::transmute_copy(&eventguid)) {
                    Ok(ok__) => {
                        pissupported.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventInterest<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::GetEventInterest(this, core::mem::transmute_copy(&ppvalues), core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *const windows_core::GUID, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::SetEventInterest(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::SetEventSink(this, core::mem::transmute_copy(&pevents)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSupportedDataFields: GetSupportedDataFields::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
            SupportsDataField: SupportsDataField::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SupportsEvent: SupportsEvent::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensor {}
windows_core::imp::define_interface!(ISensorCollection, ISensorCollection_Vtbl, 0x23571e11_e545_4dd8_a337_b89bf44b10df);
windows_core::imp::interface_hierarchy!(ISensorCollection, windows_core::IUnknown);
impl ISensorCollection {
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<ISensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, psensor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), psensor.param().abi()).ok() }
    }
    pub unsafe fn Remove<P0>(&self, psensor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), psensor.param().abi()).ok() }
    }
    pub unsafe fn RemoveByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveByID)(windows_core::Interface::as_raw(self), sensorid).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISensorCollection_Impl: windows_core::IUnknownImpl {
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<ISensor>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Add(&self, psensor: windows_core::Ref<'_, ISensor>) -> windows_core::Result<()>;
    fn Remove(&self, psensor: windows_core::Ref<'_, ISensor>) -> windows_core::Result<()>;
    fn RemoveByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl ISensorCollection_Vtbl {
    pub const fn new<Identity: ISensorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppsensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppsensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Add(this, core::mem::transmute_copy(&psensor)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Remove(this, core::mem::transmute_copy(&psensor)).into()
            }
        }
        unsafe extern "system" fn RemoveByID<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::RemoveByID(this, core::mem::transmute_copy(&sensorid)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveByID: RemoveByID::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISensorCollection {}
windows_core::imp::define_interface!(ISensorDataReport, ISensorDataReport_Vtbl, 0x0ab9df9b_c4b5_4796_8898_0470706a2e1d);
windows_core::imp::interface_hierarchy!(ISensorDataReport, windows_core::IUnknown);
impl ISensorDataReport {
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetSensorValue(&self, pkey: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorValue)(windows_core::Interface::as_raw(self), pkey, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetSensorValues<P0>(&self, pkeys: P0) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::PortableDevices::IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorValues)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetSensorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetSensorValue: usize,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetSensorValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetSensorValues: usize,
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISensorDataReport_Impl: windows_core::IUnknownImpl {
    fn GetTimestamp(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetSensorValue(&self, pkey: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetSensorValues(&self, pkeys: windows_core::Ref<'_, super::PortableDevices::IPortableDeviceKeyCollection>) -> windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISensorDataReport_Vtbl {
    pub const fn new<Identity: ISensorDataReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTimestamp<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetTimestamp(this) {
                    Ok(ok__) => {
                        ptimestamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorValue<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetSensorValue(this, core::mem::transmute_copy(&pkey)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorValues<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetSensorValues(this, core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimestamp: GetTimestamp::<Identity, OFFSET>,
            GetSensorValue: GetSensorValue::<Identity, OFFSET>,
            GetSensorValues: GetSensorValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorDataReport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensorDataReport {}
windows_core::imp::define_interface!(ISensorEvents, ISensorEvents_Vtbl, 0x5d8dcc91_4641_47e7_b7c3_b74f48a6c391);
windows_core::imp::interface_hierarchy!(ISensorEvents, windows_core::IUnknown);
impl ISensorEvents {
    pub unsafe fn OnStateChanged<P0>(&self, psensor: P0, state: SensorState) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), psensor.param().abi(), state).ok() }
    }
    pub unsafe fn OnDataUpdated<P0, P1>(&self, psensor: P0, pnewdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
        P1: windows_core::Param<ISensorDataReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDataUpdated)(windows_core::Interface::as_raw(self), psensor.param().abi(), pnewdata.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn OnEvent<P0, P2>(&self, psensor: P0, eventid: *const windows_core::GUID, peventdata: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
        P2: windows_core::Param<super::PortableDevices::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), psensor.param().abi(), eventid, peventdata.param().abi()).ok() }
    }
    pub unsafe fn OnLeave(&self, id: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLeave)(windows_core::Interface::as_raw(self), id).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
    pub OnDataUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    OnEvent: usize,
    pub OnLeave: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait ISensorEvents_Impl: windows_core::IUnknownImpl {
    fn OnStateChanged(&self, psensor: windows_core::Ref<'_, ISensor>, state: SensorState) -> windows_core::Result<()>;
    fn OnDataUpdated(&self, psensor: windows_core::Ref<'_, ISensor>, pnewdata: windows_core::Ref<'_, ISensorDataReport>) -> windows_core::Result<()>;
    fn OnEvent(&self, psensor: windows_core::Ref<'_, ISensor>, eventid: *const windows_core::GUID, peventdata: windows_core::Ref<'_, super::PortableDevices::IPortableDeviceValues>) -> windows_core::Result<()>;
    fn OnLeave(&self, id: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ISensorEvents_Vtbl {
    pub const fn new<Identity: ISensorEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStateChanged<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, state: SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnStateChanged(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn OnDataUpdated<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, pnewdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnDataUpdated(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&pnewdata)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, eventid: *const windows_core::GUID, peventdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnEvent(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&peventdata)).into()
            }
        }
        unsafe extern "system" fn OnLeave<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnLeave(this, core::mem::transmute_copy(&id)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnDataUpdated: OnDataUpdated::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
            OnLeave: OnLeave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl windows_core::RuntimeName for ISensorEvents {}
windows_core::imp::define_interface!(ISensorManager, ISensorManager_Vtbl, 0xbd77db67_45a8_42dc_8d00_6dcf15f8377a);
windows_core::imp::interface_hierarchy!(ISensorManager, windows_core::IUnknown);
impl ISensorManager {
    pub unsafe fn GetSensorsByCategory(&self, sensorcategory: *const windows_core::GUID) -> windows_core::Result<ISensorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorsByCategory)(windows_core::Interface::as_raw(self), sensorcategory, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSensorsByType(&self, sensortype: *const windows_core::GUID) -> windows_core::Result<ISensorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorsByType)(windows_core::Interface::as_raw(self), sensortype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSensorByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<ISensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorByID)(windows_core::Interface::as_raw(self), sensorid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensorManagerEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()).ok() }
    }
    pub unsafe fn RequestPermissions<P1>(&self, hparent: super::super::Foundation::HWND, psensors: P1, fmodal: bool) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ISensorCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent, psensors.param().abi(), fmodal.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSensorsByCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorsByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorByID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ISensorManager_Impl: windows_core::IUnknownImpl {
    fn GetSensorsByCategory(&self, sensorcategory: *const windows_core::GUID) -> windows_core::Result<ISensorCollection>;
    fn GetSensorsByType(&self, sensortype: *const windows_core::GUID) -> windows_core::Result<ISensorCollection>;
    fn GetSensorByID(&self, sensorid: *const windows_core::GUID) -> windows_core::Result<ISensor>;
    fn SetEventSink(&self, pevents: windows_core::Ref<'_, ISensorManagerEvents>) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::super::Foundation::HWND, psensors: windows_core::Ref<'_, ISensorCollection>, fmodal: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ISensorManager_Vtbl {
    pub const fn new<Identity: ISensorManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSensorsByCategory<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorcategory: *const windows_core::GUID, ppsensorsfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorsByCategory(this, core::mem::transmute_copy(&sensorcategory)) {
                    Ok(ok__) => {
                        ppsensorsfound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorsByType<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensortype: *const windows_core::GUID, ppsensorsfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorsByType(this, core::mem::transmute_copy(&sensortype)) {
                    Ok(ok__) => {
                        ppsensorsfound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorByID<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorid: *const windows_core::GUID, ppsensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorByID(this, core::mem::transmute_copy(&sensorid)) {
                    Ok(ok__) => {
                        ppsensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManager_Impl::SetEventSink(this, core::mem::transmute_copy(&pevents)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: *mut core::ffi::c_void, fmodal: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManager_Impl::RequestPermissions(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&psensors), core::mem::transmute_copy(&fmodal)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSensorsByCategory: GetSensorsByCategory::<Identity, OFFSET>,
            GetSensorsByType: GetSensorsByType::<Identity, OFFSET>,
            GetSensorByID: GetSensorByID::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISensorManager {}
windows_core::imp::define_interface!(ISensorManagerEvents, ISensorManagerEvents_Vtbl, 0x9b3b0b86_266a_4aad_b21f_fde5501001b7);
windows_core::imp::interface_hierarchy!(ISensorManagerEvents, windows_core::IUnknown);
impl ISensorManagerEvents {
    pub unsafe fn OnSensorEnter<P0>(&self, psensor: P0, state: SensorState) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSensorEnter)(windows_core::Interface::as_raw(self), psensor.param().abi(), state).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSensorEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
}
pub trait ISensorManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnSensorEnter(&self, psensor: windows_core::Ref<'_, ISensor>, state: SensorState) -> windows_core::Result<()>;
}
impl ISensorManagerEvents_Vtbl {
    pub const fn new<Identity: ISensorManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSensorEnter<Identity: ISensorManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, state: SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManagerEvents_Impl::OnSensorEnter(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&state)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSensorEnter: OnSensorEnter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISensorManagerEvents {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCATION_DESIRED_ACCURACY(pub i32);
pub const LOCATION_DESIRED_ACCURACY_DEFAULT: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(0i32);
pub const LOCATION_DESIRED_ACCURACY_HIGH: LOCATION_DESIRED_ACCURACY = LOCATION_DESIRED_ACCURACY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCATION_POSITION_SOURCE(pub i32);
pub const LOCATION_POSITION_SOURCE_CELLULAR: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(0i32);
pub const LOCATION_POSITION_SOURCE_IPADDRESS: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(3i32);
pub const LOCATION_POSITION_SOURCE_SATELLITE: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(1i32);
pub const LOCATION_POSITION_SOURCE_UNKNOWN: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(4i32);
pub const LOCATION_POSITION_SOURCE_WIFI: LOCATION_POSITION_SOURCE = LOCATION_POSITION_SOURCE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAGNETOMETER_ACCURACY(pub i32);
pub const MAGNETOMETER_ACCURACY_APPROXIMATE: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
pub const MAGNETOMETER_ACCURACY_HIGH: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
pub const MAGNETOMETER_ACCURACY_UNKNOWN: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
pub const MAGNETOMETER_ACCURACY_UNRELIABLE: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MATRIX3X3 {
    pub Anonymous: MATRIX3X3_0,
}
impl Default for MATRIX3X3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MATRIX3X3_0 {
    pub Anonymous1: MATRIX3X3_0_0,
    pub Anonymous2: MATRIX3X3_0_1,
    pub M: [f32; 9],
}
impl Default for MATRIX3X3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MATRIX3X3_0_0 {
    pub A11: f32,
    pub A12: f32,
    pub A13: f32,
    pub A21: f32,
    pub A22: f32,
    pub A23: f32,
    pub A31: f32,
    pub A32: f32,
    pub A33: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MATRIX3X3_0_1 {
    pub V1: VEC3D,
    pub V2: VEC3D,
    pub V3: VEC3D,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MagnetometerAccuracy(pub i32);
pub const MagnetometerAccuracy_Approximate: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(2i32);
pub const MagnetometerAccuracy_High: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(3i32);
pub const MagnetometerAccuracy_Unknown: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(0i32);
pub const MagnetometerAccuracy_Unreliable: MAGNETOMETER_ACCURACY = MAGNETOMETER_ACCURACY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEDOMETER_STEP_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEDOMETER_STEP_TYPE_COUNT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROXIMITY_SENSOR_CAPABILITIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROXIMITY_TYPE(pub i32);
pub const PedometerStepTypeCount: PEDOMETER_STEP_TYPE_COUNT = PEDOMETER_STEP_TYPE_COUNT(3i32);
pub const PedometerStepType_Force_Dword: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(-1i32);
pub const PedometerStepType_Max: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(8i32);
pub const PedometerStepType_Running: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(4i32);
pub const PedometerStepType_Unknown: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(1i32);
pub const PedometerStepType_Walking: PEDOMETER_STEP_TYPE = PEDOMETER_STEP_TYPE(2i32);
pub const ProximityType_Force_Dword: PROXIMITY_TYPE = PROXIMITY_TYPE(-1i32);
pub const ProximityType_HumanProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(1i32);
pub const ProximityType_ObjectProximity: PROXIMITY_TYPE = PROXIMITY_TYPE(0i32);
pub const Proximity_Sensor_Human_Engagement_Capable: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(2i32);
pub const Proximity_Sensor_Human_Presence_Capable: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(1i32);
pub const Proximity_Sensor_Supported_Capabilities: PROXIMITY_SENSOR_CAPABILITIES = PROXIMITY_SENSOR_CAPABILITIES(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUATERNION {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
pub const SENSOR_CATEGORY_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xc317c286_c468_4288_9975_d4c4587c442c);
pub const SENSOR_CATEGORY_BIOMETRIC: windows_core::GUID = windows_core::GUID::from_u128(0xca19690f_a2c7_477d_a99e_99ec6e2b5648);
pub const SENSOR_CATEGORY_ELECTRICAL: windows_core::GUID = windows_core::GUID::from_u128(0xfb73fcd8_fc4a_483c_ac58_27b691c6beff);
pub const SENSOR_CATEGORY_ENVIRONMENTAL: windows_core::GUID = windows_core::GUID::from_u128(0x323439aa_7f66_492b_ba0c_73e9aa0a65d5);
pub const SENSOR_CATEGORY_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0x17a665c0_9063_4216_b202_5c7a255e18ce);
pub const SENSOR_CATEGORY_LOCATION: windows_core::GUID = windows_core::GUID::from_u128(0xbfa794e4_f964_4fdb_90f6_51056bfe4b44);
pub const SENSOR_CATEGORY_MECHANICAL: windows_core::GUID = windows_core::GUID::from_u128(0x8d131d68_8ef7_4656_80b5_cccbd93791c5);
pub const SENSOR_CATEGORY_MOTION: windows_core::GUID = windows_core::GUID::from_u128(0xcd09daf1_3b2e_4c3d_b598_b5e5ff93fd46);
pub const SENSOR_CATEGORY_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x9e6c04b6_96fe_4954_b726_68682a473f69);
pub const SENSOR_CATEGORY_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x2c90e7a9_f4c9_4fa2_af37_56d471fe5a3d);
pub const SENSOR_CATEGORY_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0xb000e77e_f5b5_420f_815d_0270a726f270);
pub const SENSOR_CATEGORY_UNSUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x2beae7fa_19b0_48c5_a1f6_b5480dc206b0);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct SENSOR_COLLECTION_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [SENSOR_VALUE_PAIR; 1],
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for SENSOR_COLLECTION_LIST {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SENSOR_COLLECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SENSOR_CONNECTION_TYPES(pub i32);
pub const SENSOR_CONNECTION_TYPE_PC_ATTACHED: SensorConnectionType = SensorConnectionType(1i32);
pub const SENSOR_CONNECTION_TYPE_PC_EXTERNAL: SensorConnectionType = SensorConnectionType(2i32);
pub const SENSOR_CONNECTION_TYPE_PC_INTEGRATED: SensorConnectionType = SensorConnectionType(0i32);
pub const SENSOR_DATA_TYPE_ABSOLUTE_PRESSURE_PASCAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 5 };
pub const SENSOR_DATA_TYPE_ACCELERATION_X_G: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 2 };
pub const SENSOR_DATA_TYPE_ACCELERATION_Y_G: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 3 };
pub const SENSOR_DATA_TYPE_ACCELERATION_Z_G: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 4 };
pub const SENSOR_DATA_TYPE_ADDRESS1: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 23 };
pub const SENSOR_DATA_TYPE_ADDRESS2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 24 };
pub const SENSOR_DATA_TYPE_ALTITUDE_ANTENNA_SEALEVEL_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 36 };
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_ERROR_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 29 };
pub const SENSOR_DATA_TYPE_ALTITUDE_ELLIPSOID_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 5 };
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_ERROR_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 30 };
pub const SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 4 };
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_X_DEGREES_PER_SECOND_SQUARED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 5 };
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Y_DEGREES_PER_SECOND_SQUARED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 6 };
pub const SENSOR_DATA_TYPE_ANGULAR_ACCELERATION_Z_DEGREES_PER_SECOND_SQUARED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 7 };
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_X_DEGREES_PER_SECOND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 10 };
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Y_DEGREES_PER_SECOND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 11 };
pub const SENSOR_DATA_TYPE_ANGULAR_VELOCITY_Z_DEGREES_PER_SECOND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 12 };
pub const SENSOR_DATA_TYPE_ATMOSPHERIC_PRESSURE_BAR: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 4 };
pub const SENSOR_DATA_TYPE_BIOMETRIC_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af);
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_ARRAY_STATES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 10 };
pub const SENSOR_DATA_TYPE_BOOLEAN_SWITCH_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 2 };
pub const SENSOR_DATA_TYPE_CAPACITANCE_FARAD: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 4 };
pub const SENSOR_DATA_TYPE_CITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 25 };
pub const SENSOR_DATA_TYPE_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150);
pub const SENSOR_DATA_TYPE_COUNTRY_REGION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 28 };
pub const SENSOR_DATA_TYPE_CURRENT_AMPS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 3 };
pub const SENSOR_DATA_TYPE_CUSTOM_BOOLEAN_ARRAY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 6 };
pub const SENSOR_DATA_TYPE_CUSTOM_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f);
pub const SENSOR_DATA_TYPE_CUSTOM_USAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 5 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE1: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 7 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE10: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 16 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE11: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 17 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE12: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 18 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE13: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 19 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE14: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 20 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE15: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 21 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE16: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 22 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE17: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 23 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE18: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 24 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE19: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 25 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 8 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE20: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 26 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE21: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 27 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE22: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 28 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE23: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 29 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE24: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 30 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE25: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 31 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE26: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 32 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE27: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 33 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE28: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 34 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE3: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 9 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE4: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 10 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE5: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 11 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE6: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 12 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE7: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 13 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE8: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 14 };
pub const SENSOR_DATA_TYPE_CUSTOM_VALUE9: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb14c764f_07cf_41e8_9d82_ebe3d0776a6f), pid: 15 };
pub const SENSOR_DATA_TYPE_DGPS_DATA_AGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 35 };
pub const SENSOR_DATA_TYPE_DIFFERENTIAL_REFERENCE_STATION_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 37 };
pub const SENSOR_DATA_TYPE_DISTANCE_X_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 8 };
pub const SENSOR_DATA_TYPE_DISTANCE_Y_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 9 };
pub const SENSOR_DATA_TYPE_DISTANCE_Z_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 10 };
pub const SENSOR_DATA_TYPE_ELECTRICAL_FREQUENCY_HERTZ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 9 };
pub const SENSOR_DATA_TYPE_ELECTRICAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842);
pub const SENSOR_DATA_TYPE_ELECTRICAL_PERCENT_OF_RANGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 8 };
pub const SENSOR_DATA_TYPE_ELECTRICAL_POWER_WATTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 7 };
pub const SENSOR_DATA_TYPE_ENVIRONMENTAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4);
pub const SENSOR_DATA_TYPE_ERROR_RADIUS_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 22 };
pub const SENSOR_DATA_TYPE_FIX_QUALITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 10 };
pub const SENSOR_DATA_TYPE_FIX_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 11 };
pub const SENSOR_DATA_TYPE_FORCE_NEWTONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 4 };
pub const SENSOR_DATA_TYPE_GAUGE_PRESSURE_PASCAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 6 };
pub const SENSOR_DATA_TYPE_GEOIDAL_SEPARATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 34 };
pub const SENSOR_DATA_TYPE_GPS_OPERATION_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 32 };
pub const SENSOR_DATA_TYPE_GPS_SELECTION_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 31 };
pub const SENSOR_DATA_TYPE_GPS_STATUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 33 };
pub const SENSOR_DATA_TYPE_GUID_MECHANICAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df);
pub const SENSOR_DATA_TYPE_HORIZONAL_DILUTION_OF_PRECISION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 13 };
pub const SENSOR_DATA_TYPE_HUMAN_PRESENCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 2 };
pub const SENSOR_DATA_TYPE_HUMAN_PROXIMITY_METERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 3 };
pub const SENSOR_DATA_TYPE_INDUCTANCE_HENRY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 6 };
pub const SENSOR_DATA_TYPE_LATITUDE_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 2 };
pub const SENSOR_DATA_TYPE_LIGHT_CHROMACITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 4 };
pub const SENSOR_DATA_TYPE_LIGHT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6);
pub const SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 2 };
pub const SENSOR_DATA_TYPE_LIGHT_TEMPERATURE_KELVIN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c77ce2_dcb7_46e9_8439_4fec548833a6), pid: 3 };
pub const SENSOR_DATA_TYPE_LOCATION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4);
pub const SENSOR_DATA_TYPE_LOCATION_SOURCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 40 };
pub const SENSOR_DATA_TYPE_LONGITUDE_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 3 };
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_X_MILLIGAUSS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 19 };
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Y_MILLIGAUSS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 20 };
pub const SENSOR_DATA_TYPE_MAGNETIC_FIELD_STRENGTH_Z_MILLIGAUSS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 21 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_MAGNETIC_NORTH_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 11 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_COMPENSATED_TRUE_NORTH_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 12 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 8 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_MAGNETIC_NORTH_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 13 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_TRUE_NORTH_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 14 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_X_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 5 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Y_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 6 };
pub const SENSOR_DATA_TYPE_MAGNETIC_HEADING_Z_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 7 };
pub const SENSOR_DATA_TYPE_MAGNETIC_VARIATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 9 };
pub const SENSOR_DATA_TYPE_MAGNETOMETER_ACCURACY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 22 };
pub const SENSOR_DATA_TYPE_MOTION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5);
pub const SENSOR_DATA_TYPE_MOTION_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 9 };
pub const SENSOR_DATA_TYPE_MULTIVALUE_SWITCH_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 3 };
pub const SENSOR_DATA_TYPE_NMEA_SENTENCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 38 };
pub const SENSOR_DATA_TYPE_ORIENTATION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd);
pub const SENSOR_DATA_TYPE_POSITION_DILUTION_OF_PRECISION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 12 };
pub const SENSOR_DATA_TYPE_POSTALCODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 27 };
pub const SENSOR_DATA_TYPE_QUADRANT_ANGLE_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 15 };
pub const SENSOR_DATA_TYPE_QUATERNION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 17 };
pub const SENSOR_DATA_TYPE_RELATIVE_HUMIDITY_PERCENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 3 };
pub const SENSOR_DATA_TYPE_RESISTANCE_OHMS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 5 };
pub const SENSOR_DATA_TYPE_RFID_TAG_40_BIT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae), pid: 2 };
pub const SENSOR_DATA_TYPE_ROTATION_MATRIX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 16 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 17 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_AZIMUTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 20 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ELEVATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 19 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 39 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_PRNS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 18 };
pub const SENSOR_DATA_TYPE_SATELLITES_IN_VIEW_STN_RATIO: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 21 };
pub const SENSOR_DATA_TYPE_SATELLITES_USED_COUNT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 15 };
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 16 };
pub const SENSOR_DATA_TYPE_SATELLITES_USED_PRNS_AND_CONSTELLATIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 41 };
pub const SENSOR_DATA_TYPE_SCANNER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd7a59a3c_3421_44ab_8d3a_9de8ab6c4cae);
pub const SENSOR_DATA_TYPE_SIMPLE_DEVICE_ORIENTATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 18 };
pub const SENSOR_DATA_TYPE_SPEED_KNOTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 6 };
pub const SENSOR_DATA_TYPE_SPEED_METERS_PER_SECOND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3f8a69a2_07c5_4e48_a965_cd797aab56d5), pid: 8 };
pub const SENSOR_DATA_TYPE_STATE_PROVINCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 26 };
pub const SENSOR_DATA_TYPE_STRAIN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 7 };
pub const SENSOR_DATA_TYPE_TEMPERATURE_CELSIUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 2 };
pub const SENSOR_DATA_TYPE_TILT_X_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 2 };
pub const SENSOR_DATA_TYPE_TILT_Y_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 3 };
pub const SENSOR_DATA_TYPE_TILT_Z_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1637d8a2_4248_4275_865d_558de84aedfd), pid: 4 };
pub const SENSOR_DATA_TYPE_TIMESTAMP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdb5e0cf2_cf1f_4c18_b46c_d86011d62150), pid: 2 };
pub const SENSOR_DATA_TYPE_TOUCH_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2299288a_6d9e_4b0b_b7ec_3528f89e40af), pid: 4 };
pub const SENSOR_DATA_TYPE_TRUE_HEADING_DEGREES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 7 };
pub const SENSOR_DATA_TYPE_VERTICAL_DILUTION_OF_PRECISION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x055c74d8_ca6f_47d6_95c6_1ed3637a0ff4), pid: 14 };
pub const SENSOR_DATA_TYPE_VOLTAGE_VOLTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbbb246d1_e242_4780_a2d3_cded84f35842), pid: 2 };
pub const SENSOR_DATA_TYPE_WEIGHT_KILOGRAMS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38564a7c_f2f2_49bb_9b2b_ba60f66a58df), pid: 8 };
pub const SENSOR_DATA_TYPE_WIND_DIRECTION_DEGREES_ANTICLOCKWISE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 5 };
pub const SENSOR_DATA_TYPE_WIND_SPEED_METERS_PER_SECOND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8b0aa2f1_2d57_42ee_8cc0_4d27622b46c4), pid: 6 };
pub const SENSOR_ERROR_PARAMETER_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x77112bcd_fce1_4f43_b8b8_a88256adb4b3);
pub const SENSOR_EVENT_ACCELEROMETER_SHAKE: windows_core::GUID = windows_core::GUID::from_u128(0x825f5a94_0f48_4396_9ca0_6ecb5c99d915);
pub const SENSOR_EVENT_DATA_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0x2ed0f2a4_0087_41d3_87db_6773370b3c88);
pub const SENSOR_EVENT_PARAMETER_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28);
pub const SENSOR_EVENT_PARAMETER_EVENT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 2 };
pub const SENSOR_EVENT_PARAMETER_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x64346e30_8728_4b34_bdf6_4f52442c5c28), pid: 3 };
pub const SENSOR_EVENT_PROPERTY_CHANGED: windows_core::GUID = windows_core::GUID::from_u128(0x2358f099_84c9_4d3d_90df_c2421e2b2045);
pub const SENSOR_EVENT_STATE_CHANGED: windows_core::GUID = windows_core::GUID::from_u128(0xbfd96016_6bd7_4560_ad34_f2f6607e8f81);
pub const SENSOR_PROPERTY_ACCURACY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 17 };
pub const SENSOR_PROPERTY_CHANGE_SENSITIVITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 14 };
pub const SENSOR_PROPERTY_CLEAR_ASSISTANCE_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 2 };
pub const SENSOR_PROPERTY_COMMON_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920);
pub const SENSOR_PROPERTY_CONNECTION_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 11 };
pub const SENSOR_PROPERTY_CURRENT_REPORT_INTERVAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 13 };
pub const SENSOR_PROPERTY_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 10 };
pub const SENSOR_PROPERTY_DEVICE_PATH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 15 };
pub const SENSOR_PROPERTY_FRIENDLY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 9 };
pub const SENSOR_PROPERTY_HID_USAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 22 };
pub const SENSOR_PROPERTY_LIGHT_RESPONSE_CURVE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 16 };
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SENSOR_PROPERTY_LIST {
    pub AllocatedSizeInBytes: u32,
    pub Count: u32,
    pub List: [super::super::Foundation::PROPERTYKEY; 1],
}
impl Default for SENSOR_PROPERTY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SENSOR_PROPERTY_LIST_HEADER_SIZE: u32 = 8u32;
pub const SENSOR_PROPERTY_LOCATION_DESIRED_ACCURACY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 19 };
pub const SENSOR_PROPERTY_MANUFACTURER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 6 };
pub const SENSOR_PROPERTY_MIN_REPORT_INTERVAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 12 };
pub const SENSOR_PROPERTY_MODEL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 7 };
pub const SENSOR_PROPERTY_PERSISTENT_UNIQUE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 5 };
pub const SENSOR_PROPERTY_RADIO_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 23 };
pub const SENSOR_PROPERTY_RADIO_STATE_PREVIOUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 24 };
pub const SENSOR_PROPERTY_RANGE_MAXIMUM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 21 };
pub const SENSOR_PROPERTY_RANGE_MINIMUM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 20 };
pub const SENSOR_PROPERTY_RESOLUTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 18 };
pub const SENSOR_PROPERTY_SERIAL_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 8 };
pub const SENSOR_PROPERTY_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 3 };
pub const SENSOR_PROPERTY_TEST_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34);
pub const SENSOR_PROPERTY_TURN_ON_OFF_NMEA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe1e962f4_6e65_45f7_9c36_d487b7b1bd34), pid: 3 };
pub const SENSOR_PROPERTY_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f8383ec_d3ec_495c_a8cf_b8bbe85c2920), pid: 2 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SENSOR_STATE(pub i32);
pub const SENSOR_STATE_ACCESS_DENIED: SensorState = SensorState(4i32);
pub const SENSOR_STATE_ERROR: SensorState = SensorState(5i32);
pub const SENSOR_STATE_INITIALIZING: SensorState = SensorState(3i32);
pub const SENSOR_STATE_MAX: SensorState = SensorState(5i32);
pub const SENSOR_STATE_MIN: SensorState = SensorState(0i32);
pub const SENSOR_STATE_NOT_AVAILABLE: SensorState = SensorState(1i32);
pub const SENSOR_STATE_NO_DATA: SensorState = SensorState(2i32);
pub const SENSOR_STATE_READY: SensorState = SensorState(0i32);
pub const SENSOR_TYPE_ACCELEROMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xc04d2387_7340_4cc2_991e_3b18cb8ef2f4);
pub const SENSOR_TYPE_ACCELEROMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0xb2c517a8_f6b5_4ba6_a423_5df560b4cc07);
pub const SENSOR_TYPE_ACCELEROMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0xc2fb0f5f_e2d2_4c78_bcd0_352a9582819d);
pub const SENSOR_TYPE_AGGREGATED_DEVICE_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0xcdb5d8f7_3cfd_41c8_8542_cce622cf5d6e);
pub const SENSOR_TYPE_AGGREGATED_QUADRANT_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x9f81f1af_c4ab_4307_9904_c828bfb90829);
pub const SENSOR_TYPE_AGGREGATED_SIMPLE_DEVICE_ORIENTATION: windows_core::GUID = windows_core::GUID::from_u128(0x86a19291_0482_402c_bf4c_addac52b1c39);
pub const SENSOR_TYPE_AMBIENT_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0x97f115c8_599a_4153_8894_d2d12899918a);
pub const SENSOR_TYPE_BARCODE_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0x990b3d8f_85bb_45ff_914d_998c04f372df);
pub const SENSOR_TYPE_BOOLEAN_SWITCH: windows_core::GUID = windows_core::GUID::from_u128(0x9c7e371f_1041_460b_8d5c_71e4752e350c);
pub const SENSOR_TYPE_BOOLEAN_SWITCH_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0x545c8ba5_b143_4545_868f_ca7fd986b4f6);
pub const SENSOR_TYPE_CAPACITANCE: windows_core::GUID = windows_core::GUID::from_u128(0xca2ffb1c_2317_49c0_a0b4_b63ce63461a0);
pub const SENSOR_TYPE_COMPASS_1D: windows_core::GUID = windows_core::GUID::from_u128(0xa415f6c5_cb50_49d0_8e62_a8270bd7a26c);
pub const SENSOR_TYPE_COMPASS_2D: windows_core::GUID = windows_core::GUID::from_u128(0x15655cc0_997a_4d30_84db_57caba3648bb);
pub const SENSOR_TYPE_COMPASS_3D: windows_core::GUID = windows_core::GUID::from_u128(0x76b5ce0d_17dd_414d_93a1_e127f40bdf6e);
pub const SENSOR_TYPE_CURRENT: windows_core::GUID = windows_core::GUID::from_u128(0x5adc9fce_15a0_4bbe_a1ad_2d38a9ae831c);
pub const SENSOR_TYPE_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0xe83af229_8640_4d18_a213_e22675ebb2c3);
pub const SENSOR_TYPE_DISTANCE_1D: windows_core::GUID = windows_core::GUID::from_u128(0x5f14ab2f_1407_4306_a93f_b1dbabe4f9c0);
pub const SENSOR_TYPE_DISTANCE_2D: windows_core::GUID = windows_core::GUID::from_u128(0x5cf9a46c_a9a2_4e55_b6a1_a04aafa95a92);
pub const SENSOR_TYPE_DISTANCE_3D: windows_core::GUID = windows_core::GUID::from_u128(0xa20cae31_0e25_4772_9fe5_96608a1354b2);
pub const SENSOR_TYPE_ELECTRICAL_POWER: windows_core::GUID = windows_core::GUID::from_u128(0x212f10f5_14ab_4376_9a43_a7794098c2fe);
pub const SENSOR_TYPE_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: windows_core::GUID = windows_core::GUID::from_u128(0x0e903829_ff8a_4a93_97df_3dcbde402288);
pub const SENSOR_TYPE_ENVIRONMENTAL_HUMIDITY: windows_core::GUID = windows_core::GUID::from_u128(0x5c72bf67_bd7e_4257_990b_98a3ba3b400a);
pub const SENSOR_TYPE_ENVIRONMENTAL_TEMPERATURE: windows_core::GUID = windows_core::GUID::from_u128(0x04fd0ec4_d5da_45fa_95a9_5db38ee19306);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_DIRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x9ef57a35_9306_434d_af09_37fa5a9c00bd);
pub const SENSOR_TYPE_ENVIRONMENTAL_WIND_SPEED: windows_core::GUID = windows_core::GUID::from_u128(0xdd50607b_a45f_42cd_8efd_ec61761c4226);
pub const SENSOR_TYPE_FORCE: windows_core::GUID = windows_core::GUID::from_u128(0xc2ab2b02_1a1c_4778_a81b_954a1788cc75);
pub const SENSOR_TYPE_FREQUENCY: windows_core::GUID = windows_core::GUID::from_u128(0x8cd2cbb6_73e6_4640_a709_72ae8fb60d7f);
pub const SENSOR_TYPE_GYROMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xfa088734_f552_4584_8324_edfaf649652c);
pub const SENSOR_TYPE_GYROMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0x31ef4f83_919b_48bf_8de0_5d7a9d240556);
pub const SENSOR_TYPE_GYROMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0x09485f5a_759e_42c2_bd4b_a349b75c8643);
pub const SENSOR_TYPE_HUMAN_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0xc138c12b_ad52_451c_9375_87f518ff10c6);
pub const SENSOR_TYPE_HUMAN_PROXIMITY: windows_core::GUID = windows_core::GUID::from_u128(0x5220dae9_3179_4430_9f90_06266d2a34de);
pub const SENSOR_TYPE_INCLINOMETER_1D: windows_core::GUID = windows_core::GUID::from_u128(0xb96f98c5_7a75_4ba7_94e9_ac868c966dd8);
pub const SENSOR_TYPE_INCLINOMETER_2D: windows_core::GUID = windows_core::GUID::from_u128(0xab140f6d_83eb_4264_b70b_b16a5b256a01);
pub const SENSOR_TYPE_INCLINOMETER_3D: windows_core::GUID = windows_core::GUID::from_u128(0xb84919fb_ea85_4976_8444_6f6f5c6d31db);
pub const SENSOR_TYPE_INDUCTANCE: windows_core::GUID = windows_core::GUID::from_u128(0xdc1d933f_c435_4c7d_a2fe_607192a524d3);
pub const SENSOR_TYPE_LOCATION_BROADCAST: windows_core::GUID = windows_core::GUID::from_u128(0xd26988cf_5162_4039_bb17_4c58b698e44a);
pub const SENSOR_TYPE_LOCATION_DEAD_RECKONING: windows_core::GUID = windows_core::GUID::from_u128(0x1a37d538_f28b_42da_9fce_a9d0a2a6d829);
pub const SENSOR_TYPE_LOCATION_GPS: windows_core::GUID = windows_core::GUID::from_u128(0xed4ca589_327a_4ff9_a560_91da4b48275e);
pub const SENSOR_TYPE_LOCATION_LOOKUP: windows_core::GUID = windows_core::GUID::from_u128(0x3b2eae4a_72ce_436d_96d2_3c5b8570e987);
pub const SENSOR_TYPE_LOCATION_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x9b2d0566_0368_4f71_b88d_533f132031de);
pub const SENSOR_TYPE_LOCATION_STATIC: windows_core::GUID = windows_core::GUID::from_u128(0x095f8184_0fa9_4445_8e6e_b70f320b6b4c);
pub const SENSOR_TYPE_LOCATION_TRIANGULATION: windows_core::GUID = windows_core::GUID::from_u128(0x691c341a_5406_4fe1_942f_2246cbeb39e0);
pub const SENSOR_TYPE_MOTION_DETECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x5c7c1a12_30a5_43b9_a4b2_cf09ec5b7be8);
pub const SENSOR_TYPE_MULTIVALUE_SWITCH: windows_core::GUID = windows_core::GUID::from_u128(0xb3ee4d76_37a4_4402_b25e_99c60a775fa1);
pub const SENSOR_TYPE_POTENTIOMETER: windows_core::GUID = windows_core::GUID::from_u128(0x2b3681a9_cadc_45aa_a6ff_54957c8bb440);
pub const SENSOR_TYPE_PRESSURE: windows_core::GUID = windows_core::GUID::from_u128(0x26d31f34_6352_41cf_b793_ea0713d53d77);
pub const SENSOR_TYPE_RESISTANCE: windows_core::GUID = windows_core::GUID::from_u128(0x9993d2c8_c157_4a52_a7b5_195c76037231);
pub const SENSOR_TYPE_RFID_SCANNER: windows_core::GUID = windows_core::GUID::from_u128(0x44328ef5_02dd_4e8d_ad5d_9249832b2eca);
pub const SENSOR_TYPE_SCALE: windows_core::GUID = windows_core::GUID::from_u128(0xc06dd92c_7feb_438e_9bf6_82207fff5bb8);
pub const SENSOR_TYPE_SPEEDOMETER: windows_core::GUID = windows_core::GUID::from_u128(0x6bd73c1f_0bb4_4310_81b2_dfc18a52bf94);
pub const SENSOR_TYPE_STRAIN: windows_core::GUID = windows_core::GUID::from_u128(0xc6d1ec0e_6803_4361_ad3d_85bcc58c6d29);
pub const SENSOR_TYPE_TOUCH: windows_core::GUID = windows_core::GUID::from_u128(0x17db3018_06c4_4f7d_81af_9274b7599c27);
pub const SENSOR_TYPE_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x10ba83e3_ef4f_41ed_9885_a87d6435a8e1);
pub const SENSOR_TYPE_VOLTAGE: windows_core::GUID = windows_core::GUID::from_u128(0xc5484637_4fb7_4953_98b8_a56d8aa1fb1e);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct SENSOR_VALUE_PAIR {
    pub Key: super::super::Foundation::PROPERTYKEY,
    pub Value: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for SENSOR_VALUE_PAIR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SENSOR_VALUE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIMPLE_DEVICE_ORIENTATION(pub i32);
pub const SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED: SimpleDeviceOrientation = SimpleDeviceOrientation(0i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_180: SimpleDeviceOrientation = SimpleDeviceOrientation(2i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_270: SimpleDeviceOrientation = SimpleDeviceOrientation(3i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_90: SimpleDeviceOrientation = SimpleDeviceOrientation(1i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN: SimpleDeviceOrientation = SimpleDeviceOrientation(5i32);
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP: SimpleDeviceOrientation = SimpleDeviceOrientation(4i32);
pub const Sensor: windows_core::GUID = windows_core::GUID::from_u128(0xe97ced00_523a_4133_bf6f_d3a2dae7f6ba);
pub const SensorCollection: windows_core::GUID = windows_core::GUID::from_u128(0x79c43adb_a429_469f_aa39_2f2b74b75937);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SensorConnectionType(pub i32);
pub const SensorConnectionType_Attached: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(1i32);
pub const SensorConnectionType_External: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(2i32);
pub const SensorConnectionType_Integrated: SENSOR_CONNECTION_TYPES = SENSOR_CONNECTION_TYPES(0i32);
pub const SensorDataReport: windows_core::GUID = windows_core::GUID::from_u128(0x4ea9d6ef_694b_4218_8816_ccda8da74bba);
pub const SensorManager: windows_core::GUID = windows_core::GUID::from_u128(0x77a1c827_fcd2_4689_8915_9d613cc5fa3e);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SensorState(pub i32);
pub const SensorState_Active: SENSOR_STATE = SENSOR_STATE(2i32);
pub const SensorState_Error: SENSOR_STATE = SENSOR_STATE(3i32);
pub const SensorState_Idle: SENSOR_STATE = SENSOR_STATE(1i32);
pub const SensorState_Initializing: SENSOR_STATE = SENSOR_STATE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SimpleDeviceOrientation(pub i32);
pub const SimpleDeviceOrientation_Facedown: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(5i32);
pub const SimpleDeviceOrientation_Faceup: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(4i32);
pub const SimpleDeviceOrientation_NotRotated: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(0i32);
pub const SimpleDeviceOrientation_Rotated180DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(2i32);
pub const SimpleDeviceOrientation_Rotated270DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(3i32);
pub const SimpleDeviceOrientation_Rotated90DegreesCounterclockwise: SIMPLE_DEVICE_ORIENTATION = SIMPLE_DEVICE_ORIENTATION(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VEC3D {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
