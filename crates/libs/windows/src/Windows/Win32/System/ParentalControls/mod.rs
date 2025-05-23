pub const ARRAY_SEP_CHAR: u32 = 9u32;
pub const FACILITY_WPC: u32 = 2457u32;
windows_core::imp::define_interface!(IWPCGamesSettings, IWPCGamesSettings_Vtbl, 0x95e87780_e158_489e_b452_bbb850790715);
impl core::ops::Deref for IWPCGamesSettings {
    type Target = IWPCSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWPCGamesSettings, windows_core::IUnknown, IWPCSettings);
impl IWPCGamesSettings {
    pub unsafe fn IsBlocked(&self, guidappid: windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBlocked)(windows_core::Interface::as_raw(self), core::mem::transmute(guidappid), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCGamesSettings_Vtbl {
    pub base__: IWPCSettings_Vtbl,
    pub IsBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
pub trait IWPCGamesSettings_Impl: IWPCSettings_Impl {
    fn IsBlocked(&self, guidappid: &windows_core::GUID) -> windows_core::Result<u32>;
}
impl IWPCGamesSettings_Vtbl {
    pub const fn new<Identity: IWPCGamesSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsBlocked<Identity: IWPCGamesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidappid: windows_core::GUID, pdwreasons: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCGamesSettings_Impl::IsBlocked(this, core::mem::transmute(&guidappid)) {
                    Ok(ok__) => {
                        pdwreasons.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWPCSettings_Vtbl::new::<Identity, OFFSET>(), IsBlocked: IsBlocked::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCGamesSettings as windows_core::Interface>::IID || iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCGamesSettings {}
windows_core::imp::define_interface!(IWPCProviderConfig, IWPCProviderConfig_Vtbl, 0xbef54196_2d02_4a26_b6e5_d65af295d0f1);
windows_core::imp::interface_hierarchy!(IWPCProviderConfig, windows_core::IUnknown);
impl IWPCProviderConfig {
    pub unsafe fn GetUserSummary(&self, bstrsid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSummary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Configure(&self, hwnd: Option<super::super::Foundation::HWND>, bstrsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute_copy(bstrsid)).ok() }
    }
    pub unsafe fn RequestOverride(&self, hwnd: Option<super::super::Foundation::HWND>, bstrpath: &windows_core::BSTR, dwflags: WPCFLAG_RESTRICTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestOverride)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute_copy(bstrpath), dwflags.0 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserSummary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWPCProviderConfig_Impl: windows_core::IUnknownImpl {
    fn GetUserSummary(&self, bstrsid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn Configure(&self, hwnd: super::super::Foundation::HWND, bstrsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestOverride(&self, hwnd: super::super::Foundation::HWND, bstrpath: &windows_core::BSTR, dwflags: &WPCFLAG_RESTRICTION) -> windows_core::Result<()>;
}
impl IWPCProviderConfig_Vtbl {
    pub const fn new<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUserSummary<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsid: *mut core::ffi::c_void, pbstrusersummary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCProviderConfig_Impl::GetUserSummary(this, core::mem::transmute(&bstrsid)) {
                    Ok(ok__) => {
                        pbstrusersummary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Configure<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderConfig_Impl::Configure(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&bstrsid)).into()
            }
        }
        unsafe extern "system" fn RequestOverride<Identity: IWPCProviderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderConfig_Impl::RequestOverride(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&bstrpath), core::mem::transmute(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserSummary: GetUserSummary::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
            RequestOverride: RequestOverride::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCProviderConfig {}
windows_core::imp::define_interface!(IWPCProviderState, IWPCProviderState_Vtbl, 0x50b6a267_c4bd_450b_adb5_759073837c9e);
windows_core::imp::interface_hierarchy!(IWPCProviderState, windows_core::IUnknown);
impl IWPCProviderState {
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWPCProviderState_Impl: windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
}
impl IWPCProviderState_Vtbl {
    pub const fn new<Identity: IWPCProviderState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enable<Identity: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderState_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn Disable<Identity: IWPCProviderState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWPCProviderState_Impl::Disable(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Enable: Enable::<Identity, OFFSET>, Disable: Disable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderState as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCProviderState {}
windows_core::imp::define_interface!(IWPCProviderSupport, IWPCProviderSupport_Vtbl, 0x41eba572_23ed_4779_bec1_8df96206c44c);
windows_core::imp::interface_hierarchy!(IWPCProviderSupport, windows_core::IUnknown);
impl IWPCProviderSupport {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCProviderSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IWPCProviderSupport_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IWPCProviderSupport_Vtbl {
    pub const fn new<Identity: IWPCProviderSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IWPCProviderSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidprovider: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCProviderSupport_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        pguidprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrent: GetCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCProviderSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCProviderSupport {}
windows_core::imp::define_interface!(IWPCSettings, IWPCSettings_Vtbl, 0x8fdf6ca1_0189_47e4_b670_1a8a4636e340);
windows_core::imp::interface_hierarchy!(IWPCSettings, windows_core::IUnknown);
impl IWPCSettings {
    pub unsafe fn IsLoggingRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoggingRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLastSettingsChangeTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastSettingsChangeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRestrictions(&self) -> windows_core::Result<WPCFLAG_RESTRICTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsLoggingRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLastSettingsChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WPCFLAG_RESTRICTION) -> windows_core::HRESULT,
}
pub trait IWPCSettings_Impl: windows_core::IUnknownImpl {
    fn IsLoggingRequired(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetLastSettingsChangeTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetRestrictions(&self) -> windows_core::Result<WPCFLAG_RESTRICTION>;
}
impl IWPCSettings_Vtbl {
    pub const fn new<Identity: IWPCSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsLoggingRequired<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrequired: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::IsLoggingRequired(this) {
                    Ok(ok__) => {
                        pfrequired.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::GetLastSettingsChangeTime(this) {
                    Ok(ok__) => {
                        ptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: IWPCSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCSettings_Impl::GetRestrictions(this) {
                    Ok(ok__) => {
                        pdwrestrictions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsLoggingRequired: IsLoggingRequired::<Identity, OFFSET>,
            GetLastSettingsChangeTime: GetLastSettingsChangeTime::<Identity, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCSettings {}
windows_core::imp::define_interface!(IWPCWebSettings, IWPCWebSettings_Vtbl, 0xffccbdb8_0992_4c30_b0f1_1cbb09c240aa);
impl core::ops::Deref for IWPCWebSettings {
    type Target = IWPCSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWPCWebSettings, windows_core::IUnknown, IWPCSettings);
impl IWPCWebSettings {
    pub unsafe fn GetSettings(&self) -> windows_core::Result<WPCFLAG_WEB_SETTING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestURLOverride<P1>(&self, hwnd: Option<super::super::Foundation::HWND>, pcszurl: P1, ppcszsuburls: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestURLOverride)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, pcszurl.param().abi(), ppcszsuburls.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppcszsuburls.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWPCWebSettings_Vtbl {
    pub base__: IWPCSettings_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WPCFLAG_WEB_SETTING) -> windows_core::HRESULT,
    pub RequestURLOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWPCWebSettings_Impl: IWPCSettings_Impl {
    fn GetSettings(&self) -> windows_core::Result<WPCFLAG_WEB_SETTING>;
    fn RequestURLOverride(&self, hwnd: super::super::Foundation::HWND, pcszurl: &windows_core::PCWSTR, curls: u32, ppcszsuburls: *const windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IWPCWebSettings_Vtbl {
    pub const fn new<Identity: IWPCWebSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSettings<Identity: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCWebSettings_Impl::GetSettings(this) {
                    Ok(ok__) => {
                        pdwsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestURLOverride<Identity: IWPCWebSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: windows_core::PCWSTR, curls: u32, ppcszsuburls: *const windows_core::PCWSTR, pfchanged: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWPCWebSettings_Impl::RequestURLOverride(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&pcszurl), core::mem::transmute_copy(&curls), core::mem::transmute_copy(&ppcszsuburls)) {
                    Ok(ok__) => {
                        pfchanged.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWPCSettings_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            RequestURLOverride: RequestURLOverride::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWPCWebSettings as windows_core::Interface>::IID || iid == &<IWPCSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWPCWebSettings {}
windows_core::imp::define_interface!(IWindowsParentalControls, IWindowsParentalControls_Vtbl, 0x28b4d88b_e072_49e6_804d_26edbe21a7b9);
impl core::ops::Deref for IWindowsParentalControls {
    type Target = IWindowsParentalControlsCore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsParentalControls, windows_core::IUnknown, IWindowsParentalControlsCore);
impl IWindowsParentalControls {
    pub unsafe fn GetGamesSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCGamesSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGamesSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControls_Vtbl {
    pub base__: IWindowsParentalControlsCore_Vtbl,
    pub GetGamesSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWindowsParentalControls_Impl: IWindowsParentalControlsCore_Impl {
    fn GetGamesSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCGamesSettings>;
}
impl IWindowsParentalControls_Vtbl {
    pub const fn new<Identity: IWindowsParentalControls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGamesSettings<Identity: IWindowsParentalControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControls_Impl::GetGamesSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWindowsParentalControlsCore_Vtbl::new::<Identity, OFFSET>(), GetGamesSettings: GetGamesSettings::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControls as windows_core::Interface>::IID || iid == &<IWindowsParentalControlsCore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowsParentalControls {}
windows_core::imp::define_interface!(IWindowsParentalControlsCore, IWindowsParentalControlsCore_Vtbl, 0x4ff40a0f_3f3b_4d7c_a41b_4f39d7b44d05);
windows_core::imp::interface_hierarchy!(IWindowsParentalControlsCore, windows_core::IUnknown);
impl IWindowsParentalControlsCore {
    pub unsafe fn GetVisibility(&self) -> windows_core::Result<WPCFLAG_VISIBILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibility)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUserSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWebSettings<P0>(&self, pcszsid: P0) -> windows_core::Result<IWPCWebSettings>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWebSettings)(windows_core::Interface::as_raw(self), pcszsid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWebFilterInfo(&self, pguidid: *mut windows_core::GUID, ppszname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWebFilterInfo)(windows_core::Interface::as_raw(self), pguidid as _, ppszname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsParentalControlsCore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WPCFLAG_VISIBILITY) -> windows_core::HRESULT,
    pub GetUserSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWebSettings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWebFilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IWindowsParentalControlsCore_Impl: windows_core::IUnknownImpl {
    fn GetVisibility(&self) -> windows_core::Result<WPCFLAG_VISIBILITY>;
    fn GetUserSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCSettings>;
    fn GetWebSettings(&self, pcszsid: &windows_core::PCWSTR) -> windows_core::Result<IWPCWebSettings>;
    fn GetWebFilterInfo(&self, pguidid: *mut windows_core::GUID, ppszname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IWindowsParentalControlsCore_Vtbl {
    pub const fn new<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVisibility<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetVisibility(this) {
                    Ok(ok__) => {
                        pevisibility.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserSettings<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetUserSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWebSettings<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszsid: windows_core::PCWSTR, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsParentalControlsCore_Impl::GetWebSettings(this, core::mem::transmute(&pcszsid)) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWebFilterInfo<Identity: IWindowsParentalControlsCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidid: *mut windows_core::GUID, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsParentalControlsCore_Impl::GetWebFilterInfo(this, core::mem::transmute_copy(&pguidid), core::mem::transmute_copy(&ppszname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibility: GetVisibility::<Identity, OFFSET>,
            GetUserSettings: GetUserSettings::<Identity, OFFSET>,
            GetWebSettings: GetWebSettings::<Identity, OFFSET>,
            GetWebFilterInfo: GetWebFilterInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsParentalControlsCore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowsParentalControlsCore {}
pub const MSG_Event_AppBlocked: i32 = -1342177264i32;
pub const MSG_Event_AppOverride: i32 = -1342177263i32;
pub const MSG_Event_Application: i32 = -1342177260i32;
pub const MSG_Event_ComputerUsage: i32 = -1342177259i32;
pub const MSG_Event_ContentUsage: i32 = -1342177258i32;
pub const MSG_Event_Custom: i32 = -1342177267i32;
pub const MSG_Event_EmailContact: i32 = -1342177266i32;
pub const MSG_Event_EmailReceived: i32 = -1342177276i32;
pub const MSG_Event_EmailSent: i32 = -1342177275i32;
pub const MSG_Event_FileDownload: i32 = -1342177270i32;
pub const MSG_Event_GameStart: i32 = -1342177278i32;
pub const MSG_Event_IMContact: i32 = -1342177265i32;
pub const MSG_Event_IMFeature: i32 = -1342177269i32;
pub const MSG_Event_IMInvitation: i32 = -1342177273i32;
pub const MSG_Event_IMJoin: i32 = -1342177272i32;
pub const MSG_Event_IMLeave: i32 = -1342177271i32;
pub const MSG_Event_MediaPlayback: i32 = -1342177274i32;
pub const MSG_Event_SettingChange: i32 = -1342177279i32;
pub const MSG_Event_UrlVisit: i32 = -1342177277i32;
pub const MSG_Event_WebOverride: i32 = -1342177262i32;
pub const MSG_Event_WebsiteVisit: i32 = -1342177261i32;
pub const MSG_Keyword_ThirdParty: i32 = 268435462i32;
pub const MSG_Keyword_WPC: i32 = 268435461i32;
pub const MSG_Opcode_Launch: i32 = 805306390i32;
pub const MSG_Opcode_Locate: i32 = 805306388i32;
pub const MSG_Opcode_Modify: i32 = 805306389i32;
pub const MSG_Opcode_System: i32 = 805306391i32;
pub const MSG_Opcode_Web: i32 = 805306392i32;
pub const MSG_Publisher_Name: i32 = -1879048191i32;
pub const MSG_Task_AppBlocked: i32 = 1879048208i32;
pub const MSG_Task_AppOverride: i32 = 1879048209i32;
pub const MSG_Task_Application: i32 = 1879048212i32;
pub const MSG_Task_ComputerUsage: i32 = 1879048213i32;
pub const MSG_Task_ContentUsage: i32 = 1879048214i32;
pub const MSG_Task_Custom: i32 = 1879048205i32;
pub const MSG_Task_EmailContact: i32 = 1879048206i32;
pub const MSG_Task_EmailReceived: i32 = 1879048196i32;
pub const MSG_Task_EmailSent: i32 = 1879048197i32;
pub const MSG_Task_FileDownload: i32 = 1879048202i32;
pub const MSG_Task_GameStart: i32 = 1879048194i32;
pub const MSG_Task_IMContact: i32 = 1879048207i32;
pub const MSG_Task_IMFeature: i32 = 1879048203i32;
pub const MSG_Task_IMInvitation: i32 = 1879048199i32;
pub const MSG_Task_IMJoin: i32 = 1879048200i32;
pub const MSG_Task_IMLeave: i32 = 1879048201i32;
pub const MSG_Task_MediaPlayback: i32 = 1879048198i32;
pub const MSG_Task_SettingChange: i32 = 1879048193i32;
pub const MSG_Task_UrlVisit: i32 = 1879048195i32;
pub const MSG_Task_WebOverride: i32 = 1879048210i32;
pub const MSG_Task_WebsiteVisit: i32 = 1879048211i32;
pub const WPCCHANNEL: u32 = 16u32;
pub const WPCEVENT_APPLICATION_value: u32 = 20u32;
pub const WPCEVENT_APPOVERRIDE_value: u32 = 17u32;
pub const WPCEVENT_COMPUTERUSAGE_value: u32 = 21u32;
pub const WPCEVENT_CONTENTUSAGE_value: u32 = 22u32;
pub const WPCEVENT_CUSTOM_value: u32 = 13u32;
pub const WPCEVENT_EMAIL_CONTACT_value: u32 = 14u32;
pub const WPCEVENT_EMAIL_RECEIVED_value: u32 = 4u32;
pub const WPCEVENT_EMAIL_SENT_value: u32 = 5u32;
pub const WPCEVENT_GAME_START_value: u32 = 2u32;
pub const WPCEVENT_IM_CONTACT_value: u32 = 15u32;
pub const WPCEVENT_IM_FEATURE_value: u32 = 11u32;
pub const WPCEVENT_IM_INVITATION_value: u32 = 7u32;
pub const WPCEVENT_IM_JOIN_value: u32 = 8u32;
pub const WPCEVENT_IM_LEAVE_value: u32 = 9u32;
pub const WPCEVENT_MEDIA_PLAYBACK_value: u32 = 6u32;
pub const WPCEVENT_SYSTEM_APPBLOCKED_value: u32 = 16u32;
pub const WPCEVENT_SYS_SETTINGCHANGE_value: u32 = 1u32;
pub const WPCEVENT_WEBOVERRIDE_value: u32 = 18u32;
pub const WPCEVENT_WEB_FILEDOWNLOAD_value: u32 = 10u32;
pub const WPCEVENT_WEB_URLVISIT_value: u32 = 3u32;
pub const WPCEVENT_WEB_WEBSITEVISIT_value: u32 = 19u32;
pub const WPCFLAG_APPLICATION: WPCFLAG_OVERRIDE = WPCFLAG_OVERRIDE(1i32);
pub const WPCFLAG_APPS_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(16i32);
pub const WPCFLAG_GAMES_BLOCKED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(8i32);
pub const WPCFLAG_GAMES_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(64i32);
pub const WPCFLAG_HOURS_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_IM_FEATURE(pub i32);
pub const WPCFLAG_IM_FEATURE_ALL: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(-1i32);
pub const WPCFLAG_IM_FEATURE_AUDIO: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(2i32);
pub const WPCFLAG_IM_FEATURE_FILESWAP: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(16i32);
pub const WPCFLAG_IM_FEATURE_GAME: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(4i32);
pub const WPCFLAG_IM_FEATURE_NONE: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(0i32);
pub const WPCFLAG_IM_FEATURE_SENDING: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(-2147483648i32);
pub const WPCFLAG_IM_FEATURE_SMS: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(8i32);
pub const WPCFLAG_IM_FEATURE_URLSWAP: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(32i32);
pub const WPCFLAG_IM_FEATURE_VIDEO: WPCFLAG_IM_FEATURE = WPCFLAG_IM_FEATURE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_IM_LEAVE(pub i32);
pub const WPCFLAG_IM_LEAVE_CONVERSATION_END: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(2i32);
pub const WPCFLAG_IM_LEAVE_FORCED: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(1i32);
pub const WPCFLAG_IM_LEAVE_NORMAL: WPCFLAG_IM_LEAVE = WPCFLAG_IM_LEAVE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_ISBLOCKED(pub i32);
pub const WPCFLAG_ISBLOCKED_ATTACHMENTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(32768i32);
pub const WPCFLAG_ISBLOCKED_BADPASS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2048i32);
pub const WPCFLAG_ISBLOCKED_CATEGORYBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1048576i32);
pub const WPCFLAG_ISBLOCKED_CATEGORYNOTINLIST: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2097152i32);
pub const WPCFLAG_ISBLOCKED_CONTACTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(32i32);
pub const WPCFLAG_ISBLOCKED_DESCRIPTORBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(512i32);
pub const WPCFLAG_ISBLOCKED_DOWNLOADBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(128i32);
pub const WPCFLAG_ISBLOCKED_EMAILBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(2i32);
pub const WPCFLAG_ISBLOCKED_EXPLICITBLOCK: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1024i32);
pub const WPCFLAG_ISBLOCKED_FEATUREBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(64i32);
pub const WPCFLAG_ISBLOCKED_GAMESBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16i32);
pub const WPCFLAG_ISBLOCKED_IMBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(1i32);
pub const WPCFLAG_ISBLOCKED_INTERNALERROR: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(-1i32);
pub const WPCFLAG_ISBLOCKED_MAXHOURS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4096i32);
pub const WPCFLAG_ISBLOCKED_MEDIAPLAYBACKBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4i32);
pub const WPCFLAG_ISBLOCKED_NOACCESS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16777216i32);
pub const WPCFLAG_ISBLOCKED_NOTBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(0i32);
pub const WPCFLAG_ISBLOCKED_NOTEXPLICITLYALLOWED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(262144i32);
pub const WPCFLAG_ISBLOCKED_NOTINLIST: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(524288i32);
pub const WPCFLAG_ISBLOCKED_NOTKIDS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(4194304i32);
pub const WPCFLAG_ISBLOCKED_RATINGBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(256i32);
pub const WPCFLAG_ISBLOCKED_RECEIVERBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(131072i32);
pub const WPCFLAG_ISBLOCKED_SENDERBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(65536i32);
pub const WPCFLAG_ISBLOCKED_SETTINGSCHANGEBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(16384i32);
pub const WPCFLAG_ISBLOCKED_SPECHOURS: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8192i32);
pub const WPCFLAG_ISBLOCKED_UNRATED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8388608i32);
pub const WPCFLAG_ISBLOCKED_WEBBLOCKED: WPCFLAG_ISBLOCKED = WPCFLAG_ISBLOCKED(8i32);
pub const WPCFLAG_LOGGING_REQUIRED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_LOGOFF_TYPE(pub i32);
pub const WPCFLAG_LOGOFF_TYPE_FORCEDFUS: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(8i32);
pub const WPCFLAG_LOGOFF_TYPE_FUS: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(4i32);
pub const WPCFLAG_LOGOFF_TYPE_LOGOUT: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(0i32);
pub const WPCFLAG_LOGOFF_TYPE_RESTART: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(1i32);
pub const WPCFLAG_LOGOFF_TYPE_SHUTDOWN: WPCFLAG_LOGOFF_TYPE = WPCFLAG_LOGOFF_TYPE(2i32);
pub const WPCFLAG_NO_RESTRICTION: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_OVERRIDE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_RESTRICTION(pub i32);
pub const WPCFLAG_TIME_ALLOWANCE_RESTRICTED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_VISIBILITY(pub i32);
pub const WPCFLAG_WEB_FILTERED: WPCFLAG_RESTRICTION = WPCFLAG_RESTRICTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPCFLAG_WEB_SETTING(pub i32);
pub const WPCFLAG_WEB_SETTING_DOWNLOADSBLOCKED: WPCFLAG_WEB_SETTING = WPCFLAG_WEB_SETTING(1i32);
pub const WPCFLAG_WEB_SETTING_NOTBLOCKED: WPCFLAG_WEB_SETTING = WPCFLAG_WEB_SETTING(0i32);
pub const WPCFLAG_WPC_HIDDEN: WPCFLAG_VISIBILITY = WPCFLAG_VISIBILITY(1i32);
pub const WPCFLAG_WPC_VISIBLE: WPCFLAG_VISIBILITY = WPCFLAG_VISIBILITY(0i32);
pub const WPCPROV: windows_core::GUID = windows_core::GUID::from_u128(0x01090065_b467_4503_9b28_533766761087);
pub const WPCPROV_KEYWORD_ThirdParty: u32 = 32u32;
pub const WPCPROV_KEYWORD_WPC: u32 = 16u32;
pub const WPCPROV_TASK_AppBlocked: u32 = 16u32;
pub const WPCPROV_TASK_AppOverride: u32 = 17u32;
pub const WPCPROV_TASK_Application: u32 = 20u32;
pub const WPCPROV_TASK_ComputerUsage: u32 = 21u32;
pub const WPCPROV_TASK_ContentUsage: u32 = 22u32;
pub const WPCPROV_TASK_Custom: u32 = 13u32;
pub const WPCPROV_TASK_EmailContact: u32 = 14u32;
pub const WPCPROV_TASK_EmailReceived: u32 = 4u32;
pub const WPCPROV_TASK_EmailSent: u32 = 5u32;
pub const WPCPROV_TASK_FileDownload: u32 = 10u32;
pub const WPCPROV_TASK_GameStart: u32 = 2u32;
pub const WPCPROV_TASK_IMContact: u32 = 15u32;
pub const WPCPROV_TASK_IMFeature: u32 = 11u32;
pub const WPCPROV_TASK_IMInvitation: u32 = 7u32;
pub const WPCPROV_TASK_IMJoin: u32 = 8u32;
pub const WPCPROV_TASK_IMLeave: u32 = 9u32;
pub const WPCPROV_TASK_MediaPlayback: u32 = 6u32;
pub const WPCPROV_TASK_SettingChange: u32 = 1u32;
pub const WPCPROV_TASK_UrlVisit: u32 = 3u32;
pub const WPCPROV_TASK_WebOverride: u32 = 18u32;
pub const WPCPROV_TASK_WebsiteVisit: u32 = 19u32;
pub const WPC_APP_LAUNCH: u32 = 22u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_APPLICATIONEVENT(pub i32);
pub const WPC_ARGS_APPLICATIONEVENT_CARGS: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(5i32);
pub const WPC_ARGS_APPLICATIONEVENT_CREATIONTIME: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(3i32);
pub const WPC_ARGS_APPLICATIONEVENT_DECISION: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(1i32);
pub const WPC_ARGS_APPLICATIONEVENT_PROCESSID: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(2i32);
pub const WPC_ARGS_APPLICATIONEVENT_SERIALIZEDAPPLICATION: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(0i32);
pub const WPC_ARGS_APPLICATIONEVENT_TIMEUSED: WPC_ARGS_APPLICATIONEVENT = WPC_ARGS_APPLICATIONEVENT(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_APPOVERRIDEEVENT(pub i32);
pub const WPC_ARGS_APPOVERRIDEEVENT_CARGS: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(3i32);
pub const WPC_ARGS_APPOVERRIDEEVENT_PATH: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(1i32);
pub const WPC_ARGS_APPOVERRIDEEVENT_REASON: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(2i32);
pub const WPC_ARGS_APPOVERRIDEEVENT_USERID: WPC_ARGS_APPOVERRIDEEVENT = WPC_ARGS_APPOVERRIDEEVENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_COMPUTERUSAGEEVENT(pub i32);
pub const WPC_ARGS_COMPUTERUSAGEEVENT_CARGS: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(2i32);
pub const WPC_ARGS_COMPUTERUSAGEEVENT_ID: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(0i32);
pub const WPC_ARGS_COMPUTERUSAGEEVENT_TIMEUSED: WPC_ARGS_COMPUTERUSAGEEVENT = WPC_ARGS_COMPUTERUSAGEEVENT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_CONTENTUSAGEEVENT(pub i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_CARGS: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(7i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_CATEGORY: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(4i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_CONTENTPROVIDERID: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(0i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_CONTENTPROVIDERTITLE: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(1i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_DECISION: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(6i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_ID: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(2i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_RATINGS: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(5i32);
pub const WPC_ARGS_CONTENTUSAGEEVENT_TITLE: WPC_ARGS_CONTENTUSAGEEVENT = WPC_ARGS_CONTENTUSAGEEVENT(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_CONVERSATIONINITEVENT(pub i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(2i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_APPNAME: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(0i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_APPVERSION: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(1i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_CARGS: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(9i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_CONVID: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(3i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_REASON: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(6i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_RECIPCOUNT: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(7i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_RECIPIENT: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(8i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_REQUESTINGIP: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(4i32);
pub const WPC_ARGS_CONVERSATIONINITEVENT_SENDER: WPC_ARGS_CONVERSATIONINITEVENT = WPC_ARGS_CONVERSATIONINITEVENT(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_CONVERSATIONJOINEVENT(pub i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(2i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_APPNAME: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(0i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_APPVERSION: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(1i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_CARGS: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(10i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_CONVID: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(3i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_JOININGIP: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(4i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_JOININGUSER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(5i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_MEMBER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(8i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_MEMBERCOUNT: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(7i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_REASON: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(6i32);
pub const WPC_ARGS_CONVERSATIONJOINEVENT_SENDER: WPC_ARGS_CONVERSATIONJOINEVENT = WPC_ARGS_CONVERSATIONJOINEVENT(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_CONVERSATIONLEAVEEVENT(pub i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_ACCOUNTNAME: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(2i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_APPNAME: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(0i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_APPVERSION: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(1i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_CARGS: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(10i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_CONVID: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(3i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_FLAGS: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(9i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_LEAVINGIP: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(4i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_LEAVINGUSER: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(5i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_MEMBER: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(8i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_MEMBERCOUNT: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(7i32);
pub const WPC_ARGS_CONVERSATIONLEAVEEVENT_REASON: WPC_ARGS_CONVERSATIONLEAVEEVENT = WPC_ARGS_CONVERSATIONLEAVEEVENT(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_CUSTOMEVENT(pub i32);
pub const WPC_ARGS_CUSTOMEVENT_APPNAME: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(1i32);
pub const WPC_ARGS_CUSTOMEVENT_APPVERSION: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(2i32);
pub const WPC_ARGS_CUSTOMEVENT_BLOCKED: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(7i32);
pub const WPC_ARGS_CUSTOMEVENT_CARGS: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(9i32);
pub const WPC_ARGS_CUSTOMEVENT_EVENT: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(3i32);
pub const WPC_ARGS_CUSTOMEVENT_PUBLISHER: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(0i32);
pub const WPC_ARGS_CUSTOMEVENT_REASON: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(8i32);
pub const WPC_ARGS_CUSTOMEVENT_VALUE1: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(4i32);
pub const WPC_ARGS_CUSTOMEVENT_VALUE2: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(5i32);
pub const WPC_ARGS_CUSTOMEVENT_VALUE3: WPC_ARGS_CUSTOMEVENT = WPC_ARGS_CUSTOMEVENT(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_EMAILCONTACTEVENT(pub i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_APPNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(0i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_APPVERSION: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(1i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_CARGS: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(8i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_EMAILACCOUNT: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(7i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_NEWID: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(5i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_NEWNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(4i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_OLDID: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(3i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_OLDNAME: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(2i32);
pub const WPC_ARGS_EMAILCONTACTEVENT_REASON: WPC_ARGS_EMAILCONTACTEVENT = WPC_ARGS_EMAILCONTACTEVENT(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_EMAILRECEIEVEDEVENT(pub i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_APPNAME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(1i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_APPVERSION: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(2i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_ATTACHCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(7i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_ATTACHMENTNAME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(8i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_CARGS: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(11i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_EMAILACCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(10i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_REASON: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(4i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECEIVEDTIME: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(9i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECIPCOUNT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(5i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_RECIPIENT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(6i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_SENDER: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(0i32);
pub const WPC_ARGS_EMAILRECEIEVEDEVENT_SUBJECT: WPC_ARGS_EMAILRECEIEVEDEVENT = WPC_ARGS_EMAILRECEIEVEDEVENT(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_EMAILSENTEVENT(pub i32);
pub const WPC_ARGS_EMAILSENTEVENT_APPNAME: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(1i32);
pub const WPC_ARGS_EMAILSENTEVENT_APPVERSION: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(2i32);
pub const WPC_ARGS_EMAILSENTEVENT_ATTACHCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(7i32);
pub const WPC_ARGS_EMAILSENTEVENT_ATTACHMENTNAME: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(8i32);
pub const WPC_ARGS_EMAILSENTEVENT_CARGS: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(10i32);
pub const WPC_ARGS_EMAILSENTEVENT_EMAILACCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(9i32);
pub const WPC_ARGS_EMAILSENTEVENT_REASON: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(4i32);
pub const WPC_ARGS_EMAILSENTEVENT_RECIPCOUNT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(5i32);
pub const WPC_ARGS_EMAILSENTEVENT_RECIPIENT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(6i32);
pub const WPC_ARGS_EMAILSENTEVENT_SENDER: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(0i32);
pub const WPC_ARGS_EMAILSENTEVENT_SUBJECT: WPC_ARGS_EMAILSENTEVENT = WPC_ARGS_EMAILSENTEVENT(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_FILEDOWNLOADEVENT(pub i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_APPNAME: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(1i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_BLOCKED: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(3i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_CARGS: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(5i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_PATH: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(4i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_URL: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(0i32);
pub const WPC_ARGS_FILEDOWNLOADEVENT_VERSION: WPC_ARGS_FILEDOWNLOADEVENT = WPC_ARGS_FILEDOWNLOADEVENT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_GAMESTARTEVENT(pub i32);
pub const WPC_ARGS_GAMESTARTEVENT_APPID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(0i32);
pub const WPC_ARGS_GAMESTARTEVENT_APPVERSION: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(2i32);
pub const WPC_ARGS_GAMESTARTEVENT_CARGS: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(10i32);
pub const WPC_ARGS_GAMESTARTEVENT_DESCCOUNT: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(7i32);
pub const WPC_ARGS_GAMESTARTEVENT_DESCRIPTOR: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(8i32);
pub const WPC_ARGS_GAMESTARTEVENT_INSTANCEID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(1i32);
pub const WPC_ARGS_GAMESTARTEVENT_PATH: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(3i32);
pub const WPC_ARGS_GAMESTARTEVENT_PID: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(9i32);
pub const WPC_ARGS_GAMESTARTEVENT_RATING: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(4i32);
pub const WPC_ARGS_GAMESTARTEVENT_RATINGSYSTEM: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(5i32);
pub const WPC_ARGS_GAMESTARTEVENT_REASON: WPC_ARGS_GAMESTARTEVENT = WPC_ARGS_GAMESTARTEVENT(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_IMCONTACTEVENT(pub i32);
pub const WPC_ARGS_IMCONTACTEVENT_ACCOUNTNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(2i32);
pub const WPC_ARGS_IMCONTACTEVENT_APPNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(0i32);
pub const WPC_ARGS_IMCONTACTEVENT_APPVERSION: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(1i32);
pub const WPC_ARGS_IMCONTACTEVENT_CARGS: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(8i32);
pub const WPC_ARGS_IMCONTACTEVENT_NEWID: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(6i32);
pub const WPC_ARGS_IMCONTACTEVENT_NEWNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(5i32);
pub const WPC_ARGS_IMCONTACTEVENT_OLDID: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(4i32);
pub const WPC_ARGS_IMCONTACTEVENT_OLDNAME: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(3i32);
pub const WPC_ARGS_IMCONTACTEVENT_REASON: WPC_ARGS_IMCONTACTEVENT = WPC_ARGS_IMCONTACTEVENT(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_IMFEATUREEVENT(pub i32);
pub const WPC_ARGS_IMFEATUREEVENT_ACCOUNTNAME: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(2i32);
pub const WPC_ARGS_IMFEATUREEVENT_APPNAME: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(0i32);
pub const WPC_ARGS_IMFEATUREEVENT_APPVERSION: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(1i32);
pub const WPC_ARGS_IMFEATUREEVENT_CARGS: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(11i32);
pub const WPC_ARGS_IMFEATUREEVENT_CONVID: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(3i32);
pub const WPC_ARGS_IMFEATUREEVENT_DATA: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(10i32);
pub const WPC_ARGS_IMFEATUREEVENT_MEDIATYPE: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(4i32);
pub const WPC_ARGS_IMFEATUREEVENT_REASON: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(5i32);
pub const WPC_ARGS_IMFEATUREEVENT_RECIPCOUNT: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(6i32);
pub const WPC_ARGS_IMFEATUREEVENT_RECIPIENT: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(7i32);
pub const WPC_ARGS_IMFEATUREEVENT_SENDER: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(8i32);
pub const WPC_ARGS_IMFEATUREEVENT_SENDERIP: WPC_ARGS_IMFEATUREEVENT = WPC_ARGS_IMFEATUREEVENT(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_MEDIADOWNLOADEVENT(pub i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_ALBUM: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(6i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_APPNAME: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(0i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_APPVERSION: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(1i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_CARGS: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(9i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_EXPLICIT: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(7i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_MEDIATYPE: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(2i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_PATH: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(3i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_PML: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(5i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_REASON: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(8i32);
pub const WPC_ARGS_MEDIADOWNLOADEVENT_TITLE: WPC_ARGS_MEDIADOWNLOADEVENT = WPC_ARGS_MEDIADOWNLOADEVENT(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_MEDIAPLAYBACKEVENT(pub i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_ALBUM: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(6i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_APPNAME: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(0i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_APPVERSION: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(1i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_CARGS: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(9i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_EXPLICIT: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(7i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_MEDIATYPE: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(2i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_PATH: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(3i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_PML: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(5i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_REASON: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(8i32);
pub const WPC_ARGS_MEDIAPLAYBACKEVENT_TITLE: WPC_ARGS_MEDIAPLAYBACKEVENT = WPC_ARGS_MEDIAPLAYBACKEVENT(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_SAFERAPPBLOCKED(pub i32);
pub const WPC_ARGS_SAFERAPPBLOCKED_CARGS: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(4i32);
pub const WPC_ARGS_SAFERAPPBLOCKED_PATH: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(2i32);
pub const WPC_ARGS_SAFERAPPBLOCKED_RULEID: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(3i32);
pub const WPC_ARGS_SAFERAPPBLOCKED_TIMESTAMP: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(0i32);
pub const WPC_ARGS_SAFERAPPBLOCKED_USERID: WPC_ARGS_SAFERAPPBLOCKED = WPC_ARGS_SAFERAPPBLOCKED(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_SETTINGSCHANGEEVENT(pub i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_CARGS: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(7i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_CLASS: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(0i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_NEWVAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(4i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OLDVAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(3i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OPTIONAL: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(6i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_OWNER: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(2i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_REASON: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(5i32);
pub const WPC_ARGS_SETTINGSCHANGEEVENT_SETTING: WPC_ARGS_SETTINGSCHANGEEVENT = WPC_ARGS_SETTINGSCHANGEEVENT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_URLVISITEVENT(pub i32);
pub const WPC_ARGS_URLVISITEVENT_APPNAME: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(1i32);
pub const WPC_ARGS_URLVISITEVENT_CARGS: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(7i32);
pub const WPC_ARGS_URLVISITEVENT_CATCOUNT: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(5i32);
pub const WPC_ARGS_URLVISITEVENT_CATEGORY: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(6i32);
pub const WPC_ARGS_URLVISITEVENT_RATINGSYSTEMID: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(4i32);
pub const WPC_ARGS_URLVISITEVENT_REASON: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(3i32);
pub const WPC_ARGS_URLVISITEVENT_URL: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(0i32);
pub const WPC_ARGS_URLVISITEVENT_VERSION: WPC_ARGS_URLVISITEVENT = WPC_ARGS_URLVISITEVENT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_WEBOVERRIDEEVENT(pub i32);
pub const WPC_ARGS_WEBOVERRIDEEVENT_CARGS: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(3i32);
pub const WPC_ARGS_WEBOVERRIDEEVENT_REASON: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(2i32);
pub const WPC_ARGS_WEBOVERRIDEEVENT_URL: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(1i32);
pub const WPC_ARGS_WEBOVERRIDEEVENT_USERID: WPC_ARGS_WEBOVERRIDEEVENT = WPC_ARGS_WEBOVERRIDEEVENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_ARGS_WEBSITEVISITEVENT(pub i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_BLOCKEDCATEGORIES: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(3i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_CARGS: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(9i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_CATEGORIES: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(2i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_CONTENTTYPE: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(6i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_DECISION: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(1i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_REFERRER: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(7i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_SERIALIZEDAPPLICATION: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(4i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_TELEMETRY: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(8i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_TITLE: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(5i32);
pub const WPC_ARGS_WEBSITEVISITEVENT_URL: WPC_ARGS_WEBSITEVISITEVENT = WPC_ARGS_WEBSITEVISITEVENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_MEDIA_EXPLICIT(pub i32);
pub const WPC_MEDIA_EXPLICIT_FALSE: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(0i32);
pub const WPC_MEDIA_EXPLICIT_TRUE: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(1i32);
pub const WPC_MEDIA_EXPLICIT_UNKNOWN: WPC_MEDIA_EXPLICIT = WPC_MEDIA_EXPLICIT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_MEDIA_TYPE(pub i32);
pub const WPC_MEDIA_TYPE_AUDIO_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(3i32);
pub const WPC_MEDIA_TYPE_CD_AUDIO: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(4i32);
pub const WPC_MEDIA_TYPE_DVD: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(1i32);
pub const WPC_MEDIA_TYPE_MAX: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(7i32);
pub const WPC_MEDIA_TYPE_OTHER: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(0i32);
pub const WPC_MEDIA_TYPE_PICTURE_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(6i32);
pub const WPC_MEDIA_TYPE_RECORDED_TV: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(2i32);
pub const WPC_MEDIA_TYPE_VIDEO_FILE: WPC_MEDIA_TYPE = WPC_MEDIA_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPC_SETTINGS(pub i32);
pub const WPC_SETTINGS_ALLOW_BLOCK: WPC_SETTINGS = WPC_SETTINGS(14i32);
pub const WPC_SETTINGS_GAME_ALLOW_UNRATED: WPC_SETTINGS = WPC_SETTINGS(16i32);
pub const WPC_SETTINGS_GAME_BLOCKED: WPC_SETTINGS = WPC_SETTINGS(15i32);
pub const WPC_SETTINGS_GAME_DENIED_DESCRIPTORS: WPC_SETTINGS = WPC_SETTINGS(18i32);
pub const WPC_SETTINGS_GAME_MAX_ALLOWED: WPC_SETTINGS = WPC_SETTINGS(17i32);
pub const WPC_SETTINGS_GAME_RESTRICTED: WPC_SETTINGS = WPC_SETTINGS(36i32);
pub const WPC_SETTINGS_LOCATE: u32 = 20u32;
pub const WPC_SETTINGS_MODIFY: u32 = 21u32;
pub const WPC_SETTINGS_RATING_SYSTEM_PATH: WPC_SETTINGS = WPC_SETTINGS(32i32);
pub const WPC_SETTINGS_SYSTEM_CURRENT_RATING_SYSTEM: WPC_SETTINGS = WPC_SETTINGS(6i32);
pub const WPC_SETTINGS_SYSTEM_FILTER_ID: WPC_SETTINGS = WPC_SETTINGS(11i32);
pub const WPC_SETTINGS_SYSTEM_FILTER_NAME: WPC_SETTINGS = WPC_SETTINGS(12i32);
pub const WPC_SETTINGS_SYSTEM_HTTP_EXEMPTION_LIST: WPC_SETTINGS = WPC_SETTINGS(9i32);
pub const WPC_SETTINGS_SYSTEM_LAST_LOG_VIEW: WPC_SETTINGS = WPC_SETTINGS(7i32);
pub const WPC_SETTINGS_SYSTEM_LOCALE: WPC_SETTINGS = WPC_SETTINGS(13i32);
pub const WPC_SETTINGS_SYSTEM_LOG_VIEW_REMINDER_INTERVAL: WPC_SETTINGS = WPC_SETTINGS(8i32);
pub const WPC_SETTINGS_SYSTEM_URL_EXEMPTION_LIST: WPC_SETTINGS = WPC_SETTINGS(10i32);
pub const WPC_SETTINGS_USER_APP_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(24i32);
pub const WPC_SETTINGS_USER_HOURLY_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(21i32);
pub const WPC_SETTINGS_USER_LOGGING_REQUIRED: WPC_SETTINGS = WPC_SETTINGS(20i32);
pub const WPC_SETTINGS_USER_LOGON_HOURS: WPC_SETTINGS = WPC_SETTINGS(23i32);
pub const WPC_SETTINGS_USER_OVERRRIDE_REQUESTS: WPC_SETTINGS = WPC_SETTINGS(22i32);
pub const WPC_SETTINGS_USER_TIME_ALLOWANCE: WPC_SETTINGS = WPC_SETTINGS(34i32);
pub const WPC_SETTINGS_USER_TIME_ALLOWANCE_RESTRICTIONS: WPC_SETTINGS = WPC_SETTINGS(35i32);
pub const WPC_SETTINGS_USER_WPC_ENABLED: WPC_SETTINGS = WPC_SETTINGS(19i32);
pub const WPC_SETTINGS_WEB_BLOCKED_CATEGORY_LIST: WPC_SETTINGS = WPC_SETTINGS(28i32);
pub const WPC_SETTINGS_WEB_BLOCK_UNRATED: WPC_SETTINGS = WPC_SETTINGS(29i32);
pub const WPC_SETTINGS_WEB_DOWNLOAD_BLOCKED: WPC_SETTINGS = WPC_SETTINGS(26i32);
pub const WPC_SETTINGS_WEB_FILTER_LEVEL: WPC_SETTINGS = WPC_SETTINGS(27i32);
pub const WPC_SETTINGS_WEB_FILTER_ON: WPC_SETTINGS = WPC_SETTINGS(25i32);
pub const WPC_SETTINGS_WPC_ENABLED: WPC_SETTINGS = WPC_SETTINGS(30i32);
pub const WPC_SETTINGS_WPC_EXTENSION_DISABLEDIMAGE_PATH: WPC_SETTINGS = WPC_SETTINGS(3i32);
pub const WPC_SETTINGS_WPC_EXTENSION_IMAGE_PATH: WPC_SETTINGS = WPC_SETTINGS(2i32);
pub const WPC_SETTINGS_WPC_EXTENSION_NAME: WPC_SETTINGS = WPC_SETTINGS(4i32);
pub const WPC_SETTINGS_WPC_EXTENSION_PATH: WPC_SETTINGS = WPC_SETTINGS(0i32);
pub const WPC_SETTINGS_WPC_EXTENSION_SILO: WPC_SETTINGS = WPC_SETTINGS(1i32);
pub const WPC_SETTINGS_WPC_EXTENSION_SUB_TITLE: WPC_SETTINGS = WPC_SETTINGS(5i32);
pub const WPC_SETTINGS_WPC_LOGGING_REQUIRED: WPC_SETTINGS = WPC_SETTINGS(31i32);
pub const WPC_SETTINGS_WPC_PROVIDER_CURRENT: WPC_SETTINGS = WPC_SETTINGS(33i32);
pub const WPC_SETTING_COUNT: WPC_SETTINGS = WPC_SETTINGS(37i32);
pub const WPC_SYSTEM: u32 = 23u32;
pub const WPC_WEB: u32 = 24u32;
pub const WindowsParentalControls: windows_core::GUID = windows_core::GUID::from_u128(0xe77cc89b_7401_4c04_8ced_149db35add04);
pub const WpcProviderSupport: windows_core::GUID = windows_core::GUID::from_u128(0xbb18c7a0_2186_4be0_97d8_04847b628e02);
pub const WpcSettingsProvider: windows_core::GUID = windows_core::GUID::from_u128(0x355dffaa_3b9f_435c_b428_5d44290bc5f2);
