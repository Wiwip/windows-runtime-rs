windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpinfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpinfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfigA(hservice : SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_sys::core::PCSTR, lploadordergroup : windows_sys::core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_sys::core::PCSTR, lpservicestartname : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ChangeServiceConfigW(hservice : SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_sys::core::PCWSTR, lploadordergroup : windows_sys::core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_sys::core::PCWSTR, lpservicestartname : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn CloseServiceHandle(hscobject : SC_HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ControlService(hservice : SC_HANDLE, dwcontrol : u32, lpservicestatus : *mut SERVICE_STATUS) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ControlServiceExA(hservice : SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ControlServiceExW(hservice : SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn CreateServiceA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PCSTR, dwdesiredaccess : u32, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_sys::core::PCSTR, lploadordergroup : windows_sys::core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_sys::core::PCSTR, lpservicestartname : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn CreateServiceW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwservicetype : ENUM_SERVICE_TYPE, dwstarttype : SERVICE_START_TYPE, dwerrorcontrol : SERVICE_ERROR, lpbinarypathname : windows_sys::core::PCWSTR, lploadordergroup : windows_sys::core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_sys::core::PCWSTR, lpservicestartname : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn DeleteService(hservice : SC_HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumDependentServicesA(hservice : SC_HANDLE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumDependentServicesW(hservice : SC_HANDLE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusA(hscmanager : SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusExA(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusExW(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn EnumServicesStatusW(hscmanager : SC_HANDLE, dwservicetype : ENUM_SERVICE_TYPE, dwservicestate : ENUM_SERVICE_STATE, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-service-core-l1-1-4.dll" "system" fn GetServiceDirectory(hservicestatus : SERVICE_STATUS_HANDLE, edirectorytype : SERVICE_DIRECTORY_TYPE, lppathbuffer : windows_sys::core::PWSTR, cchpathbufferlength : u32, lpcchrequiredbufferlength : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn GetServiceDisplayNameA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PSTR, lpcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetServiceDisplayNameW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PWSTR, lpcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetServiceKeyNameA(hscmanager : SC_HANDLE, lpdisplayname : windows_sys::core::PCSTR, lpservicename : windows_sys::core::PSTR, lpcchbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn GetServiceKeyNameW(hscmanager : SC_HANDLE, lpdisplayname : windows_sys::core::PCWSTR, lpservicename : windows_sys::core::PWSTR, lpcchbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("api-ms-win-service-core-l1-1-3.dll" "system" fn GetServiceRegistryStateKey(servicestatushandle : SERVICE_STATUS_HANDLE, statetype : SERVICE_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::Registry:: HKEY) -> u32);
windows_targets::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceDirectory(servicehandle : SC_HANDLE, directorytype : SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer : windows_sys::core::PWSTR, pathbufferlength : u32, requiredbufferlength : *mut u32) -> u32);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceRegistryStateKey(servicehandle : SC_HANDLE, statetype : SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::Registry:: HKEY) -> u32);
windows_targets::link!("advapi32.dll" "system" fn LockServiceDatabase(hscmanager : SC_HANDLE) -> *mut core::ffi::c_void);
windows_targets::link!("advapi32.dll" "system" fn NotifyBootConfigStatus(bootacceptable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeA(hservice : SC_HANDLE, dwnotifymask : SERVICE_NOTIFY, pnotifybuffer : *const SERVICE_NOTIFY_2A) -> u32);
windows_targets::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeW(hservice : SC_HANDLE, dwnotifymask : SERVICE_NOTIFY, pnotifybuffer : *const SERVICE_NOTIFY_2W) -> u32);
windows_targets::link!("advapi32.dll" "system" fn OpenSCManagerA(lpmachinename : windows_sys::core::PCSTR, lpdatabasename : windows_sys::core::PCSTR, dwdesiredaccess : u32) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn OpenSCManagerW(lpmachinename : windows_sys::core::PCWSTR, lpdatabasename : windows_sys::core::PCWSTR, dwdesiredaccess : u32) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn OpenServiceA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, dwdesiredaccess : u32) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn OpenServiceW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, dwdesiredaccess : u32) -> SC_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : SERVICE_CONFIG, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfigA(hservice : SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceConfigW(hservice : SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceDynamicInformation(hservicestatus : SERVICE_STATUS_HANDLE, dwinfolevel : u32, ppdynamicinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceLockStatusA(hscmanager : SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceLockStatusW(hscmanager : SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn QueryServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : u32, lpsecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceStatus(hservice : SC_HANDLE, lpservicestatus : *mut SERVICE_STATUS) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn QueryServiceStatusEx(hservice : SC_HANDLE, infolevel : SC_STATUS_TYPE, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerA(lpservicename : windows_sys::core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExA(lpservicename : windows_sys::core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW(lpservicename : windows_sys::core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerW(lpservicename : windows_sys::core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
windows_targets::link!("advapi32.dll" "system" fn SetServiceBits(hservicestatus : SERVICE_STATUS_HANDLE, dwservicebits : u32, bsetbitson : windows_sys::core::BOOL, bupdateimmediately : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn SetServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SetServiceStatus(hservicestatus : SERVICE_STATUS_HANDLE, lpservicestatus : *const SERVICE_STATUS) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn StartServiceA(hservice : SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherA(lpservicestarttable : *const SERVICE_TABLE_ENTRYA) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherW(lpservicestarttable : *const SERVICE_TABLE_ENTRYW) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn StartServiceW(hservice : SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("sechost.dll" "system" fn SubscribeServiceChangeNotifications(hservice : SC_HANDLE, eeventtype : SC_EVENT_TYPE, pcallback : PSC_NOTIFICATION_CALLBACK, pcallbackcontext : *const core::ffi::c_void, psubscription : *mut PSC_NOTIFICATION_REGISTRATION) -> u32);
windows_targets::link!("advapi32.dll" "system" fn UnlockServiceDatabase(sclock : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("sechost.dll" "system" fn UnsubscribeServiceChangeNotifications(psubscription : PSC_NOTIFICATION_REGISTRATION));
windows_targets::link!("advapi32.dll" "system" fn WaitServiceState(hservice : SC_HANDLE, dwnotify : u32, dwtimeout : u32, hcancelevent : super::super::Foundation:: HANDLE) -> u32);
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
pub const DOMAIN_JOIN_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
pub type ENUM_SERVICE_STATE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl Default for ENUM_SERVICE_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl Default for ENUM_SERVICE_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl Default for ENUM_SERVICE_STATUS_PROCESSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl Default for ENUM_SERVICE_STATUS_PROCESSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ENUM_SERVICE_TYPE = u32;
pub const FIREWALL_PORT_CLOSE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
pub type HANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type HANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPHANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPSERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_sys::core::PSTR)>;
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_sys::core::PWSTR)>;
pub const MACHINE_POLICY_PRESENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = 2i32;
pub const NAMED_PIPE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
pub type PFN_SC_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(pparameter: *const core::ffi::c_void)>;
pub type PSC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const core::ffi::c_void)>;
pub type PSC_NOTIFICATION_REGISTRATION = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: windows_sys::core::PSTR,
    pub lpLoadOrderGroup: windows_sys::core::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_sys::core::PSTR,
    pub lpServiceStartName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
}
impl Default for QUERY_SERVICE_CONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: windows_sys::core::PWSTR,
    pub lpLoadOrderGroup: windows_sys::core::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_sys::core::PWSTR,
    pub lpServiceStartName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
}
impl Default for QUERY_SERVICE_CONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_sys::core::PSTR,
    pub dwLockDuration: u32,
}
impl Default for QUERY_SERVICE_LOCK_STATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_sys::core::PWSTR,
    pub dwLockDuration: u32,
}
impl Default for QUERY_SERVICE_LOCK_STATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_INTERFACE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
pub const SC_ACTION_NONE: SC_ACTION_TYPE = 0i32;
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = 4i32;
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = 2i32;
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = 1i32;
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = 3i32;
pub type SC_ACTION_TYPE = i32;
pub const SC_AGGREGATE_STORAGE_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("System\\CurrentControlSet\\Control\\ServiceAggregatedEvents");
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = 0i32;
pub type SC_ENUM_TYPE = i32;
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = 0i32;
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = 1i32;
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = 2i32;
pub type SC_EVENT_TYPE = i32;
pub type SC_HANDLE = *mut core::ffi::c_void;
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
pub const SC_MANAGER_CONNECT: u32 = 1u32;
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
pub const SC_MANAGER_LOCK: u32 = 8u32;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = 0i32;
pub type SC_STATUS_TYPE = i32;
pub const SERVICES_ACTIVE_DATABASE: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEA: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEW: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesActive");
pub const SERVICES_FAILED_DATABASE: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEA: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEW: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesFailed");
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = 1u32;
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = 4u32;
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = 2u32;
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = 0u32;
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
pub type SERVICE_CONFIG = u32;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = 3u32;
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = 1u32;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = 2u32;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = 4u32;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = 12u32;
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = 9u32;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = 7u32;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = 6u32;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = 5u32;
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = 8u32;
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = 5u32;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: windows_sys::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl Default for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: windows_sys::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl Default for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: windows_sys::core::BOOL,
}
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = 3u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: windows_sys::core::PSTR,
}
impl Default for SERVICE_DESCRIPTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: windows_sys::core::PWSTR,
}
impl Default for SERVICE_DESCRIPTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVICE_DIRECTORY_TYPE = i32;
pub const SERVICE_DISABLED: SERVICE_START_TYPE = 4u32;
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = 11u32;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
pub type SERVICE_ERROR = u32;
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = 3u32;
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = 0u32;
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = 1u32;
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_sys::core::PSTR,
    pub lpCommand: windows_sys::core::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_sys::core::PWSTR,
    pub lpCommand: windows_sys::core::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: windows_sys::core::BOOL,
}
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = 2u32;
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = 2u32;
pub const SERVICE_INTERROGATE: u32 = 128u32;
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = 1u32;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
pub type SERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8)>;
pub type SERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_sys::core::PWSTR)>;
pub type SERVICE_NOTIFY = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl Default for SERVICE_NOTIFY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_sys::core::PSTR,
}
impl Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_sys::core::PWSTR,
}
impl Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = 16u32;
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = 128u32;
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = 256u32;
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = 512u32;
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = 64u32;
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = 32u32;
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = 8u32;
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = 2u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = 1u32;
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = 4u32;
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = 7u32;
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = 8u32;
pub type SERVICE_REGISTRY_STATE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: windows_sys::core::PSTR,
}
impl Default for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: windows_sys::core::PWSTR,
}
impl Default for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = 4u32;
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = 0u32;
pub type SERVICE_RUNS_IN_PROCESS = u32;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = 1u32;
pub type SERVICE_SHARED_DIRECTORY_TYPE = i32;
pub type SERVICE_SHARED_REGISTRY_STATE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
pub const SERVICE_START: u32 = 16u32;
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
pub type SERVICE_START_TYPE = u32;
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
pub type SERVICE_STATUS_CURRENT_STATE = u32;
pub type SERVICE_STATUS_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: SERVICE_RUNS_IN_PROCESS,
}
pub const SERVICE_STOP: u32 = 32u32;
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = 1u32;
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = 3u32;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
impl Default for SERVICE_TABLE_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
impl Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: SERVICE_TRIGGER_TYPE,
    pub dwAction: SERVICE_TRIGGER_ACTION,
    pub pTriggerSubtype: *mut windows_sys::core::GUID,
    pub cDataItems: u32,
    pub pDataItems: *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
impl Default for SERVICE_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVICE_TRIGGER_ACTION = u32;
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = 1u32;
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 1u32;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 5u32;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 4u32;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 3u32;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: *mut SERVICE_TRIGGER,
    pub pReserved: *mut u8,
}
impl Default for SERVICE_TRIGGER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE,
    pub cbData: u32,
    pub pData: *mut u8,
}
impl Default for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = u32;
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: windows_sys::core::PCWSTR = windows_sys::core::w!("TriggerStarted");
pub type SERVICE_TRIGGER_TYPE = u32;
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = 20u32;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = 1u32;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = 3u32;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = 4u32;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = 5u32;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = 2u32;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = 6u32;
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = 80u32;
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = 96u32;
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = 48u32;
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = 16u32;
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = 32u32;
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = 0i32;
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = 1i32;
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = 0i32;
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = 1i32;
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = 0i32;
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = 0i32;
pub const USER_POLICY_PRESENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
