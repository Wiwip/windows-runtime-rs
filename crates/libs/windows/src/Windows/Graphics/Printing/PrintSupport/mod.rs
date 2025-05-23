windows_core::imp::define_interface!(IPrintSupportCommunicationErrorDetectedEventArgs, IPrintSupportCommunicationErrorDetectedEventArgs_Vtbl, 0x9c90151e_ad1b_5081_a491_4a2d94244f2d);
impl windows_core::RuntimeType for IPrintSupportCommunicationErrorDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportCommunicationErrorDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppCommunicationErrorKind) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub CommunicationConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionSession, IPrintSupportExtensionSession_Vtbl, 0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
impl windows_core::RuntimeType for IPrintSupportExtensionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub PrintTicketValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrintTicketValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PrintDeviceCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrintDeviceCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionSession2, IPrintSupportExtensionSession2_Vtbl, 0x10fa8c11_6de8_5765_8fcf_e716e0f27ed1);
impl windows_core::RuntimeType for IPrintSupportExtensionSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrinterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrinterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionSession3, IPrintSupportExtensionSession3_Vtbl, 0x0d1b755d_1273_5e14_81d3_b6bb582b9ed8);
impl windows_core::RuntimeType for IPrintSupportExtensionSession3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommunicationErrorDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCommunicationErrorDetected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportExtensionTriggerDetails, IPrintSupportExtensionTriggerDetails_Vtbl, 0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
impl windows_core::RuntimeType for IPrintSupportExtensionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportIppCommunicationConfiguration, IPrintSupportIppCommunicationConfiguration_Vtbl, 0xdbc36e0b_2d90_53b9_90d2_93faf30dafdd);
impl windows_core::RuntimeType for IPrintSupportIppCommunicationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportIppCommunicationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommunicationKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IppPrinterCommunicationKind) -> windows_core::HRESULT,
    pub CanModifyTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IppAttributeTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IppJobTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportIppCommunicationTimeouts, IPrintSupportIppCommunicationTimeouts_Vtbl, 0xa3b2de71_564c_5806_a1a9_c6043ca5d373);
impl windows_core::RuntimeType for IPrintSupportIppCommunicationTimeouts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportIppCommunicationTimeouts_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SendTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetSendTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ReceiveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetReceiveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportMxdcImageQualityConfiguration, IPrintSupportMxdcImageQualityConfiguration_Vtbl, 0x0e0d0b86_d202_58a3_a1ed_2ef9dbc0f291);
impl windows_core::RuntimeType for IPrintSupportMxdcImageQualityConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportMxdcImageQualityConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NormalOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetNormalOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub DraftOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetDraftOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub HighOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetHighOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub PhotographicOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetPhotographicOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub TextOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetTextOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub AutomaticOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetAutomaticOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
    pub FaxOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XpsImageQuality) -> windows_core::HRESULT,
    pub SetFaxOutputQuality: unsafe extern "system" fn(*mut core::ffi::c_void, XpsImageQuality) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl, 0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceCapabilities: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceCapabilities: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl, 0x469df9e7_fd07_5eeb_a07d_9fcc67f089ba);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSupportedPdlPassthroughContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceResources: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceResources: usize,
    pub SetPrintDeviceCapabilitiesUpdatePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_Vtbl, 0xd4e9b3fc_8094_5cb6_a343_ce7a97187b45);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommunicationConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_Vtbl, 0x31734ad5_9bfb_5bfb_bdef_8476258e3390);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MxdcImageQualityConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl, 0x5f5fc025_8c35_5529_8038_8cdc3634bbcd);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl, 0x3d9e1a70_7c39_551f_aa1f_f8ca35b3119e);
impl windows_core::RuntimeType for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePeriodicRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePrintJobRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintTicketElement, IPrintSupportPrintTicketElement_Vtbl, 0x4b2a4489_730d_5be7_80e6_8332941abf13);
impl windows_core::RuntimeType for IPrintSupportPrintTicketElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrintTicketValidationRequestedEventArgs, IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl, 0x338e4e69_db55_55c7_8338_ef64680a8f90);
impl windows_core::RuntimeType for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    pub SetPrintTicketValidationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, WorkflowPrintTicketValidationStatus) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportPrinterSelectedEventArgs, IPrintSupportPrinterSelectedEventArgs_Vtbl, 0x7b1cb7d9_a8a4_5c09_adb2_66165f817977);
impl windows_core::RuntimeType for IPrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrinterSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SetPrintTicket: usize,
    pub SetAdditionalFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAdditionalParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowedAdditionalFeaturesAndParametersCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetAdaptiveCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetAdaptiveCard: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportSessionInfo, IPrintSupportSessionInfo_Vtbl, 0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
