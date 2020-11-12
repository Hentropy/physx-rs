use crate::{owner::Owner, traits::Class};

use physx_sys::PxConvexMesh_release_mut;

#[repr(transparent)]
pub struct ConvexMesh {
    obj: physx_sys::PxConvexMesh,
}

crate::DeriveClassForNewType!(ConvexMesh: PxConvexMesh, PxBase);

impl ConvexMesh {
    /// Safety: Owner's drop the pointer they wrap when they are dropped.  Use `into_ptr`
    /// to retrieve the pointer and consume the Owner without dropping the pointee.
    pub(crate) unsafe fn from_raw(ptr: *mut physx_sys::PxConvexMesh) -> Option<Owner<ConvexMesh>> {
        Owner::from_raw(ptr as *mut Self)
    }
}

unsafe impl Send for ConvexMesh {}
unsafe impl Sync for ConvexMesh {}

impl Drop for ConvexMesh {
    fn drop(&mut self) {
        unsafe { PxConvexMesh_release_mut(self.as_mut_ptr()) }
    }
}
