#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activatable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Activatable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Activatable {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Activatable,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn WithValue(arg: i32) -> windows_core::Result<Activatable> {
        Self::IActivatableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WithValue)(
                windows_core::Interface::as_raw(this),
                arg,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IActivatableFactory<R, F: FnOnce(&IActivatableFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Activatable, IActivatableFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Activatable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IActivatable>();
}
unsafe impl windows_core::Interface for Activatable {
    type Vtable = <IActivatable as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivatable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Activatable {
    const NAME: &'static str = "test_constructors.Activatable";
}
unsafe impl Send for Activatable {}
unsafe impl Sync for Activatable {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Composable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Composable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Composable {
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn new() -> windows_core::Result<Composable> {
        Self::IComposableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn WithValue(arg: i32) -> windows_core::Result<Composable> {
        Self::IComposableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WithValue)(
                windows_core::Interface::as_raw(this),
                arg,
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IComposableFactory<R, F: FnOnce(&IComposableFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Composable, IComposableFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Composable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IComposable>();
}
unsafe impl windows_core::Interface for Composable {
    type Vtable = <IComposable as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IComposable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Composable {
    const NAME: &'static str = "test_constructors.Composable";
}
unsafe impl Send for Composable {}
unsafe impl Sync for Composable {}
windows_core::imp::define_interface!(
    IActivatable,
    IActivatable_Vtbl,
    0xe566522b_9c26_582b_950d_177b05d36efd
);
impl windows_core::RuntimeType for IActivatable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IActivatable {
    const NAME: &'static str = "test_constructors.IActivatable";
}
pub trait IActivatable_Impl: windows_core::IUnknownImpl {
    fn Property(&self) -> windows_core::Result<i32>;
}
impl IActivatable_Vtbl {
    pub const fn new<Identity: IActivatable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Property<Identity: IActivatable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatable_Impl::Property(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatable, OFFSET>(),
            Property: Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IActivatableFactory,
    IActivatableFactory_Vtbl,
    0xafc5aee9_aa78_5da6_85a2_69e67b45c620
);
impl windows_core::RuntimeType for IActivatableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IActivatableFactory {
    const NAME: &'static str = "test_constructors.IActivatableFactory";
}
pub trait IActivatableFactory_Impl: windows_core::IUnknownImpl {
    fn WithValue(&self, arg: i32) -> windows_core::Result<Activatable>;
}
impl IActivatableFactory_Vtbl {
    pub const fn new<Identity: IActivatableFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WithValue<
            Identity: IActivatableFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            arg: i32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableFactory_Impl::WithValue(this, arg) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatableFactory, OFFSET>(),
            WithValue: WithValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatableFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WithValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IComposable,
    IComposable_Vtbl,
    0xff2595d6_461d_5118_9296_f2a2b1e64544
);
impl windows_core::RuntimeType for IComposable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IComposable {
    const NAME: &'static str = "test_constructors.IComposable";
}
pub trait IComposable_Impl: windows_core::IUnknownImpl {
    fn Property(&self) -> windows_core::Result<i32>;
}
impl IComposable_Vtbl {
    pub const fn new<Identity: IComposable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Property<Identity: IComposable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComposable_Impl::Property(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IComposable, OFFSET>(),
            Property: Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComposable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComposable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IComposableFactory,
    IComposableFactory_Vtbl,
    0x6a461099_83c0_5810_9e20_2e8b9521d143
);
impl windows_core::RuntimeType for IComposableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IComposableFactory {
    const NAME: &'static str = "test_constructors.IComposableFactory";
}
pub trait IComposableFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<Composable>;
    fn WithValue(
        &self,
        arg: i32,
        baseInterface: windows_core::Ref<'_, windows_core::IInspectable>,
        innerInterface: windows_core::OutRef<'_, windows_core::IInspectable>,
    ) -> windows_core::Result<Composable>;
}
impl IComposableFactory_Vtbl {
    pub const fn new<Identity: IComposableFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IComposableFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComposableFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WithValue<
            Identity: IComposableFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            arg: i32,
            baseinterface: *mut core::ffi::c_void,
            innerinterface: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComposableFactory_Impl::WithValue(
                    this,
                    arg,
                    core::mem::transmute_copy(&baseinterface),
                    core::mem::transmute_copy(&innerinterface),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IComposableFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            WithValue: WithValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComposableFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComposableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub WithValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