impl windows_core::RuntimeType for IPrintSupportSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
}
windows_core::imp::define_interface!(IPrintSupportSettingsActivatedEventArgs, IPrintSupportSettingsActivatedEventArgs_Vtbl, 0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
impl windows_core::RuntimeType for IPrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintSupportSettingsActivatedEventArgs2, IPrintSupportSettingsActivatedEventArgs2_Vtbl, 0xabe45f6e_dc9d_5403_8107_c864d9276367);
impl windows_core::RuntimeType for IPrintSupportSettingsActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub OwnerWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::WindowId) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    OwnerWindowId: usize,
}
windows_core::imp::define_interface!(IPrintSupportSettingsUISession, IPrintSupportSettingsUISession_Vtbl, 0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
impl windows_core::RuntimeType for IPrintSupportSettingsUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SessionPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SessionPrintTicket: usize,
    pub DocumentTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SettingsLaunchKind) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub UpdatePrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    UpdatePrintTicket: usize,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppCommunicationErrorKind(pub i32);
impl IppCommunicationErrorKind {
    pub const Other: Self = Self(0i32);
    pub const Timeout: Self = Self(1i32);
    pub const ConnectionError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl windows_core::TypeKind for IppCommunicationErrorKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppCommunicationErrorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.IppCommunicationErrorKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IppPrinterCommunicationKind(pub i32);
impl IppPrinterCommunicationKind {
    pub const Network: Self = Self(0i32);
    pub const Usb: Self = Self(1i32);
    pub const PrinterConnection: Self = Self(2i32);
    pub const UniversalPrint: Self = Self(3i32);
    pub const VirtualPrinter: Self = Self(4i32);
}
impl windows_core::TypeKind for IppPrinterCommunicationKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IppPrinterCommunicationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.IppPrinterCommunicationKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportCommunicationErrorDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportCommunicationErrorDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportCommunicationErrorDetectedEventArgs {
    pub fn ErrorKind(&self) -> windows_core::Result<IppCommunicationErrorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CommunicationConfiguration(&self) -> windows_core::Result<PrintSupportIppCommunicationConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommunicationConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportCommunicationErrorDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportCommunicationErrorDetectedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportCommunicationErrorDetectedEventArgs {
    type Vtable = <IPrintSupportCommunicationErrorDetectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportCommunicationErrorDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportCommunicationErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs";
}
unsafe impl Send for PrintSupportCommunicationErrorDetectedEventArgs {}
unsafe impl Sync for PrintSupportCommunicationErrorDetectedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportExtensionSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportExtensionSession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportExtensionSession {
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrintTicketValidationRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicketValidationRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintTicketValidationRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintTicketValidationRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrintDeviceCapabilitiesChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintDeviceCapabilitiesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrintDeviceCapabilitiesChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrintDeviceCapabilitiesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PrinterSelected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrinterSelectedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrinterSelected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrinterSelected(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrinterSelected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CommunicationErrorDetected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportCommunicationErrorDetectedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommunicationErrorDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCommunicationErrorDetected(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrintSupportExtensionSession3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCommunicationErrorDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportExtensionSession>();
}
unsafe impl windows_core::Interface for PrintSupportExtensionSession {
    type Vtable = <IPrintSupportExtensionSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportExtensionSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
unsafe impl Send for PrintSupportExtensionSession {}
unsafe impl Sync for PrintSupportExtensionSession {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportExtensionTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportExtensionTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportExtensionTriggerDetails {
    pub fn Session(&self) -> windows_core::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportExtensionTriggerDetails>();
}
unsafe impl windows_core::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = <IPrintSupportExtensionTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportExtensionTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
unsafe impl Send for PrintSupportExtensionTriggerDetails {}
unsafe impl Sync for PrintSupportExtensionTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportIppCommunicationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportIppCommunicationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportIppCommunicationConfiguration {
    pub fn CommunicationKind(&self) -> windows_core::Result<IppPrinterCommunicationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommunicationKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanModifyTimeouts(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanModifyTimeouts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IppAttributeTimeouts(&self) -> windows_core::Result<PrintSupportIppCommunicationTimeouts> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IppAttributeTimeouts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IppJobTimeouts(&self) -> windows_core::Result<PrintSupportIppCommunicationTimeouts> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IppJobTimeouts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportIppCommunicationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportIppCommunicationConfiguration>();
}
unsafe impl windows_core::Interface for PrintSupportIppCommunicationConfiguration {
    type Vtable = <IPrintSupportIppCommunicationConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportIppCommunicationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportIppCommunicationConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration";
}
unsafe impl Send for PrintSupportIppCommunicationConfiguration {}
unsafe impl Sync for PrintSupportIppCommunicationConfiguration {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportIppCommunicationTimeouts(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportIppCommunicationTimeouts, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportIppCommunicationTimeouts {
    pub fn ConnectTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConnectTimeout(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConnectTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SendTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSendTimeout(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSendTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReceiveTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceiveTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReceiveTimeout(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReceiveTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportIppCommunicationTimeouts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportIppCommunicationTimeouts>();
}
unsafe impl windows_core::Interface for PrintSupportIppCommunicationTimeouts {
    type Vtable = <IPrintSupportIppCommunicationTimeouts as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportIppCommunicationTimeouts as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportIppCommunicationTimeouts {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts";
}
unsafe impl Send for PrintSupportIppCommunicationTimeouts {}
unsafe impl Sync for PrintSupportIppCommunicationTimeouts {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportMxdcImageQualityConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportMxdcImageQualityConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportMxdcImageQualityConfiguration {
    pub fn NormalOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DraftOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DraftOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDraftOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDraftOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHighOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHighOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotographicOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotographicOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPhotographicOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhotographicOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TextOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTextOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutomaticOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutomaticOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FaxOutputQuality(&self) -> windows_core::Result<XpsImageQuality> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FaxOutputQuality)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFaxOutputQuality(&self, value: XpsImageQuality) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFaxOutputQuality)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportMxdcImageQualityConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportMxdcImageQualityConfiguration>();
}
unsafe impl windows_core::Interface for PrintSupportMxdcImageQualityConfiguration {
    type Vtable = <IPrintSupportMxdcImageQualityConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportMxdcImageQualityConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportMxdcImageQualityConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration";
}
unsafe impl Send for PrintSupportMxdcImageQualityConfiguration {}
unsafe impl Sync for PrintSupportMxdcImageQualityConfiguration {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceCapabilities(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPrintDeviceCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities<P0>(&self, updatedpdc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintDeviceCapabilities)(windows_core::Interface::as_raw(this), updatedpdc.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSupportedPdlPassthroughContentTypes<P0>(&self, supportedpdlcontenttypes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSupportedPdlPassthroughContentTypes)(windows_core::Interface::as_raw(this), supportedpdlcontenttypes.param().abi()).ok() }
    }
    pub fn ResourceLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceResources(&self) -> windows_core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPrintDeviceResources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceResources<P0>(&self, updatedpdr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Data::Xml::Dom::XmlDocument>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintDeviceResources)(windows_core::Interface::as_raw(this), updatedpdr.param().abi()).ok() }
    }
    pub fn SetPrintDeviceCapabilitiesUpdatePolicy<P0>(&self, updatepolicy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PrintSupportPrintDeviceCapabilitiesUpdatePolicy>,
    {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrintDeviceCapabilitiesUpdatePolicy)(windows_core::Interface::as_raw(this), updatepolicy.param().abi()).ok() }
    }
    pub fn CommunicationConfiguration(&self) -> windows_core::Result<PrintSupportIppCommunicationConfiguration> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommunicationConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MxdcImageQualityConfiguration(&self) -> windows_core::Result<PrintSupportMxdcImageQualityConfiguration> {
        let this = &windows_core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MxdcImageQualityConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
unsafe impl Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportPrintDeviceCapabilitiesUpdatePolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesUpdatePolicy, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    pub fn CreatePeriodicRefresh(updateperiod: super::super::super::Foundation::TimeSpan) -> windows_core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeriodicRefresh)(windows_core::Interface::as_raw(this), updateperiod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePrintJobRefresh(numberofjobs: u32) -> windows_core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePrintJobRefresh)(windows_core::Interface::as_raw(this), numberofjobs, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics<R, F: FnOnce(&IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintDeviceCapabilitiesUpdatePolicy>();
}
unsafe impl windows_core::Interface for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = <IPrintSupportPrintDeviceCapabilitiesUpdatePolicy as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportPrintDeviceCapabilitiesUpdatePolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
}
unsafe impl Send for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
unsafe impl Sync for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportPrintTicketElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintTicketElement, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintTicketElement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PrintSupportPrintTicketElement, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLocalName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNamespaceUri(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNamespaceUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintTicketElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintTicketElement>();
}
unsafe impl windows_core::Interface for PrintSupportPrintTicketElement {
    type Vtable = <IPrintSupportPrintTicketElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportPrintTicketElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintTicketElement {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
}
unsafe impl Send for PrintSupportPrintTicketElement {}
unsafe impl Sync for PrintSupportPrintTicketElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrintTicketValidationRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrintTicketValidationRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintTicketValidationStatus)(windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrintTicketValidationRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = <IPrintSupportPrintTicketValidationRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportPrintTicketValidationRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
unsafe impl Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportPrinterSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportPrinterSelectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportPrinterSelectedEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SetPrintTicket<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintTicket)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetAdditionalFeatures<P0>(&self, features: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdditionalFeatures)(windows_core::Interface::as_raw(this), features.param().abi()).ok() }
    }
    pub fn SetAdditionalParameters<P0>(&self, parameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdditionalParameters)(windows_core::Interface::as_raw(this), parameters.param().abi()).ok() }
    }
    pub fn AllowedAdditionalFeaturesAndParametersCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedAdditionalFeaturesAndParametersCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Shell")]
    pub fn SetAdaptiveCard<P0>(&self, adaptivecard: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::UI::Shell::IAdaptiveCard>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdaptiveCard)(windows_core::Interface::as_raw(this), adaptivecard.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportPrinterSelectedEventArgs>();
}
unsafe impl windows_core::Interface for PrintSupportPrinterSelectedEventArgs {
    type Vtable = <IPrintSupportPrinterSelectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportPrinterSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportPrinterSelectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
}
unsafe impl Send for PrintSupportPrinterSelectedEventArgs {}
unsafe impl Sync for PrintSupportPrinterSelectedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportSessionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportSessionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportSessionInfo {
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Printer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSessionInfo>();
}
unsafe impl windows_core::Interface for PrintSupportSessionInfo {
    type Vtable = <IPrintSupportSessionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportSessionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
unsafe impl Send for PrintSupportSessionInfo {}
unsafe impl Sync for PrintSupportSessionInfo {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportSettingsActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(PrintSupportSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(PrintSupportSettingsActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgs, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
#[cfg(feature = "ApplicationModel_Activation")]
impl PrintSupportSettingsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Session(&self) -> windows_core::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI")]
    pub fn OwnerWindowId(&self) -> windows_core::Result<super::super::super::UI::WindowId> {
        let this = &windows_core::Interface::cast::<IPrintSupportSettingsActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerWindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSettingsActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = <IPrintSupportSettingsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for PrintSupportSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for PrintSupportSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintSupportSettingsUISession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintSupportSettingsUISession, windows_core::IUnknown, windows_core::IInspectable);
impl PrintSupportSettingsUISession {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SessionPrintTicket(&self) -> windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionPrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentTitle)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LaunchKind(&self) -> windows_core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket<P0>(&self, printticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdatePrintTicket)(windows_core::Interface::as_raw(this), printticket.param().abi()).ok() }
    }
    pub fn SessionInfo(&self) -> windows_core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintSupportSettingsUISession>();
}
unsafe impl windows_core::Interface for PrintSupportSettingsUISession {
    type Vtable = <IPrintSupportSettingsUISession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintSupportSettingsUISession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
unsafe impl Send for PrintSupportSettingsUISession {}
unsafe impl Sync for PrintSupportSettingsUISession {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl windows_core::TypeKind for SettingsLaunchKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl windows_core::TypeKind for WorkflowPrintTicketValidationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XpsImageQuality(pub i32);
impl XpsImageQuality {
    pub const JpegHighCompression: Self = Self(0i32);
    pub const JpegMediumCompression: Self = Self(1i32);
    pub const JpegLowCompression: Self = Self(2i32);
    pub const Png: Self = Self(3i32);
}
impl windows_core::TypeKind for XpsImageQuality {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for XpsImageQuality {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.XpsImageQuality;i4)");
}
