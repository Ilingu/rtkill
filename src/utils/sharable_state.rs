use std::{
    mem::ManuallyDrop,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

pub struct SharableState<T> {
    pub data: Arc<AtomicPtr<ManuallyDrop<T>>>,
}

impl<T> SharableState<T> {
    pub fn new(data: T) -> SharableState<T> {
        SharableState {
            data: Arc::new(AtomicPtr::new(Box::into_raw(Box::new(ManuallyDrop::new(
                data,
            ))))),
        }
    }

    pub fn read(&self) -> &ManuallyDrop<T> {
        let data_ptr = self.data.load(Ordering::Acquire);
        unsafe { &*data_ptr }
    }

    pub fn mutate<F: FnOnce(&mut ManuallyDrop<Box<ManuallyDrop<T>>>)>(&self, mutation: F) {
        // pre mutation
        let data_writer = Arc::clone(&self.data);
        let data_ptr = data_writer.load(Ordering::Relaxed);
        let mut data_box = unsafe { ManuallyDrop::new(Box::from_raw(data_ptr)) };

        // mutate data
        mutation(&mut data_box);

        // post mutation
        let new_box = Box::into_raw(ManuallyDrop::into_inner(data_box));
        data_writer.store(new_box, Ordering::Release);
    }
}
