use crate::vk::aliases::*;
use crate::vk::bitflags::*;
use crate::vk::definitions::*;
use crate::vk::enums::*;
use crate::vk::platform_types::*;
use std::os::raw::*;
impl KhrSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 25u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    surface: SurfaceKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrSurfaceFn {
    pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pub get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    pub get_physical_device_surface_present_modes_khr:
        PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
}
unsafe impl Send for KhrSurfaceFn {}
unsafe impl Sync for KhrSurfaceFn {}
impl KhrSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            destroy_surface_khr: unsafe {
                unsafe extern "system" fn destroy_surface_khr(
                    _instance: Instance,
                    _surface: SurfaceKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_surface_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _surface: SurfaceKHR,
                    _p_supported: *mut Bool32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_capabilities_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_formats_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_formats_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_format_count: *mut u32,
                    _p_surface_formats: *mut SurfaceFormatKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_formats_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_formats_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_present_modes_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_present_mode_count: *mut u32,
                    _p_present_modes: *mut PresentModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_surface'"]
impl ObjectType {
    pub const SURFACE_KHR: Self = Self(1_000_000_000);
}
#[doc = "Generated from 'VK_KHR_surface'"]
impl Result {
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1_000_000_000);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1_000_000_001);
}
impl KhrSwapchainFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") }
    }
    pub const SPEC_VERSION: u32 = 70u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchain: *mut SwapchainKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut Image,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueuePresentKHR =
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: Device,
    surface: SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut Rect2D,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const AcquireNextImageInfoKHR,
    p_image_index: *mut u32,
) -> Result;
#[derive(Clone)]
pub struct KhrSwapchainFn {
    pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    pub queue_present_khr: PFN_vkQueuePresentKHR,
    pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
    pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
unsafe impl Send for KhrSwapchainFn {}
unsafe impl Sync for KhrSwapchainFn {}
impl KhrSwapchainFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_swapchain_khr: unsafe {
                unsafe extern "system" fn create_swapchain_khr(
                    _device: Device,
                    _p_create_info: *const SwapchainCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_swapchain: *mut SwapchainKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_swapchain_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_swapchain_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_swapchain_khr: unsafe {
                unsafe extern "system" fn destroy_swapchain_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_swapchain_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_swapchain_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_images_khr: unsafe {
                unsafe extern "system" fn get_swapchain_images_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_swapchain_image_count: *mut u32,
                    _p_swapchain_images: *mut Image,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_images_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_images_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image_khr: unsafe {
                unsafe extern "system" fn acquire_next_image_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _timeout: u64,
                    _semaphore: Semaphore,
                    _fence: Fence,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_present_khr: unsafe {
                unsafe extern "system" fn queue_present_khr(
                    _queue: Queue,
                    _p_present_info: *const PresentInfoKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_present_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_present_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_present_capabilities_khr: unsafe {
                unsafe extern "system" fn get_device_group_present_capabilities_khr(
                    _device: Device,
                    _p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_present_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_present_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                    _device: Device,
                    _surface: SurfaceKHR,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_present_rectangles_khr: unsafe {
                unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_rect_count: *mut u32,
                    _p_rects: *mut Rect2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_present_rectangles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_present_rectangles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image2_khr: unsafe {
                unsafe extern "system" fn acquire_next_image2_khr(
                    _device: Device,
                    _p_acquire_info: *const AcquireNextImageInfoKHR,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl ImageLayout {
    pub const PRESENT_SRC_KHR: Self = Self(1_000_001_002);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl ObjectType {
    pub const SWAPCHAIN_KHR: Self = Self(1_000_001_000);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl Result {
    pub const SUBOPTIMAL_KHR: Self = Self(1_000_001_003);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1_000_001_004);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl StructureType {
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_001_000);
    pub const PRESENT_INFO_KHR: Self = Self(1_000_001_001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1_000_060_007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_060_008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1_000_060_009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1_000_060_010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1_000_060_011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_060_012);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl SwapchainCreateFlagsKHR {
    #[doc = "Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT"]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(0b1);
    #[doc = "Swapchain is protected"]
    pub const PROTECTED: Self = Self(0b10);
}
impl KhrDisplayFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display\0") }
    }
    pub const SPEC_VERSION: u32 = 23u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_mode: *mut DisplayModeKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrDisplayFn {
    pub get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    pub get_physical_device_display_plane_properties_khr:
        PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
}
unsafe impl Send for KhrDisplayFn {}
unsafe impl Sync for KhrDisplayFn {}
impl KhrDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_display_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_display_plane_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_plane_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPlanePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_plane_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_plane_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_supported_displays_khr: unsafe {
                unsafe extern "system" fn get_display_plane_supported_displays_khr(
                    _physical_device: PhysicalDevice,
                    _plane_index: u32,
                    _p_display_count: *mut u32,
                    _p_displays: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_supported_displays_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_supported_displays_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_mode_properties_khr: unsafe {
                unsafe extern "system" fn get_display_mode_properties_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayModePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_mode_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayModePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_mode_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_display_mode_khr: unsafe {
                unsafe extern "system" fn create_display_mode_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_create_info: *const DisplayModeCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_mode: *mut DisplayModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_display_mode_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_display_mode_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_capabilities_khr: unsafe {
                unsafe extern "system" fn get_display_plane_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _mode: DisplayModeKHR,
                    _plane_index: u32,
                    _p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_display_plane_surface_khr: unsafe {
                unsafe extern "system" fn create_display_plane_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const DisplaySurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_display_plane_surface_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDisplayPlaneSurfaceKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_display_plane_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_display'"]
impl ObjectType {
    pub const DISPLAY_KHR: Self = Self(1_000_002_000);
    pub const DISPLAY_MODE_KHR: Self = Self(1_000_002_001);
}
#[doc = "Generated from 'VK_KHR_display'"]
impl StructureType {
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1_000_002_000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_002_001);
}
impl KhrDisplaySwapchainFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display_swapchain\0") }
    }
    pub const SPEC_VERSION: u32 = 10u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_create_infos: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchains: *mut SwapchainKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrDisplaySwapchainFn {
    pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
}
unsafe impl Send for KhrDisplaySwapchainFn {}
unsafe impl Sync for KhrDisplaySwapchainFn {}
impl KhrDisplaySwapchainFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_shared_swapchains_khr: unsafe {
                unsafe extern "system" fn create_shared_swapchains_khr(
                    _device: Device,
                    _swapchain_count: u32,
                    _p_create_infos: *const SwapchainCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_swapchains: *mut SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_shared_swapchains_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSharedSwapchainsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_shared_swapchains_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_display_swapchain'"]
impl Result {
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1_000_003_001);
}
#[doc = "Generated from 'VK_KHR_display_swapchain'"]
impl StructureType {
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1_000_003_000);
}
impl KhrXlibSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xlib_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
#[derive(Clone)]
pub struct KhrXlibSurfaceFn {
    pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    pub get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
unsafe impl Send for KhrXlibSurfaceFn {}
unsafe impl Sync for KhrXlibSurfaceFn {}
impl KhrXlibSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_xlib_surface_khr: unsafe {
                unsafe extern "system" fn create_xlib_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const XlibSurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_xlib_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_xlib_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_xlib_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_xlib_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _dpy: *mut Display,
                    _visual_id: VisualID,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_xlib_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_xlib_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_xlib_surface'"]
impl StructureType {
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_004_000);
}
impl KhrXcbSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
#[derive(Clone)]
pub struct KhrXcbSurfaceFn {
    pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    pub get_physical_device_xcb_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
}
unsafe impl Send for KhrXcbSurfaceFn {}
unsafe impl Sync for KhrXcbSurfaceFn {}
impl KhrXcbSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_xcb_surface_khr: unsafe {
                unsafe extern "system" fn create_xcb_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const XcbSurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_xcb_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_xcb_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_xcb_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_xcb_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _connection: *mut xcb_connection_t,
                    _visual_id: xcb_visualid_t,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_xcb_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_xcb_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_xcb_surface'"]
impl StructureType {
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_005_000);
}
impl KhrWaylandSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_wayland_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
)
    -> Bool32;
#[derive(Clone)]
pub struct KhrWaylandSurfaceFn {
    pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    pub get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}
unsafe impl Send for KhrWaylandSurfaceFn {}
unsafe impl Sync for KhrWaylandSurfaceFn {}
impl KhrWaylandSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_wayland_surface_khr: unsafe {
                unsafe extern "system" fn create_wayland_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const WaylandSurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_wayland_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_wayland_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_wayland_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_wayland_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _display: *mut wl_display,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_wayland_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_wayland_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_wayland_surface'"]
impl StructureType {
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_006_000);
}
impl KhrMirSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_mir_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct KhrMirSurfaceFn {}
unsafe impl Send for KhrMirSurfaceFn {}
unsafe impl Sync for KhrMirSurfaceFn {}
impl KhrMirSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrAndroidSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_android_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const AndroidSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrAndroidSurfaceFn {
    pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}
unsafe impl Send for KhrAndroidSurfaceFn {}
unsafe impl Sync for KhrAndroidSurfaceFn {}
impl KhrAndroidSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_android_surface_khr: unsafe {
                unsafe extern "system" fn create_android_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const AndroidSurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_android_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_android_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_android_surface'"]
impl StructureType {
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_008_000);
}
impl KhrWin32SurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
#[derive(Clone)]
pub struct KhrWin32SurfaceFn {
    pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    pub get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}
unsafe impl Send for KhrWin32SurfaceFn {}
unsafe impl Sync for KhrWin32SurfaceFn {}
impl KhrWin32SurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_win32_surface_khr: unsafe {
                unsafe extern "system" fn create_win32_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const Win32SurfaceCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_win32_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_win32_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_win32_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_win32_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_win32_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_win32_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_win32_surface'"]
impl StructureType {
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_009_000);
}
impl AndroidNativeBufferFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ANDROID_native_buffer\0") }
    }
    pub const SPEC_VERSION: u32 = 8u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainGrallocUsageANDROID = unsafe extern "system" fn(
    device: Device,
    format: Format,
    image_usage: ImageUsageFlags,
    gralloc_usage: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireImageANDROID = unsafe extern "system" fn(
    device: Device,
    image: Image,
    native_fence_fd: c_int,
    semaphore: Semaphore,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSignalReleaseImageANDROID = unsafe extern "system" fn(
    queue: Queue,
    wait_semaphore_count: u32,
    p_wait_semaphores: *const Semaphore,
    image: Image,
    p_native_fence_fd: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainGrallocUsage2ANDROID = unsafe extern "system" fn(
    device: Device,
    format: Format,
    image_usage: ImageUsageFlags,
    swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
    gralloc_consumer_usage: *mut u64,
    gralloc_producer_usage: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct AndroidNativeBufferFn {
    pub get_swapchain_gralloc_usage_android: PFN_vkGetSwapchainGrallocUsageANDROID,
    pub acquire_image_android: PFN_vkAcquireImageANDROID,
    pub queue_signal_release_image_android: PFN_vkQueueSignalReleaseImageANDROID,
    pub get_swapchain_gralloc_usage2_android: PFN_vkGetSwapchainGrallocUsage2ANDROID,
}
unsafe impl Send for AndroidNativeBufferFn {}
unsafe impl Sync for AndroidNativeBufferFn {}
impl AndroidNativeBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_swapchain_gralloc_usage_android: unsafe {
                unsafe extern "system" fn get_swapchain_gralloc_usage_android(
                    _device: Device,
                    _format: Format,
                    _image_usage: ImageUsageFlags,
                    _gralloc_usage: *mut c_int,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_gralloc_usage_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSwapchainGrallocUsageANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_gralloc_usage_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_image_android: unsafe {
                unsafe extern "system" fn acquire_image_android(
                    _device: Device,
                    _image: Image,
                    _native_fence_fd: c_int,
                    _semaphore: Semaphore,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_image_android)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireImageANDROID\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_image_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_signal_release_image_android: unsafe {
                unsafe extern "system" fn queue_signal_release_image_android(
                    _queue: Queue,
                    _wait_semaphore_count: u32,
                    _p_wait_semaphores: *const Semaphore,
                    _image: Image,
                    _p_native_fence_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_signal_release_image_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueSignalReleaseImageANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_signal_release_image_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_gralloc_usage2_android: unsafe {
                unsafe extern "system" fn get_swapchain_gralloc_usage2_android(
                    _device: Device,
                    _format: Format,
                    _image_usage: ImageUsageFlags,
                    _swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
                    _gralloc_consumer_usage: *mut u64,
                    _gralloc_producer_usage: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_gralloc_usage2_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSwapchainGrallocUsage2ANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_gralloc_usage2_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_ANDROID_native_buffer'"]
impl StructureType {
    pub const NATIVE_BUFFER_ANDROID: Self = Self(1_000_010_000);
    pub const SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID: Self = Self(1_000_010_001);
    pub const PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID: Self = Self(1_000_010_002);
}
impl ExtDebugReportFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_report\0") }
    }
    pub const SPEC_VERSION: u32 = 10u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_callback: *mut DebugReportCallbackEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
);
#[derive(Clone)]
pub struct ExtDebugReportFn {
    pub create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
    pub destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
    pub debug_report_message_ext: PFN_vkDebugReportMessageEXT,
}
unsafe impl Send for ExtDebugReportFn {}
unsafe impl Sync for ExtDebugReportFn {}
impl ExtDebugReportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_debug_report_callback_ext: unsafe {
                unsafe extern "system" fn create_debug_report_callback_ext(
                    _instance: Instance,
                    _p_create_info: *const DebugReportCallbackCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_callback: *mut DebugReportCallbackEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_debug_report_callback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDebugReportCallbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_debug_report_callback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_debug_report_callback_ext: unsafe {
                unsafe extern "system" fn destroy_debug_report_callback_ext(
                    _instance: Instance,
                    _callback: DebugReportCallbackEXT,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_debug_report_callback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugReportCallbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_debug_report_callback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            debug_report_message_ext: unsafe {
                unsafe extern "system" fn debug_report_message_ext(
                    _instance: Instance,
                    _flags: DebugReportFlagsEXT,
                    _object_type: DebugReportObjectTypeEXT,
                    _object: u64,
                    _location: usize,
                    _message_code: i32,
                    _p_layer_prefix: *const c_char,
                    _p_message: *const c_char,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_report_message_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDebugReportMessageEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    debug_report_message_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1_000_156_000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1_000_085_000);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl ObjectType {
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1_000_011_000);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl Result {
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self(-1_000_011_001);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl StructureType {
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1_000_011_000);
}
impl NvGlslShaderFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvGlslShaderFn {}
unsafe impl Send for NvGlslShaderFn {}
unsafe impl Sync for NvGlslShaderFn {}
impl NvGlslShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_glsl_shader'"]
impl Result {
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1_000_012_000);
}
impl ExtDepthRangeUnrestrictedFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_range_unrestricted\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthRangeUnrestrictedFn {}
unsafe impl Send for ExtDepthRangeUnrestrictedFn {}
unsafe impl Sync for ExtDepthRangeUnrestrictedFn {}
impl ExtDepthRangeUnrestrictedFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrSamplerMirrorClampToEdgeFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_sampler_mirror_clamp_to_edge\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrSamplerMirrorClampToEdgeFn {}
unsafe impl Send for KhrSamplerMirrorClampToEdgeFn {}
unsafe impl Sync for KhrSamplerMirrorClampToEdgeFn {}
impl KhrSamplerMirrorClampToEdgeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_sampler_mirror_clamp_to_edge'"]
impl SamplerAddressMode {
    #[doc = "Note that this defines what was previously a core enum, and so uses the 'value' attribute rather than 'offset', and does not have a suffix. This is a special case, and should not be repeated"]
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
    #[deprecated = "Alias introduced for consistency with extension suffixing rules"]
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self::MIRROR_CLAMP_TO_EDGE;
}
impl ImgFilterCubicFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_filter_cubic\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ImgFilterCubicFn {}
unsafe impl Send for ImgFilterCubicFn {}
unsafe impl Sync for ImgFilterCubicFn {}
impl ImgFilterCubicFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_IMG_filter_cubic'"]
impl Filter {
    pub const CUBIC_IMG: Self = Self(1_000_015_000);
}
#[doc = "Generated from 'VK_IMG_filter_cubic'"]
impl FormatFeatureFlags {
    #[doc = "Format can be filtered with VK_FILTER_CUBIC_IMG when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self(0b10_0000_0000_0000);
}
impl AmdExtension17Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_17\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension17Fn {}
unsafe impl Send for AmdExtension17Fn {}
unsafe impl Sync for AmdExtension17Fn {}
impl AmdExtension17Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension18Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_18\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension18Fn {}
unsafe impl Send for AmdExtension18Fn {}
unsafe impl Sync for AmdExtension18Fn {}
impl AmdExtension18Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdRasterizationOrderFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_rasterization_order\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdRasterizationOrderFn {}
unsafe impl Send for AmdRasterizationOrderFn {}
unsafe impl Sync for AmdRasterizationOrderFn {}
impl AmdRasterizationOrderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_rasterization_order'"]
impl StructureType {
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1_000_018_000);
}
impl AmdExtension20Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_20\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension20Fn {}
unsafe impl Send for AmdExtension20Fn {}
unsafe impl Sync for AmdExtension20Fn {}
impl AmdExtension20Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdShaderTrinaryMinmaxFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_trinary_minmax\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderTrinaryMinmaxFn {}
unsafe impl Send for AmdShaderTrinaryMinmaxFn {}
unsafe impl Sync for AmdShaderTrinaryMinmaxFn {}
impl AmdShaderTrinaryMinmaxFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdShaderExplicitVertexParameterFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_AMD_shader_explicit_vertex_parameter\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderExplicitVertexParameterFn {}
unsafe impl Send for AmdShaderExplicitVertexParameterFn {}
unsafe impl Sync for AmdShaderExplicitVertexParameterFn {}
impl AmdShaderExplicitVertexParameterFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtDebugMarkerFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_marker\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugMarkerObjectTagInfoEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: Device,
    p_name_info: *const DebugMarkerObjectNameInfoEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT,
);
#[derive(Clone)]
pub struct ExtDebugMarkerFn {
    pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
unsafe impl Send for ExtDebugMarkerFn {}
unsafe impl Sync for ExtDebugMarkerFn {}
impl ExtDebugMarkerFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            debug_marker_set_object_tag_ext: unsafe {
                unsafe extern "system" fn debug_marker_set_object_tag_ext(
                    _device: Device,
                    _p_tag_info: *const DebugMarkerObjectTagInfoEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_marker_set_object_tag_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDebugMarkerSetObjectTagEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    debug_marker_set_object_tag_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            debug_marker_set_object_name_ext: unsafe {
                unsafe extern "system" fn debug_marker_set_object_name_ext(
                    _device: Device,
                    _p_name_info: *const DebugMarkerObjectNameInfoEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_marker_set_object_name_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDebugMarkerSetObjectNameEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    debug_marker_set_object_name_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_begin_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_begin_ext(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const DebugMarkerMarkerInfoEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_begin_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerBeginEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_begin_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_end_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_end_ext(_command_buffer: CommandBuffer) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_end_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerEndEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_end_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_insert_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_insert_ext(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const DebugMarkerMarkerInfoEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_insert_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerInsertEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_insert_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_marker'"]
impl StructureType {
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1_000_022_000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1_000_022_001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1_000_022_002);
}
impl KhrVideoQueueFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_queue\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_profile: *const VideoProfileKHR,
    p_capabilities: *mut VideoCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session: *mut VideoSessionKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_video_session_memory_requirements_count: *mut u32,
    p_video_session_memory_requirements: *mut VideoGetMemoryPropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    video_session_bind_memory_count: u32,
    p_video_session_bind_memories: *const VideoBindMemoryKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const VideoBeginCodingInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_end_coding_info: *const VideoEndCodingInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_coding_control_info: *const VideoCodingControlInfoKHR,
);
#[derive(Clone)]
pub struct KhrVideoQueueFn {
    pub get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    pub get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
    pub create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    pub destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    pub get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    pub bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    pub create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    pub update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    pub destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    pub cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    pub cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
    pub cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
}
unsafe impl Send for KhrVideoQueueFn {}
unsafe impl Sync for KhrVideoQueueFn {}
impl KhrVideoQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_video_capabilities_khr: unsafe {
                unsafe extern "system" fn get_physical_device_video_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _p_video_profile: *const VideoProfileKHR,
                    _p_capabilities: *mut VideoCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_video_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceVideoCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_video_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_video_format_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_video_format_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
                    _p_video_format_property_count: *mut u32,
                    _p_video_format_properties: *mut VideoFormatPropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_video_format_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceVideoFormatPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_video_format_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_video_session_khr: unsafe {
                unsafe extern "system" fn create_video_session_khr(
                    _device: Device,
                    _p_create_info: *const VideoSessionCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_video_session: *mut VideoSessionKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_video_session_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateVideoSessionKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_video_session_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_video_session_khr: unsafe {
                unsafe extern "system" fn destroy_video_session_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_video_session_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyVideoSessionKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_video_session_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_video_session_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_video_session_memory_requirements_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _p_video_session_memory_requirements_count: *mut u32,
                    _p_video_session_memory_requirements: *mut VideoGetMemoryPropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_video_session_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetVideoSessionMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_video_session_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_video_session_memory_khr: unsafe {
                unsafe extern "system" fn bind_video_session_memory_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _video_session_bind_memory_count: u32,
                    _p_video_session_bind_memories: *const VideoBindMemoryKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_video_session_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBindVideoSessionMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    bind_video_session_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn create_video_session_parameters_khr(
                    _device: Device,
                    _p_create_info: *const VideoSessionParametersCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_video_session_parameters: *mut VideoSessionParametersKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            update_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn update_video_session_parameters_khr(
                    _device: Device,
                    _video_session_parameters: VideoSessionParametersKHR,
                    _p_update_info: *const VideoSessionParametersUpdateInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    update_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn destroy_video_session_parameters_khr(
                    _device: Device,
                    _video_session_parameters: VideoSessionParametersKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_begin_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_begin_info: *const VideoBeginCodingInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_video_coding_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginVideoCodingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_end_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_end_coding_info: *const VideoEndCodingInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_video_coding_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndVideoCodingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_control_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_control_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_coding_control_info: *const VideoCodingControlInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_control_video_coding_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdControlVideoCodingKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_control_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl ObjectType {
    #[doc = "VkVideoSessionKHR"]
    pub const VIDEO_SESSION_KHR: Self = Self(1_000_023_000);
    #[doc = "VkVideoSessionParametersKHR"]
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1_000_023_001);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl QueryResultFlags {
    pub const WITH_STATUS_KHR: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl QueryType {
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1_000_023_000);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl Result {
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_000);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_001);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_002);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_003);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_004);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_005);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl StructureType {
    pub const VIDEO_PROFILE_KHR: Self = Self(1_000_023_000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1_000_023_001);
    pub const VIDEO_PICTURE_RESOURCE_KHR: Self = Self(1_000_023_002);
    pub const VIDEO_GET_MEMORY_PROPERTIES_KHR: Self = Self(1_000_023_003);
    pub const VIDEO_BIND_MEMORY_KHR: Self = Self(1_000_023_004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1_000_023_005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_023_006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1_000_023_007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1_000_023_008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1_000_023_009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1_000_023_010);
    pub const VIDEO_REFERENCE_SLOT_KHR: Self = Self(1_000_023_011);
    pub const VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self(1_000_023_012);
    pub const VIDEO_PROFILES_KHR: Self = Self(1_000_023_013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1_000_023_014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1_000_023_015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR: Self = Self(1_000_023_016);
}
impl KhrVideoDecodeQueueFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_queue\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_frame_info: *const VideoDecodeInfoKHR,
);
#[derive(Clone)]
pub struct KhrVideoDecodeQueueFn {
    pub cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}
unsafe impl Send for KhrVideoDecodeQueueFn {}
unsafe impl Sync for KhrVideoDecodeQueueFn {}
impl KhrVideoDecodeQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_decode_video_khr: unsafe {
                unsafe extern "system" fn cmd_decode_video_khr(
                    _command_buffer: CommandBuffer,
                    _p_frame_info: *const VideoDecodeInfoKHR,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_decode_video_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDecodeVideoKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_decode_video_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl AccessFlags2 {
    pub const VIDEO_DECODE_READ_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl BufferUsageFlags {
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0b10_0000_0000_0000);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl FormatFeatureFlags {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl FormatFeatureFlags2 {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl ImageLayout {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1_000_024_000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1_000_024_001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1_000_024_002);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl ImageUsageFlags {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0b100_0000_0000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0b1000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl PipelineStageFlags2 {
    pub const VIDEO_DECODE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl QueueFlags {
    pub const VIDEO_DECODE_KHR: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl StructureType {
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1_000_024_000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1_000_024_001);
}
impl AmdGcnShaderFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gcn_shader\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdGcnShaderFn {}
unsafe impl Send for AmdGcnShaderFn {}
unsafe impl Sync for AmdGcnShaderFn {}
impl AmdGcnShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvDedicatedAllocationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvDedicatedAllocationFn {}
unsafe impl Send for NvDedicatedAllocationFn {}
unsafe impl Sync for NvDedicatedAllocationFn {}
impl NvDedicatedAllocationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_dedicated_allocation'"]
impl StructureType {
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1_000_026_000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1_000_026_001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1_000_026_002);
}
impl ExtExtension28Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_28\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension28Fn {}
unsafe impl Send for ExtExtension28Fn {}
unsafe impl Sync for ExtExtension28Fn {}
impl ExtExtension28Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtTransformFeedbackFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_transform_feedback\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);
#[derive(Clone)]
pub struct ExtTransformFeedbackFn {
    pub cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
    pub cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
    pub cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
    pub cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
    pub cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
    pub cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
}
unsafe impl Send for ExtTransformFeedbackFn {}
unsafe impl Sync for ExtTransformFeedbackFn {}
impl ExtTransformFeedbackFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_transform_feedback_buffers_ext: unsafe {
                unsafe extern "system" fn cmd_bind_transform_feedback_buffers_ext(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_transform_feedback_buffers_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindTransformFeedbackBuffersEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_transform_feedback_buffers_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_transform_feedback_ext: unsafe {
                unsafe extern "system" fn cmd_begin_transform_feedback_ext(
                    _command_buffer: CommandBuffer,
                    _first_counter_buffer: u32,
                    _counter_buffer_count: u32,
                    _p_counter_buffers: *const Buffer,
                    _p_counter_buffer_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_transform_feedback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginTransformFeedbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_transform_feedback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_transform_feedback_ext: unsafe {
                unsafe extern "system" fn cmd_end_transform_feedback_ext(
                    _command_buffer: CommandBuffer,
                    _first_counter_buffer: u32,
                    _counter_buffer_count: u32,
                    _p_counter_buffers: *const Buffer,
                    _p_counter_buffer_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_transform_feedback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndTransformFeedbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_transform_feedback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_query_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_begin_query_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _flags: QueryControlFlags,
                    _index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_query_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQueryIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_query_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_query_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_end_query_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_query_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQueryIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_query_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indirect_byte_count_ext: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_byte_count_ext(
                    _command_buffer: CommandBuffer,
                    _instance_count: u32,
                    _first_instance: u32,
                    _counter_buffer: Buffer,
                    _counter_buffer_offset: DeviceSize,
                    _counter_offset: u32,
                    _vertex_stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_byte_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndirectByteCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_byte_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl AccessFlags {
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl BufferUsageFlags {
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(0b1000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl PipelineStageFlags {
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl QueryType {
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1_000_028_004);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1_000_028_000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1_000_028_001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1_000_028_002);
}
impl NvxBinaryImportFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_binary_import\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_module: *mut CuModuleNVX,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_function: *mut CuFunctionNVX,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    module: CuModuleNVX,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    function: CuFunctionNVX,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCuLaunchKernelNVX =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_launch_info: *const CuLaunchInfoNVX);
#[derive(Clone)]
pub struct NvxBinaryImportFn {
    pub create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    pub create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    pub destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    pub destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    pub cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
}
unsafe impl Send for NvxBinaryImportFn {}
unsafe impl Sync for NvxBinaryImportFn {}
impl NvxBinaryImportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_cu_module_nvx: unsafe {
                unsafe extern "system" fn create_cu_module_nvx(
                    _device: Device,
                    _p_create_info: *const CuModuleCreateInfoNVX,
                    _p_allocator: *const AllocationCallbacks,
                    _p_module: *mut CuModuleNVX,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_cu_module_nvx)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCuModuleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cu_module_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_cu_function_nvx: unsafe {
                unsafe extern "system" fn create_cu_function_nvx(
                    _device: Device,
                    _p_create_info: *const CuFunctionCreateInfoNVX,
                    _p_allocator: *const AllocationCallbacks,
                    _p_function: *mut CuFunctionNVX,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_cu_function_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCuFunctionNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cu_function_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cu_module_nvx: unsafe {
                unsafe extern "system" fn destroy_cu_module_nvx(
                    _device: Device,
                    _module: CuModuleNVX,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cu_module_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuModuleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cu_module_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cu_function_nvx: unsafe {
                unsafe extern "system" fn destroy_cu_function_nvx(
                    _device: Device,
                    _function: CuFunctionNVX,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cu_function_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuFunctionNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cu_function_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_cu_launch_kernel_nvx: unsafe {
                unsafe extern "system" fn cmd_cu_launch_kernel_nvx(
                    _command_buffer: CommandBuffer,
                    _p_launch_info: *const CuLaunchInfoNVX,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_cu_launch_kernel_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCuLaunchKernelNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_cu_launch_kernel_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl DebugReportObjectTypeEXT {
    pub const CU_MODULE_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_NVX: Self = Self(1_000_029_001);
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl ObjectType {
    pub const CU_MODULE_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_NVX: Self = Self(1_000_029_001);
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl StructureType {
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1_000_029_001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1_000_029_002);
}
impl NvxImageViewHandleFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_image_view_handle\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u32;
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX,
) -> Result;
#[derive(Clone)]
pub struct NvxImageViewHandleFn {
    pub get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    pub get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
}
unsafe impl Send for NvxImageViewHandleFn {}
unsafe impl Sync for NvxImageViewHandleFn {}
impl NvxImageViewHandleFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_view_handle_nvx: unsafe {
                unsafe extern "system" fn get_image_view_handle_nvx(
                    _device: Device,
                    _p_info: *const ImageViewHandleInfoNVX,
                ) -> u32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_view_handle_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewHandleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_view_handle_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_view_address_nvx: unsafe {
                unsafe extern "system" fn get_image_view_address_nvx(
                    _device: Device,
                    _image_view: ImageView,
                    _p_properties: *mut ImageViewAddressPropertiesNVX,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_view_address_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewAddressNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_view_address_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NVX_image_view_handle'"]
impl StructureType {
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1_000_030_000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1_000_030_001);
}
impl AmdExtension32Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_32\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension32Fn {}
unsafe impl Send for AmdExtension32Fn {}
unsafe impl Sync for AmdExtension32Fn {}
impl AmdExtension32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension33Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_33\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension33Fn {}
unsafe impl Send for AmdExtension33Fn {}
unsafe impl Sync for AmdExtension33Fn {}
impl AmdExtension33Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdDrawIndirectCountFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_draw_indirect_count\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[derive(Clone)]
pub struct AmdDrawIndirectCountFn {
    pub cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCount,
}
unsafe impl Send for AmdDrawIndirectCountFn {}
unsafe impl Sync for AmdDrawIndirectCountFn {}
impl AmdDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_indirect_count_amd: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_count_amd(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_count_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_count_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect_count_amd: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect_count_amd(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect_count_amd)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountAMD\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect_count_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl AmdExtension35Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_35\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension35Fn {}
unsafe impl Send for AmdExtension35Fn {}
unsafe impl Sync for AmdExtension35Fn {}
impl AmdExtension35Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdNegativeViewportHeightFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_negative_viewport_height\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdNegativeViewportHeightFn {}
unsafe impl Send for AmdNegativeViewportHeightFn {}
unsafe impl Sync for AmdNegativeViewportHeightFn {}
impl AmdNegativeViewportHeightFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdGpuShaderHalfFloatFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_half_float\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdGpuShaderHalfFloatFn {}
unsafe impl Send for AmdGpuShaderHalfFloatFn {}
unsafe impl Sync for AmdGpuShaderHalfFloatFn {}
impl AmdGpuShaderHalfFloatFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdShaderBallotFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_ballot\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderBallotFn {}
unsafe impl Send for AmdShaderBallotFn {}
unsafe impl Sync for AmdShaderBallotFn {}
impl AmdShaderBallotFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtVideoEncodeH264Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h264\0") }
    }
    pub const SPEC_VERSION: u32 = 7u32;
}
#[derive(Clone)]
pub struct ExtVideoEncodeH264Fn {}
unsafe impl Send for ExtVideoEncodeH264Fn {}
unsafe impl Sync for ExtVideoEncodeH264Fn {}
impl ExtVideoEncodeH264Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_video_encode_h264'"]
impl StructureType {
    pub const VIDEO_ENCODE_H264_CAPABILITIES_EXT: Self = Self(1_000_038_000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1_000_038_001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1_000_038_002);
    pub const VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT: Self = Self(1_000_038_003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1_000_038_004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_EXT: Self = Self(1_000_038_005);
    pub const VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT: Self = Self(1_000_038_006);
    pub const VIDEO_ENCODE_H264_PROFILE_EXT: Self = Self(1_000_038_007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT: Self = Self(1_000_038_008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1_000_038_009);
    pub const VIDEO_ENCODE_H264_REFERENCE_LISTS_EXT: Self = Self(1_000_038_010);
}
#[doc = "Generated from 'VK_EXT_video_encode_h264'"]
impl VideoCodecOperationFlagsKHR {
    pub const ENCODE_H264_EXT: Self = Self(0b1_0000_0000_0000_0000);
}
impl ExtVideoEncodeH265Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h265\0") }
    }
    pub const SPEC_VERSION: u32 = 7u32;
}
#[derive(Clone)]
pub struct ExtVideoEncodeH265Fn {}
unsafe impl Send for ExtVideoEncodeH265Fn {}
unsafe impl Sync for ExtVideoEncodeH265Fn {}
impl ExtVideoEncodeH265Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_video_encode_h265'"]
impl StructureType {
    pub const VIDEO_ENCODE_H265_CAPABILITIES_EXT: Self = Self(1_000_039_000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1_000_039_001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1_000_039_002);
    pub const VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT: Self = Self(1_000_039_003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1_000_039_004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_EXT: Self = Self(1_000_039_005);
    pub const VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_EXT: Self = Self(1_000_039_006);
    pub const VIDEO_ENCODE_H265_PROFILE_EXT: Self = Self(1_000_039_007);
    pub const VIDEO_ENCODE_H265_REFERENCE_LISTS_EXT: Self = Self(1_000_039_008);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT: Self = Self(1_000_039_009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT: Self = Self(1_000_039_010);
}
#[doc = "Generated from 'VK_EXT_video_encode_h265'"]
impl VideoCodecOperationFlagsKHR {
    pub const ENCODE_H265_EXT: Self = Self(0b10_0000_0000_0000_0000);
}
impl ExtVideoDecodeH264Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h264\0") }
    }
    pub const SPEC_VERSION: u32 = 5u32;
}
#[derive(Clone)]
pub struct ExtVideoDecodeH264Fn {}
unsafe impl Send for ExtVideoDecodeH264Fn {}
unsafe impl Sync for ExtVideoDecodeH264Fn {}
impl ExtVideoDecodeH264Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_video_decode_h264'"]
impl StructureType {
    pub const VIDEO_DECODE_H264_CAPABILITIES_EXT: Self = Self(1_000_040_000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_EXT: Self = Self(1_000_040_001);
    pub const VIDEO_DECODE_H264_MVC_EXT: Self = Self(1_000_040_002);
    pub const VIDEO_DECODE_H264_PROFILE_EXT: Self = Self(1_000_040_003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1_000_040_004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1_000_040_005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1_000_040_006);
}
#[doc = "Generated from 'VK_EXT_video_decode_h264'"]
impl VideoCodecOperationFlagsKHR {
    pub const DECODE_H264_EXT: Self = Self(0b1);
}
impl AmdTextureGatherBiasLodFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_texture_gather_bias_lod\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdTextureGatherBiasLodFn {}
unsafe impl Send for AmdTextureGatherBiasLodFn {}
unsafe impl Sync for AmdTextureGatherBiasLodFn {}
impl AmdTextureGatherBiasLodFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_texture_gather_bias_lod'"]
impl StructureType {
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1_000_041_000);
}
impl AmdShaderInfoFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_info\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut c_void,
) -> Result;
#[derive(Clone)]
pub struct AmdShaderInfoFn {
    pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
unsafe impl Send for AmdShaderInfoFn {}
unsafe impl Sync for AmdShaderInfoFn {}
impl AmdShaderInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_shader_info_amd: unsafe {
                unsafe extern "system" fn get_shader_info_amd(
                    _device: Device,
                    _pipeline: Pipeline,
                    _shader_stage: ShaderStageFlags,
                    _info_type: ShaderInfoTypeAMD,
                    _p_info_size: *mut usize,
                    _p_info: *mut c_void,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_shader_info_amd)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetShaderInfoAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    get_shader_info_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl AmdExtension44Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_44\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension44Fn {}
unsafe impl Send for AmdExtension44Fn {}
unsafe impl Sync for AmdExtension44Fn {}
impl AmdExtension44Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrDynamicRenderingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dynamic_rendering\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_rendering_info: *const RenderingInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct KhrDynamicRenderingFn {
    pub cmd_begin_rendering_khr: PFN_vkCmdBeginRendering,
    pub cmd_end_rendering_khr: PFN_vkCmdEndRendering,
}
unsafe impl Send for KhrDynamicRenderingFn {}
unsafe impl Sync for KhrDynamicRenderingFn {}
impl KhrDynamicRenderingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_begin_rendering_khr: unsafe {
                unsafe extern "system" fn cmd_begin_rendering_khr(
                    _command_buffer: CommandBuffer,
                    _p_rendering_info: *const RenderingInfo,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_rendering_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_rendering_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_rendering_khr: unsafe {
                unsafe extern "system" fn cmd_end_rendering_khr(_command_buffer: CommandBuffer) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_rendering_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_rendering_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl AttachmentStoreOp {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl PipelineCreateFlags {
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl StructureType {
    pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self =
        Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1_000_044_006);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1_000_044_007);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1_000_044_008);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1_000_044_009);
}
impl AmdExtension46Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_46\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension46Fn {}
unsafe impl Send for AmdExtension46Fn {}
unsafe impl Sync for AmdExtension46Fn {}
impl AmdExtension46Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdShaderImageLoadStoreLodFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_image_load_store_lod\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderImageLoadStoreLodFn {}
unsafe impl Send for AmdShaderImageLoadStoreLodFn {}
unsafe impl Sync for AmdShaderImageLoadStoreLodFn {}
impl AmdShaderImageLoadStoreLodFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvxExtension48Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_extension_48\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvxExtension48Fn {}
unsafe impl Send for NvxExtension48Fn {}
unsafe impl Sync for NvxExtension48Fn {}
impl NvxExtension48Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension49Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_49\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension49Fn {}
unsafe impl Send for GoogleExtension49Fn {}
unsafe impl Sync for GoogleExtension49Fn {}
impl GoogleExtension49Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpStreamDescriptorSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_stream_descriptor_surface\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct GgpStreamDescriptorSurfaceFn {
    pub create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
}
unsafe impl Send for GgpStreamDescriptorSurfaceFn {}
unsafe impl Sync for GgpStreamDescriptorSurfaceFn {}
impl GgpStreamDescriptorSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_stream_descriptor_surface_ggp: unsafe {
                unsafe extern "system" fn create_stream_descriptor_surface_ggp(
                    _instance: Instance,
                    _p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_stream_descriptor_surface_ggp)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateStreamDescriptorSurfaceGGP\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_stream_descriptor_surface_ggp
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_GGP_stream_descriptor_surface'"]
impl StructureType {
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1_000_049_000);
}
impl NvCornerSampledImageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_corner_sampled_image\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvCornerSampledImageFn {}
unsafe impl Send for NvCornerSampledImageFn {}
unsafe impl Sync for NvCornerSampledImageFn {}
impl NvCornerSampledImageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_corner_sampled_image'"]
impl ImageCreateFlags {
    pub const CORNER_SAMPLED_NV: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_corner_sampled_image'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1_000_050_000);
}
impl NvExtension52Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_52\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension52Fn {}
unsafe impl Send for NvExtension52Fn {}
unsafe impl Sync for NvExtension52Fn {}
impl NvExtension52Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_extension_52'"]
impl PipelineShaderStageCreateFlags {
    pub const RESERVED_2_NV: Self = Self(0b100);
}
#[doc = "Generated from 'VK_NV_extension_52'"]
impl ShaderModuleCreateFlags {
    pub const RESERVED_0_NV: Self = Self(0b1);
}
impl NvExtension53Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_53\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension53Fn {}
unsafe impl Send for NvExtension53Fn {}
unsafe impl Sync for NvExtension53Fn {}
impl NvExtension53Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrMultiviewFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_multiview\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrMultiviewFn {}
unsafe impl Send for KhrMultiviewFn {}
unsafe impl Sync for KhrMultiviewFn {}
impl KhrMultiviewFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_multiview'"]
impl DependencyFlags {
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
}
#[doc = "Generated from 'VK_KHR_multiview'"]
impl StructureType {
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
}
impl ImgFormatPvrtcFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_format_pvrtc\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ImgFormatPvrtcFn {}
unsafe impl Send for ImgFormatPvrtcFn {}
unsafe impl Sync for ImgFormatPvrtcFn {}
impl ImgFormatPvrtcFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_IMG_format_pvrtc'"]
impl Format {
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_007);
}
impl NvExternalMemoryCapabilitiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_capabilities\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryCapabilitiesFn {
    pub get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}
unsafe impl Send for NvExternalMemoryCapabilitiesFn {}
unsafe impl Sync for NvExternalMemoryCapabilitiesFn {}
impl NvExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_image_format_properties_nv: unsafe {
                unsafe extern "system" fn get_physical_device_external_image_format_properties_nv(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _tiling: ImageTiling,
                    _usage: ImageUsageFlags,
                    _flags: ImageCreateFlags,
                    _external_handle_type: ExternalMemoryHandleTypeFlagsNV,
                    _p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_image_format_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_image_format_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl NvExternalMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvExternalMemoryFn {}
unsafe impl Send for NvExternalMemoryFn {}
unsafe impl Sync for NvExternalMemoryFn {}
impl NvExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_external_memory'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1_000_056_000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1_000_056_001);
}
impl NvExternalMemoryWin32Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_win32\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryWin32Fn {
    pub get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
}
unsafe impl Send for NvExternalMemoryWin32Fn {}
unsafe impl Sync for NvExternalMemoryWin32Fn {}
impl NvExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_win32_handle_nv: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_nv(
                    _device: Device,
                    _memory: DeviceMemory,
                    _handle_type: ExternalMemoryHandleTypeFlagsNV,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_external_memory_win32'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1_000_057_000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1_000_057_001);
}
impl NvWin32KeyedMutexFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_win32_keyed_mutex\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvWin32KeyedMutexFn {}
unsafe impl Send for NvWin32KeyedMutexFn {}
unsafe impl Sync for NvWin32KeyedMutexFn {}
impl NvWin32KeyedMutexFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_win32_keyed_mutex'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1_000_058_000);
}
impl KhrGetPhysicalDeviceProperties2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_get_physical_device_properties2\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
);
#[derive(Clone)]
pub struct KhrGetPhysicalDeviceProperties2Fn {
    pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_queue_family_properties2_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_sparse_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
}
unsafe impl Send for KhrGetPhysicalDeviceProperties2Fn {}
unsafe impl Sync for KhrGetPhysicalDeviceProperties2Fn {}
impl KhrGetPhysicalDeviceProperties2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_features2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_features2_khr(
                    _physical_device: PhysicalDevice,
                    _p_features: *mut PhysicalDeviceFeatures2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_features2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFeatures2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_features2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_properties: *mut PhysicalDeviceProperties2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _p_format_properties: *mut FormatProperties2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_image_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_image_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
                    _p_image_format_properties: *mut ImageFormatProperties2,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_image_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_image_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_queue_family_property_count: *mut u32,
                    _p_queue_family_properties: *mut QueueFamilyProperties2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_memory_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_memory_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_memory_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_memory_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_sparse_image_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_sparse_image_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
                    _p_property_count: *mut u32,
                    _p_properties: *mut SparseImageFormatProperties2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_sparse_image_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_sparse_image_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_physical_device_properties2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self =
        Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
}
impl KhrDeviceGroupFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[derive(Clone)]
pub struct KhrDeviceGroupFn {
    pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMask,
    pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBase,
    pub get_device_group_present_capabilities_khr:
        crate::vk::PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr:
        crate::vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub get_physical_device_present_rectangles_khr:
        crate::vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR,
    pub acquire_next_image2_khr: crate::vk::PFN_vkAcquireNextImage2KHR,
}
unsafe impl Send for KhrDeviceGroupFn {}
unsafe impl Sync for KhrDeviceGroupFn {}
impl KhrDeviceGroupFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_group_peer_memory_features_khr: unsafe {
                unsafe extern "system" fn get_device_group_peer_memory_features_khr(
                    _device: Device,
                    _heap_index: u32,
                    _local_device_index: u32,
                    _remote_device_index: u32,
                    _p_peer_memory_features: *mut PeerMemoryFeatureFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_peer_memory_features_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_peer_memory_features_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_device_mask_khr: unsafe {
                unsafe extern "system" fn cmd_set_device_mask_khr(
                    _command_buffer: CommandBuffer,
                    _device_mask: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_device_mask_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMaskKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_device_mask_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_dispatch_base_khr: unsafe {
                unsafe extern "system" fn cmd_dispatch_base_khr(
                    _command_buffer: CommandBuffer,
                    _base_group_x: u32,
                    _base_group_y: u32,
                    _base_group_z: u32,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_base_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBaseKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_base_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_present_capabilities_khr: unsafe {
                unsafe extern "system" fn get_device_group_present_capabilities_khr(
                    _device: Device,
                    _p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_present_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_present_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                    _device: Device,
                    _surface: SurfaceKHR,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_present_rectangles_khr: unsafe {
                unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_rect_count: *mut u32,
                    _p_rects: *mut Rect2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_present_rectangles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_present_rectangles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image2_khr: unsafe {
                unsafe extern "system" fn acquire_next_image2_khr(
                    _device: Device,
                    _p_acquire_info: *const AcquireNextImageInfoKHR,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl DependencyFlags {
    pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl ImageCreateFlags {
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl MemoryAllocateFlags {
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl PeerMemoryFeatureFlags {
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl PipelineCreateFlags {
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl StructureType {
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
}
impl ExtValidationFlagsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_flags\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtValidationFlagsFn {}
unsafe impl Send for ExtValidationFlagsFn {}
unsafe impl Sync for ExtValidationFlagsFn {}
impl ExtValidationFlagsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_validation_flags'"]
impl StructureType {
    pub const VALIDATION_FLAGS_EXT: Self = Self(1_000_061_000);
}
impl NnViSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NN_vi_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ViSurfaceCreateInfoNN,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct NnViSurfaceFn {
    pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
unsafe impl Send for NnViSurfaceFn {}
unsafe impl Sync for NnViSurfaceFn {}
impl NnViSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_vi_surface_nn: unsafe {
                unsafe extern "system" fn create_vi_surface_nn(
                    _instance: Instance,
                    _p_create_info: *const ViSurfaceCreateInfoNN,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_vi_surface_nn)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateViSurfaceNN\0");
                let val = _f(cname);
                if val.is_null() {
                    create_vi_surface_nn
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NN_vi_surface'"]
impl StructureType {
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1_000_062_000);
}
impl KhrShaderDrawParametersFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_draw_parameters\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderDrawParametersFn {}
unsafe impl Send for KhrShaderDrawParametersFn {}
unsafe impl Sync for KhrShaderDrawParametersFn {}
impl KhrShaderDrawParametersFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderSubgroupBallotFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_ballot\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderSubgroupBallotFn {}
unsafe impl Send for ExtShaderSubgroupBallotFn {}
unsafe impl Sync for ExtShaderSubgroupBallotFn {}
impl ExtShaderSubgroupBallotFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderSubgroupVoteFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_vote\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderSubgroupVoteFn {}
unsafe impl Send for ExtShaderSubgroupVoteFn {}
unsafe impl Sync for ExtShaderSubgroupVoteFn {}
impl ExtShaderSubgroupVoteFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtTextureCompressionAstcHdrFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_texture_compression_astc_hdr\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtTextureCompressionAstcHdrFn {}
unsafe impl Send for ExtTextureCompressionAstcHdrFn {}
unsafe impl Sync for ExtTextureCompressionAstcHdrFn {}
impl ExtTextureCompressionAstcHdrFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_texture_compression_astc_hdr'"]
impl Format {
    pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_4X4_SFLOAT_BLOCK;
    pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X4_SFLOAT_BLOCK;
    pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X5_SFLOAT_BLOCK;
    pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X5_SFLOAT_BLOCK;
    pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X6_SFLOAT_BLOCK;
    pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X5_SFLOAT_BLOCK;
    pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X6_SFLOAT_BLOCK;
    pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X8_SFLOAT_BLOCK;
    pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X5_SFLOAT_BLOCK;
    pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X6_SFLOAT_BLOCK;
    pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X8_SFLOAT_BLOCK;
    pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X10_SFLOAT_BLOCK;
    pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X10_SFLOAT_BLOCK;
    pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X12_SFLOAT_BLOCK;
}
#[doc = "Generated from 'VK_EXT_texture_compression_astc_hdr'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
}
impl ExtAstcDecodeModeFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_astc_decode_mode\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtAstcDecodeModeFn {}
unsafe impl Send for ExtAstcDecodeModeFn {}
unsafe impl Sync for ExtAstcDecodeModeFn {}
impl ExtAstcDecodeModeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_astc_decode_mode'"]
impl StructureType {
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1_000_067_000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1_000_067_001);
}
impl ImgExtension69Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_extension_69\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ImgExtension69Fn {}
unsafe impl Send for ImgExtension69Fn {}
unsafe impl Sync for ImgExtension69Fn {}
impl ImgExtension69Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrMaintenance1Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance1\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
#[derive(Clone)]
pub struct KhrMaintenance1Fn {
    pub trim_command_pool_khr: PFN_vkTrimCommandPool,
}
unsafe impl Send for KhrMaintenance1Fn {}
unsafe impl Sync for KhrMaintenance1Fn {}
impl KhrMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            trim_command_pool_khr: unsafe {
                unsafe extern "system" fn trim_command_pool_khr(
                    _device: Device,
                    _command_pool: CommandPool,
                    _flags: CommandPoolTrimFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(trim_command_pool_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    trim_command_pool_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl FormatFeatureFlags {
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl ImageCreateFlags {
    pub const TYPE_2D_ARRAY_COMPATIBLE_KHR: Self = Self::TYPE_2D_ARRAY_COMPATIBLE;
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl Result {
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
}
impl KhrDeviceGroupCreationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group_creation\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> Result;
#[derive(Clone)]
pub struct KhrDeviceGroupCreationFn {
    pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroups,
}
unsafe impl Send for KhrDeviceGroupCreationFn {}
unsafe impl Sync for KhrDeviceGroupCreationFn {}
impl KhrDeviceGroupCreationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_physical_device_groups_khr: unsafe {
                unsafe extern "system" fn enumerate_physical_device_groups_khr(
                    _instance: Instance,
                    _p_physical_device_group_count: *mut u32,
                    _p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_physical_device_groups_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceGroupsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_device_groups_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_device_group_creation'"]
impl MemoryHeapFlags {
    pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
}
#[doc = "Generated from 'VK_KHR_device_group_creation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
}
impl KhrExternalMemoryCapabilitiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_external_memory_capabilities\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
);
#[derive(Clone)]
pub struct KhrExternalMemoryCapabilitiesFn {
    pub get_physical_device_external_buffer_properties_khr:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
}
unsafe impl Send for KhrExternalMemoryCapabilitiesFn {}
unsafe impl Sync for KhrExternalMemoryCapabilitiesFn {}
impl KhrExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_buffer_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_buffer_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
                    _p_external_buffer_properties: *mut ExternalBufferProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_buffer_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_buffer_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
}
impl KhrExternalMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalMemoryFn {}
unsafe impl Send for KhrExternalMemoryFn {}
unsafe impl Sync for KhrExternalMemoryFn {}
impl KhrExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_external_memory'"]
impl Result {
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
}
#[doc = "Generated from 'VK_KHR_external_memory'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self =
        Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
}
impl KhrExternalMemoryWin32Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_win32\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalMemoryWin32Fn {
    pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
unsafe impl Send for KhrExternalMemoryWin32Fn {}
unsafe impl Sync for KhrExternalMemoryWin32Fn {}
impl KhrExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_win32_handle_properties_khr: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_properties_khr(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _handle: HANDLE,
                    _p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryWin32HandlePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_win32'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1_000_073_002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_003);
}
impl KhrExternalMemoryFdFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_fd\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR,
    p_fd: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalMemoryFdFn {
    pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
unsafe impl Send for KhrExternalMemoryFdFn {}
unsafe impl Sync for KhrExternalMemoryFdFn {}
impl KhrExternalMemoryFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_fd_khr: unsafe {
                unsafe extern "system" fn get_memory_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const MemoryGetFdInfoKHR,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_memory_fd_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_fd_properties_khr: unsafe {
                unsafe extern "system" fn get_memory_fd_properties_khr(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _fd: c_int,
                    _p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_fd_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryFdPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_fd_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_fd'"]
impl StructureType {
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1_000_074_000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1_000_074_001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1_000_074_002);
}
impl KhrWin32KeyedMutexFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_keyed_mutex\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrWin32KeyedMutexFn {}
unsafe impl Send for KhrWin32KeyedMutexFn {}
unsafe impl Sync for KhrWin32KeyedMutexFn {}
impl KhrWin32KeyedMutexFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_win32_keyed_mutex'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1_000_075_000);
}
impl KhrExternalSemaphoreCapabilitiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_external_semaphore_capabilities\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
#[derive(Clone)]
pub struct KhrExternalSemaphoreCapabilitiesFn {
    pub get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
unsafe impl Send for KhrExternalSemaphoreCapabilitiesFn {}
unsafe impl Sync for KhrExternalSemaphoreCapabilitiesFn {}
impl KhrExternalSemaphoreCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_semaphore_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_semaphore_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
                    _p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_semaphore_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_semaphore_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
}
impl KhrExternalSemaphoreFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalSemaphoreFn {}
unsafe impl Send for KhrExternalSemaphoreFn {}
unsafe impl Sync for KhrExternalSemaphoreFn {}
impl KhrExternalSemaphoreFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore'"]
impl SemaphoreImportFlags {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
#[doc = "Generated from 'VK_KHR_external_semaphore'"]
impl StructureType {
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
}
impl KhrExternalSemaphoreWin32Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_win32\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalSemaphoreWin32Fn {
    pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}
unsafe impl Send for KhrExternalSemaphoreWin32Fn {}
unsafe impl Sync for KhrExternalSemaphoreWin32Fn {}
impl KhrExternalSemaphoreWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_win32_handle_khr: unsafe {
                unsafe extern "system" fn import_semaphore_win32_handle_khr(
                    _device: Device,
                    _p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_semaphore_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_win32'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1_000_078_002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_003);
}
impl KhrExternalSemaphoreFdFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_fd\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR,
    p_fd: *mut c_int,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalSemaphoreFdFn {
    pub import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
    pub get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
}
unsafe impl Send for KhrExternalSemaphoreFdFn {}
unsafe impl Sync for KhrExternalSemaphoreFdFn {}
impl KhrExternalSemaphoreFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_fd_khr: unsafe {
                unsafe extern "system" fn import_semaphore_fd_khr(
                    _device: Device,
                    _p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_fd_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkImportSemaphoreFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_fd_khr: unsafe {
                unsafe extern "system" fn get_semaphore_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const SemaphoreGetFdInfoKHR,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_semaphore_fd_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_fd'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1_000_079_000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1_000_079_001);
}
impl KhrPushDescriptorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_push_descriptor\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    p_data: *const c_void,
);
#[derive(Clone)]
pub struct KhrPushDescriptorFn {
    pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
    pub cmd_push_descriptor_set_with_template_khr: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
unsafe impl Send for KhrPushDescriptorFn {}
unsafe impl Sync for KhrPushDescriptorFn {}
impl KhrPushDescriptorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_push_descriptor_set_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_khr(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _set: u32,
                    _descriptor_write_count: u32,
                    _p_descriptor_writes: *const WriteDescriptorSet,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSetKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                    _command_buffer: CommandBuffer,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _layout: PipelineLayout,
                    _set: u32,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl DescriptorSetLayoutCreateFlags {
    #[doc = "Descriptors are pushed via flink:vkCmdPushDescriptorSetKHR"]
    pub const PUSH_DESCRIPTOR_KHR: Self = Self(0b1);
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl DescriptorUpdateTemplateType {
    #[doc = "Create descriptor update template for pushed descriptor updates"]
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1_000_080_000);
}
impl ExtConditionalRenderingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conditional_rendering\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct ExtConditionalRenderingFn {
    pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}
unsafe impl Send for ExtConditionalRenderingFn {}
unsafe impl Sync for ExtConditionalRenderingFn {}
impl ExtConditionalRenderingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_begin_conditional_rendering_ext: unsafe {
                unsafe extern "system" fn cmd_begin_conditional_rendering_ext(
                    _command_buffer: CommandBuffer,
                    _p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_conditional_rendering_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginConditionalRenderingEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_conditional_rendering_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_conditional_rendering_ext: unsafe {
                unsafe extern "system" fn cmd_end_conditional_rendering_ext(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_conditional_rendering_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndConditionalRenderingEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_conditional_rendering_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl AccessFlags {
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl BufferUsageFlags {
    #[doc = "Specifies the buffer can be used as predicate in conditional rendering"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl PipelineStageFlags {
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl StructureType {
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1_000_081_000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1_000_081_001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1_000_081_002);
}
impl KhrShaderFloat16Int8Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float16_int8\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderFloat16Int8Fn {}
unsafe impl Send for KhrShaderFloat16Int8Fn {}
unsafe impl Sync for KhrShaderFloat16Int8Fn {}
impl KhrShaderFloat16Int8Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_float16_int8'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
}
impl Khr16bitStorageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_16bit_storage\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Khr16bitStorageFn {}
unsafe impl Send for Khr16bitStorageFn {}
unsafe impl Sync for Khr16bitStorageFn {}
impl Khr16bitStorageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_16bit_storage'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
}
impl KhrIncrementalPresentFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_incremental_present\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct KhrIncrementalPresentFn {}
unsafe impl Send for KhrIncrementalPresentFn {}
unsafe impl Sync for KhrIncrementalPresentFn {}
impl KhrIncrementalPresentFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_incremental_present'"]
impl StructureType {
    pub const PRESENT_REGIONS_KHR: Self = Self(1_000_084_000);
}
impl KhrDescriptorUpdateTemplateFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_descriptor_update_template\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const c_void,
);
#[derive(Clone)]
pub struct KhrDescriptorUpdateTemplateFn {
    pub create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplate,
    pub update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplate,
    pub cmd_push_descriptor_set_with_template_khr:
        crate::vk::PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
unsafe impl Send for KhrDescriptorUpdateTemplateFn {}
unsafe impl Sync for KhrDescriptorUpdateTemplateFn {}
impl KhrDescriptorUpdateTemplateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_descriptor_update_template_khr: unsafe {
                unsafe extern "system" fn create_descriptor_update_template_khr(
                    _device: Device,
                    _p_create_info: *const DescriptorUpdateTemplateCreateInfo,
                    _p_allocator: *const AllocationCallbacks,
                    _p_descriptor_update_template: *mut DescriptorUpdateTemplate,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_descriptor_update_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorUpdateTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_descriptor_update_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_descriptor_update_template_khr: unsafe {
                unsafe extern "system" fn destroy_descriptor_update_template_khr(
                    _device: Device,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_descriptor_update_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorUpdateTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_descriptor_update_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            update_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn update_descriptor_set_with_template_khr(
                    _device: Device,
                    _descriptor_set: DescriptorSet,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    update_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                    _command_buffer: CommandBuffer,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _layout: PipelineLayout,
                    _set: u32,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl DebugReportObjectTypeEXT {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl ObjectType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl StructureType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self =
        Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
}
impl NvxDeviceGeneratedCommandsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_device_generated_commands\0")
        }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct NvxDeviceGeneratedCommandsFn {}
unsafe impl Send for NvxDeviceGeneratedCommandsFn {}
unsafe impl Sync for NvxDeviceGeneratedCommandsFn {}
impl NvxDeviceGeneratedCommandsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvClipSpaceWScalingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_clip_space_w_scaling\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const ViewportWScalingNV,
);
#[derive(Clone)]
pub struct NvClipSpaceWScalingFn {
    pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}
unsafe impl Send for NvClipSpaceWScalingFn {}
unsafe impl Sync for NvClipSpaceWScalingFn {}
impl NvClipSpaceWScalingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_viewport_w_scaling_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_w_scaling_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_viewport_w_scalings: *const ViewportWScalingNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_w_scaling_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWScalingNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_w_scaling_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_clip_space_w_scaling'"]
impl DynamicState {
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1_000_087_000);
}
#[doc = "Generated from 'VK_NV_clip_space_w_scaling'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1_000_087_000);
}
impl ExtDirectModeDisplayFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_direct_mode_display\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseDisplayEXT =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[derive(Clone)]
pub struct ExtDirectModeDisplayFn {
    pub release_display_ext: PFN_vkReleaseDisplayEXT,
}
unsafe impl Send for ExtDirectModeDisplayFn {}
unsafe impl Sync for ExtDirectModeDisplayFn {}
impl ExtDirectModeDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            release_display_ext: unsafe {
                unsafe extern "system" fn release_display_ext(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(release_display_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkReleaseDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    release_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtAcquireXlibDisplayFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_xlib_display\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    display: DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    rr_output: RROutput,
    p_display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtAcquireXlibDisplayFn {
    pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
}
unsafe impl Send for ExtAcquireXlibDisplayFn {}
unsafe impl Sync for ExtAcquireXlibDisplayFn {}
impl ExtAcquireXlibDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_xlib_display_ext: unsafe {
                unsafe extern "system" fn acquire_xlib_display_ext(
                    _physical_device: PhysicalDevice,
                    _dpy: *mut Display,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_xlib_display_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireXlibDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_xlib_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_rand_r_output_display_ext: unsafe {
                unsafe extern "system" fn get_rand_r_output_display_ext(
                    _physical_device: PhysicalDevice,
                    _dpy: *mut Display,
                    _rr_output: RROutput,
                    _p_display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_rand_r_output_display_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRandROutputDisplayEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_rand_r_output_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtDisplaySurfaceCounterFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_surface_counter\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> Result;
#[derive(Clone)]
pub struct ExtDisplaySurfaceCounterFn {
    pub get_physical_device_surface_capabilities2_ext:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}
unsafe impl Send for ExtDisplaySurfaceCounterFn {}
unsafe impl Sync for ExtDisplaySurfaceCounterFn {}
impl ExtDisplaySurfaceCounterFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_capabilities2_ext: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities2_ext(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_capabilities: *mut SurfaceCapabilities2EXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_display_surface_counter'"]
impl StructureType {
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1_000_090_000);
}
impl ExtDisplayControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_control\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    p_counter_value: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct ExtDisplayControlFn {
    pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
}
unsafe impl Send for ExtDisplayControlFn {}
unsafe impl Sync for ExtDisplayControlFn {}
impl ExtDisplayControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            display_power_control_ext: unsafe {
                unsafe extern "system" fn display_power_control_ext(
                    _device: Device,
                    _display: DisplayKHR,
                    _p_display_power_info: *const DisplayPowerInfoEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(display_power_control_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDisplayPowerControlEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    display_power_control_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            register_device_event_ext: unsafe {
                unsafe extern "system" fn register_device_event_ext(
                    _device: Device,
                    _p_device_event_info: *const DeviceEventInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_fence: *mut Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(register_device_event_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkRegisterDeviceEventEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    register_device_event_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            register_display_event_ext: unsafe {
                unsafe extern "system" fn register_display_event_ext(
                    _device: Device,
                    _display: DisplayKHR,
                    _p_display_event_info: *const DisplayEventInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_fence: *mut Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(register_display_event_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkRegisterDisplayEventEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    register_display_event_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_counter_ext: unsafe {
                unsafe extern "system" fn get_swapchain_counter_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _counter: SurfaceCounterFlagsEXT,
                    _p_counter_value: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_counter_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainCounterEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_counter_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_display_control'"]
impl StructureType {
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1_000_091_000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1_000_091_001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1_000_091_002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1_000_091_003);
}
impl GoogleDisplayTimingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_display_timing\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_presentation_timing_count: *mut u32,
    p_presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> Result;
#[derive(Clone)]
pub struct GoogleDisplayTimingFn {
    pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
unsafe impl Send for GoogleDisplayTimingFn {}
unsafe impl Sync for GoogleDisplayTimingFn {}
impl GoogleDisplayTimingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_refresh_cycle_duration_google: unsafe {
                unsafe extern "system" fn get_refresh_cycle_duration_google(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_refresh_cycle_duration_google)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRefreshCycleDurationGOOGLE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_refresh_cycle_duration_google
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_past_presentation_timing_google: unsafe {
                unsafe extern "system" fn get_past_presentation_timing_google(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_presentation_timing_count: *mut u32,
                    _p_presentation_timings: *mut PastPresentationTimingGOOGLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_past_presentation_timing_google)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPastPresentationTimingGOOGLE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_past_presentation_timing_google
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_GOOGLE_display_timing'"]
impl StructureType {
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1_000_092_000);
}
impl NvSampleMaskOverrideCoverageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_NV_sample_mask_override_coverage\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvSampleMaskOverrideCoverageFn {}
unsafe impl Send for NvSampleMaskOverrideCoverageFn {}
unsafe impl Sync for NvSampleMaskOverrideCoverageFn {}
impl NvSampleMaskOverrideCoverageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvGeometryShaderPassthroughFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_geometry_shader_passthrough\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvGeometryShaderPassthroughFn {}
unsafe impl Send for NvGeometryShaderPassthroughFn {}
unsafe impl Sync for NvGeometryShaderPassthroughFn {}
impl NvGeometryShaderPassthroughFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvViewportArray2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_array2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvViewportArray2Fn {}
unsafe impl Send for NvViewportArray2Fn {}
unsafe impl Sync for NvViewportArray2Fn {}
impl NvViewportArray2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvxMultiviewPerViewAttributesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_NVX_multiview_per_view_attributes\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvxMultiviewPerViewAttributesFn {}
unsafe impl Send for NvxMultiviewPerViewAttributesFn {}
unsafe impl Sync for NvxMultiviewPerViewAttributesFn {}
impl NvxMultiviewPerViewAttributesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NVX_multiview_per_view_attributes'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self =
        Self(1_000_097_000);
}
#[doc = "Generated from 'VK_NVX_multiview_per_view_attributes'"]
impl SubpassDescriptionFlags {
    pub const PER_VIEW_ATTRIBUTES_NVX: Self = Self(0b1);
    pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = Self(0b10);
}
impl NvViewportSwizzleFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_swizzle\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvViewportSwizzleFn {}
unsafe impl Send for NvViewportSwizzleFn {}
unsafe impl Sync for NvViewportSwizzleFn {}
impl NvViewportSwizzleFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_viewport_swizzle'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1_000_098_000);
}
impl ExtDiscardRectanglesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_discard_rectangles\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const Rect2D,
);
#[derive(Clone)]
pub struct ExtDiscardRectanglesFn {
    pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
}
unsafe impl Send for ExtDiscardRectanglesFn {}
unsafe impl Sync for ExtDiscardRectanglesFn {}
impl ExtDiscardRectanglesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_discard_rectangle_ext: unsafe {
                unsafe extern "system" fn cmd_set_discard_rectangle_ext(
                    _command_buffer: CommandBuffer,
                    _first_discard_rectangle: u32,
                    _discard_rectangle_count: u32,
                    _p_discard_rectangles: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_discard_rectangle_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDiscardRectangleEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_discard_rectangle_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_discard_rectangles'"]
impl DynamicState {
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1_000_099_000);
}
#[doc = "Generated from 'VK_EXT_discard_rectangles'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1_000_099_000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1_000_099_001);
}
impl NvExtension101Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_101\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension101Fn {}
unsafe impl Send for NvExtension101Fn {}
unsafe impl Sync for NvExtension101Fn {}
impl NvExtension101Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtConservativeRasterizationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conservative_rasterization\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtConservativeRasterizationFn {}
unsafe impl Send for ExtConservativeRasterizationFn {}
unsafe impl Sync for ExtConservativeRasterizationFn {}
impl ExtConservativeRasterizationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_conservative_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1_000_101_000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1_000_101_001);
}
impl ExtDepthClipEnableFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_enable\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthClipEnableFn {}
unsafe impl Send for ExtDepthClipEnableFn {}
unsafe impl Sync for ExtDepthClipEnableFn {}
impl ExtDepthClipEnableFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_depth_clip_enable'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1_000_102_000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1_000_102_001);
}
impl NvExtension104Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_104\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension104Fn {}
unsafe impl Send for NvExtension104Fn {}
unsafe impl Sync for NvExtension104Fn {}
impl NvExtension104Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtSwapchainColorspaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_colorspace\0") }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct ExtSwapchainColorspaceFn {}
unsafe impl Send for ExtSwapchainColorspaceFn {}
unsafe impl Sync for ExtSwapchainColorspaceFn {}
impl ExtSwapchainColorspaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_swapchain_colorspace'"]
impl ColorSpaceKHR {
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1_000_104_001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1_000_104_002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1_000_104_003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1_000_104_004);
    pub const BT709_LINEAR_EXT: Self = Self(1_000_104_005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1_000_104_006);
    pub const BT2020_LINEAR_EXT: Self = Self(1_000_104_007);
    pub const HDR10_ST2084_EXT: Self = Self(1_000_104_008);
    pub const DOLBYVISION_EXT: Self = Self(1_000_104_009);
    pub const HDR10_HLG_EXT: Self = Self(1_000_104_010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1_000_104_011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1_000_104_012);
    pub const PASS_THROUGH_EXT: Self = Self(1_000_104_013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1_000_104_014);
}
impl ExtHdrMetadataFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_hdr_metadata\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT,
);
#[derive(Clone)]
pub struct ExtHdrMetadataFn {
    pub set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
}
unsafe impl Send for ExtHdrMetadataFn {}
unsafe impl Sync for ExtHdrMetadataFn {}
impl ExtHdrMetadataFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_hdr_metadata_ext: unsafe {
                unsafe extern "system" fn set_hdr_metadata_ext(
                    _device: Device,
                    _swapchain_count: u32,
                    _p_swapchains: *const SwapchainKHR,
                    _p_metadata: *const HdrMetadataEXT,
                ) {
                    panic!(concat!("Unable to load ", stringify!(set_hdr_metadata_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetHdrMetadataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    set_hdr_metadata_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_hdr_metadata'"]
impl StructureType {
    pub const HDR_METADATA_EXT: Self = Self(1_000_105_000);
}
impl ImgExtension107Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_extension_107\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ImgExtension107Fn {}
unsafe impl Send for ImgExtension107Fn {}
unsafe impl Sync for ImgExtension107Fn {}
impl ImgExtension107Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ImgExtension108Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_extension_108\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ImgExtension108Fn {}
unsafe impl Send for ImgExtension108Fn {}
unsafe impl Sync for ImgExtension108Fn {}
impl ImgExtension108Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrImagelessFramebufferFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_imageless_framebuffer\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrImagelessFramebufferFn {}
unsafe impl Send for KhrImagelessFramebufferFn {}
unsafe impl Sync for KhrImagelessFramebufferFn {}
impl KhrImagelessFramebufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_imageless_framebuffer'"]
impl FramebufferCreateFlags {
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}
#[doc = "Generated from 'VK_KHR_imageless_framebuffer'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self =
        Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
}
impl KhrCreateRenderpass2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_create_renderpass2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfo,
);
#[derive(Clone)]
pub struct KhrCreateRenderpass2Fn {
    pub create_render_pass2_khr: PFN_vkCreateRenderPass2,
    pub cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2,
    pub cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2,
    pub cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2,
}
unsafe impl Send for KhrCreateRenderpass2Fn {}
unsafe impl Sync for KhrCreateRenderpass2Fn {}
impl KhrCreateRenderpass2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_render_pass2_khr: unsafe {
                unsafe extern "system" fn create_render_pass2_khr(
                    _device: Device,
                    _p_create_info: *const RenderPassCreateInfo2,
                    _p_allocator: *const AllocationCallbacks,
                    _p_render_pass: *mut RenderPass,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_render_pass2_khr: unsafe {
                unsafe extern "system" fn cmd_begin_render_pass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_render_pass_begin: *const RenderPassBeginInfo,
                    _p_subpass_begin_info: *const SubpassBeginInfo,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_next_subpass2_khr: unsafe {
                unsafe extern "system" fn cmd_next_subpass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_subpass_begin_info: *const SubpassBeginInfo,
                    _p_subpass_end_info: *const SubpassEndInfo,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_next_subpass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_next_subpass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_render_pass2_khr: unsafe {
                unsafe extern "system" fn cmd_end_render_pass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_subpass_end_info: *const SubpassEndInfo,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_create_renderpass2'"]
impl StructureType {
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
}
impl ImgExtension111Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_extension_111\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ImgExtension111Fn {}
unsafe impl Send for ImgExtension111Fn {}
unsafe impl Sync for ImgExtension111Fn {}
impl ImgExtension111Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrSharedPresentableImageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shared_presentable_image\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[derive(Clone)]
pub struct KhrSharedPresentableImageFn {
    pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
unsafe impl Send for KhrSharedPresentableImageFn {}
unsafe impl Sync for KhrSharedPresentableImageFn {}
impl KhrSharedPresentableImageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_swapchain_status_khr: unsafe {
                unsafe extern "system" fn get_swapchain_status_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_status_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainStatusKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_status_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl ImageLayout {
    pub const SHARED_PRESENT_KHR: Self = Self(1_000_111_000);
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH: Self = Self(1_000_111_000);
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1_000_111_001);
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl StructureType {
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1_000_111_000);
}
impl KhrExternalFenceCapabilitiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_capabilities\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
);
#[derive(Clone)]
pub struct KhrExternalFenceCapabilitiesFn {
    pub get_physical_device_external_fence_properties_khr:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
}
unsafe impl Send for KhrExternalFenceCapabilitiesFn {}
unsafe impl Sync for KhrExternalFenceCapabilitiesFn {}
impl KhrExternalFenceCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_fence_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_fence_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
                    _p_external_fence_properties: *mut ExternalFenceProperties,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_fence_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_fence_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
}
impl KhrExternalFenceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalFenceFn {}
unsafe impl Send for KhrExternalFenceFn {}
unsafe impl Sync for KhrExternalFenceFn {}
impl KhrExternalFenceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_external_fence'"]
impl FenceImportFlags {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
#[doc = "Generated from 'VK_KHR_external_fence'"]
impl StructureType {
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
}
impl KhrExternalFenceWin32Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_win32\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalFenceWin32Fn {
    pub import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
    pub get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
}
unsafe impl Send for KhrExternalFenceWin32Fn {}
unsafe impl Sync for KhrExternalFenceWin32Fn {}
impl KhrExternalFenceWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_fence_win32_handle_khr: unsafe {
                unsafe extern "system" fn import_fence_win32_handle_khr(
                    _device: Device,
                    _p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_fence_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportFenceWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_fence_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_fence_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_fence_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_fence_win32_handle_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetFenceWin32HandleKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_fence_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_win32'"]
impl StructureType {
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_002);
}
impl KhrExternalFenceFdFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_fd\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const FenceGetFdInfoKHR,
    p_fd: *mut c_int,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalFenceFdFn {
    pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}
unsafe impl Send for KhrExternalFenceFdFn {}
unsafe impl Sync for KhrExternalFenceFdFn {}
impl KhrExternalFenceFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_fence_fd_khr: unsafe {
                unsafe extern "system" fn import_fence_fd_khr(
                    _device: Device,
                    _p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(import_fence_fd_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkImportFenceFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    import_fence_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_fence_fd_khr: unsafe {
                unsafe extern "system" fn get_fence_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const FenceGetFdInfoKHR,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_fence_fd_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetFenceFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_fence_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_fd'"]
impl StructureType {
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1_000_115_000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1_000_115_001);
}
impl KhrPerformanceQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_performance_query\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
        p_num_passes: *mut u32,
    );
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireProfilingLockKHR =
    unsafe extern "system" fn(device: Device, p_info: *const AcquireProfilingLockInfoKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: Device);
#[derive(Clone)]
pub struct KhrPerformanceQueryFn {
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
    pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
}
unsafe impl Send for KhrPerformanceQueryFn {}
unsafe impl Sync for KhrPerformanceQueryFn {}
impl KhrPerformanceQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_physical_device_queue_family_performance_query_counters_khr: unsafe {
                unsafe extern "system" fn enumerate_physical_device_queue_family_performance_query_counters_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _p_counter_count: *mut u32,
                    _p_counters: *mut PerformanceCounterKHR,
                    _p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(
                            enumerate_physical_device_queue_family_performance_query_counters_khr
                        )
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_device_queue_family_performance_query_counters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_performance_query_passes_khr: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_performance_query_passes_khr(
                    _physical_device: PhysicalDevice,
                    _p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
                    _p_num_passes: *mut u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_performance_query_passes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_performance_query_passes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_profiling_lock_khr: unsafe {
                unsafe extern "system" fn acquire_profiling_lock_khr(
                    _device: Device,
                    _p_info: *const AcquireProfilingLockInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_profiling_lock_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireProfilingLockKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_profiling_lock_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_profiling_lock_khr: unsafe {
                unsafe extern "system" fn release_profiling_lock_khr(_device: Device) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_profiling_lock_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkReleaseProfilingLockKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    release_profiling_lock_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_performance_query'"]
impl QueryType {
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1_000_116_000);
}
#[doc = "Generated from 'VK_KHR_performance_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1_000_116_000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1_000_116_001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1_000_116_002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1_000_116_003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1_000_116_004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1_000_116_005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1_000_116_006);
}
impl KhrMaintenance2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrMaintenance2Fn {}
unsafe impl Send for KhrMaintenance2Fn {}
unsafe impl Sync for KhrMaintenance2Fn {}
impl KhrMaintenance2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl ImageCreateFlags {
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
    pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl ImageLayout {
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self =
        Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self =
        Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self =
        Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
}
impl KhrExtension119Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_119\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension119Fn {}
unsafe impl Send for KhrExtension119Fn {}
unsafe impl Sync for KhrExtension119Fn {}
impl KhrExtension119Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrGetSurfaceCapabilities2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_surface_capabilities2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormat2KHR,
) -> Result;
#[derive(Clone)]
pub struct KhrGetSurfaceCapabilities2Fn {
    pub get_physical_device_surface_capabilities2_khr:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
}
unsafe impl Send for KhrGetSurfaceCapabilities2Fn {}
unsafe impl Sync for KhrGetSurfaceCapabilities2Fn {}
impl KhrGetSurfaceCapabilities2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_capabilities2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities2_khr(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
                    _p_surface_capabilities: *mut SurfaceCapabilities2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_formats2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_formats2_khr(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
                    _p_surface_format_count: *mut u32,
                    _p_surface_formats: *mut SurfaceFormat2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_formats2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_formats2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_surface_capabilities2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1_000_119_000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1_000_119_001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1_000_119_002);
}
impl KhrVariablePointersFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_variable_pointers\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrVariablePointersFn {}
unsafe impl Send for KhrVariablePointersFn {}
unsafe impl Sync for KhrVariablePointersFn {}
impl KhrVariablePointersFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_variable_pointers'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
}
impl KhrGetDisplayProperties2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_display_properties2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> Result;
#[derive(Clone)]
pub struct KhrGetDisplayProperties2Fn {
    pub get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    pub get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
unsafe impl Send for KhrGetDisplayProperties2Fn {}
unsafe impl Sync for KhrGetDisplayProperties2Fn {}
impl KhrGetDisplayProperties2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_display_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayProperties2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_display_plane_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_plane_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPlaneProperties2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_plane_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_plane_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_mode_properties2_khr: unsafe {
                unsafe extern "system" fn get_display_mode_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayModeProperties2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_mode_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayModeProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_mode_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_capabilities2_khr: unsafe {
                unsafe extern "system" fn get_display_plane_capabilities2_khr(
                    _physical_device: PhysicalDevice,
                    _p_display_plane_info: *const DisplayPlaneInfo2KHR,
                    _p_capabilities: *mut DisplayPlaneCapabilities2KHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_capabilities2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilities2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_capabilities2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_display_properties2'"]
impl StructureType {
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1_000_121_000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1_000_121_001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1_000_121_002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1_000_121_003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1_000_121_004);
}
impl MvkIosSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_ios_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const IOSSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct MvkIosSurfaceFn {
    pub create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
}
unsafe impl Send for MvkIosSurfaceFn {}
unsafe impl Sync for MvkIosSurfaceFn {}
impl MvkIosSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_ios_surface_mvk: unsafe {
                unsafe extern "system" fn create_ios_surface_mvk(
                    _instance: Instance,
                    _p_create_info: *const IOSSurfaceCreateInfoMVK,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ios_surface_mvk)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateIOSSurfaceMVK\0");
                let val = _f(cname);
                if val.is_null() {
                    create_ios_surface_mvk
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_MVK_ios_surface'"]
impl StructureType {
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1_000_122_000);
}
impl MvkMacosSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_macos_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MacOSSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct MvkMacosSurfaceFn {
    pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
}
unsafe impl Send for MvkMacosSurfaceFn {}
unsafe impl Sync for MvkMacosSurfaceFn {}
impl MvkMacosSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_mac_os_surface_mvk: unsafe {
                unsafe extern "system" fn create_mac_os_surface_mvk(
                    _instance: Instance,
                    _p_create_info: *const MacOSSurfaceCreateInfoMVK,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_mac_os_surface_mvk)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateMacOSSurfaceMVK\0");
                let val = _f(cname);
                if val.is_null() {
                    create_mac_os_surface_mvk
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_MVK_macos_surface'"]
impl StructureType {
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1_000_123_000);
}
impl MvkMoltenvkFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_moltenvk\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct MvkMoltenvkFn {}
unsafe impl Send for MvkMoltenvkFn {}
unsafe impl Sync for MvkMoltenvkFn {}
impl MvkMoltenvkFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExternalMemoryDmaBufFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_dma_buf\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtExternalMemoryDmaBufFn {}
unsafe impl Send for ExtExternalMemoryDmaBufFn {}
unsafe impl Sync for ExtExternalMemoryDmaBufFn {}
impl ExtExternalMemoryDmaBufFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_external_memory_dma_buf'"]
impl ExternalMemoryHandleTypeFlags {
    pub const DMA_BUF_EXT: Self = Self(0b10_0000_0000);
}
impl ExtQueueFamilyForeignFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_queue_family_foreign\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtQueueFamilyForeignFn {}
unsafe impl Send for ExtQueueFamilyForeignFn {}
unsafe impl Sync for ExtQueueFamilyForeignFn {}
impl ExtQueueFamilyForeignFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrDedicatedAllocationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dedicated_allocation\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrDedicatedAllocationFn {}
unsafe impl Send for KhrDedicatedAllocationFn {}
unsafe impl Sync for KhrDedicatedAllocationFn {}
impl KhrDedicatedAllocationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_dedicated_allocation'"]
impl StructureType {
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
}
impl ExtDebugUtilsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: Device,
    p_name_info: *const DebugUtilsObjectNameInfoEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: Queue);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);
#[derive(Clone)]
pub struct ExtDebugUtilsFn {
    pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
    pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
    pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
    pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
    pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
    pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
    pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
    pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
    pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}
unsafe impl Send for ExtDebugUtilsFn {}
unsafe impl Sync for ExtDebugUtilsFn {}
impl ExtDebugUtilsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_debug_utils_object_name_ext: unsafe {
                unsafe extern "system" fn set_debug_utils_object_name_ext(
                    _device: Device,
                    _p_name_info: *const DebugUtilsObjectNameInfoEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_debug_utils_object_name_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDebugUtilsObjectNameEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_debug_utils_object_name_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_debug_utils_object_tag_ext: unsafe {
                unsafe extern "system" fn set_debug_utils_object_tag_ext(
                    _device: Device,
                    _p_tag_info: *const DebugUtilsObjectTagInfoEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_debug_utils_object_tag_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDebugUtilsObjectTagEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_debug_utils_object_tag_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_begin_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_begin_debug_utils_label_ext(
                    _queue: Queue,
                    _p_label_info: *const DebugUtilsLabelEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_begin_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueBeginDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_begin_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_end_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_end_debug_utils_label_ext(_queue: Queue) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_end_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueEndDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_end_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_insert_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_insert_debug_utils_label_ext(
                    _queue: Queue,
                    _p_label_info: *const DebugUtilsLabelEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_insert_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueInsertDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_insert_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_begin_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                    _p_label_info: *const DebugUtilsLabelEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_end_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_insert_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_insert_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                    _p_label_info: *const DebugUtilsLabelEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_insert_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdInsertDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_insert_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_debug_utils_messenger_ext: unsafe {
                unsafe extern "system" fn create_debug_utils_messenger_ext(
                    _instance: Instance,
                    _p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_messenger: *mut DebugUtilsMessengerEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_debug_utils_messenger_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDebugUtilsMessengerEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_debug_utils_messenger_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_debug_utils_messenger_ext: unsafe {
                unsafe extern "system" fn destroy_debug_utils_messenger_ext(
                    _instance: Instance,
                    _messenger: DebugUtilsMessengerEXT,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_debug_utils_messenger_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugUtilsMessengerEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_debug_utils_messenger_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            submit_debug_utils_message_ext: unsafe {
                unsafe extern "system" fn submit_debug_utils_message_ext(
                    _instance: Instance,
                    _message_severity: DebugUtilsMessageSeverityFlagsEXT,
                    _message_types: DebugUtilsMessageTypeFlagsEXT,
                    _p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(submit_debug_utils_message_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSubmitDebugUtilsMessageEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    submit_debug_utils_message_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_utils'"]
impl ObjectType {
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1_000_128_000);
}
#[doc = "Generated from 'VK_EXT_debug_utils'"]
impl StructureType {
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1_000_128_000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1_000_128_001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1_000_128_002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1_000_128_003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1_000_128_004);
}
impl AndroidExternalMemoryAndroidHardwareBufferFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_ANDROID_external_memory_android_hardware_buffer\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 5u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: Device,
    buffer: *const AHardwareBuffer,
    p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: *mut *mut AHardwareBuffer,
) -> Result;
#[derive(Clone)]
pub struct AndroidExternalMemoryAndroidHardwareBufferFn {
    pub get_android_hardware_buffer_properties_android:
        PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    pub get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
}
unsafe impl Send for AndroidExternalMemoryAndroidHardwareBufferFn {}
unsafe impl Sync for AndroidExternalMemoryAndroidHardwareBufferFn {}
impl AndroidExternalMemoryAndroidHardwareBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_android_hardware_buffer_properties_android: unsafe {
                unsafe extern "system" fn get_android_hardware_buffer_properties_android(
                    _device: Device,
                    _buffer: *const AHardwareBuffer,
                    _p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_android_hardware_buffer_properties_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAndroidHardwareBufferPropertiesANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_android_hardware_buffer_properties_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_android_hardware_buffer_android: unsafe {
                unsafe extern "system" fn get_memory_android_hardware_buffer_android(
                    _device: Device,
                    _p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
                    _p_buffer: *mut *mut AHardwareBuffer,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_android_hardware_buffer_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryAndroidHardwareBufferANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_android_hardware_buffer_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_ANDROID_external_memory_android_hardware_buffer'"]
impl ExternalMemoryHandleTypeFlags {
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_ANDROID_external_memory_android_hardware_buffer'"]
impl StructureType {
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1_000_129_000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1_000_129_001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1_000_129_002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1_000_129_003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1_000_129_004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1_000_129_005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1_000_129_006);
}
impl ExtSamplerFilterMinmaxFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sampler_filter_minmax\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSamplerFilterMinmaxFn {}
unsafe impl Send for ExtSamplerFilterMinmaxFn {}
unsafe impl Sync for ExtSamplerFilterMinmaxFn {}
impl ExtSamplerFilterMinmaxFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
}
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN_EXT: Self = Self::MIN;
    pub const MAX_EXT: Self = Self::MAX;
}
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self =
        Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
}
impl KhrStorageBufferStorageClassFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_storage_buffer_storage_class\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrStorageBufferStorageClassFn {}
unsafe impl Send for KhrStorageBufferStorageClassFn {}
unsafe impl Sync for KhrStorageBufferStorageClassFn {}
impl KhrStorageBufferStorageClassFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdGpuShaderInt16Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_int16\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdGpuShaderInt16Fn {}
unsafe impl Send for AmdGpuShaderInt16Fn {}
unsafe impl Sync for AmdGpuShaderInt16Fn {}
impl AmdGpuShaderInt16Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension134Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_134\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension134Fn {}
unsafe impl Send for AmdExtension134Fn {}
unsafe impl Sync for AmdExtension134Fn {}
impl AmdExtension134Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension135Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_135\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension135Fn {}
unsafe impl Send for AmdExtension135Fn {}
unsafe impl Sync for AmdExtension135Fn {}
impl AmdExtension135Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_extension_135'"]
impl BufferUsageFlags {
    pub const RESERVED_25_AMD: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
}
impl AmdExtension136Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_136\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension136Fn {}
unsafe impl Send for AmdExtension136Fn {}
unsafe impl Sync for AmdExtension136Fn {}
impl AmdExtension136Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdMixedAttachmentSamplesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_mixed_attachment_samples\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdMixedAttachmentSamplesFn {}
unsafe impl Send for AmdMixedAttachmentSamplesFn {}
unsafe impl Sync for AmdMixedAttachmentSamplesFn {}
impl AmdMixedAttachmentSamplesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdShaderFragmentMaskFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_fragment_mask\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderFragmentMaskFn {}
unsafe impl Send for AmdShaderFragmentMaskFn {}
unsafe impl Sync for AmdShaderFragmentMaskFn {}
impl AmdShaderFragmentMaskFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtInlineUniformBlockFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_inline_uniform_block\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtInlineUniformBlockFn {}
unsafe impl Send for ExtInlineUniformBlockFn {}
unsafe impl Sync for ExtInlineUniformBlockFn {}
impl ExtInlineUniformBlockFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_inline_uniform_block'"]
impl DescriptorType {
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self::INLINE_UNIFORM_BLOCK;
}
#[doc = "Generated from 'VK_EXT_inline_uniform_block'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self =
        Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
}
impl AmdExtension140Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_140\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension140Fn {}
unsafe impl Send for AmdExtension140Fn {}
unsafe impl Sync for AmdExtension140Fn {}
impl AmdExtension140Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderStencilExportFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_stencil_export\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderStencilExportFn {}
unsafe impl Send for ExtShaderStencilExportFn {}
unsafe impl Sync for ExtShaderStencilExportFn {}
impl ExtShaderStencilExportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension142Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_142\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension142Fn {}
unsafe impl Send for AmdExtension142Fn {}
unsafe impl Sync for AmdExtension142Fn {}
impl AmdExtension142Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension143Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_143\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension143Fn {}
unsafe impl Send for AmdExtension143Fn {}
unsafe impl Sync for AmdExtension143Fn {}
impl AmdExtension143Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtSampleLocationsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sample_locations\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_sample_locations_info: *const SampleLocationsInfoEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlags,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
);
#[derive(Clone)]
pub struct ExtSampleLocationsFn {
    pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
    pub get_physical_device_multisample_properties_ext:
        PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}
unsafe impl Send for ExtSampleLocationsFn {}
unsafe impl Sync for ExtSampleLocationsFn {}
impl ExtSampleLocationsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_sample_locations_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_locations_ext(
                    _command_buffer: CommandBuffer,
                    _p_sample_locations_info: *const SampleLocationsInfoEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_locations_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetSampleLocationsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_locations_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_multisample_properties_ext: unsafe {
                unsafe extern "system" fn get_physical_device_multisample_properties_ext(
                    _physical_device: PhysicalDevice,
                    _samples: SampleCountFlags,
                    _p_multisample_properties: *mut MultisamplePropertiesEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_multisample_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_multisample_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl DynamicState {
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1_000_143_000);
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl ImageCreateFlags {
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl StructureType {
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1_000_143_000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1_000_143_001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1_000_143_002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1_000_143_003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1_000_143_004);
}
impl KhrRelaxedBlockLayoutFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_relaxed_block_layout\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrRelaxedBlockLayoutFn {}
unsafe impl Send for KhrRelaxedBlockLayoutFn {}
unsafe impl Sync for KhrRelaxedBlockLayoutFn {}
impl KhrRelaxedBlockLayoutFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrGetMemoryRequirements2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_memory_requirements2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[derive(Clone)]
pub struct KhrGetMemoryRequirements2Fn {
    pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2,
    pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2,
    pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2,
}
unsafe impl Send for KhrGetMemoryRequirements2Fn {}
unsafe impl Sync for KhrGetMemoryRequirements2Fn {}
impl KhrGetMemoryRequirements2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_image_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const ImageMemoryRequirementsInfo2,
                    _p_memory_requirements: *mut MemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_buffer_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const BufferMemoryRequirementsInfo2,
                    _p_memory_requirements: *mut MemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_sparse_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_image_sparse_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const ImageSparseMemoryRequirementsInfo2,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_sparse_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_sparse_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_memory_requirements2'"]
impl StructureType {
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
        Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self =
        Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
}
impl KhrImageFormatListFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_image_format_list\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrImageFormatListFn {}
unsafe impl Send for KhrImageFormatListFn {}
unsafe impl Sync for KhrImageFormatListFn {}
impl KhrImageFormatListFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_image_format_list'"]
impl StructureType {
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
}
impl ExtBlendOperationAdvancedFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_blend_operation_advanced\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtBlendOperationAdvancedFn {}
unsafe impl Send for ExtBlendOperationAdvancedFn {}
unsafe impl Sync for ExtBlendOperationAdvancedFn {}
impl ExtBlendOperationAdvancedFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl AccessFlags {
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl BlendOp {
    pub const ZERO_EXT: Self = Self(1_000_148_000);
    pub const SRC_EXT: Self = Self(1_000_148_001);
    pub const DST_EXT: Self = Self(1_000_148_002);
    pub const SRC_OVER_EXT: Self = Self(1_000_148_003);
    pub const DST_OVER_EXT: Self = Self(1_000_148_004);
    pub const SRC_IN_EXT: Self = Self(1_000_148_005);
    pub const DST_IN_EXT: Self = Self(1_000_148_006);
    pub const SRC_OUT_EXT: Self = Self(1_000_148_007);
    pub const DST_OUT_EXT: Self = Self(1_000_148_008);
    pub const SRC_ATOP_EXT: Self = Self(1_000_148_009);
    pub const DST_ATOP_EXT: Self = Self(1_000_148_010);
    pub const XOR_EXT: Self = Self(1_000_148_011);
    pub const MULTIPLY_EXT: Self = Self(1_000_148_012);
    pub const SCREEN_EXT: Self = Self(1_000_148_013);
    pub const OVERLAY_EXT: Self = Self(1_000_148_014);
    pub const DARKEN_EXT: Self = Self(1_000_148_015);
    pub const LIGHTEN_EXT: Self = Self(1_000_148_016);
    pub const COLORDODGE_EXT: Self = Self(1_000_148_017);
    pub const COLORBURN_EXT: Self = Self(1_000_148_018);
    pub const HARDLIGHT_EXT: Self = Self(1_000_148_019);
    pub const SOFTLIGHT_EXT: Self = Self(1_000_148_020);
    pub const DIFFERENCE_EXT: Self = Self(1_000_148_021);
    pub const EXCLUSION_EXT: Self = Self(1_000_148_022);
    pub const INVERT_EXT: Self = Self(1_000_148_023);
    pub const INVERT_RGB_EXT: Self = Self(1_000_148_024);
    pub const LINEARDODGE_EXT: Self = Self(1_000_148_025);
    pub const LINEARBURN_EXT: Self = Self(1_000_148_026);
    pub const VIVIDLIGHT_EXT: Self = Self(1_000_148_027);
    pub const LINEARLIGHT_EXT: Self = Self(1_000_148_028);
    pub const PINLIGHT_EXT: Self = Self(1_000_148_029);
    pub const HARDMIX_EXT: Self = Self(1_000_148_030);
    pub const HSL_HUE_EXT: Self = Self(1_000_148_031);
    pub const HSL_SATURATION_EXT: Self = Self(1_000_148_032);
    pub const HSL_COLOR_EXT: Self = Self(1_000_148_033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1_000_148_034);
    pub const PLUS_EXT: Self = Self(1_000_148_035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1_000_148_036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1_000_148_037);
    pub const PLUS_DARKER_EXT: Self = Self(1_000_148_038);
    pub const MINUS_EXT: Self = Self(1_000_148_039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1_000_148_040);
    pub const CONTRAST_EXT: Self = Self(1_000_148_041);
    pub const INVERT_OVG_EXT: Self = Self(1_000_148_042);
    pub const RED_EXT: Self = Self(1_000_148_043);
    pub const GREEN_EXT: Self = Self(1_000_148_044);
    pub const BLUE_EXT: Self = Self(1_000_148_045);
}
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1_000_148_000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1_000_148_001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1_000_148_002);
}
impl NvFragmentCoverageToColorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_coverage_to_color\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFragmentCoverageToColorFn {}
unsafe impl Send for NvFragmentCoverageToColorFn {}
unsafe impl Sync for NvFragmentCoverageToColorFn {}
impl NvFragmentCoverageToColorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_fragment_coverage_to_color'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1_000_149_000);
}
impl KhrAccelerationStructureFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_acceleration_structure\0")
        }
    }
    pub const SPEC_VERSION: u32 = 13u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    p_indirect_device_addresses: *const DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut c_void,
    stride: usize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
#[derive(Clone)]
pub struct KhrAccelerationStructureFn {
    pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
    pub cmd_build_acceleration_structures_indirect_khr:
        PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
    pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
    pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    pub copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    pub write_acceleration_structures_properties_khr:
        PFN_vkWriteAccelerationStructuresPropertiesKHR,
    pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    pub cmd_copy_acceleration_structure_to_memory_khr:
        PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    pub cmd_copy_memory_to_acceleration_structure_khr:
        PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    pub get_acceleration_structure_device_address_khr:
        PFN_vkGetAccelerationStructureDeviceAddressKHR,
    pub cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    pub get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
    pub get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
}
unsafe impl Send for KhrAccelerationStructureFn {}
unsafe impl Sync for KhrAccelerationStructureFn {}
impl KhrAccelerationStructureFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn create_acceleration_structure_khr(
                    _device: Device,
                    _p_create_info: *const AccelerationStructureCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_acceleration_structure: *mut AccelerationStructureKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn destroy_acceleration_structure_khr(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structures_khr: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structures_khr(
                    _command_buffer: CommandBuffer,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
                    _pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structures_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structures_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structures_indirect_khr: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structures_indirect_khr(
                    _command_buffer: CommandBuffer,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
                    _p_indirect_device_addresses: *const DeviceAddress,
                    _p_indirect_strides: *const u32,
                    _pp_max_primitive_counts: *const *const u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structures_indirect_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresIndirectKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structures_indirect_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            build_acceleration_structures_khr: unsafe {
                unsafe extern "system" fn build_acceleration_structures_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
                    _pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(build_acceleration_structures_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBuildAccelerationStructuresKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    build_acceleration_structures_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn copy_acceleration_structure_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyAccelerationStructureInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_acceleration_structure_to_memory_khr: unsafe {
                unsafe extern "system" fn copy_acceleration_structure_to_memory_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_acceleration_structure_to_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyAccelerationStructureToMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_acceleration_structure_to_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_memory_to_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn copy_memory_to_acceleration_structure_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_memory_to_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyMemoryToAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_memory_to_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            write_acceleration_structures_properties_khr: unsafe {
                unsafe extern "system" fn write_acceleration_structures_properties_khr(
                    _device: Device,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureKHR,
                    _query_type: QueryType,
                    _data_size: usize,
                    _p_data: *mut c_void,
                    _stride: usize,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(write_acceleration_structures_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkWriteAccelerationStructuresPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    write_acceleration_structures_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyAccelerationStructureInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_to_memory_khr: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_to_memory_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_to_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureToMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_to_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_memory_to_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn cmd_copy_memory_to_acceleration_structure_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_memory_to_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMemoryToAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_memory_to_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_device_address_khr: unsafe {
                unsafe extern "system" fn get_acceleration_structure_device_address_khr(
                    _device: Device,
                    _p_info: *const AccelerationStructureDeviceAddressInfoKHR,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_device_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureDeviceAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_device_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_acceleration_structures_properties_khr: unsafe {
                unsafe extern "system" fn cmd_write_acceleration_structures_properties_khr(
                    _command_buffer: CommandBuffer,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureKHR,
                    _query_type: QueryType,
                    _query_pool: QueryPool,
                    _first_query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_acceleration_structures_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_acceleration_structures_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_acceleration_structure_compatibility_khr: unsafe {
                unsafe extern "system" fn get_device_acceleration_structure_compatibility_khr(
                    _device: Device,
                    _p_version_info: *const AccelerationStructureVersionInfoKHR,
                    _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_acceleration_structure_compatibility_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceAccelerationStructureCompatibilityKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_acceleration_structure_compatibility_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_build_sizes_khr: unsafe {
                unsafe extern "system" fn get_acceleration_structure_build_sizes_khr(
                    _device: Device,
                    _build_type: AccelerationStructureBuildTypeKHR,
                    _p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
                    _p_max_primitive_counts: *const u32,
                    _p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_build_sizes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureBuildSizesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_build_sizes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl AccessFlags {
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl BufferUsageFlags {
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl DescriptorType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl FormatFeatureFlags {
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl FormatFeatureFlags2 {
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl IndexType {
    pub const NONE_KHR: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl ObjectType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl PipelineStageFlags {
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1_000_150_000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1_000_150_001);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl StructureType {
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1_000_150_000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1_000_150_002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1_000_150_003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1_000_150_004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1_000_150_005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1_000_150_006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1_000_150_009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1_000_150_010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1_000_150_011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1_000_150_012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1_000_150_013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1_000_150_014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1_000_150_017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1_000_150_020);
}
impl KhrRayTracingPipelineFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_pipeline\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
#[derive(Clone)]
pub struct KhrRayTracingPipelineFn {
    pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    pub get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
}
unsafe impl Send for KhrRayTracingPipelineFn {}
unsafe impl Sync for KhrRayTracingPipelineFn {}
impl KhrRayTracingPipelineFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_trace_rays_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_khr(
                    _command_buffer: CommandBuffer,
                    _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _width: u32,
                    _height: u32,
                    _depth: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_ray_tracing_pipelines_khr: unsafe {
                unsafe extern "system" fn create_ray_tracing_pipelines_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const RayTracingPipelineCreateInfoKHR,
                    _p_allocator: *const AllocationCallbacks,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ray_tracing_pipelines_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateRayTracingPipelinesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_ray_tracing_pipelines_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_handles_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_handles_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_handles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_handles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_capture_replay_shader_group_handles_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_capture_replay_shader_group_handles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_capture_replay_shader_group_handles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_trace_rays_indirect_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_indirect_khr(
                    _command_buffer: CommandBuffer,
                    _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _indirect_device_address: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_trace_rays_indirect_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirectKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_indirect_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_stack_size_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_stack_size_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _group: u32,
                    _group_shader: ShaderGroupShaderKHR,
                ) -> DeviceSize {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_stack_size_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupStackSizeKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_stack_size_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_ray_tracing_pipeline_stack_size_khr: unsafe {
                unsafe extern "system" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
                    _command_buffer: CommandBuffer,
                    _pipeline_stack_size: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_ray_tracing_pipeline_stack_size_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRayTracingPipelineStackSizeKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_ray_tracing_pipeline_stack_size_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl BufferUsageFlags {
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl DynamicState {
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1_000_347_000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineBindPoint {
    pub const RAY_TRACING_KHR: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(0b100_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(0b1000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(0b10_0000_0000_0000_0000);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(0b1_0000_0000_0000);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(0b10_0000_0000_0000);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
        Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineStageFlags {
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl ShaderStageFlags {
    pub const RAYGEN_KHR: Self = Self(0b1_0000_0000);
    pub const ANY_HIT_KHR: Self = Self(0b10_0000_0000);
    pub const CLOSEST_HIT_KHR: Self = Self(0b100_0000_0000);
    pub const MISS_KHR: Self = Self(0b1000_0000_0000);
    pub const INTERSECTION_KHR: Self = Self(0b1_0000_0000_0000);
    pub const CALLABLE_KHR: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1_000_347_000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1_000_347_001);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1_000_150_015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1_000_150_016);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1_000_150_018);
}
impl KhrRayQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_query\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrRayQueryFn {}
unsafe impl Send for KhrRayQueryFn {}
unsafe impl Sync for KhrRayQueryFn {}
impl KhrRayQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_ray_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1_000_348_013);
}
impl NvExtension152Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_152\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension152Fn {}
unsafe impl Send for NvExtension152Fn {}
unsafe impl Sync for NvExtension152Fn {}
impl NvExtension152Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvFramebufferMixedSamplesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_framebuffer_mixed_samples\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFramebufferMixedSamplesFn {}
unsafe impl Send for NvFramebufferMixedSamplesFn {}
unsafe impl Sync for NvFramebufferMixedSamplesFn {}
impl NvFramebufferMixedSamplesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_framebuffer_mixed_samples'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1_000_152_000);
}
impl NvFillRectangleFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fill_rectangle\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFillRectangleFn {}
unsafe impl Send for NvFillRectangleFn {}
unsafe impl Sync for NvFillRectangleFn {}
impl NvFillRectangleFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_fill_rectangle'"]
impl PolygonMode {
    pub const FILL_RECTANGLE_NV: Self = Self(1_000_153_000);
}
impl NvShaderSmBuiltinsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_sm_builtins\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvShaderSmBuiltinsFn {}
unsafe impl Send for NvShaderSmBuiltinsFn {}
unsafe impl Sync for NvShaderSmBuiltinsFn {}
impl NvShaderSmBuiltinsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_shader_sm_builtins'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1_000_154_000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1_000_154_001);
}
impl ExtPostDepthCoverageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_post_depth_coverage\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPostDepthCoverageFn {}
unsafe impl Send for ExtPostDepthCoverageFn {}
unsafe impl Sync for ExtPostDepthCoverageFn {}
impl ExtPostDepthCoverageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrSamplerYcbcrConversionFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_sampler_ycbcr_conversion\0")
        }
    }
    pub const SPEC_VERSION: u32 = 14u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks,
);
#[derive(Clone)]
pub struct KhrSamplerYcbcrConversionFn {
    pub create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
}
unsafe impl Send for KhrSamplerYcbcrConversionFn {}
unsafe impl Sync for KhrSamplerYcbcrConversionFn {}
impl KhrSamplerYcbcrConversionFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_sampler_ycbcr_conversion_khr: unsafe {
                unsafe extern "system" fn create_sampler_ycbcr_conversion_khr(
                    _device: Device,
                    _p_create_info: *const SamplerYcbcrConversionCreateInfo,
                    _p_allocator: *const AllocationCallbacks,
                    _p_ycbcr_conversion: *mut SamplerYcbcrConversion,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_sampler_ycbcr_conversion_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSamplerYcbcrConversionKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_sampler_ycbcr_conversion_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_sampler_ycbcr_conversion_khr: unsafe {
                unsafe extern "system" fn destroy_sampler_ycbcr_conversion_khr(
                    _device: Device,
                    _ycbcr_conversion: SamplerYcbcrConversion,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_sampler_ycbcr_conversion_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroySamplerYcbcrConversionKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_sampler_ycbcr_conversion_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ChromaLocation {
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl Format {
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
        Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
        Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
        Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
        Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
        Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
        Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl FormatFeatureFlags {
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ImageAspectFlags {
    pub const PLANE_0_KHR: Self = Self::PLANE_0;
    pub const PLANE_1_KHR: Self = Self::PLANE_1;
    pub const PLANE_2_KHR: Self = Self::PLANE_2;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ImageCreateFlags {
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ObjectType {
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl SamplerYcbcrRange {
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl StructureType {
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self =
        Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
}
impl KhrBindMemory2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_bind_memory2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> Result;
#[derive(Clone)]
pub struct KhrBindMemory2Fn {
    pub bind_buffer_memory2_khr: PFN_vkBindBufferMemory2,
    pub bind_image_memory2_khr: PFN_vkBindImageMemory2,
}
unsafe impl Send for KhrBindMemory2Fn {}
unsafe impl Sync for KhrBindMemory2Fn {}
impl KhrBindMemory2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            bind_buffer_memory2_khr: unsafe {
                unsafe extern "system" fn bind_buffer_memory2_khr(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindBufferMemoryInfo,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_buffer_memory2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_buffer_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_image_memory2_khr: unsafe {
                unsafe extern "system" fn bind_image_memory2_khr(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindImageMemoryInfo,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_image_memory2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_image_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_bind_memory2'"]
impl ImageCreateFlags {
    pub const ALIAS_KHR: Self = Self::ALIAS;
}
#[doc = "Generated from 'VK_KHR_bind_memory2'"]
impl StructureType {
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
}
impl ExtImageDrmFormatModifierFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_drm_format_modifier\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> Result;
#[derive(Clone)]
pub struct ExtImageDrmFormatModifierFn {
    pub get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}
unsafe impl Send for ExtImageDrmFormatModifierFn {}
unsafe impl Sync for ExtImageDrmFormatModifierFn {}
impl ExtImageDrmFormatModifierFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_drm_format_modifier_properties_ext: unsafe {
                unsafe extern "system" fn get_image_drm_format_modifier_properties_ext(
                    _device: Device,
                    _image: Image,
                    _p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_drm_format_modifier_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageDrmFormatModifierPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_drm_format_modifier_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl ImageAspectFlags {
    pub const MEMORY_PLANE_0_EXT: Self = Self(0b1000_0000);
    pub const MEMORY_PLANE_1_EXT: Self = Self(0b1_0000_0000);
    pub const MEMORY_PLANE_2_EXT: Self = Self(0b10_0000_0000);
    pub const MEMORY_PLANE_3_EXT: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl ImageTiling {
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1_000_158_000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl Result {
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1_000_158_000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl StructureType {
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1_000_158_000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1_000_158_002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1_000_158_003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1_000_158_004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1_000_158_005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1_000_158_006);
}
impl ExtExtension160Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_160\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension160Fn {}
unsafe impl Send for ExtExtension160Fn {}
unsafe impl Sync for ExtExtension160Fn {}
impl ExtExtension160Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtValidationCacheFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_cache\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ValidationCacheCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_validation_cache: *mut ValidationCacheEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[derive(Clone)]
pub struct ExtValidationCacheFn {
    pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
}
unsafe impl Send for ExtValidationCacheFn {}
unsafe impl Sync for ExtValidationCacheFn {}
impl ExtValidationCacheFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_validation_cache_ext: unsafe {
                unsafe extern "system" fn create_validation_cache_ext(
                    _device: Device,
                    _p_create_info: *const ValidationCacheCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_validation_cache: *mut ValidationCacheEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_validation_cache_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateValidationCacheEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_validation_cache_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_validation_cache_ext: unsafe {
                unsafe extern "system" fn destroy_validation_cache_ext(
                    _device: Device,
                    _validation_cache: ValidationCacheEXT,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_validation_cache_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyValidationCacheEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_validation_cache_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            merge_validation_caches_ext: unsafe {
                unsafe extern "system" fn merge_validation_caches_ext(
                    _device: Device,
                    _dst_cache: ValidationCacheEXT,
                    _src_cache_count: u32,
                    _p_src_caches: *const ValidationCacheEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(merge_validation_caches_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkMergeValidationCachesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    merge_validation_caches_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_validation_cache_data_ext: unsafe {
                unsafe extern "system" fn get_validation_cache_data_ext(
                    _device: Device,
                    _validation_cache: ValidationCacheEXT,
                    _p_data_size: *mut usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_validation_cache_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetValidationCacheDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_validation_cache_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_validation_cache'"]
impl ObjectType {
    pub const VALIDATION_CACHE_EXT: Self = Self(1_000_160_000);
}
#[doc = "Generated from 'VK_EXT_validation_cache'"]
impl StructureType {
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1_000_160_000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1_000_160_001);
}
impl ExtDescriptorIndexingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_descriptor_indexing\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtDescriptorIndexingFn {}
unsafe impl Send for ExtDescriptorIndexingFn {}
unsafe impl Sync for ExtDescriptorIndexingFn {}
impl ExtDescriptorIndexingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorPoolCreateFlags {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorSetLayoutCreateFlags {
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl Result {
    pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl StructureType {
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
}
impl ExtShaderViewportIndexLayerFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_viewport_index_layer\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderViewportIndexLayerFn {}
unsafe impl Send for ExtShaderViewportIndexLayerFn {}
unsafe impl Sync for ExtShaderViewportIndexLayerFn {}
impl ExtShaderViewportIndexLayerFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrPortabilitySubsetFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPortabilitySubsetFn {}
unsafe impl Send for KhrPortabilitySubsetFn {}
unsafe impl Sync for KhrPortabilitySubsetFn {}
impl KhrPortabilitySubsetFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_portability_subset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1_000_163_000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1_000_163_001);
}
impl NvShadingRateImageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shading_rate_image\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const ShadingRatePaletteNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
);
#[derive(Clone)]
pub struct NvShadingRateImageFn {
    pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
}
unsafe impl Send for NvShadingRateImageFn {}
unsafe impl Sync for NvShadingRateImageFn {}
impl NvShadingRateImageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_shading_rate_image_nv: unsafe {
                unsafe extern "system" fn cmd_bind_shading_rate_image_nv(
                    _command_buffer: CommandBuffer,
                    _image_view: ImageView,
                    _image_layout: ImageLayout,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_shading_rate_image_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindShadingRateImageNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_shading_rate_image_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_shading_rate_palette_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_shading_rate_palette_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_shading_rate_palettes: *const ShadingRatePaletteNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_shading_rate_palette_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportShadingRatePaletteNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_shading_rate_palette_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coarse_sample_order_nv: unsafe {
                unsafe extern "system" fn cmd_set_coarse_sample_order_nv(
                    _command_buffer: CommandBuffer,
                    _sample_order_type: CoarseSampleOrderTypeNV,
                    _custom_sample_order_count: u32,
                    _p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coarse_sample_order_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoarseSampleOrderNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coarse_sample_order_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl AccessFlags {
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl DynamicState {
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1_000_164_004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1_000_164_006);
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl ImageLayout {
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl ImageUsageFlags {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl PipelineStageFlags {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1_000_164_000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1_000_164_001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1_000_164_002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self =
        Self(1_000_164_005);
}
impl NvRayTracingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2KHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const AccelerationStructureInfoNV,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table_buffer: Buffer,
    raygen_shader_binding_offset: DeviceSize,
    miss_shader_binding_table_buffer: Buffer,
    miss_shader_binding_offset: DeviceSize,
    miss_shader_binding_stride: DeviceSize,
    hit_shader_binding_table_buffer: Buffer,
    hit_shader_binding_offset: DeviceSize,
    hit_shader_binding_stride: DeviceSize,
    callable_shader_binding_table_buffer: Buffer,
    callable_shader_binding_offset: DeviceSize,
    callable_shader_binding_stride: DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCompileDeferredNV =
    unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> Result;
#[derive(Clone)]
pub struct NvRayTracingFn {
    pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
    pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
    pub get_acceleration_structure_memory_requirements_nv:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
    pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
    pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
    pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
    pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
    pub get_ray_tracing_shader_group_handles_nv:
        crate::vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
    pub cmd_write_acceleration_structures_properties_nv:
        PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    pub compile_deferred_nv: PFN_vkCompileDeferredNV,
}
unsafe impl Send for NvRayTracingFn {}
unsafe impl Sync for NvRayTracingFn {}
impl NvRayTracingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn create_acceleration_structure_nv(
                    _device: Device,
                    _p_create_info: *const AccelerationStructureCreateInfoNV,
                    _p_allocator: *const AllocationCallbacks,
                    _p_acceleration_structure: *mut AccelerationStructureNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn destroy_acceleration_structure_nv(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureNV,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_memory_requirements_nv: unsafe {
                unsafe extern "system" fn get_acceleration_structure_memory_requirements_nv(
                    _device: Device,
                    _p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
                    _p_memory_requirements: *mut MemoryRequirements2KHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_memory_requirements_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureMemoryRequirementsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_memory_requirements_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_acceleration_structure_memory_nv: unsafe {
                unsafe extern "system" fn bind_acceleration_structure_memory_nv(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_acceleration_structure_memory_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBindAccelerationStructureMemoryNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    bind_acceleration_structure_memory_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structure_nv(
                    _command_buffer: CommandBuffer,
                    _p_info: *const AccelerationStructureInfoNV,
                    _instance_data: Buffer,
                    _instance_offset: DeviceSize,
                    _update: Bool32,
                    _dst: AccelerationStructureNV,
                    _src: AccelerationStructureNV,
                    _scratch: Buffer,
                    _scratch_offset: DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_nv(
                    _command_buffer: CommandBuffer,
                    _dst: AccelerationStructureNV,
                    _src: AccelerationStructureNV,
                    _mode: CopyAccelerationStructureModeKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_trace_rays_nv: unsafe {
                unsafe extern "system" fn cmd_trace_rays_nv(
                    _command_buffer: CommandBuffer,
                    _raygen_shader_binding_table_buffer: Buffer,
                    _raygen_shader_binding_offset: DeviceSize,
                    _miss_shader_binding_table_buffer: Buffer,
                    _miss_shader_binding_offset: DeviceSize,
                    _miss_shader_binding_stride: DeviceSize,
                    _hit_shader_binding_table_buffer: Buffer,
                    _hit_shader_binding_offset: DeviceSize,
                    _hit_shader_binding_stride: DeviceSize,
                    _callable_shader_binding_table_buffer: Buffer,
                    _callable_shader_binding_offset: DeviceSize,
                    _callable_shader_binding_stride: DeviceSize,
                    _width: u32,
                    _height: u32,
                    _depth: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_nv)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_ray_tracing_pipelines_nv: unsafe {
                unsafe extern "system" fn create_ray_tracing_pipelines_nv(
                    _device: Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const RayTracingPipelineCreateInfoNV,
                    _p_allocator: *const AllocationCallbacks,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ray_tracing_pipelines_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateRayTracingPipelinesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_ray_tracing_pipelines_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_handles_nv: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_handles_nv(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_handles_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_handles_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_handle_nv: unsafe {
                unsafe extern "system" fn get_acceleration_structure_handle_nv(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureNV,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_handle_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureHandleNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_handle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_acceleration_structures_properties_nv: unsafe {
                unsafe extern "system" fn cmd_write_acceleration_structures_properties_nv(
                    _command_buffer: CommandBuffer,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureNV,
                    _query_type: QueryType,
                    _query_pool: QueryPool,
                    _first_query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_acceleration_structures_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_acceleration_structures_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            compile_deferred_nv: unsafe {
                unsafe extern "system" fn compile_deferred_nv(
                    _device: Device,
                    _pipeline: Pipeline,
                    _shader: u32,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(compile_deferred_nv)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCompileDeferredNV\0");
                let val = _f(cname);
                if val.is_null() {
                    compile_deferred_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL;
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl AccessFlags {
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl BufferUsageFlags {
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_NV: Self = Self::CLONE;
    pub const COMPACT_NV: Self = Self::COMPACT;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl DescriptorType {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryFlagsKHR {
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryTypeKHR {
    pub const TRIANGLES_NV: Self = Self::TRIANGLES;
    pub const AABBS_NV: Self = Self::AABBS;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl IndexType {
    pub const NONE_NV: Self = Self::NONE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl ObjectType {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineBindPoint {
    pub const RAY_TRACING_NV: Self = Self::RAY_TRACING_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineCreateFlags {
    pub const DEFER_COMPILE_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineStageFlags {
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_NV: Self = Self::GENERAL;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl ShaderStageFlags {
    pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
    pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
    pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
    pub const MISS_NV: Self = Self::MISS_KHR;
    pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
    pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl StructureType {
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1_000_165_000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1_000_165_001);
    pub const GEOMETRY_NV: Self = Self(1_000_165_003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1_000_165_004);
    pub const GEOMETRY_AABB_NV: Self = Self(1_000_165_005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1_000_165_006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1_000_165_008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1_000_165_009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1_000_165_011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1_000_165_012);
}
impl NvRepresentativeFragmentTestFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_representative_fragment_test\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvRepresentativeFragmentTestFn {}
unsafe impl Send for NvRepresentativeFragmentTestFn {}
unsafe impl Sync for NvRepresentativeFragmentTestFn {}
impl NvRepresentativeFragmentTestFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_representative_fragment_test'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1_000_166_000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self =
        Self(1_000_166_001);
}
impl NvExtension168Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_168\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension168Fn {}
unsafe impl Send for NvExtension168Fn {}
unsafe impl Sync for NvExtension168Fn {}
impl NvExtension168Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrMaintenance3Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance3\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_support: *mut DescriptorSetLayoutSupport,
);
#[derive(Clone)]
pub struct KhrMaintenance3Fn {
    pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupport,
}
unsafe impl Send for KhrMaintenance3Fn {}
unsafe impl Sync for KhrMaintenance3Fn {}
impl KhrMaintenance3Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_descriptor_set_layout_support_khr: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_support_khr(
                    _device: Device,
                    _p_create_info: *const DescriptorSetLayoutCreateInfo,
                    _p_support: *mut DescriptorSetLayoutSupport,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance3'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
}
impl KhrDrawIndirectCountFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_draw_indirect_count\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDrawIndirectCountFn {
    pub cmd_draw_indirect_count_khr: crate::vk::PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count_khr: crate::vk::PFN_vkCmdDrawIndexedIndirectCount,
}
unsafe impl Send for KhrDrawIndirectCountFn {}
unsafe impl Sync for KhrDrawIndirectCountFn {}
impl KhrDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_indirect_count_khr: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_count_khr(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_count_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_count_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect_count_khr: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect_count_khr(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect_count_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect_count_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtFilterCubicFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_filter_cubic\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtFilterCubicFn {}
unsafe impl Send for ExtFilterCubicFn {}
unsafe impl Sync for ExtFilterCubicFn {}
impl ExtFilterCubicFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl Filter {
    pub const CUBIC_EXT: Self = Self::CUBIC_IMG;
}
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_IMG;
}
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1_000_170_000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1_000_170_001);
}
impl QcomRenderPassShaderResolveFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_shader_resolve\0")
        }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct QcomRenderPassShaderResolveFn {}
unsafe impl Send for QcomRenderPassShaderResolveFn {}
unsafe impl Sync for QcomRenderPassShaderResolveFn {}
impl QcomRenderPassShaderResolveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_render_pass_shader_resolve'"]
impl SubpassDescriptionFlags {
    pub const FRAGMENT_REGION_QCOM: Self = Self(0b100);
    pub const SHADER_RESOLVE_QCOM: Self = Self(0b1000);
}
impl QcomExtension173Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_173\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension173Fn {}
unsafe impl Send for QcomExtension173Fn {}
unsafe impl Sync for QcomExtension173Fn {}
impl QcomExtension173Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_extension_173'"]
impl BufferUsageFlags {
    pub const RESERVED_18_QCOM: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_extension_173'"]
impl ImageUsageFlags {
    pub const RESERVED_16_QCOM: Self = Self(0b1_0000_0000_0000_0000);
    pub const RESERVED_17_QCOM: Self = Self(0b10_0000_0000_0000_0000);
}
impl QcomExtension174Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_174\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension174Fn {}
unsafe impl Send for QcomExtension174Fn {}
unsafe impl Sync for QcomExtension174Fn {}
impl QcomExtension174Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtGlobalPriorityFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_global_priority\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtGlobalPriorityFn {}
unsafe impl Send for ExtGlobalPriorityFn {}
unsafe impl Sync for ExtGlobalPriorityFn {}
impl ExtGlobalPriorityFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_global_priority'"]
impl Result {
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self::ERROR_NOT_PERMITTED_KHR;
}
#[doc = "Generated from 'VK_EXT_global_priority'"]
impl StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;
}
impl KhrShaderSubgroupExtendedTypesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_shader_subgroup_extended_types\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderSubgroupExtendedTypesFn {}
unsafe impl Send for KhrShaderSubgroupExtendedTypesFn {}
unsafe impl Sync for KhrShaderSubgroupExtendedTypesFn {}
impl KhrShaderSubgroupExtendedTypesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_subgroup_extended_types'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
}
impl ExtExtension177Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_177\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension177Fn {}
unsafe impl Send for ExtExtension177Fn {}
unsafe impl Sync for ExtExtension177Fn {}
impl ExtExtension177Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl Khr8bitStorageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_8bit_storage\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Khr8bitStorageFn {}
unsafe impl Send for Khr8bitStorageFn {}
unsafe impl Sync for Khr8bitStorageFn {}
impl Khr8bitStorageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_8bit_storage'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
}
impl ExtExternalMemoryHostFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_host\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    p_host_pointer: *const c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> Result;
#[derive(Clone)]
pub struct ExtExternalMemoryHostFn {
    pub get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
}
unsafe impl Send for ExtExternalMemoryHostFn {}
unsafe impl Sync for ExtExternalMemoryHostFn {}
impl ExtExternalMemoryHostFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_host_pointer_properties_ext: unsafe {
                unsafe extern "system" fn get_memory_host_pointer_properties_ext(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _p_host_pointer: *const c_void,
                    _p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_host_pointer_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryHostPointerPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_host_pointer_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_external_memory_host'"]
impl ExternalMemoryHandleTypeFlags {
    pub const HOST_ALLOCATION_EXT: Self = Self(0b1000_0000);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_EXT_external_memory_host'"]
impl StructureType {
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1_000_178_000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1_000_178_001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1_000_178_002);
}
impl AmdBufferMarkerFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_buffer_marker\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[derive(Clone)]
pub struct AmdBufferMarkerFn {
    pub cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
}
unsafe impl Send for AmdBufferMarkerFn {}
unsafe impl Sync for AmdBufferMarkerFn {}
impl AmdBufferMarkerFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_write_buffer_marker_amd: unsafe {
                unsafe extern "system" fn cmd_write_buffer_marker_amd(
                    _command_buffer: CommandBuffer,
                    _pipeline_stage: PipelineStageFlags,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _marker: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_buffer_marker_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarkerAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_buffer_marker_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl KhrShaderAtomicInt64Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_atomic_int64\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderAtomicInt64Fn {}
unsafe impl Send for KhrShaderAtomicInt64Fn {}
unsafe impl Sync for KhrShaderAtomicInt64Fn {}
impl KhrShaderAtomicInt64Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_atomic_int64'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
}
impl KhrShaderClockFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_clock\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderClockFn {}
unsafe impl Send for KhrShaderClockFn {}
unsafe impl Sync for KhrShaderClockFn {}
impl KhrShaderClockFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_clock'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1_000_181_000);
}
impl AmdExtension183Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_183\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension183Fn {}
unsafe impl Send for AmdExtension183Fn {}
unsafe impl Sync for AmdExtension183Fn {}
impl AmdExtension183Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdPipelineCompilerControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_pipeline_compiler_control\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdPipelineCompilerControlFn {}
unsafe impl Send for AmdPipelineCompilerControlFn {}
unsafe impl Sync for AmdPipelineCompilerControlFn {}
impl AmdPipelineCompilerControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_pipeline_compiler_control'"]
impl StructureType {
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1_000_183_000);
}
impl ExtCalibratedTimestampsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_calibrated_timestamps\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoEXT,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct ExtCalibratedTimestampsFn {
    pub get_physical_device_calibrateable_time_domains_ext:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
    pub get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsEXT,
}
unsafe impl Send for ExtCalibratedTimestampsFn {}
unsafe impl Sync for ExtCalibratedTimestampsFn {}
impl ExtCalibratedTimestampsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_calibrateable_time_domains_ext: unsafe {
                unsafe extern "system" fn get_physical_device_calibrateable_time_domains_ext(
                    _physical_device: PhysicalDevice,
                    _p_time_domain_count: *mut u32,
                    _p_time_domains: *mut TimeDomainEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_calibrateable_time_domains_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_calibrateable_time_domains_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_calibrated_timestamps_ext: unsafe {
                unsafe extern "system" fn get_calibrated_timestamps_ext(
                    _device: Device,
                    _timestamp_count: u32,
                    _p_timestamp_infos: *const CalibratedTimestampInfoEXT,
                    _p_timestamps: *mut u64,
                    _p_max_deviation: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_calibrated_timestamps_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetCalibratedTimestampsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_calibrated_timestamps_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_calibrated_timestamps'"]
impl StructureType {
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1_000_184_000);
}
impl AmdShaderCorePropertiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdShaderCorePropertiesFn {}
unsafe impl Send for AmdShaderCorePropertiesFn {}
unsafe impl Sync for AmdShaderCorePropertiesFn {}
impl AmdShaderCorePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_shader_core_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1_000_185_000);
}
impl AmdExtension187Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_187\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension187Fn {}
unsafe impl Send for AmdExtension187Fn {}
unsafe impl Sync for AmdExtension187Fn {}
impl AmdExtension187Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtVideoDecodeH265Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h265\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtVideoDecodeH265Fn {}
unsafe impl Send for ExtVideoDecodeH265Fn {}
unsafe impl Sync for ExtVideoDecodeH265Fn {}
impl ExtVideoDecodeH265Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_video_decode_h265'"]
impl StructureType {
    pub const VIDEO_DECODE_H265_CAPABILITIES_EXT: Self = Self(1_000_187_000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1_000_187_001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1_000_187_002);
    pub const VIDEO_DECODE_H265_PROFILE_EXT: Self = Self(1_000_187_003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_EXT: Self = Self(1_000_187_004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT: Self = Self(1_000_187_005);
}
#[doc = "Generated from 'VK_EXT_video_decode_h265'"]
impl VideoCodecOperationFlagsKHR {
    pub const DECODE_H265_EXT: Self = Self(0b10);
}
impl KhrGlobalPriorityFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_global_priority\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrGlobalPriorityFn {}
unsafe impl Send for KhrGlobalPriorityFn {}
unsafe impl Sync for KhrGlobalPriorityFn {}
impl KhrGlobalPriorityFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_global_priority'"]
impl Result {
    pub const ERROR_NOT_PERMITTED_KHR: Self = Self(-1_000_174_001);
}
#[doc = "Generated from 'VK_KHR_global_priority'"]
impl StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1_000_174_000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(1_000_388_000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1_000_388_001);
}
impl AmdMemoryOverallocationBehaviorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_AMD_memory_overallocation_behavior\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdMemoryOverallocationBehaviorFn {}
unsafe impl Send for AmdMemoryOverallocationBehaviorFn {}
unsafe impl Sync for AmdMemoryOverallocationBehaviorFn {}
impl AmdMemoryOverallocationBehaviorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_memory_overallocation_behavior'"]
impl StructureType {
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1_000_189_000);
}
impl ExtVertexAttributeDivisorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_attribute_divisor\0")
        }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtVertexAttributeDivisorFn {}
unsafe impl Send for ExtVertexAttributeDivisorFn {}
unsafe impl Sync for ExtVertexAttributeDivisorFn {}
impl ExtVertexAttributeDivisorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_vertex_attribute_divisor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1_000_190_000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = Self(1_000_190_001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self = Self(1_000_190_002);
}
impl GgpFrameTokenFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_frame_token\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GgpFrameTokenFn {}
unsafe impl Send for GgpFrameTokenFn {}
unsafe impl Sync for GgpFrameTokenFn {}
impl GgpFrameTokenFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_GGP_frame_token'"]
impl StructureType {
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1_000_191_000);
}
impl ExtPipelineCreationFeedbackFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_creation_feedback\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPipelineCreationFeedbackFn {}
unsafe impl Send for ExtPipelineCreationFeedbackFn {}
unsafe impl Sync for ExtPipelineCreationFeedbackFn {}
impl ExtPipelineCreationFeedbackFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_feedback'"]
impl StructureType {
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self =
        Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
}
impl GoogleExtension194Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_194\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension194Fn {}
unsafe impl Send for GoogleExtension194Fn {}
unsafe impl Sync for GoogleExtension194Fn {}
impl GoogleExtension194Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension195Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_195\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension195Fn {}
unsafe impl Send for GoogleExtension195Fn {}
unsafe impl Sync for GoogleExtension195Fn {}
impl GoogleExtension195Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension196Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_196\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension196Fn {}
unsafe impl Send for GoogleExtension196Fn {}
unsafe impl Sync for GoogleExtension196Fn {}
impl GoogleExtension196Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_GOOGLE_extension_196'"]
impl PipelineCacheCreateFlags {
    pub const RESERVED_1_EXT: Self = Self(0b10);
}
impl KhrDriverPropertiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_driver_properties\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDriverPropertiesFn {}
unsafe impl Send for KhrDriverPropertiesFn {}
unsafe impl Sync for KhrDriverPropertiesFn {}
impl KhrDriverPropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_driver_properties'"]
impl DriverId {
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
}
#[doc = "Generated from 'VK_KHR_driver_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
}
impl KhrShaderFloatControlsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float_controls\0")
        }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct KhrShaderFloatControlsFn {}
unsafe impl Send for KhrShaderFloatControlsFn {}
unsafe impl Sync for KhrShaderFloatControlsFn {}
impl KhrShaderFloatControlsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_float_controls'"]
impl ShaderFloatControlsIndependence {
    pub const TYPE_32_ONLY_KHR: Self = Self::TYPE_32_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_shader_float_controls'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
}
impl NvShaderSubgroupPartitionedFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_subgroup_partitioned\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvShaderSubgroupPartitionedFn {}
unsafe impl Send for NvShaderSubgroupPartitionedFn {}
unsafe impl Sync for NvShaderSubgroupPartitionedFn {}
impl NvShaderSubgroupPartitionedFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_shader_subgroup_partitioned'"]
impl SubgroupFeatureFlags {
    pub const PARTITIONED_NV: Self = Self(0b1_0000_0000);
}
impl KhrDepthStencilResolveFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_depth_stencil_resolve\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDepthStencilResolveFn {}
unsafe impl Send for KhrDepthStencilResolveFn {}
unsafe impl Sync for KhrDepthStencilResolveFn {}
impl KhrDepthStencilResolveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_depth_stencil_resolve'"]
impl ResolveModeFlags {
    pub const NONE_KHR: Self = Self::NONE;
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    pub const MIN_KHR: Self = Self::MIN;
    pub const MAX_KHR: Self = Self::MAX;
}
#[doc = "Generated from 'VK_KHR_depth_stencil_resolve'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self =
        Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
}
impl KhrSwapchainMutableFormatFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain_mutable_format\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSwapchainMutableFormatFn {}
unsafe impl Send for KhrSwapchainMutableFormatFn {}
unsafe impl Sync for KhrSwapchainMutableFormatFn {}
impl KhrSwapchainMutableFormatFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_swapchain_mutable_format'"]
impl SwapchainCreateFlagsKHR {
    pub const MUTABLE_FORMAT: Self = Self(0b100);
}
impl NvComputeShaderDerivativesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_compute_shader_derivatives\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvComputeShaderDerivativesFn {}
unsafe impl Send for NvComputeShaderDerivativesFn {}
unsafe impl Sync for NvComputeShaderDerivativesFn {}
impl NvComputeShaderDerivativesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_compute_shader_derivatives'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(1_000_201_000);
}
impl NvMeshShaderFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_mesh_shader\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[derive(Clone)]
pub struct NvMeshShaderFn {
    pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
}
unsafe impl Send for NvMeshShaderFn {}
unsafe impl Sync for NvMeshShaderFn {}
impl NvMeshShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_mesh_tasks_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_nv(
                    _command_buffer: CommandBuffer,
                    _task_count: u32,
                    _first_task: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_nv(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_count_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_nv(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_count_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectCountNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_count_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl PipelineStageFlags {
    pub const TASK_SHADER_NV: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MESH_SHADER_NV: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl ShaderStageFlags {
    pub const TASK_NV: Self = Self(0b100_0000);
    pub const MESH_NV: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1_000_202_000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1_000_202_001);
}
impl NvFragmentShaderBarycentricFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shader_barycentric\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFragmentShaderBarycentricFn {}
unsafe impl Send for NvFragmentShaderBarycentricFn {}
unsafe impl Sync for NvFragmentShaderBarycentricFn {}
impl NvFragmentShaderBarycentricFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_fragment_shader_barycentric'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
}
impl NvShaderImageFootprintFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_image_footprint\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvShaderImageFootprintFn {}
unsafe impl Send for NvShaderImageFootprintFn {}
unsafe impl Sync for NvShaderImageFootprintFn {}
impl NvShaderImageFootprintFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_shader_image_footprint'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1_000_204_000);
}
impl NvScissorExclusiveFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_scissor_exclusive\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const Rect2D,
);
#[derive(Clone)]
pub struct NvScissorExclusiveFn {
    pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
}
unsafe impl Send for NvScissorExclusiveFn {}
unsafe impl Sync for NvScissorExclusiveFn {}
impl NvScissorExclusiveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_exclusive_scissor_nv: unsafe {
                unsafe extern "system" fn cmd_set_exclusive_scissor_nv(
                    _command_buffer: CommandBuffer,
                    _first_exclusive_scissor: u32,
                    _exclusive_scissor_count: u32,
                    _p_exclusive_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_exclusive_scissor_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetExclusiveScissorNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_exclusive_scissor_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_scissor_exclusive'"]
impl DynamicState {
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1_000_205_001);
}
#[doc = "Generated from 'VK_NV_scissor_exclusive'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1_000_205_000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1_000_205_002);
}
impl NvDeviceDiagnosticCheckpointsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_NV_device_diagnostic_checkpoints\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
);
#[derive(Clone)]
pub struct NvDeviceDiagnosticCheckpointsFn {
    pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
}
unsafe impl Send for NvDeviceDiagnosticCheckpointsFn {}
unsafe impl Sync for NvDeviceDiagnosticCheckpointsFn {}
impl NvDeviceDiagnosticCheckpointsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_checkpoint_nv: unsafe {
                unsafe extern "system" fn cmd_set_checkpoint_nv(
                    _command_buffer: CommandBuffer,
                    _p_checkpoint_marker: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_checkpoint_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCheckpointNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_checkpoint_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_queue_checkpoint_data_nv: unsafe {
                unsafe extern "system" fn get_queue_checkpoint_data_nv(
                    _queue: Queue,
                    _p_checkpoint_data_count: *mut u32,
                    _p_checkpoint_data: *mut CheckpointDataNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_queue_checkpoint_data_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetQueueCheckpointDataNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_queue_checkpoint_data_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_device_diagnostic_checkpoints'"]
impl StructureType {
    pub const CHECKPOINT_DATA_NV: Self = Self(1_000_206_000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1_000_206_001);
}
impl KhrTimelineSemaphoreFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_timeline_semaphore\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: Device,
    p_wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphore =
    unsafe extern "system" fn(device: Device, p_signal_info: *const SemaphoreSignalInfo) -> Result;
#[derive(Clone)]
pub struct KhrTimelineSemaphoreFn {
    pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValue,
    pub wait_semaphores_khr: PFN_vkWaitSemaphores,
    pub signal_semaphore_khr: PFN_vkSignalSemaphore,
}
unsafe impl Send for KhrTimelineSemaphoreFn {}
unsafe impl Sync for KhrTimelineSemaphoreFn {}
impl KhrTimelineSemaphoreFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_semaphore_counter_value_khr: unsafe {
                unsafe extern "system" fn get_semaphore_counter_value_khr(
                    _device: Device,
                    _semaphore: Semaphore,
                    _p_value: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_counter_value_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreCounterValueKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_counter_value_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            wait_semaphores_khr: unsafe {
                unsafe extern "system" fn wait_semaphores_khr(
                    _device: Device,
                    _p_wait_info: *const SemaphoreWaitInfo,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_semaphores_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphoresKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_semaphores_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            signal_semaphore_khr: unsafe {
                unsafe extern "system" fn signal_semaphore_khr(
                    _device: Device,
                    _p_signal_info: *const SemaphoreSignalInfo,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(signal_semaphore_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphoreKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    signal_semaphore_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl SemaphoreType {
    pub const BINARY_KHR: Self = Self::BINARY;
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl SemaphoreWaitFlags {
    pub const ANY_KHR: Self = Self::ANY;
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
}
impl KhrExtension209Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_209\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension209Fn {}
unsafe impl Send for KhrExtension209Fn {}
unsafe impl Sync for KhrExtension209Fn {}
impl KhrExtension209Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl IntelShaderIntegerFunctions2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_shader_integer_functions2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct IntelShaderIntegerFunctions2Fn {}
unsafe impl Send for IntelShaderIntegerFunctions2Fn {}
unsafe impl Sync for IntelShaderIntegerFunctions2Fn {}
impl IntelShaderIntegerFunctions2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_INTEL_shader_integer_functions2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1_000_209_000);
}
impl IntelPerformanceQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_performance_query\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    configuration: PerformanceConfigurationINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL,
) -> Result;
#[derive(Clone)]
pub struct IntelPerformanceQueryFn {
    pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    pub queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
}
unsafe impl Send for IntelPerformanceQueryFn {}
unsafe impl Sync for IntelPerformanceQueryFn {}
impl IntelPerformanceQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            initialize_performance_api_intel: unsafe {
                unsafe extern "system" fn initialize_performance_api_intel(
                    _device: Device,
                    _p_initialize_info: *const InitializePerformanceApiInfoINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(initialize_performance_api_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkInitializePerformanceApiINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    initialize_performance_api_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            uninitialize_performance_api_intel: unsafe {
                unsafe extern "system" fn uninitialize_performance_api_intel(_device: Device) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(uninitialize_performance_api_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUninitializePerformanceApiINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    uninitialize_performance_api_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_marker_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_marker_intel(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const PerformanceMarkerInfoINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_marker_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceMarkerINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_marker_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_stream_marker_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_stream_marker_intel(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_stream_marker_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceStreamMarkerINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_stream_marker_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_override_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_override_intel(
                    _command_buffer: CommandBuffer,
                    _p_override_info: *const PerformanceOverrideInfoINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_override_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceOverrideINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_override_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_performance_configuration_intel: unsafe {
                unsafe extern "system" fn acquire_performance_configuration_intel(
                    _device: Device,
                    _p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
                    _p_configuration: *mut PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquirePerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    acquire_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_performance_configuration_intel: unsafe {
                unsafe extern "system" fn release_performance_configuration_intel(
                    _device: Device,
                    _configuration: PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkReleasePerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    release_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_set_performance_configuration_intel: unsafe {
                unsafe extern "system" fn queue_set_performance_configuration_intel(
                    _queue: Queue,
                    _configuration: PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_set_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueSetPerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_set_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_performance_parameter_intel: unsafe {
                unsafe extern "system" fn get_performance_parameter_intel(
                    _device: Device,
                    _parameter: PerformanceParameterTypeINTEL,
                    _p_value: *mut PerformanceValueINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_performance_parameter_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPerformanceParameterINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_performance_parameter_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl ObjectType {
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1_000_210_000);
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl QueryType {
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1_000_210_000);
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl StructureType {
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1_000_210_000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1_000_210_001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1_000_210_002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1_000_210_003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1_000_210_004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1_000_210_005);
}
impl KhrVulkanMemoryModelFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_vulkan_memory_model\0") }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrVulkanMemoryModelFn {}
unsafe impl Send for KhrVulkanMemoryModelFn {}
unsafe impl Sync for KhrVulkanMemoryModelFn {}
impl KhrVulkanMemoryModelFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_vulkan_memory_model'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
}
impl ExtPciBusInfoFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pci_bus_info\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtPciBusInfoFn {}
unsafe impl Send for ExtPciBusInfoFn {}
unsafe impl Sync for ExtPciBusInfoFn {}
impl ExtPciBusInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_pci_bus_info'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1_000_212_000);
}
impl AmdDisplayNativeHdrFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_display_native_hdr\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
#[derive(Clone)]
pub struct AmdDisplayNativeHdrFn {
    pub set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
}
unsafe impl Send for AmdDisplayNativeHdrFn {}
unsafe impl Sync for AmdDisplayNativeHdrFn {}
impl AmdDisplayNativeHdrFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_local_dimming_amd: unsafe {
                unsafe extern "system" fn set_local_dimming_amd(
                    _device: Device,
                    _swap_chain: SwapchainKHR,
                    _local_dimming_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_local_dimming_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetLocalDimmingAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    set_local_dimming_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_AMD_display_native_hdr'"]
impl ColorSpaceKHR {
    pub const DISPLAY_NATIVE_AMD: Self = Self(1_000_213_000);
}
#[doc = "Generated from 'VK_AMD_display_native_hdr'"]
impl StructureType {
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1_000_213_000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1_000_213_001);
}
impl FuchsiaImagepipeSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_imagepipe_surface\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaImagepipeSurfaceFn {
    pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
}
unsafe impl Send for FuchsiaImagepipeSurfaceFn {}
unsafe impl Sync for FuchsiaImagepipeSurfaceFn {}
impl FuchsiaImagepipeSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_image_pipe_surface_fuchsia: unsafe {
                unsafe extern "system" fn create_image_pipe_surface_fuchsia(
                    _instance: Instance,
                    _p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_image_pipe_surface_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateImagePipeSurfaceFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_image_pipe_surface_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_imagepipe_surface'"]
impl StructureType {
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1_000_214_000);
}
impl KhrShaderTerminateInvocationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_terminate_invocation\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderTerminateInvocationFn {}
unsafe impl Send for KhrShaderTerminateInvocationFn {}
unsafe impl Sync for KhrShaderTerminateInvocationFn {}
impl KhrShaderTerminateInvocationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_terminate_invocation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
}
impl GoogleExtension217Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_217\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension217Fn {}
unsafe impl Send for GoogleExtension217Fn {}
unsafe impl Sync for GoogleExtension217Fn {}
impl GoogleExtension217Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtMetalSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtMetalSurfaceFn {
    pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}
unsafe impl Send for ExtMetalSurfaceFn {}
unsafe impl Sync for ExtMetalSurfaceFn {}
impl ExtMetalSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_metal_surface_ext: unsafe {
                unsafe extern "system" fn create_metal_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const MetalSurfaceCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_metal_surface_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    create_metal_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_metal_surface'"]
impl StructureType {
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_217_000);
}
impl ExtFragmentDensityMapFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtFragmentDensityMapFn {}
unsafe impl Send for ExtFragmentDensityMapFn {}
unsafe impl Sync for ExtFragmentDensityMapFn {}
impl ExtFragmentDensityMapFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl AccessFlags {
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl FormatFeatureFlags {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl FormatFeatureFlags2 {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageCreateFlags {
    pub const SUBSAMPLED_EXT: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageLayout {
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1_000_218_000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageUsageFlags {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageViewCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(0b1);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl PipelineStageFlags {
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl SamplerCreateFlags {
    pub const SUBSAMPLED_EXT: Self = Self(0b1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1_000_218_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1_000_218_001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1_000_218_002);
}
impl ExtExtension220Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_220\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension220Fn {}
unsafe impl Send for ExtExtension220Fn {}
unsafe impl Sync for ExtExtension220Fn {}
impl ExtExtension220Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension221Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_221\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension221Fn {}
unsafe impl Send for KhrExtension221Fn {}
unsafe impl Sync for KhrExtension221Fn {}
impl KhrExtension221Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_extension_221'"]
impl RenderPassCreateFlags {
    pub const RESERVED_0_KHR: Self = Self(0b1);
}
impl ExtScalarBlockLayoutFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_scalar_block_layout\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtScalarBlockLayoutFn {}
unsafe impl Send for ExtScalarBlockLayoutFn {}
unsafe impl Sync for ExtScalarBlockLayoutFn {}
impl ExtScalarBlockLayoutFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_scalar_block_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
}
impl ExtExtension223Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_223\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension223Fn {}
unsafe impl Send for ExtExtension223Fn {}
unsafe impl Sync for ExtExtension223Fn {}
impl ExtExtension223Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleHlslFunctionality1Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_hlsl_functionality1\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleHlslFunctionality1Fn {}
unsafe impl Send for GoogleHlslFunctionality1Fn {}
unsafe impl Sync for GoogleHlslFunctionality1Fn {}
impl GoogleHlslFunctionality1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleDecorateStringFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_decorate_string\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleDecorateStringFn {}
unsafe impl Send for GoogleDecorateStringFn {}
unsafe impl Sync for GoogleDecorateStringFn {}
impl GoogleDecorateStringFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtSubgroupSizeControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_subgroup_size_control\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSubgroupSizeControlFn {}
unsafe impl Send for ExtSubgroupSizeControlFn {}
unsafe impl Sync for ExtSubgroupSizeControlFn {}
impl ExtSubgroupSizeControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_subgroup_size_control'"]
impl PipelineShaderStageCreateFlags {
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
}
#[doc = "Generated from 'VK_EXT_subgroup_size_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
}
impl KhrFragmentShadingRateFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shading_rate\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_fragment_size: *const Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
);
#[derive(Clone)]
pub struct KhrFragmentShadingRateFn {
    pub get_physical_device_fragment_shading_rates_khr:
        PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
    pub cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
}
unsafe impl Send for KhrFragmentShadingRateFn {}
unsafe impl Sync for KhrFragmentShadingRateFn {}
impl KhrFragmentShadingRateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_fragment_shading_rates_khr: unsafe {
                unsafe extern "system" fn get_physical_device_fragment_shading_rates_khr(
                    _physical_device: PhysicalDevice,
                    _p_fragment_shading_rate_count: *mut u32,
                    _p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_fragment_shading_rates_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFragmentShadingRatesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_fragment_shading_rates_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_fragment_shading_rate_khr: unsafe {
                unsafe extern "system" fn cmd_set_fragment_shading_rate_khr(
                    _command_buffer: CommandBuffer,
                    _p_fragment_size: *const Extent2D,
                    _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_fragment_shading_rate_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetFragmentShadingRateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_fragment_shading_rate_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl AccessFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
        Self(0b1000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl DynamicState {
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1_000_226_000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl FormatFeatureFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl FormatFeatureFlags2 {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl ImageLayout {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1_000_164_003);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl ImageUsageFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl PipelineStageFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl StructureType {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1_000_226_000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1_000_226_001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1_000_226_002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1_000_226_003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1_000_226_004);
}
impl AmdShaderCoreProperties2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderCoreProperties2Fn {}
unsafe impl Send for AmdShaderCoreProperties2Fn {}
unsafe impl Sync for AmdShaderCoreProperties2Fn {}
impl AmdShaderCoreProperties2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_shader_core_properties2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1_000_227_000);
}
impl AmdExtension229Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_229\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension229Fn {}
unsafe impl Send for AmdExtension229Fn {}
unsafe impl Sync for AmdExtension229Fn {}
impl AmdExtension229Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdDeviceCoherentMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_device_coherent_memory\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdDeviceCoherentMemoryFn {}
unsafe impl Send for AmdDeviceCoherentMemoryFn {}
unsafe impl Sync for AmdDeviceCoherentMemoryFn {}
impl AmdDeviceCoherentMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_device_coherent_memory'"]
impl MemoryPropertyFlags {
    pub const DEVICE_COHERENT_AMD: Self = Self(0b100_0000);
    pub const DEVICE_UNCACHED_AMD: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_AMD_device_coherent_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1_000_229_000);
}
impl AmdExtension231Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_231\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension231Fn {}
unsafe impl Send for AmdExtension231Fn {}
unsafe impl Sync for AmdExtension231Fn {}
impl AmdExtension231Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension232Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_232\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension232Fn {}
unsafe impl Send for AmdExtension232Fn {}
unsafe impl Sync for AmdExtension232Fn {}
impl AmdExtension232Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension233Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_233\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension233Fn {}
unsafe impl Send for AmdExtension233Fn {}
unsafe impl Sync for AmdExtension233Fn {}
impl AmdExtension233Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension234Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_234\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension234Fn {}
unsafe impl Send for AmdExtension234Fn {}
unsafe impl Sync for AmdExtension234Fn {}
impl AmdExtension234Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderImageAtomicInt64Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_image_atomic_int64\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderImageAtomicInt64Fn {}
unsafe impl Send for ExtShaderImageAtomicInt64Fn {}
unsafe impl Sync for ExtShaderImageAtomicInt64Fn {}
impl ExtShaderImageAtomicInt64Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_shader_image_atomic_int64'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1_000_234_000);
}
impl AmdExtension236Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_236\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension236Fn {}
unsafe impl Send for AmdExtension236Fn {}
unsafe impl Sync for AmdExtension236Fn {}
impl AmdExtension236Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrSpirv14Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_spirv_1_4\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSpirv14Fn {}
unsafe impl Send for KhrSpirv14Fn {}
unsafe impl Sync for KhrSpirv14Fn {}
impl KhrSpirv14Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtMemoryBudgetFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_budget\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMemoryBudgetFn {}
unsafe impl Send for ExtMemoryBudgetFn {}
unsafe impl Sync for ExtMemoryBudgetFn {}
impl ExtMemoryBudgetFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_memory_budget'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1_000_237_000);
}
impl ExtMemoryPriorityFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_priority\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMemoryPriorityFn {}
unsafe impl Send for ExtMemoryPriorityFn {}
unsafe impl Sync for ExtMemoryPriorityFn {}
impl ExtMemoryPriorityFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_memory_priority'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1_000_238_000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1_000_238_001);
}
impl KhrSurfaceProtectedCapabilitiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_surface_protected_capabilities\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSurfaceProtectedCapabilitiesFn {}
unsafe impl Send for KhrSurfaceProtectedCapabilitiesFn {}
unsafe impl Sync for KhrSurfaceProtectedCapabilitiesFn {}
impl KhrSurfaceProtectedCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_surface_protected_capabilities'"]
impl StructureType {
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1_000_239_000);
}
impl NvDedicatedAllocationImageAliasingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_NV_dedicated_allocation_image_aliasing\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvDedicatedAllocationImageAliasingFn {}
unsafe impl Send for NvDedicatedAllocationImageAliasingFn {}
unsafe impl Sync for NvDedicatedAllocationImageAliasingFn {}
impl NvDedicatedAllocationImageAliasingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_dedicated_allocation_image_aliasing'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
        Self(1_000_240_000);
}
impl KhrSeparateDepthStencilLayoutsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_separate_depth_stencil_layouts\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSeparateDepthStencilLayoutsFn {}
unsafe impl Send for KhrSeparateDepthStencilLayoutsFn {}
unsafe impl Sync for KhrSeparateDepthStencilLayoutsFn {}
impl KhrSeparateDepthStencilLayoutsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_separate_depth_stencil_layouts'"]
impl ImageLayout {
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_separate_depth_stencil_layouts'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
}
impl IntelExtension243Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_extension_243\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct IntelExtension243Fn {}
unsafe impl Send for IntelExtension243Fn {}
unsafe impl Sync for IntelExtension243Fn {}
impl IntelExtension243Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl MesaExtension244Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MESA_extension_244\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct MesaExtension244Fn {}
unsafe impl Send for MesaExtension244Fn {}
unsafe impl Sync for MesaExtension244Fn {}
impl MesaExtension244Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtBufferDeviceAddressFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_buffer_device_address\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;
#[derive(Clone)]
pub struct ExtBufferDeviceAddressFn {
    pub get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddress,
}
unsafe impl Send for ExtBufferDeviceAddressFn {}
unsafe impl Sync for ExtBufferDeviceAddressFn {}
impl ExtBufferDeviceAddressFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_buffer_device_address_ext: unsafe {
                unsafe extern "system" fn get_buffer_device_address_ext(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_device_address_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferDeviceAddressEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_device_address_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl BufferCreateFlags {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl BufferUsageFlags {
    pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl Result {
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1_000_244_000);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1_000_244_002);
}
impl ExtToolingInfoFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_tooling_info\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut PhysicalDeviceToolProperties,
) -> Result;
#[derive(Clone)]
pub struct ExtToolingInfoFn {
    pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolProperties,
}
unsafe impl Send for ExtToolingInfoFn {}
unsafe impl Sync for ExtToolingInfoFn {}
impl ExtToolingInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_tool_properties_ext: unsafe {
                unsafe extern "system" fn get_physical_device_tool_properties_ext(
                    _physical_device: PhysicalDevice,
                    _p_tool_count: *mut u32,
                    _p_tool_properties: *mut PhysicalDeviceToolProperties,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_tool_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceToolPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_tool_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_tooling_info'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
}
#[doc = "Generated from 'VK_EXT_tooling_info'"]
impl ToolPurposeFlags {
    pub const DEBUG_REPORTING_EXT: Self = Self(0b10_0000);
    pub const DEBUG_MARKERS_EXT: Self = Self(0b100_0000);
}
impl ExtSeparateStencilUsageFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_separate_stencil_usage\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtSeparateStencilUsageFn {}
unsafe impl Send for ExtSeparateStencilUsageFn {}
unsafe impl Sync for ExtSeparateStencilUsageFn {}
impl ExtSeparateStencilUsageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_separate_stencil_usage'"]
impl StructureType {
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
}
impl ExtValidationFeaturesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_features\0") }
    }
    pub const SPEC_VERSION: u32 = 5u32;
}
#[derive(Clone)]
pub struct ExtValidationFeaturesFn {}
unsafe impl Send for ExtValidationFeaturesFn {}
unsafe impl Sync for ExtValidationFeaturesFn {}
impl ExtValidationFeaturesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_validation_features'"]
impl StructureType {
    pub const VALIDATION_FEATURES_EXT: Self = Self(1_000_247_000);
}
impl KhrPresentWaitFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_wait\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> Result;
#[derive(Clone)]
pub struct KhrPresentWaitFn {
    pub wait_for_present_khr: PFN_vkWaitForPresentKHR,
}
unsafe impl Send for KhrPresentWaitFn {}
unsafe impl Sync for KhrPresentWaitFn {}
impl KhrPresentWaitFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            wait_for_present_khr: unsafe {
                unsafe extern "system" fn wait_for_present_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _present_id: u64,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_for_present_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitForPresentKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_for_present_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_present_wait'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1_000_248_000);
}
impl NvCooperativeMatrixFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_cooperative_matrix\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV,
)
    -> Result;
#[derive(Clone)]
pub struct NvCooperativeMatrixFn {
    pub get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}
unsafe impl Send for NvCooperativeMatrixFn {}
unsafe impl Sync for NvCooperativeMatrixFn {}
impl NvCooperativeMatrixFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_cooperative_matrix_properties_nv: unsafe {
                unsafe extern "system" fn get_physical_device_cooperative_matrix_properties_nv(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut CooperativeMatrixPropertiesNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_cooperative_matrix_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_cooperative_matrix_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_cooperative_matrix'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1_000_249_000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1_000_249_001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1_000_249_002);
}
impl NvCoverageReductionModeFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_coverage_reduction_mode\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> Result;
#[derive(Clone)]
pub struct NvCoverageReductionModeFn {
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}
unsafe impl Send for NvCoverageReductionModeFn {}
unsafe impl Sync for NvCoverageReductionModeFn {}
impl NvCoverageReductionModeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: unsafe {
                unsafe extern "system" fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                    _physical_device: PhysicalDevice,
                    _p_combination_count: *mut u32,
                    _p_combinations: *mut FramebufferMixedSamplesCombinationNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(
                            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
                        )
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_coverage_reduction_mode'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1_000_250_000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1_000_250_001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1_000_250_002);
}
impl ExtFragmentShaderInterlockFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_shader_interlock\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtFragmentShaderInterlockFn {}
unsafe impl Send for ExtFragmentShaderInterlockFn {}
unsafe impl Sync for ExtFragmentShaderInterlockFn {}
impl ExtFragmentShaderInterlockFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_fragment_shader_interlock'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1_000_251_000);
}
impl ExtYcbcrImageArraysFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_image_arrays\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtYcbcrImageArraysFn {}
unsafe impl Send for ExtYcbcrImageArraysFn {}
unsafe impl Sync for ExtYcbcrImageArraysFn {}
impl ExtYcbcrImageArraysFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_ycbcr_image_arrays'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1_000_252_000);
}
impl KhrUniformBufferStandardLayoutFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_uniform_buffer_standard_layout\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrUniformBufferStandardLayoutFn {}
unsafe impl Send for KhrUniformBufferStandardLayoutFn {}
unsafe impl Sync for KhrUniformBufferStandardLayoutFn {}
impl KhrUniformBufferStandardLayoutFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_uniform_buffer_standard_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
}
impl ExtProvokingVertexFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_provoking_vertex\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtProvokingVertexFn {}
unsafe impl Send for ExtProvokingVertexFn {}
unsafe impl Sync for ExtProvokingVertexFn {}
impl ExtProvokingVertexFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_provoking_vertex'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1_000_254_000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
        Self(1_000_254_001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1_000_254_002);
}
impl ExtFullScreenExclusiveFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_full_screen_exclusive\0")
        }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtFullScreenExclusiveFn {
    pub get_physical_device_surface_present_modes2_ext:
        PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
    pub acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    pub release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    pub get_device_group_surface_present_modes2_ext: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
}
unsafe impl Send for ExtFullScreenExclusiveFn {}
unsafe impl Sync for ExtFullScreenExclusiveFn {}
impl ExtFullScreenExclusiveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_present_modes2_ext: unsafe {
                unsafe extern "system" fn get_physical_device_surface_present_modes2_ext(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
                    _p_present_mode_count: *mut u32,
                    _p_present_modes: *mut PresentModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_present_modes2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_present_modes2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_full_screen_exclusive_mode_ext: unsafe {
                unsafe extern "system" fn acquire_full_screen_exclusive_mode_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_full_screen_exclusive_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquireFullScreenExclusiveModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    acquire_full_screen_exclusive_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_full_screen_exclusive_mode_ext: unsafe {
                unsafe extern "system" fn release_full_screen_exclusive_mode_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_full_screen_exclusive_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkReleaseFullScreenExclusiveModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    release_full_screen_exclusive_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes2_ext: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes2_ext(
                    _device: Device,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModes2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_full_screen_exclusive'"]
impl Result {
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1_000_255_000);
}
#[doc = "Generated from 'VK_EXT_full_screen_exclusive'"]
impl StructureType {
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1_000_255_000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1_000_255_002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1_000_255_001);
}
impl ExtHeadlessSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_headless_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtHeadlessSurfaceFn {
    pub create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}
unsafe impl Send for ExtHeadlessSurfaceFn {}
unsafe impl Sync for ExtHeadlessSurfaceFn {}
impl ExtHeadlessSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_headless_surface_ext: unsafe {
                unsafe extern "system" fn create_headless_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const HeadlessSurfaceCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_headless_surface_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateHeadlessSurfaceEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_headless_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_headless_surface'"]
impl StructureType {
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_256_000);
}
impl KhrBufferDeviceAddressFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_buffer_device_address\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> u64;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
#[derive(Clone)]
pub struct KhrBufferDeviceAddressFn {
    pub get_buffer_device_address_khr: crate::vk::PFN_vkGetBufferDeviceAddress,
    pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
unsafe impl Send for KhrBufferDeviceAddressFn {}
unsafe impl Sync for KhrBufferDeviceAddressFn {}
impl KhrBufferDeviceAddressFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_buffer_device_address_khr: unsafe {
                unsafe extern "system" fn get_buffer_device_address_khr(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_device_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferDeviceAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_device_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_opaque_capture_address_khr: unsafe {
                unsafe extern "system" fn get_buffer_opaque_capture_address_khr(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_opaque_capture_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_opaque_capture_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_memory_opaque_capture_address_khr: unsafe {
                unsafe extern "system" fn get_device_memory_opaque_capture_address_khr(
                    _device: Device,
                    _p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_memory_opaque_capture_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_memory_opaque_capture_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl BufferCreateFlags {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl BufferUsageFlags {
    pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl MemoryAllocateFlags {
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl Result {
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self =
        Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self =
        Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self =
        Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self =
        Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
}
impl ExtExtension259Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_259\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension259Fn {}
unsafe impl Send for ExtExtension259Fn {}
unsafe impl Sync for ExtExtension259Fn {}
impl ExtExtension259Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtLineRasterizationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_line_rasterization\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);
#[derive(Clone)]
pub struct ExtLineRasterizationFn {
    pub cmd_set_line_stipple_ext: PFN_vkCmdSetLineStippleEXT,
}
unsafe impl Send for ExtLineRasterizationFn {}
unsafe impl Sync for ExtLineRasterizationFn {}
impl ExtLineRasterizationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_line_stipple_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_stipple_ext(
                    _command_buffer: CommandBuffer,
                    _line_stipple_factor: u32,
                    _line_stipple_pattern: u16,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_stipple_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_stipple_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_line_rasterization'"]
impl DynamicState {
    pub const LINE_STIPPLE_EXT: Self = Self(1_000_259_000);
}
#[doc = "Generated from 'VK_EXT_line_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1_000_259_000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1_000_259_001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1_000_259_002);
}
impl ExtShaderAtomicFloatFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderAtomicFloatFn {}
unsafe impl Send for ExtShaderAtomicFloatFn {}
unsafe impl Sync for ExtShaderAtomicFloatFn {}
impl ExtShaderAtomicFloatFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_shader_atomic_float'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1_000_260_000);
}
impl ExtHostQueryResetFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_host_query_reset\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[derive(Clone)]
pub struct ExtHostQueryResetFn {
    pub reset_query_pool_ext: PFN_vkResetQueryPool,
}
unsafe impl Send for ExtHostQueryResetFn {}
unsafe impl Sync for ExtHostQueryResetFn {}
impl ExtHostQueryResetFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            reset_query_pool_ext: unsafe {
                unsafe extern "system" fn reset_query_pool_ext(
                    _device: Device,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(reset_query_pool_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPoolEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_query_pool_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_host_query_reset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
}
impl GgpExtension263Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_263\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension263Fn {}
unsafe impl Send for GgpExtension263Fn {}
unsafe impl Sync for GgpExtension263Fn {}
impl GgpExtension263Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl BrcmExtension264Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_BRCM_extension_264\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct BrcmExtension264Fn {}
unsafe impl Send for BrcmExtension264Fn {}
unsafe impl Sync for BrcmExtension264Fn {}
impl BrcmExtension264Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl BrcmExtension265Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_BRCM_extension_265\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct BrcmExtension265Fn {}
unsafe impl Send for BrcmExtension265Fn {}
unsafe impl Sync for BrcmExtension265Fn {}
impl BrcmExtension265Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtIndexTypeUint8Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_index_type_uint8\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtIndexTypeUint8Fn {}
unsafe impl Send for ExtIndexTypeUint8Fn {}
unsafe impl Sync for ExtIndexTypeUint8Fn {}
impl ExtIndexTypeUint8Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_index_type_uint8'"]
impl IndexType {
    pub const UINT8_EXT: Self = Self(1_000_265_000);
}
#[doc = "Generated from 'VK_EXT_index_type_uint8'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self = Self(1_000_265_000);
}
impl ExtExtension267Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_267\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension267Fn {}
unsafe impl Send for ExtExtension267Fn {}
unsafe impl Sync for ExtExtension267Fn {}
impl ExtExtension267Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtendedDynamicStateFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
    p_strides: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);
#[derive(Clone)]
pub struct ExtExtendedDynamicStateFn {
    pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
    pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
    pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
    pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
    pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
    pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
}
unsafe impl Send for ExtExtendedDynamicStateFn {}
unsafe impl Sync for ExtExtendedDynamicStateFn {}
impl ExtExtendedDynamicStateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_cull_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_cull_mode_ext(
                    _command_buffer: CommandBuffer,
                    _cull_mode: CullModeFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_cull_mode_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_cull_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_front_face_ext: unsafe {
                unsafe extern "system" fn cmd_set_front_face_ext(
                    _command_buffer: CommandBuffer,
                    _front_face: FrontFace,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_front_face_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_front_face_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_topology_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_topology_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_topology: PrimitiveTopology,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_topology_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveTopologyEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_topology_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _viewport_count: u32,
                    _p_viewports: *const Viewport,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_scissor_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _scissor_count: u32,
                    _p_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_scissor_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetScissorWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_scissor_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_vertex_buffers2_ext: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                    _p_strides: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_vertex_buffers2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindVertexBuffers2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_vertex_buffers2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_write_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_write_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_write_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthWriteEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_write_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_compare_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                    _command_buffer: CommandBuffer,
                    _depth_compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_compare_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_compare_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bounds_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bounds_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bounds_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBoundsTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bounds_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _stencil_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetStencilTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_op_ext(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _fail_op: StencilOp,
                    _pass_op: StencilOp,
                    _depth_fail_op: StencilOp,
                    _compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state'"]
impl DynamicState {
    pub const CULL_MODE_EXT: Self = Self::CULL_MODE;
    pub const FRONT_FACE_EXT: Self = Self::FRONT_FACE;
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self::PRIMITIVE_TOPOLOGY;
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self::VIEWPORT_WITH_COUNT;
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self::SCISSOR_WITH_COUNT;
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self::VERTEX_INPUT_BINDING_STRIDE;
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self::DEPTH_TEST_ENABLE;
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self::DEPTH_WRITE_ENABLE;
    pub const DEPTH_COMPARE_OP_EXT: Self = Self::DEPTH_COMPARE_OP;
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self::DEPTH_BOUNDS_TEST_ENABLE;
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self::STENCIL_TEST_ENABLE;
    pub const STENCIL_OP_EXT: Self = Self::STENCIL_OP;
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1_000_267_000);
}
impl KhrDeferredHostOperationsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_deferred_host_operations\0")
        }
    }
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    operation: DeferredOperationKHR,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
#[derive(Clone)]
pub struct KhrDeferredHostOperationsFn {
    pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    pub get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
}
unsafe impl Send for KhrDeferredHostOperationsFn {}
unsafe impl Sync for KhrDeferredHostOperationsFn {}
impl KhrDeferredHostOperationsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_deferred_operation_khr: unsafe {
                unsafe extern "system" fn create_deferred_operation_khr(
                    _device: Device,
                    _p_allocator: *const AllocationCallbacks,
                    _p_deferred_operation: *mut DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_deferred_operation_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDeferredOperationKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_deferred_operation_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_deferred_operation_khr: unsafe {
                unsafe extern "system" fn destroy_deferred_operation_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_deferred_operation_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDeferredOperationKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_deferred_operation_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_deferred_operation_max_concurrency_khr: unsafe {
                unsafe extern "system" fn get_deferred_operation_max_concurrency_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> u32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_deferred_operation_max_concurrency_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationMaxConcurrencyKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_deferred_operation_max_concurrency_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_deferred_operation_result_khr: unsafe {
                unsafe extern "system" fn get_deferred_operation_result_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_deferred_operation_result_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationResultKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_deferred_operation_result_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            deferred_operation_join_khr: unsafe {
                unsafe extern "system" fn deferred_operation_join_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(deferred_operation_join_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDeferredOperationJoinKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    deferred_operation_join_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_deferred_host_operations'"]
impl ObjectType {
    pub const DEFERRED_OPERATION_KHR: Self = Self(1_000_268_000);
}
#[doc = "Generated from 'VK_KHR_deferred_host_operations'"]
impl Result {
    pub const THREAD_IDLE_KHR: Self = Self(1_000_268_000);
    pub const THREAD_DONE_KHR: Self = Self(1_000_268_001);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1_000_268_002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1_000_268_003);
}
impl KhrPipelineExecutablePropertiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_pipeline_executable_properties\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> Result;
#[derive(Clone)]
pub struct KhrPipelineExecutablePropertiesFn {
    pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    pub get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
unsafe impl Send for KhrPipelineExecutablePropertiesFn {}
unsafe impl Sync for KhrPipelineExecutablePropertiesFn {}
impl KhrPipelineExecutablePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_pipeline_executable_properties_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_properties_khr(
                    _device: Device,
                    _p_pipeline_info: *const PipelineInfoKHR,
                    _p_executable_count: *mut u32,
                    _p_properties: *mut PipelineExecutablePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutablePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_pipeline_executable_statistics_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_statistics_khr(
                    _device: Device,
                    _p_executable_info: *const PipelineExecutableInfoKHR,
                    _p_statistic_count: *mut u32,
                    _p_statistics: *mut PipelineExecutableStatisticKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_statistics_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableStatisticsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_statistics_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_pipeline_executable_internal_representations_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_internal_representations_khr(
                    _device: Device,
                    _p_executable_info: *const PipelineExecutableInfoKHR,
                    _p_internal_representation_count: *mut u32,
                    _p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_internal_representations_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableInternalRepresentationsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_internal_representations_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_pipeline_executable_properties'"]
impl PipelineCreateFlags {
    pub const CAPTURE_STATISTICS_KHR: Self = Self(0b100_0000);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_KHR_pipeline_executable_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self =
        Self(1_000_269_000);
    pub const PIPELINE_INFO_KHR: Self = Self(1_000_269_001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1_000_269_002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1_000_269_003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1_000_269_004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1_000_269_005);
}
impl IntelExtension271Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_extension_271\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct IntelExtension271Fn {}
unsafe impl Send for IntelExtension271Fn {}
unsafe impl Sync for IntelExtension271Fn {}
impl IntelExtension271Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_INTEL_extension_271'"]
impl ImageUsageFlags {
    pub const RESERVED_22_EXT: Self = Self(0b100_0000_0000_0000_0000_0000);
}
impl IntelExtension272Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_extension_272\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct IntelExtension272Fn {}
unsafe impl Send for IntelExtension272Fn {}
unsafe impl Sync for IntelExtension272Fn {}
impl IntelExtension272Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl IntelExtension273Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_extension_273\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct IntelExtension273Fn {}
unsafe impl Send for IntelExtension273Fn {}
unsafe impl Sync for IntelExtension273Fn {}
impl IntelExtension273Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderAtomicFloat2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderAtomicFloat2Fn {}
unsafe impl Send for ExtShaderAtomicFloat2Fn {}
unsafe impl Sync for ExtShaderAtomicFloat2Fn {}
impl ExtShaderAtomicFloat2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_shader_atomic_float2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1_000_273_000);
}
impl KhrExtension275Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_275\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension275Fn {}
unsafe impl Send for KhrExtension275Fn {}
unsafe impl Sync for KhrExtension275Fn {}
impl KhrExtension275Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension276Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_276\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension276Fn {}
unsafe impl Send for KhrExtension276Fn {}
unsafe impl Sync for KhrExtension276Fn {}
impl KhrExtension276Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderDemoteToHelperInvocationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_shader_demote_to_helper_invocation\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderDemoteToHelperInvocationFn {}
unsafe impl Send for ExtShaderDemoteToHelperInvocationFn {}
unsafe impl Sync for ExtShaderDemoteToHelperInvocationFn {}
impl ExtShaderDemoteToHelperInvocationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_shader_demote_to_helper_invocation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
}
impl NvDeviceGeneratedCommandsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_generated_commands\0")
        }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks,
);
#[derive(Clone)]
pub struct NvDeviceGeneratedCommandsFn {
    pub get_generated_commands_memory_requirements_nv:
        PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    pub cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    pub cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    pub cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    pub create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    pub destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
}
unsafe impl Send for NvDeviceGeneratedCommandsFn {}
unsafe impl Sync for NvDeviceGeneratedCommandsFn {}
impl NvDeviceGeneratedCommandsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_generated_commands_memory_requirements_nv: unsafe {
                unsafe extern "system" fn get_generated_commands_memory_requirements_nv(
                    _device: Device,
                    _p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
                    _p_memory_requirements: *mut MemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_generated_commands_memory_requirements_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetGeneratedCommandsMemoryRequirementsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_generated_commands_memory_requirements_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_preprocess_generated_commands_nv: unsafe {
                unsafe extern "system" fn cmd_preprocess_generated_commands_nv(
                    _command_buffer: CommandBuffer,
                    _p_generated_commands_info: *const GeneratedCommandsInfoNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_preprocess_generated_commands_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPreprocessGeneratedCommandsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_preprocess_generated_commands_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_execute_generated_commands_nv: unsafe {
                unsafe extern "system" fn cmd_execute_generated_commands_nv(
                    _command_buffer: CommandBuffer,
                    _is_preprocessed: Bool32,
                    _p_generated_commands_info: *const GeneratedCommandsInfoNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_execute_generated_commands_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdExecuteGeneratedCommandsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_execute_generated_commands_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_pipeline_shader_group_nv: unsafe {
                unsafe extern "system" fn cmd_bind_pipeline_shader_group_nv(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _pipeline: Pipeline,
                    _group_index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_pipeline_shader_group_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindPipelineShaderGroupNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_pipeline_shader_group_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_indirect_commands_layout_nv: unsafe {
                unsafe extern "system" fn create_indirect_commands_layout_nv(
                    _device: Device,
                    _p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
                    _p_allocator: *const AllocationCallbacks,
                    _p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_indirect_commands_layout_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateIndirectCommandsLayoutNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_indirect_commands_layout_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_indirect_commands_layout_nv: unsafe {
                unsafe extern "system" fn destroy_indirect_commands_layout_nv(
                    _device: Device,
                    _indirect_commands_layout: IndirectCommandsLayoutNV,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_indirect_commands_layout_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyIndirectCommandsLayoutNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_indirect_commands_layout_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl AccessFlags {
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl ObjectType {
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1_000_277_000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl PipelineCreateFlags {
    pub const INDIRECT_BINDABLE_NV: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl PipelineStageFlags {
    pub const COMMAND_PREPROCESS_NV: Self = Self(0b10_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1_000_277_000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1_000_277_001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1_000_277_002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1_000_277_003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1_000_277_004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1_000_277_005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1_000_277_006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1_000_277_007);
}
impl NvInheritedViewportScissorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_inherited_viewport_scissor\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvInheritedViewportScissorFn {}
unsafe impl Send for NvInheritedViewportScissorFn {}
unsafe impl Sync for NvInheritedViewportScissorFn {}
impl NvInheritedViewportScissorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_inherited_viewport_scissor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1_000_278_000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1_000_278_001);
}
impl KhrExtension280Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_280\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension280Fn {}
unsafe impl Send for KhrExtension280Fn {}
unsafe impl Sync for KhrExtension280Fn {}
impl KhrExtension280Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrShaderIntegerDotProductFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_integer_dot_product\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderIntegerDotProductFn {}
unsafe impl Send for KhrShaderIntegerDotProductFn {}
unsafe impl Sync for KhrShaderIntegerDotProductFn {}
impl KhrShaderIntegerDotProductFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_integer_dot_product'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
}
impl ExtTexelBufferAlignmentFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_texel_buffer_alignment\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtTexelBufferAlignmentFn {}
unsafe impl Send for ExtTexelBufferAlignmentFn {}
unsafe impl Sync for ExtTexelBufferAlignmentFn {}
impl ExtTexelBufferAlignmentFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_texel_buffer_alignment'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1_000_281_000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
}
impl QcomRenderPassTransformFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_transform\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct QcomRenderPassTransformFn {}
unsafe impl Send for QcomRenderPassTransformFn {}
unsafe impl Sync for QcomRenderPassTransformFn {}
impl QcomRenderPassTransformFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_render_pass_transform'"]
impl RenderPassCreateFlags {
    pub const TRANSFORM_QCOM: Self = Self(0b10);
}
#[doc = "Generated from 'VK_QCOM_render_pass_transform'"]
impl StructureType {
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self =
        Self(1_000_282_000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1_000_282_001);
}
impl ExtExtension284Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_284\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension284Fn {}
unsafe impl Send for ExtExtension284Fn {}
unsafe impl Sync for ExtExtension284Fn {}
impl ExtExtension284Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtDeviceMemoryReportFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_memory_report\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtDeviceMemoryReportFn {}
unsafe impl Send for ExtDeviceMemoryReportFn {}
unsafe impl Sync for ExtDeviceMemoryReportFn {}
impl ExtDeviceMemoryReportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_device_memory_report'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1_000_284_000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1_000_284_001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1_000_284_002);
}
impl ExtAcquireDrmDisplayFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_drm_display\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    display: DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtAcquireDrmDisplayFn {
    pub acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    pub get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
}
unsafe impl Send for ExtAcquireDrmDisplayFn {}
unsafe impl Sync for ExtAcquireDrmDisplayFn {}
impl ExtAcquireDrmDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_drm_display_ext: unsafe {
                unsafe extern "system" fn acquire_drm_display_ext(
                    _physical_device: PhysicalDevice,
                    _drm_fd: i32,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_drm_display_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireDrmDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_drm_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_drm_display_ext: unsafe {
                unsafe extern "system" fn get_drm_display_ext(
                    _physical_device: PhysicalDevice,
                    _drm_fd: i32,
                    _connector_id: u32,
                    _display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_drm_display_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDrmDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_drm_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtRobustness2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_robustness2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtRobustness2Fn {}
unsafe impl Send for ExtRobustness2Fn {}
unsafe impl Sync for ExtRobustness2Fn {}
impl ExtRobustness2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_robustness2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1_000_286_000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1_000_286_001);
}
impl ExtCustomBorderColorFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_custom_border_color\0") }
    }
    pub const SPEC_VERSION: u32 = 12u32;
}
#[derive(Clone)]
pub struct ExtCustomBorderColorFn {}
unsafe impl Send for ExtCustomBorderColorFn {}
unsafe impl Sync for ExtCustomBorderColorFn {}
impl ExtCustomBorderColorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_custom_border_color'"]
impl BorderColor {
    pub const FLOAT_CUSTOM_EXT: Self = Self(1_000_287_003);
    pub const INT_CUSTOM_EXT: Self = Self(1_000_287_004);
}
#[doc = "Generated from 'VK_EXT_custom_border_color'"]
impl StructureType {
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1_000_287_000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1_000_287_001);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1_000_287_002);
}
impl ExtExtension289Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_289\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension289Fn {}
unsafe impl Send for ExtExtension289Fn {}
unsafe impl Sync for ExtExtension289Fn {}
impl ExtExtension289Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_289'"]
impl Format {
    pub const ASTC_3X3X3_UNORM_BLOCK_EXT: Self = Self(1_000_288_000);
    pub const ASTC_3X3X3_SRGB_BLOCK_EXT: Self = Self(1_000_288_001);
    pub const ASTC_3X3X3_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_002);
    pub const ASTC_4X3X3_UNORM_BLOCK_EXT: Self = Self(1_000_288_003);
    pub const ASTC_4X3X3_SRGB_BLOCK_EXT: Self = Self(1_000_288_004);
    pub const ASTC_4X3X3_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_005);
    pub const ASTC_4X4X3_UNORM_BLOCK_EXT: Self = Self(1_000_288_006);
    pub const ASTC_4X4X3_SRGB_BLOCK_EXT: Self = Self(1_000_288_007);
    pub const ASTC_4X4X3_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_008);
    pub const ASTC_4X4X4_UNORM_BLOCK_EXT: Self = Self(1_000_288_009);
    pub const ASTC_4X4X4_SRGB_BLOCK_EXT: Self = Self(1_000_288_010);
    pub const ASTC_4X4X4_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_011);
    pub const ASTC_5X4X4_UNORM_BLOCK_EXT: Self = Self(1_000_288_012);
    pub const ASTC_5X4X4_SRGB_BLOCK_EXT: Self = Self(1_000_288_013);
    pub const ASTC_5X4X4_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_014);
    pub const ASTC_5X5X4_UNORM_BLOCK_EXT: Self = Self(1_000_288_015);
    pub const ASTC_5X5X4_SRGB_BLOCK_EXT: Self = Self(1_000_288_016);
    pub const ASTC_5X5X4_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_017);
    pub const ASTC_5X5X5_UNORM_BLOCK_EXT: Self = Self(1_000_288_018);
    pub const ASTC_5X5X5_SRGB_BLOCK_EXT: Self = Self(1_000_288_019);
    pub const ASTC_5X5X5_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_020);
    pub const ASTC_6X5X5_UNORM_BLOCK_EXT: Self = Self(1_000_288_021);
    pub const ASTC_6X5X5_SRGB_BLOCK_EXT: Self = Self(1_000_288_022);
    pub const ASTC_6X5X5_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_023);
    pub const ASTC_6X6X5_UNORM_BLOCK_EXT: Self = Self(1_000_288_024);
    pub const ASTC_6X6X5_SRGB_BLOCK_EXT: Self = Self(1_000_288_025);
    pub const ASTC_6X6X5_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_026);
    pub const ASTC_6X6X6_UNORM_BLOCK_EXT: Self = Self(1_000_288_027);
    pub const ASTC_6X6X6_SRGB_BLOCK_EXT: Self = Self(1_000_288_028);
    pub const ASTC_6X6X6_SFLOAT_BLOCK_EXT: Self = Self(1_000_288_029);
}
impl GoogleUserTypeFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_user_type\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleUserTypeFn {}
unsafe impl Send for GoogleUserTypeFn {}
unsafe impl Sync for GoogleUserTypeFn {}
impl GoogleUserTypeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrPipelineLibraryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_library\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPipelineLibraryFn {}
unsafe impl Send for KhrPipelineLibraryFn {}
unsafe impl Sync for KhrPipelineLibraryFn {}
impl KhrPipelineLibraryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_pipeline_library'"]
impl PipelineCreateFlags {
    pub const LIBRARY_KHR: Self = Self(0b1000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_pipeline_library'"]
impl StructureType {
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1_000_290_000);
}
impl NvExtension292Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_292\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension292Fn {}
unsafe impl Send for NvExtension292Fn {}
unsafe impl Sync for NvExtension292Fn {}
impl NvExtension292Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension293Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_293\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension293Fn {}
unsafe impl Send for NvExtension293Fn {}
unsafe impl Sync for NvExtension293Fn {}
impl NvExtension293Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrShaderNonSemanticInfoFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_non_semantic_info\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderNonSemanticInfoFn {}
unsafe impl Send for KhrShaderNonSemanticInfoFn {}
unsafe impl Sync for KhrShaderNonSemanticInfoFn {}
impl KhrShaderNonSemanticInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrPresentIdFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_id\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPresentIdFn {}
unsafe impl Send for KhrPresentIdFn {}
unsafe impl Sync for KhrPresentIdFn {}
impl KhrPresentIdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_present_id'"]
impl StructureType {
    pub const PRESENT_ID_KHR: Self = Self(1_000_294_000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1_000_294_001);
}
impl ExtPrivateDataFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_private_data\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PrivateDataSlotCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_private_data_slot: *mut PrivateDataSlot,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    private_data_slot: PrivateDataSlot,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    p_data: *mut u64,
);
#[derive(Clone)]
pub struct ExtPrivateDataFn {
    pub create_private_data_slot_ext: PFN_vkCreatePrivateDataSlot,
    pub destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlot,
    pub set_private_data_ext: PFN_vkSetPrivateData,
    pub get_private_data_ext: PFN_vkGetPrivateData,
}
unsafe impl Send for ExtPrivateDataFn {}
unsafe impl Sync for ExtPrivateDataFn {}
impl ExtPrivateDataFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_private_data_slot_ext: unsafe {
                unsafe extern "system" fn create_private_data_slot_ext(
                    _device: Device,
                    _p_create_info: *const PrivateDataSlotCreateInfo,
                    _p_allocator: *const AllocationCallbacks,
                    _p_private_data_slot: *mut PrivateDataSlot,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_private_data_slot_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreatePrivateDataSlotEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_private_data_slot_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_private_data_slot_ext: unsafe {
                unsafe extern "system" fn destroy_private_data_slot_ext(
                    _device: Device,
                    _private_data_slot: PrivateDataSlot,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_private_data_slot_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyPrivateDataSlotEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_private_data_slot_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_private_data_ext: unsafe {
                unsafe extern "system" fn set_private_data_ext(
                    _device: Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _data: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(set_private_data_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetPrivateDataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    set_private_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_private_data_ext: unsafe {
                unsafe extern "system" fn get_private_data_ext(
                    _device: Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _p_data: *mut u64,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_private_data_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetPrivateDataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_private_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_private_data'"]
impl ObjectType {
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self::PRIVATE_DATA_SLOT;
}
#[doc = "Generated from 'VK_EXT_private_data'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
}
impl KhrExtension297Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_297\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension297Fn {}
unsafe impl Send for KhrExtension297Fn {}
unsafe impl Sync for KhrExtension297Fn {}
impl KhrExtension297Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_extension_297'"]
impl PipelineShaderStageCreateFlags {
    pub const RESERVED_3_KHR: Self = Self(0b1000);
}
impl ExtPipelineCreationCacheControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_pipeline_creation_cache_control\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtPipelineCreationCacheControlFn {}
unsafe impl Send for ExtPipelineCreationCacheControlFn {}
unsafe impl Sync for ExtPipelineCreationCacheControlFn {}
impl ExtPipelineCreationCacheControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl PipelineCacheCreateFlags {
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl PipelineCreateFlags {
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
    pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl Result {
    pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
}
impl KhrExtension299Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_299\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension299Fn {}
unsafe impl Send for KhrExtension299Fn {}
unsafe impl Sync for KhrExtension299Fn {}
impl KhrExtension299Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_extension_299'"]
impl MemoryHeapFlags {
    pub const RESERVED_2_KHR: Self = Self(0b100);
}
#[doc = "Generated from 'VK_KHR_extension_299'"]
impl PipelineCacheCreateFlags {
    pub const RESERVED_1_KHR: Self = Self::RESERVED_1_EXT;
    pub const RESERVED_2_KHR: Self = Self(0b100);
}
impl KhrVideoEncodeQueueFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_queue\0") }
    }
    pub const SPEC_VERSION: u32 = 5u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_encode_info: *const VideoEncodeInfoKHR,
);
#[derive(Clone)]
pub struct KhrVideoEncodeQueueFn {
    pub cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
}
unsafe impl Send for KhrVideoEncodeQueueFn {}
unsafe impl Sync for KhrVideoEncodeQueueFn {}
impl KhrVideoEncodeQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_encode_video_khr: unsafe {
                unsafe extern "system" fn cmd_encode_video_khr(
                    _command_buffer: CommandBuffer,
                    _p_encode_info: *const VideoEncodeInfoKHR,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_encode_video_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEncodeVideoKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_encode_video_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl AccessFlags2 {
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_WRITE_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl BufferUsageFlags {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0b1000_0000_0000_0000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0b1_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl FormatFeatureFlags {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl FormatFeatureFlags2 {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl ImageLayout {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1_000_299_000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1_000_299_001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1_000_299_002);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl ImageUsageFlags {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0b10_0000_0000_0000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0b100_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl PipelineStageFlags2 {
    pub const VIDEO_ENCODE_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl QueryType {
    pub const VIDEO_ENCODESTREAM_BUFFER_RANGE_KHR: Self = Self(1_000_299_000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl QueueFlags {
    pub const VIDEO_ENCODE_KHR: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl StructureType {
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1_000_299_000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1_000_299_001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1_000_299_002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1_000_299_003);
}
impl NvDeviceDiagnosticsConfigFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostics_config\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvDeviceDiagnosticsConfigFn {}
unsafe impl Send for NvDeviceDiagnosticsConfigFn {}
unsafe impl Sync for NvDeviceDiagnosticsConfigFn {}
impl NvDeviceDiagnosticsConfigFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_device_diagnostics_config'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1_000_300_000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1_000_300_001);
}
impl QcomRenderPassStoreOpsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_store_ops\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct QcomRenderPassStoreOpsFn {}
unsafe impl Send for QcomRenderPassStoreOpsFn {}
unsafe impl Sync for QcomRenderPassStoreOpsFn {}
impl QcomRenderPassStoreOpsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_render_pass_store_ops'"]
impl AttachmentStoreOp {
    pub const NONE_QCOM: Self = Self::NONE;
}
impl QcomExtension303Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_303\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension303Fn {}
unsafe impl Send for QcomExtension303Fn {}
unsafe impl Sync for QcomExtension303Fn {}
impl QcomExtension303Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension304Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_304\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension304Fn {}
unsafe impl Send for QcomExtension304Fn {}
unsafe impl Sync for QcomExtension304Fn {}
impl QcomExtension304Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension305Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_305\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension305Fn {}
unsafe impl Send for QcomExtension305Fn {}
unsafe impl Sync for QcomExtension305Fn {}
impl QcomExtension305Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension306Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_306\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension306Fn {}
unsafe impl Send for QcomExtension306Fn {}
unsafe impl Sync for QcomExtension306Fn {}
impl QcomExtension306Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension307Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_307\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension307Fn {}
unsafe impl Send for QcomExtension307Fn {}
unsafe impl Sync for QcomExtension307Fn {}
impl QcomExtension307Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension308Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_308\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension308Fn {}
unsafe impl Send for NvExtension308Fn {}
unsafe impl Sync for NvExtension308Fn {}
impl NvExtension308Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension309Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_309\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension309Fn {}
unsafe impl Send for KhrExtension309Fn {}
unsafe impl Sync for KhrExtension309Fn {}
impl KhrExtension309Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension310Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_310\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension310Fn {}
unsafe impl Send for QcomExtension310Fn {}
unsafe impl Sync for QcomExtension310Fn {}
impl QcomExtension310Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_extension_310'"]
impl StructureType {
    pub const RESERVED_QCOM: Self = Self(1_000_309_000);
}
impl NvExtension311Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_311\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension311Fn {}
unsafe impl Send for NvExtension311Fn {}
unsafe impl Sync for NvExtension311Fn {}
impl NvExtension311Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtMetalObjectsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_objects\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkExportMetalObjectsEXT =
    unsafe extern "system" fn(device: Device, p_metal_objects_info: *mut ExportMetalObjectsInfoEXT);
#[derive(Clone)]
pub struct ExtMetalObjectsFn {
    pub export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
}
unsafe impl Send for ExtMetalObjectsFn {}
unsafe impl Sync for ExtMetalObjectsFn {}
impl ExtMetalObjectsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            export_metal_objects_ext: unsafe {
                unsafe extern "system" fn export_metal_objects_ext(
                    _device: Device,
                    _p_metal_objects_info: *mut ExportMetalObjectsInfoEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(export_metal_objects_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkExportMetalObjectsEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    export_metal_objects_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_metal_objects'"]
impl StructureType {
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1_000_311_000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1_000_311_001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1_000_311_002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1_000_311_003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1_000_311_004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1_000_311_005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1_000_311_006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1_000_311_007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1_000_311_008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1_000_311_009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1_000_311_010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1_000_311_011);
}
impl ExtExtension313Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_313\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension313Fn {}
unsafe impl Send for ExtExtension313Fn {}
unsafe impl Sync for ExtExtension313Fn {}
impl ExtExtension313Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension314Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_314\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension314Fn {}
unsafe impl Send for AmdExtension314Fn {}
unsafe impl Sync for AmdExtension314Fn {}
impl AmdExtension314Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrSynchronization2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_synchronization2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    p_dependency_info: *const DependencyInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    p_dependency_infos: *const DependencyInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_dependency_info: *const DependencyInfo,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV,
);
#[derive(Clone)]
pub struct KhrSynchronization2Fn {
    pub cmd_set_event2_khr: PFN_vkCmdSetEvent2,
    pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2,
    pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2,
    pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2,
    pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2,
    pub queue_submit2_khr: PFN_vkQueueSubmit2,
    pub cmd_write_buffer_marker2_amd: PFN_vkCmdWriteBufferMarker2AMD,
    pub get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
}
unsafe impl Send for KhrSynchronization2Fn {}
unsafe impl Sync for KhrSynchronization2Fn {}
impl KhrSynchronization2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_event2_khr: unsafe {
                unsafe extern "system" fn cmd_set_event2_khr(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _p_dependency_info: *const DependencyInfo,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_event2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_event2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_reset_event2_khr: unsafe {
                unsafe extern "system" fn cmd_reset_event2_khr(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_reset_event2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_reset_event2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_wait_events2_khr: unsafe {
                unsafe extern "system" fn cmd_wait_events2_khr(
                    _command_buffer: CommandBuffer,
                    _event_count: u32,
                    _p_events: *const Event,
                    _p_dependency_infos: *const DependencyInfo,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_wait_events2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_wait_events2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_pipeline_barrier2_khr: unsafe {
                unsafe extern "system" fn cmd_pipeline_barrier2_khr(
                    _command_buffer: CommandBuffer,
                    _p_dependency_info: *const DependencyInfo,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_pipeline_barrier2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_pipeline_barrier2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_timestamp2_khr: unsafe {
                unsafe extern "system" fn cmd_write_timestamp2_khr(
                    _command_buffer: CommandBuffer,
                    _stage: PipelineStageFlags2,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_timestamp2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_timestamp2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_submit2_khr: unsafe {
                unsafe extern "system" fn queue_submit2_khr(
                    _queue: Queue,
                    _submit_count: u32,
                    _p_submits: *const SubmitInfo2,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_submit2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_submit2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_buffer_marker2_amd: unsafe {
                unsafe extern "system" fn cmd_write_buffer_marker2_amd(
                    _command_buffer: CommandBuffer,
                    _stage: PipelineStageFlags2,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _marker: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_buffer_marker2_amd)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteBufferMarker2AMD\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_buffer_marker2_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_queue_checkpoint_data2_nv: unsafe {
                unsafe extern "system" fn get_queue_checkpoint_data2_nv(
                    _queue: Queue,
                    _p_checkpoint_data_count: *mut u32,
                    _p_checkpoint_data: *mut CheckpointData2NV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_queue_checkpoint_data2_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetQueueCheckpointData2NV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_queue_checkpoint_data2_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl AccessFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl AccessFlags2 {
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000);
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(0b100_0000_0000_0000_0000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
        Self(0b1000_0000_0000_0000_0000_0000);
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl EventCreateFlags {
    pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl ImageLayout {
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self::READ_ONLY_OPTIMAL;
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self::ATTACHMENT_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl PipelineStageFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl PipelineStageFlags2 {
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b100_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const TASK_SHADER_NV: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MESH_SHADER_NV: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl StructureType {
    pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
    pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
    pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1_000_314_008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1_000_314_009);
}
impl AmdExtension316Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_316\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension316Fn {}
unsafe impl Send for AmdExtension316Fn {}
unsafe impl Sync for AmdExtension316Fn {}
impl AmdExtension316Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension317Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_317\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension317Fn {}
unsafe impl Send for AmdExtension317Fn {}
unsafe impl Sync for AmdExtension317Fn {}
impl AmdExtension317Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl AccessFlags2 {
    pub const RESERVED_41_AMD: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl BufferCreateFlags {
    pub const RESERVED_5_AMD: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl BufferUsageFlags {
    pub const RESERVED_21_AMD: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RESERVED_22_AMD: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl DescriptorSetLayoutCreateFlags {
    pub const RESERVED_4_AMD: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl ImageCreateFlags {
    pub const RESERVED_16_AMD: Self = Self(0b1_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl ImageViewCreateFlags {
    pub const RESERVED_2_AMD: Self = Self(0b100);
}
#[doc = "Generated from 'VK_AMD_extension_317'"]
impl SamplerCreateFlags {
    pub const RESERVED_3_AMD: Self = Self(0b1000);
}
impl AmdExtension318Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_318\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension318Fn {}
unsafe impl Send for AmdExtension318Fn {}
unsafe impl Sync for AmdExtension318Fn {}
impl AmdExtension318Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension319Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_319\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension319Fn {}
unsafe impl Send for AmdExtension319Fn {}
unsafe impl Sync for AmdExtension319Fn {}
impl AmdExtension319Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_extension_319'"]
impl DescriptorSetLayoutCreateFlags {
    pub const RESERVED_3_AMD: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_AMD_extension_319'"]
impl PipelineLayoutCreateFlags {
    pub const RESERVED_0_AMD: Self = Self(0b1);
}
impl AmdExtension320Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_320\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension320Fn {}
unsafe impl Send for AmdExtension320Fn {}
unsafe impl Sync for AmdExtension320Fn {}
impl AmdExtension320Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtGraphicsPipelineLibraryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_graphics_pipeline_library\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtGraphicsPipelineLibraryFn {}
unsafe impl Send for ExtGraphicsPipelineLibraryFn {}
unsafe impl Sync for ExtGraphicsPipelineLibraryFn {}
impl ExtGraphicsPipelineLibraryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl PipelineCreateFlags {
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl PipelineLayoutCreateFlags {
    pub const INDEPENDENT_SETS_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1_000_320_000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1_000_320_001);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1_000_320_002);
}
impl AmdShaderEarlyAndLateFragmentTestsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_AMD_shader_early_and_late_fragment_tests\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderEarlyAndLateFragmentTestsFn {}
unsafe impl Send for AmdShaderEarlyAndLateFragmentTestsFn {}
unsafe impl Sync for AmdShaderEarlyAndLateFragmentTestsFn {}
impl AmdShaderEarlyAndLateFragmentTestsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_AMD_shader_early_and_late_fragment_tests'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
        Self(1_000_321_000);
}
impl KhrFragmentShaderBarycentricFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shader_barycentric\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrFragmentShaderBarycentricFn {}
unsafe impl Send for KhrFragmentShaderBarycentricFn {}
unsafe impl Sync for KhrFragmentShaderBarycentricFn {}
impl KhrFragmentShaderBarycentricFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_fragment_shader_barycentric'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1_000_203_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self =
        Self(1_000_322_000);
}
impl KhrShaderSubgroupUniformControlFlowFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_shader_subgroup_uniform_control_flow\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderSubgroupUniformControlFlowFn {}
unsafe impl Send for KhrShaderSubgroupUniformControlFlowFn {}
unsafe impl Sync for KhrShaderSubgroupUniformControlFlowFn {}
impl KhrShaderSubgroupUniformControlFlowFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_shader_subgroup_uniform_control_flow'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
        Self(1_000_323_000);
}
impl KhrExtension325Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_325\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension325Fn {}
unsafe impl Send for KhrExtension325Fn {}
unsafe impl Sync for KhrExtension325Fn {}
impl KhrExtension325Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrZeroInitializeWorkgroupMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_zero_initialize_workgroup_memory\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrZeroInitializeWorkgroupMemoryFn {}
unsafe impl Send for KhrZeroInitializeWorkgroupMemoryFn {}
unsafe impl Sync for KhrZeroInitializeWorkgroupMemoryFn {}
impl KhrZeroInitializeWorkgroupMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_zero_initialize_workgroup_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
}
impl NvFragmentShadingRateEnumsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shading_rate_enums\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
);
#[derive(Clone)]
pub struct NvFragmentShadingRateEnumsFn {
    pub cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
}
unsafe impl Send for NvFragmentShadingRateEnumsFn {}
unsafe impl Sync for NvFragmentShadingRateEnumsFn {}
impl NvFragmentShadingRateEnumsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_fragment_shading_rate_enum_nv: unsafe {
                unsafe extern "system" fn cmd_set_fragment_shading_rate_enum_nv(
                    _command_buffer: CommandBuffer,
                    _shading_rate: FragmentShadingRateNV,
                    _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_fragment_shading_rate_enum_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetFragmentShadingRateEnumNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_fragment_shading_rate_enum_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_fragment_shading_rate_enums'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1_000_326_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1_000_326_001);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1_000_326_002);
}
impl NvRayTracingMotionBlurFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_motion_blur\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvRayTracingMotionBlurFn {}
unsafe impl Send for NvRayTracingMotionBlurFn {}
unsafe impl Sync for NvRayTracingMotionBlurFn {}
impl NvRayTracingMotionBlurFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl AccelerationStructureCreateFlagsKHR {
    pub const MOTION_NV: Self = Self(0b100);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const MOTION_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl StructureType {
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1_000_327_000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1_000_327_001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1_000_327_002);
}
impl NvExtension329Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_329\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension329Fn {}
unsafe impl Send for NvExtension329Fn {}
unsafe impl Sync for NvExtension329Fn {}
impl NvExtension329Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension330Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_330\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension330Fn {}
unsafe impl Send for NvExtension330Fn {}
unsafe impl Sync for NvExtension330Fn {}
impl NvExtension330Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtYcbcr2plane444FormatsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_2plane_444_formats\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtYcbcr2plane444FormatsFn {}
unsafe impl Send for ExtYcbcr2plane444FormatsFn {}
unsafe impl Sync for ExtYcbcr2plane444FormatsFn {}
impl ExtYcbcr2plane444FormatsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_ycbcr_2plane_444_formats'"]
impl Format {
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self::G8_B8R8_2PLANE_444_UNORM;
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self::G16_B16R16_2PLANE_444_UNORM;
}
#[doc = "Generated from 'VK_EXT_ycbcr_2plane_444_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1_000_330_000);
}
impl NvExtension332Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_332\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension332Fn {}
unsafe impl Send for NvExtension332Fn {}
unsafe impl Sync for NvExtension332Fn {}
impl NvExtension332Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtFragmentDensityMap2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtFragmentDensityMap2Fn {}
unsafe impl Send for ExtFragmentDensityMap2Fn {}
unsafe impl Sync for ExtFragmentDensityMap2Fn {}
impl ExtFragmentDensityMap2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_fragment_density_map2'"]
impl ImageViewCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1_000_332_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1_000_332_001);
}
impl QcomRotatedCopyCommandsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_rotated_copy_commands\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomRotatedCopyCommandsFn {}
unsafe impl Send for QcomRotatedCopyCommandsFn {}
unsafe impl Sync for QcomRotatedCopyCommandsFn {}
impl QcomRotatedCopyCommandsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_rotated_copy_commands'"]
impl StructureType {
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1_000_333_000);
}
impl KhrExtension335Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_335\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension335Fn {}
unsafe impl Send for KhrExtension335Fn {}
unsafe impl Sync for KhrExtension335Fn {}
impl KhrExtension335Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtImageRobustnessFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_robustness\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageRobustnessFn {}
unsafe impl Send for ExtImageRobustnessFn {}
unsafe impl Sync for ExtImageRobustnessFn {}
impl ExtImageRobustnessFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_image_robustness'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
}
impl KhrWorkgroupMemoryExplicitLayoutFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_KHR_workgroup_memory_explicit_layout\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrWorkgroupMemoryExplicitLayoutFn {}
unsafe impl Send for KhrWorkgroupMemoryExplicitLayoutFn {}
unsafe impl Sync for KhrWorkgroupMemoryExplicitLayoutFn {}
impl KhrWorkgroupMemoryExplicitLayoutFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_workgroup_memory_explicit_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
        Self(1_000_336_000);
}
impl KhrCopyCommands2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_copy_commands2\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_info: *const CopyBufferInfo2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_info: *const CopyImageInfo2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_blit_image_info: *const BlitImageInfo2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_resolve_image_info: *const ResolveImageInfo2,
);
#[derive(Clone)]
pub struct KhrCopyCommands2Fn {
    pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2,
    pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2,
    pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2,
    pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2,
    pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2,
    pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2,
}
unsafe impl Send for KhrCopyCommands2Fn {}
unsafe impl Sync for KhrCopyCommands2Fn {}
impl KhrCopyCommands2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_copy_buffer2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_buffer2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_info: *const CopyBufferInfo2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_image2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_info: *const CopyImageInfo2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_image2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_buffer_to_image2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_buffer_to_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_buffer_to_image2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyBufferToImage2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer_to_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_image_to_buffer2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_image_to_buffer2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_image_to_buffer2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyImageToBuffer2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image_to_buffer2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_blit_image2_khr: unsafe {
                unsafe extern "system" fn cmd_blit_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_blit_image_info: *const BlitImageInfo2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_blit_image2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_blit_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_resolve_image2_khr: unsafe {
                unsafe extern "system" fn cmd_resolve_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_resolve_image_info: *const ResolveImageInfo2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_resolve_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_resolve_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_copy_commands2'"]
impl StructureType {
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
    pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
    pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
    pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
    pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
}
impl ExtImageCompressionControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_compression_control\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout2EXT = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource2EXT,
    p_layout: *mut SubresourceLayout2EXT,
);
#[derive(Clone)]
pub struct ExtImageCompressionControlFn {
    pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2EXT,
}
unsafe impl Send for ExtImageCompressionControlFn {}
unsafe impl Sync for ExtImageCompressionControlFn {}
impl ExtImageCompressionControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_subresource_layout2_ext: unsafe {
                unsafe extern "system" fn get_image_subresource_layout2_ext(
                    _device: Device,
                    _image: Image,
                    _p_subresource: *const ImageSubresource2EXT,
                    _p_layout: *mut SubresourceLayout2EXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_subresource_layout2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSubresourceLayout2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_subresource_layout2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_image_compression_control'"]
impl Result {
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1_000_338_000);
}
#[doc = "Generated from 'VK_EXT_image_compression_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1_000_338_000);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1_000_338_001);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self(1_000_338_002);
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self(1_000_338_003);
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1_000_338_004);
}
impl ExtExtension340Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_340\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension340Fn {}
unsafe impl Send for ExtExtension340Fn {}
unsafe impl Sync for ExtExtension340Fn {}
impl ExtExtension340Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_340'"]
impl DependencyFlags {
    pub const RESERVED_3_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_extension_340'"]
impl ImageUsageFlags {
    pub const RESERVED_19_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_extension_340'"]
impl PipelineCreateFlags {
    pub const RESERVED_25_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_26_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
impl Ext4444FormatsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_4444_formats\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Ext4444FormatsFn {}
unsafe impl Send for Ext4444FormatsFn {}
unsafe impl Sync for Ext4444FormatsFn {}
impl Ext4444FormatsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_4444_formats'"]
impl Format {
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self::A4R4G4B4_UNORM_PACK16;
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self::A4B4G4R4_UNORM_PACK16;
}
#[doc = "Generated from 'VK_EXT_4444_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1_000_340_000);
}
impl ExtExtension342Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_342\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension342Fn {}
unsafe impl Send for ExtExtension342Fn {}
unsafe impl Sync for ExtExtension342Fn {}
impl ExtExtension342Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ArmRasterizationOrderAttachmentAccessFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_ARM_rasterization_order_attachment_access\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ArmRasterizationOrderAttachmentAccessFn {}
unsafe impl Send for ArmRasterizationOrderAttachmentAccessFn {}
unsafe impl Sync for ArmRasterizationOrderAttachmentAccessFn {}
impl ArmRasterizationOrderAttachmentAccessFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl PipelineColorBlendStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self = Self(0b1);
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl PipelineDepthStencilStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self = Self(0b1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self = Self(0b10);
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
        Self(1_000_342_000);
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl SubpassDescriptionFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self = Self(0b1_0000);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self = Self(0b10_0000);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self = Self(0b100_0000);
}
impl ArmExtension344Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_344\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension344Fn {}
unsafe impl Send for ArmExtension344Fn {}
unsafe impl Sync for ArmExtension344Fn {}
impl ArmExtension344Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtRgba10x6FormatsFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_rgba10x6_formats\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtRgba10x6FormatsFn {}
unsafe impl Send for ExtRgba10x6FormatsFn {}
unsafe impl Sync for ExtRgba10x6FormatsFn {}
impl ExtRgba10x6FormatsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_rgba10x6_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1_000_344_000);
}
impl NvAcquireWinrtDisplayFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_acquire_winrt_display\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireWinrtDisplayNV =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct NvAcquireWinrtDisplayFn {
    pub acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
    pub get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
}
unsafe impl Send for NvAcquireWinrtDisplayFn {}
unsafe impl Sync for NvAcquireWinrtDisplayFn {}
impl NvAcquireWinrtDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_winrt_display_nv: unsafe {
                unsafe extern "system" fn acquire_winrt_display_nv(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_winrt_display_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireWinrtDisplayNV\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_winrt_display_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_winrt_display_nv: unsafe {
                unsafe extern "system" fn get_winrt_display_nv(
                    _physical_device: PhysicalDevice,
                    _device_relative_id: u32,
                    _p_display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_winrt_display_nv)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetWinrtDisplayNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_winrt_display_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtDirectfbSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_directfb_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
#[derive(Clone)]
pub struct ExtDirectfbSurfaceFn {
    pub create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
    pub get_physical_device_direct_fb_presentation_support_ext:
        PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
}
unsafe impl Send for ExtDirectfbSurfaceFn {}
unsafe impl Sync for ExtDirectfbSurfaceFn {}
impl ExtDirectfbSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_direct_fb_surface_ext: unsafe {
                unsafe extern "system" fn create_direct_fb_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const DirectFBSurfaceCreateInfoEXT,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_direct_fb_surface_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDirectFBSurfaceEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_direct_fb_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_direct_fb_presentation_support_ext: unsafe {
                unsafe extern "system" fn get_physical_device_direct_fb_presentation_support_ext(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _dfb: *mut IDirectFB,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_direct_fb_presentation_support_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_direct_fb_presentation_support_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_directfb_surface'"]
impl StructureType {
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_346_000);
}
impl KhrExtension350Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_350\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension350Fn {}
unsafe impl Send for KhrExtension350Fn {}
unsafe impl Sync for KhrExtension350Fn {}
impl KhrExtension350Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension351Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_351\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension351Fn {}
unsafe impl Send for NvExtension351Fn {}
unsafe impl Sync for NvExtension351Fn {}
impl NvExtension351Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ValveMutableDescriptorTypeFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_mutable_descriptor_type\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ValveMutableDescriptorTypeFn {}
unsafe impl Send for ValveMutableDescriptorTypeFn {}
unsafe impl Sync for ValveMutableDescriptorTypeFn {}
impl ValveMutableDescriptorTypeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorPoolCreateFlags {
    pub const HOST_ONLY_VALVE: Self = Self(0b100);
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorSetLayoutCreateFlags {
    pub const HOST_ONLY_POOL_VALVE: Self = Self(0b100);
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorType {
    pub const MUTABLE_VALVE: Self = Self(1_000_351_000);
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self(1_000_351_000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self(1_000_351_002);
}
impl ExtVertexInputDynamicStateFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_input_dynamic_state\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
);
#[derive(Clone)]
pub struct ExtVertexInputDynamicStateFn {
    pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
}
unsafe impl Send for ExtVertexInputDynamicStateFn {}
unsafe impl Sync for ExtVertexInputDynamicStateFn {}
impl ExtVertexInputDynamicStateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_vertex_input_ext: unsafe {
                unsafe extern "system" fn cmd_set_vertex_input_ext(
                    _command_buffer: CommandBuffer,
                    _vertex_binding_description_count: u32,
                    _p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
                    _vertex_attribute_description_count: u32,
                    _p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_vertex_input_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_vertex_input_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_vertex_input_dynamic_state'"]
impl DynamicState {
    pub const VERTEX_INPUT_EXT: Self = Self(1_000_352_000);
}
#[doc = "Generated from 'VK_EXT_vertex_input_dynamic_state'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1_000_352_000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1_000_352_001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1_000_352_002);
}
impl ExtPhysicalDeviceDrmFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_physical_device_drm\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPhysicalDeviceDrmFn {}
unsafe impl Send for ExtPhysicalDeviceDrmFn {}
unsafe impl Sync for ExtPhysicalDeviceDrmFn {}
impl ExtPhysicalDeviceDrmFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_physical_device_drm'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1_000_353_000);
}
impl ExtExtension355Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_355\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension355Fn {}
unsafe impl Send for ExtExtension355Fn {}
unsafe impl Sync for ExtExtension355Fn {}
impl ExtExtension355Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtDepthClipControlFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_control\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthClipControlFn {}
unsafe impl Send for ExtDepthClipControlFn {}
unsafe impl Sync for ExtDepthClipControlFn {}
impl ExtDepthClipControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_depth_clip_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1_000_355_000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1_000_355_001);
}
impl ExtPrimitiveTopologyListRestartFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_primitive_topology_list_restart\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPrimitiveTopologyListRestartFn {}
unsafe impl Send for ExtPrimitiveTopologyListRestartFn {}
unsafe impl Sync for ExtPrimitiveTopologyListRestartFn {}
impl ExtPrimitiveTopologyListRestartFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_primitive_topology_list_restart'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self =
        Self(1_000_356_000);
}
impl KhrExtension358Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_358\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension358Fn {}
unsafe impl Send for KhrExtension358Fn {}
unsafe impl Sync for KhrExtension358Fn {}
impl KhrExtension358Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension359Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_359\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension359Fn {}
unsafe impl Send for ExtExtension359Fn {}
unsafe impl Sync for ExtExtension359Fn {}
impl ExtExtension359Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension360Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_360\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension360Fn {}
unsafe impl Send for ExtExtension360Fn {}
unsafe impl Sync for ExtExtension360Fn {}
impl ExtExtension360Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrFormatFeatureFlags2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_format_feature_flags2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrFormatFeatureFlags2Fn {}
unsafe impl Send for KhrFormatFeatureFlags2Fn {}
unsafe impl Sync for KhrFormatFeatureFlags2Fn {}
impl KhrFormatFeatureFlags2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_format_feature_flags2'"]
impl StructureType {
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
}
impl ExtExtension362Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_362\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension362Fn {}
unsafe impl Send for ExtExtension362Fn {}
unsafe impl Sync for ExtExtension362Fn {}
impl ExtExtension362Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension363Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_363\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension363Fn {}
unsafe impl Send for ExtExtension363Fn {}
unsafe impl Sync for ExtExtension363Fn {}
impl ExtExtension363Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl FuchsiaExtension364Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_extension_364\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct FuchsiaExtension364Fn {}
unsafe impl Send for FuchsiaExtension364Fn {}
unsafe impl Sync for FuchsiaExtension364Fn {}
impl FuchsiaExtension364Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl FuchsiaExternalMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_memory\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: zx_handle_t,
    p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaExternalMemoryFn {
    pub get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
    pub get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
}
unsafe impl Send for FuchsiaExternalMemoryFn {}
unsafe impl Sync for FuchsiaExternalMemoryFn {}
impl FuchsiaExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn get_memory_zircon_handle_fuchsia(
                    _device: Device,
                    _p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
                    _p_zircon_handle: *mut zx_handle_t,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_zircon_handle_properties_fuchsia: unsafe {
                unsafe extern "system" fn get_memory_zircon_handle_properties_fuchsia(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _zircon_handle: zx_handle_t,
                    _p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_zircon_handle_properties_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryZirconHandlePropertiesFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_zircon_handle_properties_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_external_memory'"]
impl ExternalMemoryHandleTypeFlags {
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(0b1000_0000_0000);
}
#[doc = "Generated from 'VK_FUCHSIA_external_memory'"]
impl StructureType {
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_364_000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1_000_364_001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_364_002);
}
impl FuchsiaExternalSemaphoreFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_semaphore\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaExternalSemaphoreFn {
    pub import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
    pub get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
}
unsafe impl Send for FuchsiaExternalSemaphoreFn {}
unsafe impl Sync for FuchsiaExternalSemaphoreFn {}
impl FuchsiaExternalSemaphoreFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn import_semaphore_zircon_handle_fuchsia(
                    _device: Device,
                    _p_import_semaphore_zircon_handle_info : * const ImportSemaphoreZirconHandleInfoFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn get_semaphore_zircon_handle_fuchsia(
                    _device: Device,
                    _p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
                    _p_zircon_handle: *mut zx_handle_t,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_external_semaphore'"]
impl ExternalSemaphoreHandleTypeFlags {
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_FUCHSIA_external_semaphore'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_365_000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_365_001);
}
impl FuchsiaBufferCollectionFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_buffer_collection\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaBufferCollectionFn {
    pub create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    pub set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    pub set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    pub destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    pub get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
}
unsafe impl Send for FuchsiaBufferCollectionFn {}
unsafe impl Sync for FuchsiaBufferCollectionFn {}
impl FuchsiaBufferCollectionFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_buffer_collection_fuchsia: unsafe {
                unsafe extern "system" fn create_buffer_collection_fuchsia(
                    _device: Device,
                    _p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
                    _p_allocator: *const AllocationCallbacks,
                    _p_collection: *mut BufferCollectionFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_buffer_collection_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateBufferCollectionFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_buffer_collection_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_buffer_collection_image_constraints_fuchsia: unsafe {
                unsafe extern "system" fn set_buffer_collection_image_constraints_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_buffer_collection_image_constraints_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetBufferCollectionImageConstraintsFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_buffer_collection_image_constraints_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_buffer_collection_buffer_constraints_fuchsia: unsafe {
                unsafe extern "system" fn set_buffer_collection_buffer_constraints_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_buffer_collection_buffer_constraints_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetBufferCollectionBufferConstraintsFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_buffer_collection_buffer_constraints_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_buffer_collection_fuchsia: unsafe {
                unsafe extern "system" fn destroy_buffer_collection_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_buffer_collection_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyBufferCollectionFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_buffer_collection_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_collection_properties_fuchsia: unsafe {
                unsafe extern "system" fn get_buffer_collection_properties_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_properties: *mut BufferCollectionPropertiesFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_collection_properties_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferCollectionPropertiesFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_collection_properties_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl DebugReportObjectTypeEXT {
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_000);
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl ObjectType {
    #[doc = "VkBufferCollectionFUCHSIA"]
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_000);
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl StructureType {
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1_000_366_003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1_000_366_008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_009);
}
impl FuchsiaExtension368Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_extension_368\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct FuchsiaExtension368Fn {}
unsafe impl Send for FuchsiaExtension368Fn {}
unsafe impl Sync for FuchsiaExtension368Fn {}
impl FuchsiaExtension368Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension369Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_369\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension369Fn {}
unsafe impl Send for QcomExtension369Fn {}
unsafe impl Sync for QcomExtension369Fn {}
impl QcomExtension369Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_extension_369'"]
impl DescriptorBindingFlags {
    pub const RESERVED_4_QCOM: Self = Self(0b1_0000);
}
impl HuaweiSubpassShadingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_subpass_shading\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct HuaweiSubpassShadingFn {
    pub get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    pub cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
}
unsafe impl Send for HuaweiSubpassShadingFn {}
unsafe impl Sync for HuaweiSubpassShadingFn {}
impl HuaweiSubpassShadingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_subpass_shading_max_workgroup_size_huawei: unsafe {
                unsafe extern "system" fn get_device_subpass_shading_max_workgroup_size_huawei(
                    _device: Device,
                    _renderpass: RenderPass,
                    _p_max_workgroup_size: *mut Extent2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_subpass_shading_max_workgroup_size_huawei)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_subpass_shading_max_workgroup_size_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_subpass_shading_huawei: unsafe {
                unsafe extern "system" fn cmd_subpass_shading_huawei(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_subpass_shading_huawei)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSubpassShadingHUAWEI\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_subpass_shading_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl PipelineBindPoint {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1_000_369_003);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl PipelineStageFlags2 {
    pub const SUBPASS_SHADING_HUAWEI: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl ShaderStageFlags {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl StructureType {
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1_000_369_000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1_000_369_001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1_000_369_002);
}
impl HuaweiInvocationMaskFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_invocation_mask\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[derive(Clone)]
pub struct HuaweiInvocationMaskFn {
    pub cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
}
unsafe impl Send for HuaweiInvocationMaskFn {}
unsafe impl Sync for HuaweiInvocationMaskFn {}
impl HuaweiInvocationMaskFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_invocation_mask_huawei: unsafe {
                unsafe extern "system" fn cmd_bind_invocation_mask_huawei(
                    _command_buffer: CommandBuffer,
                    _image_view: ImageView,
                    _image_layout: ImageLayout,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_invocation_mask_huawei)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindInvocationMaskHUAWEI\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_invocation_mask_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl AccessFlags2 {
    pub const INVOCATION_MASK_READ_HUAWEI: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl ImageUsageFlags {
    pub const INVOCATION_MASK_HUAWEI: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl PipelineStageFlags2 {
    pub const INVOCATION_MASK_HUAWEI: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1_000_370_000);
}
impl NvExternalMemoryRdmaFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_rdma\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    p_address: *mut RemoteAddressNV,
) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryRdmaFn {
    pub get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
}
unsafe impl Send for NvExternalMemoryRdmaFn {}
unsafe impl Sync for NvExternalMemoryRdmaFn {}
impl NvExternalMemoryRdmaFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_remote_address_nv: unsafe {
                unsafe extern "system" fn get_memory_remote_address_nv(
                    _device: Device,
                    _p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
                    _p_address: *mut RemoteAddressNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_remote_address_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryRemoteAddressNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_remote_address_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl ExternalMemoryHandleTypeFlags {
    pub const RDMA_ADDRESS_NV: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl MemoryPropertyFlags {
    pub const RDMA_CAPABLE_NV: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl StructureType {
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1_000_371_000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1_000_371_001);
}
impl ExtPipelinePropertiesFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_properties\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoEXT,
    p_pipeline_properties: *mut BaseOutStructure,
) -> Result;
#[derive(Clone)]
pub struct ExtPipelinePropertiesFn {
    pub get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
}
unsafe impl Send for ExtPipelinePropertiesFn {}
unsafe impl Sync for ExtPipelinePropertiesFn {}
impl ExtPipelinePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_pipeline_properties_ext: unsafe {
                unsafe extern "system" fn get_pipeline_properties_ext(
                    _device: Device,
                    _p_pipeline_info: *const PipelineInfoEXT,
                    _p_pipeline_properties: *mut BaseOutStructure,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelinePropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_pipeline_properties'"]
impl StructureType {
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1_000_372_000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1_000_372_001);
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
}
impl NvExtension374Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_374\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension374Fn {}
unsafe impl Send for NvExtension374Fn {}
unsafe impl Sync for NvExtension374Fn {}
impl NvExtension374Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_extension_374'"]
impl ExternalFenceHandleTypeFlags {
    pub const RESERVED_4_NV: Self = Self(0b1_0000);
    pub const RESERVED_5_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_extension_374'"]
impl ExternalSemaphoreHandleTypeFlags {
    pub const RESERVED_5_NV: Self = Self(0b10_0000);
}
impl NvExtension375Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_375\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension375Fn {}
unsafe impl Send for NvExtension375Fn {}
unsafe impl Sync for NvExtension375Fn {}
impl NvExtension375Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_extension_375'"]
impl ExternalMemoryHandleTypeFlags {
    pub const RESERVED_13_NV: Self = Self(0b10_0000_0000_0000);
}
impl ExtExtension376Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_376\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension376Fn {}
unsafe impl Send for ExtExtension376Fn {}
unsafe impl Sync for ExtExtension376Fn {}
impl ExtExtension376Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtMultisampledRenderToSingleSampledFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_multisampled_render_to_single_sampled\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMultisampledRenderToSingleSampledFn {}
unsafe impl Send for ExtMultisampledRenderToSingleSampledFn {}
unsafe impl Sync for ExtMultisampledRenderToSingleSampledFn {}
impl ExtMultisampledRenderToSingleSampledFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_multisampled_render_to_single_sampled'"]
impl ImageCreateFlags {
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_multisampled_render_to_single_sampled'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
        Self(1_000_376_000);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1_000_376_001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1_000_376_002);
}
impl ExtExtendedDynamicState2Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state2\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
#[derive(Clone)]
pub struct ExtExtendedDynamicState2Fn {
    pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
}
unsafe impl Send for ExtExtendedDynamicState2Fn {}
unsafe impl Sync for ExtExtendedDynamicState2Fn {}
impl ExtExtendedDynamicState2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_patch_control_points_ext: unsafe {
                unsafe extern "system" fn cmd_set_patch_control_points_ext(
                    _command_buffer: CommandBuffer,
                    _patch_control_points: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_patch_control_points_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPatchControlPointsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_patch_control_points_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterizer_discard_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                    _command_buffer: CommandBuffer,
                    _rasterizer_discard_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterizer_discard_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizerDiscardEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterizer_discard_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bias_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bias_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bias_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBiasEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_logic_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_logic_op_ext(
                    _command_buffer: CommandBuffer,
                    _logic_op: LogicOp,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_logic_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_restart_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_restart_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_restart_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveRestartEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_restart_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state2'"]
impl DynamicState {
    #[doc = "Not promoted to 1.3"]
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1_000_377_000);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
    #[doc = "Not promoted to 1.3"]
    pub const LOGIC_OP_EXT: Self = Self(1_000_377_003);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state2'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1_000_377_000);
}
impl QnxScreenSurfaceFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QNX_screen_surface\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ScreenSurfaceCreateInfoQNX,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32;
#[derive(Clone)]
pub struct QnxScreenSurfaceFn {
    pub create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    pub get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
}
unsafe impl Send for QnxScreenSurfaceFn {}
unsafe impl Sync for QnxScreenSurfaceFn {}
impl QnxScreenSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_screen_surface_qnx: unsafe {
                unsafe extern "system" fn create_screen_surface_qnx(
                    _instance: Instance,
                    _p_create_info: *const ScreenSurfaceCreateInfoQNX,
                    _p_allocator: *const AllocationCallbacks,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_screen_surface_qnx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateScreenSurfaceQNX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_screen_surface_qnx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_screen_presentation_support_qnx: unsafe {
                unsafe extern "system" fn get_physical_device_screen_presentation_support_qnx(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _window: *mut _screen_window,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_screen_presentation_support_qnx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceScreenPresentationSupportQNX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_screen_presentation_support_qnx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_QNX_screen_surface'"]
impl StructureType {
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1_000_378_000);
}
impl KhrExtension380Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_380\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension380Fn {}
unsafe impl Send for KhrExtension380Fn {}
unsafe impl Sync for KhrExtension380Fn {}
impl KhrExtension380Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension381Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_381\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension381Fn {}
unsafe impl Send for KhrExtension381Fn {}
unsafe impl Sync for KhrExtension381Fn {}
impl KhrExtension381Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtColorWriteEnableFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_color_write_enable\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
#[derive(Clone)]
pub struct ExtColorWriteEnableFn {
    pub cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
}
unsafe impl Send for ExtColorWriteEnableFn {}
unsafe impl Sync for ExtColorWriteEnableFn {}
impl ExtColorWriteEnableFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_color_write_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_write_enable_ext(
                    _command_buffer: CommandBuffer,
                    _attachment_count: u32,
                    _p_color_write_enables: *const Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_write_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorWriteEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_write_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_color_write_enable'"]
impl DynamicState {
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1_000_381_000);
}
#[doc = "Generated from 'VK_EXT_color_write_enable'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1_000_381_000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1_000_381_001);
}
impl ExtPrimitivesGeneratedQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_primitives_generated_query\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPrimitivesGeneratedQueryFn {}
unsafe impl Send for ExtPrimitivesGeneratedQueryFn {}
unsafe impl Sync for ExtPrimitivesGeneratedQueryFn {}
impl ExtPrimitivesGeneratedQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_primitives_generated_query'"]
impl QueryType {
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1_000_382_000);
}
#[doc = "Generated from 'VK_EXT_primitives_generated_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1_000_382_000);
}
impl ExtExtension384Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_384\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension384Fn {}
unsafe impl Send for ExtExtension384Fn {}
unsafe impl Sync for ExtExtension384Fn {}
impl ExtExtension384Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl MesaExtension385Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MESA_extension_385\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct MesaExtension385Fn {}
unsafe impl Send for MesaExtension385Fn {}
unsafe impl Sync for MesaExtension385Fn {}
impl MesaExtension385Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension386Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_386\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension386Fn {}
unsafe impl Send for GoogleExtension386Fn {}
unsafe impl Sync for GoogleExtension386Fn {}
impl GoogleExtension386Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrRayTracingMaintenance1Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_maintenance1\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_device_address: DeviceAddress,
);
#[derive(Clone)]
pub struct KhrRayTracingMaintenance1Fn {
    pub cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
}
unsafe impl Send for KhrRayTracingMaintenance1Fn {}
unsafe impl Sync for KhrRayTracingMaintenance1Fn {}
impl KhrRayTracingMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_trace_rays_indirect2_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_indirect2_khr(
                    _command_buffer: CommandBuffer,
                    _indirect_device_address: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_trace_rays_indirect2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdTraceRaysIndirect2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_indirect2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl AccessFlags2 {
    pub const SHADER_BINDING_TABLE_READ_KHR: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl PipelineStageFlags2 {
    pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1_000_386_000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1_000_386_001);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1_000_386_000);
}
impl ExtExtension388Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_388\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension388Fn {}
unsafe impl Send for ExtExtension388Fn {}
unsafe impl Sync for ExtExtension388Fn {}
impl ExtExtension388Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtGlobalPriorityQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_global_priority_query\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtGlobalPriorityQueryFn {}
unsafe impl Send for ExtGlobalPriorityQueryFn {}
unsafe impl Sync for ExtGlobalPriorityQueryFn {}
impl ExtGlobalPriorityQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_global_priority_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;
}
impl ExtExtension390Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_390\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension390Fn {}
unsafe impl Send for ExtExtension390Fn {}
unsafe impl Sync for ExtExtension390Fn {}
impl ExtExtension390Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension391Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_391\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension391Fn {}
unsafe impl Send for ExtExtension391Fn {}
unsafe impl Sync for ExtExtension391Fn {}
impl ExtExtension391Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtImageViewMinLodFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_view_min_lod\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageViewMinLodFn {}
unsafe impl Send for ExtImageViewMinLodFn {}
unsafe impl Sync for ExtImageViewMinLodFn {}
impl ExtImageViewMinLodFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_image_view_min_lod'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1_000_391_000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1_000_391_001);
}
impl ExtMultiDrawFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_multi_draw\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: *const i32,
);
#[derive(Clone)]
pub struct ExtMultiDrawFn {
    pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
    pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
}
unsafe impl Send for ExtMultiDrawFn {}
unsafe impl Sync for ExtMultiDrawFn {}
impl ExtMultiDrawFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_multi_ext: unsafe {
                unsafe extern "system" fn cmd_draw_multi_ext(
                    _command_buffer: CommandBuffer,
                    _draw_count: u32,
                    _p_vertex_info: *const MultiDrawInfoEXT,
                    _instance_count: u32,
                    _first_instance: u32,
                    _stride: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_draw_multi_ext)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_multi_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_multi_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_draw_multi_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _draw_count: u32,
                    _p_index_info: *const MultiDrawIndexedInfoEXT,
                    _instance_count: u32,
                    _first_instance: u32,
                    _stride: u32,
                    _p_vertex_offset: *const i32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_multi_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_multi_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_multi_draw'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1_000_392_000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1_000_392_001);
}
impl ExtImage2dViewOf3dFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_2d_view_of_3d\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImage2dViewOf3dFn {}
unsafe impl Send for ExtImage2dViewOf3dFn {}
unsafe impl Sync for ExtImage2dViewOf3dFn {}
impl ExtImage2dViewOf3dFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_image_2d_view_of_3d'"]
impl ImageCreateFlags {
    #[doc = "Image is created with a layout where individual slices are capable of being used as 2D images"]
    pub const TYPE_2D_VIEW_COMPATIBLE_EXT: Self = Self(0b10_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_image_2d_view_of_3d'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1_000_393_000);
}
impl KhrPortabilityEnumerationFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_enumeration\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPortabilityEnumerationFn {}
unsafe impl Send for KhrPortabilityEnumerationFn {}
unsafe impl Sync for KhrPortabilityEnumerationFn {}
impl KhrPortabilityEnumerationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_KHR_portability_enumeration'"]
impl InstanceCreateFlags {
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(0b1);
}
impl KhrExtension396Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_396\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension396Fn {}
unsafe impl Send for KhrExtension396Fn {}
unsafe impl Sync for KhrExtension396Fn {}
impl KhrExtension396Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension397Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_397\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension397Fn {}
unsafe impl Send for NvExtension397Fn {}
unsafe impl Sync for NvExtension397Fn {}
impl NvExtension397Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl AccessFlags2 {
    pub const RESERVED_44_NV: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_45_NV: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl BufferUsageFlags {
    pub const RESERVED_23_NV: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const RESERVED_24_NV: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const RESERVED_6_NV: Self = Self(0b100_0000);
    pub const RESERVED_7_NV: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl GeometryInstanceFlagsKHR {
    pub const RESERVED_4_NV: Self = Self(0b1_0000);
    pub const RESERVED_5_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl PipelineCreateFlags {
    pub const RESERVED_24_NV: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_397'"]
impl PipelineStageFlags2 {
    pub const RESERVED_30_NV: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
impl NvExtension398Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_398\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension398Fn {}
unsafe impl Send for NvExtension398Fn {}
unsafe impl Sync for NvExtension398Fn {}
impl NvExtension398Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl JuiceExtension399Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_JUICE_extension_399\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct JuiceExtension399Fn {}
unsafe impl Send for JuiceExtension399Fn {}
unsafe impl Sync for JuiceExtension399Fn {}
impl JuiceExtension399Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl JuiceExtension400Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_JUICE_extension_400\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct JuiceExtension400Fn {}
unsafe impl Send for JuiceExtension400Fn {}
unsafe impl Sync for JuiceExtension400Fn {}
impl JuiceExtension400Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtLoadStoreOpNoneFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_load_store_op_none\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtLoadStoreOpNoneFn {}
unsafe impl Send for ExtLoadStoreOpNoneFn {}
unsafe impl Sync for ExtLoadStoreOpNoneFn {}
impl ExtLoadStoreOpNoneFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_load_store_op_none'"]
impl AttachmentLoadOp {
    pub const NONE_EXT: Self = Self(1_000_400_000);
}
#[doc = "Generated from 'VK_EXT_load_store_op_none'"]
impl AttachmentStoreOp {
    pub const NONE_EXT: Self = Self::NONE;
}
impl FbExtension402Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FB_extension_402\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct FbExtension402Fn {}
unsafe impl Send for FbExtension402Fn {}
unsafe impl Sync for FbExtension402Fn {}
impl FbExtension402Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl FbExtension403Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FB_extension_403\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct FbExtension403Fn {}
unsafe impl Send for FbExtension403Fn {}
unsafe impl Sync for FbExtension403Fn {}
impl FbExtension403Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl FbExtension404Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FB_extension_404\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct FbExtension404Fn {}
unsafe impl Send for FbExtension404Fn {}
unsafe impl Sync for FbExtension404Fn {}
impl FbExtension404Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl HuaweiExtension405Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_extension_405\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct HuaweiExtension405Fn {}
unsafe impl Send for HuaweiExtension405Fn {}
unsafe impl Sync for HuaweiExtension405Fn {}
impl HuaweiExtension405Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl HuaweiExtension406Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_extension_406\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct HuaweiExtension406Fn {}
unsafe impl Send for HuaweiExtension406Fn {}
unsafe impl Sync for HuaweiExtension406Fn {}
impl HuaweiExtension406Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpExtension407Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_407\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension407Fn {}
unsafe impl Send for GgpExtension407Fn {}
unsafe impl Sync for GgpExtension407Fn {}
impl GgpExtension407Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpExtension408Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_408\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension408Fn {}
unsafe impl Send for GgpExtension408Fn {}
unsafe impl Sync for GgpExtension408Fn {}
impl GgpExtension408Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpExtension409Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_409\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension409Fn {}
unsafe impl Send for GgpExtension409Fn {}
unsafe impl Sync for GgpExtension409Fn {}
impl GgpExtension409Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpExtension410Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_410\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension410Fn {}
unsafe impl Send for GgpExtension410Fn {}
unsafe impl Sync for GgpExtension410Fn {}
impl GgpExtension410Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GgpExtension411Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_extension_411\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GgpExtension411Fn {}
unsafe impl Send for GgpExtension411Fn {}
unsafe impl Sync for GgpExtension411Fn {}
impl GgpExtension411Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtBorderColorSwizzleFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_border_color_swizzle\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtBorderColorSwizzleFn {}
unsafe impl Send for ExtBorderColorSwizzleFn {}
unsafe impl Sync for ExtBorderColorSwizzleFn {}
impl ExtBorderColorSwizzleFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_border_color_swizzle'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1_000_411_000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1_000_411_001);
}
impl ExtPageableDeviceLocalMemoryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_pageable_device_local_memory\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
#[derive(Clone)]
pub struct ExtPageableDeviceLocalMemoryFn {
    pub set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
}
unsafe impl Send for ExtPageableDeviceLocalMemoryFn {}
unsafe impl Sync for ExtPageableDeviceLocalMemoryFn {}
impl ExtPageableDeviceLocalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_device_memory_priority_ext: unsafe {
                unsafe extern "system" fn set_device_memory_priority_ext(
                    _device: Device,
                    _memory: DeviceMemory,
                    _priority: f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_device_memory_priority_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDeviceMemoryPriorityEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_device_memory_priority_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_pageable_device_local_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1_000_412_000);
}
impl KhrMaintenance4Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance4\0") }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[derive(Clone)]
pub struct KhrMaintenance4Fn {
    pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
    pub get_device_image_sparse_memory_requirements_khr:
        PFN_vkGetDeviceImageSparseMemoryRequirements,
}
unsafe impl Send for KhrMaintenance4Fn {}
unsafe impl Sync for KhrMaintenance4Fn {}
impl KhrMaintenance4Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_buffer_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_buffer_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceBufferMemoryRequirements,
                    _p_memory_requirements: *mut MemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_buffer_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceBufferMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_buffer_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_image_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_image_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceImageMemoryRequirements,
                    _p_memory_requirements: *mut MemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_image_sparse_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_image_sparse_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceImageMemoryRequirements,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_sparse_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageSparseMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_sparse_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance4'"]
impl ImageAspectFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_maintenance4'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
}
impl HuaweiExtension415Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_extension_415\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct HuaweiExtension415Fn {}
unsafe impl Send for HuaweiExtension415Fn {}
unsafe impl Sync for HuaweiExtension415Fn {}
impl HuaweiExtension415Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ArmExtension416Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_416\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension416Fn {}
unsafe impl Send for ArmExtension416Fn {}
unsafe impl Sync for ArmExtension416Fn {}
impl ArmExtension416Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension417Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_417\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension417Fn {}
unsafe impl Send for KhrExtension417Fn {}
unsafe impl Sync for KhrExtension417Fn {}
impl KhrExtension417Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ArmExtension418Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_418\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension418Fn {}
unsafe impl Send for ArmExtension418Fn {}
unsafe impl Sync for ArmExtension418Fn {}
impl ArmExtension418Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension419Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_419\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension419Fn {}
unsafe impl Send for ExtExtension419Fn {}
unsafe impl Sync for ExtExtension419Fn {}
impl ExtExtension419Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_419'"]
impl ImageViewCreateFlags {
    pub const RESERVED_3_EXT: Self = Self(0b1000);
}
impl ExtExtension420Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_420\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension420Fn {}
unsafe impl Send for ExtExtension420Fn {}
unsafe impl Sync for ExtExtension420Fn {}
impl ExtExtension420Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ValveDescriptorSetHostMappingFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_VALVE_descriptor_set_host_mapping\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut c_void,
);
#[derive(Clone)]
pub struct ValveDescriptorSetHostMappingFn {
    pub get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    pub get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
}
unsafe impl Send for ValveDescriptorSetHostMappingFn {}
unsafe impl Sync for ValveDescriptorSetHostMappingFn {}
impl ValveDescriptorSetHostMappingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_descriptor_set_layout_host_mapping_info_valve: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_host_mapping_info_valve(
                    _device: Device,
                    _p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
                    _p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_host_mapping_info_valve)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutHostMappingInfoVALVE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_host_mapping_info_valve
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_descriptor_set_host_mapping_valve: unsafe {
                unsafe extern "system" fn get_descriptor_set_host_mapping_valve(
                    _device: Device,
                    _descriptor_set: DescriptorSet,
                    _pp_data: *mut *mut c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_host_mapping_valve)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetHostMappingVALVE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_host_mapping_valve
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_VALVE_descriptor_set_host_mapping'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self =
        Self(1_000_420_000);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1_000_420_001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1_000_420_002);
}
impl ExtExtension422Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_422\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension422Fn {}
unsafe impl Send for ExtExtension422Fn {}
unsafe impl Sync for ExtExtension422Fn {}
impl ExtExtension422Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtNonSeamlessCubeMapFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_non_seamless_cube_map\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtNonSeamlessCubeMapFn {}
unsafe impl Send for ExtNonSeamlessCubeMapFn {}
unsafe impl Sync for ExtNonSeamlessCubeMapFn {}
impl ExtNonSeamlessCubeMapFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_non_seamless_cube_map'"]
impl SamplerCreateFlags {
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(0b100);
}
#[doc = "Generated from 'VK_EXT_non_seamless_cube_map'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1_000_422_000);
}
impl ArmExtension424Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_424\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension424Fn {}
unsafe impl Send for ArmExtension424Fn {}
unsafe impl Sync for ArmExtension424Fn {}
impl ArmExtension424Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ArmExtension425Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_425\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension425Fn {}
unsafe impl Send for ArmExtension425Fn {}
unsafe impl Sync for ArmExtension425Fn {}
impl ArmExtension425Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomFragmentDensityMapOffsetFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_QCOM_fragment_density_map_offset\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomFragmentDensityMapOffsetFn {}
unsafe impl Send for QcomFragmentDensityMapOffsetFn {}
unsafe impl Sync for QcomFragmentDensityMapOffsetFn {}
impl QcomFragmentDensityMapOffsetFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_fragment_density_map_offset'"]
impl ImageCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self(0b1000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_fragment_density_map_offset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1_000_425_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self =
        Self(1_000_425_001);
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1_000_425_002);
}
impl NvExtension427Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_427\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension427Fn {}
unsafe impl Send for NvExtension427Fn {}
unsafe impl Sync for NvExtension427Fn {}
impl NvExtension427Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension428Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_428\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension428Fn {}
unsafe impl Send for NvExtension428Fn {}
unsafe impl Sync for NvExtension428Fn {}
impl NvExtension428Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension429Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_429\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension429Fn {}
unsafe impl Send for NvExtension429Fn {}
unsafe impl Sync for NvExtension429Fn {}
impl NvExtension429Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension430Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_430\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension430Fn {}
unsafe impl Send for NvExtension430Fn {}
unsafe impl Sync for NvExtension430Fn {}
impl NvExtension430Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvLinearColorAttachmentFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_linear_color_attachment\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvLinearColorAttachmentFn {}
unsafe impl Send for NvLinearColorAttachmentFn {}
unsafe impl Sync for NvLinearColorAttachmentFn {}
impl NvLinearColorAttachmentFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_linear_color_attachment'"]
impl FormatFeatureFlags2 {
    #[doc = "Format support linear image as render target, it cannot be mixed with non linear attachment"]
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_linear_color_attachment'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1_000_430_000);
}
impl NvExtension432Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_432\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension432Fn {}
unsafe impl Send for NvExtension432Fn {}
unsafe impl Sync for NvExtension432Fn {}
impl NvExtension432Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension433Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_433\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension433Fn {}
unsafe impl Send for NvExtension433Fn {}
unsafe impl Sync for NvExtension433Fn {}
impl NvExtension433Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleSurfacelessQueryFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_surfaceless_query\0") }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleSurfacelessQueryFn {}
unsafe impl Send for GoogleSurfacelessQueryFn {}
unsafe impl Sync for GoogleSurfacelessQueryFn {}
impl GoogleSurfacelessQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl KhrExtension435Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_extension_435\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct KhrExtension435Fn {}
unsafe impl Send for KhrExtension435Fn {}
unsafe impl Sync for KhrExtension435Fn {}
impl KhrExtension435Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension436Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_436\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension436Fn {}
unsafe impl Send for NvExtension436Fn {}
unsafe impl Sync for NvExtension436Fn {}
impl NvExtension436Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension437Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_437\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension437Fn {}
unsafe impl Send for ExtExtension437Fn {}
unsafe impl Sync for ExtExtension437Fn {}
impl ExtExtension437Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtImageCompressionControlSwapchainFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_EXT_image_compression_control_swapchain\0",
            )
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageCompressionControlSwapchainFn {}
unsafe impl Send for ExtImageCompressionControlSwapchainFn {}
unsafe impl Sync for ExtImageCompressionControlSwapchainFn {}
impl ExtImageCompressionControlSwapchainFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_image_compression_control_swapchain'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1_000_437_000);
}
impl SecExtension439Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_extension_439\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct SecExtension439Fn {}
unsafe impl Send for SecExtension439Fn {}
unsafe impl Sync for SecExtension439Fn {}
impl SecExtension439Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl QcomExtension440Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_440\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension440Fn {}
unsafe impl Send for QcomExtension440Fn {}
unsafe impl Sync for QcomExtension440Fn {}
impl QcomExtension440Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_extension_440'"]
impl DeviceQueueCreateFlags {
    pub const RESERVED_1_QCOM: Self = Self(0b10);
}
#[doc = "Generated from 'VK_QCOM_extension_440'"]
impl QueueFlags {
    pub const RESERVED_7_QCOM: Self = Self(0b1000_0000);
}
impl QcomExtension441Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_extension_441\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct QcomExtension441Fn {}
unsafe impl Send for QcomExtension441Fn {}
unsafe impl Sync for QcomExtension441Fn {}
impl QcomExtension441Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_QCOM_extension_441'"]
impl FormatFeatureFlags2 {
    pub const RESERVED_34_QCOM: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_35_QCOM: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_36_QCOM: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_37_QCOM: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_extension_441'"]
impl ImageUsageFlags {
    pub const RESERVED_20_QCOM: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const RESERVED_21_QCOM: Self = Self(0b10_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_extension_441'"]
impl SamplerCreateFlags {
    pub const IMAGE_PROCESSING_QCOM: Self = Self(0b1_0000);
}
impl CoreaviExtension442Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_442\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension442Fn {}
unsafe impl Send for CoreaviExtension442Fn {}
unsafe impl Sync for CoreaviExtension442Fn {}
impl CoreaviExtension442Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl CoreaviExtension443Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_443\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension443Fn {}
unsafe impl Send for CoreaviExtension443Fn {}
unsafe impl Sync for CoreaviExtension443Fn {}
impl CoreaviExtension443Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl CoreaviExtension444Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_444\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension444Fn {}
unsafe impl Send for CoreaviExtension444Fn {}
unsafe impl Sync for CoreaviExtension444Fn {}
impl CoreaviExtension444Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_COREAVI_extension_444'"]
impl CommandPoolResetFlags {
    pub const RESERVED_1_COREAVI: Self = Self(0b10);
}
impl CoreaviExtension445Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_445\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension445Fn {}
unsafe impl Send for CoreaviExtension445Fn {}
unsafe impl Sync for CoreaviExtension445Fn {}
impl CoreaviExtension445Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl CoreaviExtension446Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_446\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension446Fn {}
unsafe impl Send for CoreaviExtension446Fn {}
unsafe impl Sync for CoreaviExtension446Fn {}
impl CoreaviExtension446Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl CoreaviExtension447Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_COREAVI_extension_447\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct CoreaviExtension447Fn {}
unsafe impl Send for CoreaviExtension447Fn {}
unsafe impl Sync for CoreaviExtension447Fn {}
impl CoreaviExtension447Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl SecExtension448Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_extension_448\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct SecExtension448Fn {}
unsafe impl Send for SecExtension448Fn {}
unsafe impl Sync for SecExtension448Fn {}
impl SecExtension448Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl SecExtension449Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_extension_449\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct SecExtension449Fn {}
unsafe impl Send for SecExtension449Fn {}
unsafe impl Sync for SecExtension449Fn {}
impl SecExtension449Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl SecExtension450Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_extension_450\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct SecExtension450Fn {}
unsafe impl Send for SecExtension450Fn {}
unsafe impl Sync for SecExtension450Fn {}
impl SecExtension450Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl SecExtension451Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_extension_451\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct SecExtension451Fn {}
unsafe impl Send for SecExtension451Fn {}
unsafe impl Sync for SecExtension451Fn {}
impl SecExtension451Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension452Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_452\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension452Fn {}
unsafe impl Send for NvExtension452Fn {}
unsafe impl Sync for NvExtension452Fn {}
impl NvExtension452Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ArmExtension453Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_extension_453\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ArmExtension453Fn {}
unsafe impl Send for ArmExtension453Fn {}
unsafe impl Sync for ArmExtension453Fn {}
impl ArmExtension453Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension454Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_454\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension454Fn {}
unsafe impl Send for GoogleExtension454Fn {}
unsafe impl Sync for GoogleExtension454Fn {}
impl GoogleExtension454Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl GoogleExtension455Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_extension_455\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct GoogleExtension455Fn {}
unsafe impl Send for GoogleExtension455Fn {}
unsafe impl Sync for GoogleExtension455Fn {}
impl GoogleExtension455Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension456Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_456\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension456Fn {}
unsafe impl Send for NvExtension456Fn {}
unsafe impl Sync for NvExtension456Fn {}
impl NvExtension456Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension457Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_457\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension457Fn {}
unsafe impl Send for ExtExtension457Fn {}
unsafe impl Sync for ExtExtension457Fn {}
impl ExtExtension457Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension458Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_458\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension458Fn {}
unsafe impl Send for ExtExtension458Fn {}
unsafe impl Sync for ExtExtension458Fn {}
impl ExtExtension458Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtSubpassMergeFeedbackFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_subpass_merge_feedback\0")
        }
    }
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSubpassMergeFeedbackFn {}
unsafe impl Send for ExtSubpassMergeFeedbackFn {}
unsafe impl Sync for ExtSubpassMergeFeedbackFn {}
impl ExtSubpassMergeFeedbackFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_subpass_merge_feedback'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1_000_458_000);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1_000_458_001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1_000_458_002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1_000_458_003);
}
impl ExtExtension460Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_460\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension460Fn {}
unsafe impl Send for ExtExtension460Fn {}
unsafe impl Sync for ExtExtension460Fn {}
impl ExtExtension460Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension461Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_461\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension461Fn {}
unsafe impl Send for ExtExtension461Fn {}
unsafe impl Sync for ExtExtension461Fn {}
impl ExtExtension461Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_461'"]
impl FormatFeatureFlags2 {
    pub const RESERVED_39_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
impl ExtExtension462Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_462\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension462Fn {}
unsafe impl Send for ExtExtension462Fn {}
unsafe impl Sync for ExtExtension462Fn {}
impl ExtExtension462Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtShaderModuleIdentifierFn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe {
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_module_identifier\0")
        }
    }
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_identifier: *mut ShaderModuleIdentifierEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_identifier: *mut ShaderModuleIdentifierEXT,
);
#[derive(Clone)]
pub struct ExtShaderModuleIdentifierFn {
    pub get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    pub get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
}
unsafe impl Send for ExtShaderModuleIdentifierFn {}
unsafe impl Sync for ExtShaderModuleIdentifierFn {}
impl ExtShaderModuleIdentifierFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_shader_module_identifier_ext: unsafe {
                unsafe extern "system" fn get_shader_module_identifier_ext(
                    _device: Device,
                    _shader_module: ShaderModule,
                    _p_identifier: *mut ShaderModuleIdentifierEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_shader_module_identifier_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetShaderModuleIdentifierEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_shader_module_identifier_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_shader_module_create_info_identifier_ext: unsafe {
                unsafe extern "system" fn get_shader_module_create_info_identifier_ext(
                    _device: Device,
                    _p_create_info: *const ShaderModuleCreateInfo,
                    _p_identifier: *mut ShaderModuleIdentifierEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_shader_module_create_info_identifier_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetShaderModuleCreateInfoIdentifierEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_shader_module_create_info_identifier_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_shader_module_identifier'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1_000_462_000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1_000_462_001);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1_000_462_002);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1_000_462_003);
}
impl ExtExtension464Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_464\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension464Fn {}
unsafe impl Send for ExtExtension464Fn {}
unsafe impl Sync for ExtExtension464Fn {}
impl ExtExtension464Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl NvExtension465Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extension_465\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct NvExtension465Fn {}
unsafe impl Send for NvExtension465Fn {}
unsafe impl Sync for NvExtension465Fn {}
impl NvExtension465Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_NV_extension_465'"]
impl AccessFlags2 {
    pub const RESERVED_42_NV: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_43_NV: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_465'"]
impl FormatFeatureFlags2 {
    pub const RESERVED_40_NV: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_41_NV: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_42_NV: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESERVED_43_NV: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_465'"]
impl PipelineStageFlags2 {
    pub const RESERVED_29_NV: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_extension_465'"]
impl QueueFlags {
    pub const RESERVED_8_NV: Self = Self(0b1_0000_0000);
}
impl ExtExtension466Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_466\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension466Fn {}
unsafe impl Send for ExtExtension466Fn {}
unsafe impl Sync for ExtExtension466Fn {}
impl ExtExtension466Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_466'"]
impl RenderingFlags {
    pub const RESERVED_3_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_extension_466'"]
impl SubpassDescriptionFlags {
    pub const RESERVED_7_EXT: Self = Self(0b1000_0000);
}
impl ExtExtension467Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_467\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension467Fn {}
unsafe impl Send for ExtExtension467Fn {}
unsafe impl Sync for ExtExtension467Fn {}
impl ExtExtension467Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_467'"]
impl PipelineCreateFlags {
    pub const RESERVED_27_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
}
impl ExtExtension468Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_468\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension468Fn {}
unsafe impl Send for ExtExtension468Fn {}
unsafe impl Sync for ExtExtension468Fn {}
impl ExtExtension468Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AndroidExtension469Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ANDROID_extension_469\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AndroidExtension469Fn {}
unsafe impl Send for AndroidExtension469Fn {}
unsafe impl Sync for AndroidExtension469Fn {}
impl AndroidExtension469Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension470Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_470\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension470Fn {}
unsafe impl Send for AmdExtension470Fn {}
unsafe impl Sync for AmdExtension470Fn {}
impl AmdExtension470Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension471Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_471\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension471Fn {}
unsafe impl Send for AmdExtension471Fn {}
unsafe impl Sync for AmdExtension471Fn {}
impl AmdExtension471Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension472Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_472\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension472Fn {}
unsafe impl Send for AmdExtension472Fn {}
unsafe impl Sync for AmdExtension472Fn {}
impl AmdExtension472Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension473Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_473\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension473Fn {}
unsafe impl Send for AmdExtension473Fn {}
unsafe impl Sync for AmdExtension473Fn {}
impl AmdExtension473Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension474Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_474\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension474Fn {}
unsafe impl Send for AmdExtension474Fn {}
unsafe impl Sync for AmdExtension474Fn {}
impl AmdExtension474Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension475Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_475\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension475Fn {}
unsafe impl Send for AmdExtension475Fn {}
unsafe impl Sync for AmdExtension475Fn {}
impl AmdExtension475Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension476Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_476\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension476Fn {}
unsafe impl Send for AmdExtension476Fn {}
unsafe impl Sync for AmdExtension476Fn {}
impl AmdExtension476Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension477Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_477\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension477Fn {}
unsafe impl Send for AmdExtension477Fn {}
unsafe impl Sync for AmdExtension477Fn {}
impl AmdExtension477Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension478Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_478\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension478Fn {}
unsafe impl Send for AmdExtension478Fn {}
unsafe impl Sync for AmdExtension478Fn {}
impl AmdExtension478Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl AmdExtension479Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_extension_479\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct AmdExtension479Fn {}
unsafe impl Send for AmdExtension479Fn {}
unsafe impl Sync for AmdExtension479Fn {}
impl AmdExtension479Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension480Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_480\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension480Fn {}
unsafe impl Send for ExtExtension480Fn {}
unsafe impl Sync for ExtExtension480Fn {}
impl ExtExtension480Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension481Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_481\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension481Fn {}
unsafe impl Send for ExtExtension481Fn {}
unsafe impl Sync for ExtExtension481Fn {}
impl ExtExtension481Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension482Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_482\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension482Fn {}
unsafe impl Send for ExtExtension482Fn {}
unsafe impl Sync for ExtExtension482Fn {}
impl ExtExtension482Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
impl ExtExtension483Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_483\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension483Fn {}
unsafe impl Send for ExtExtension483Fn {}
unsafe impl Sync for ExtExtension483Fn {}
impl ExtExtension483Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[doc = "Generated from 'VK_EXT_extension_483'"]
impl ShaderStageFlags {
    pub const EXT_483_RESERVE_15: Self = Self(0b1000_0000_0000_0000);
    pub const EXT_483_RESERVE_16: Self = Self(0b1_0000_0000_0000_0000);
    pub const EXT_483_RESERVE_17: Self = Self(0b10_0000_0000_0000_0000);
}
impl ExtExtension484Fn {
    pub const fn name() -> &'static ::std::ffi::CStr {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extension_484\0") }
    }
    pub const SPEC_VERSION: u32 = 0u32;
}
#[derive(Clone)]
pub struct ExtExtension484Fn {}
unsafe impl Send for ExtExtension484Fn {}
unsafe impl Sync for ExtExtension484Fn {}
impl ExtExtension484Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
