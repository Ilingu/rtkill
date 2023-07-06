use std::{
    mem::ManuallyDrop,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

/// `**instable and unsafe**` way of storing and writing data accross threads with real-time update
///
/// The reason why I didn't use Arc+Mutex or RwLock is because I wanted to be able to streams and renders data as It was found
///
/// When the app first load it'll scan in the background for the 'target' dirs
///
/// But this can be quite long, so for the user to not wait until all the 'target' dirs are found and after that retuned to be displayed
///
/// I prefered to push to the ui the one that were already found and continue in the background the scan
///
/// However rust does not allows that without unsafe code (from what I've found)
///
/// I've done everything I can to mitigate the memory bugs or leaks but if writing and reading are too frequent (like idk 100 times per second for 5 second) the app will crashes
///
/// So in the code you'll find that I largely reduced the amount of write and read
///
/// Reads happen once every frame (which is at most 10 times a second)
///
/// Writes happen every time a user search for 'target' dirs (so at app startup or when refreshing)
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

    /// read current data in the state
    pub fn read(&self) -> &ManuallyDrop<T> {
        let data_ptr = self.data.load(Ordering::Acquire);
        unsafe { &*data_ptr }
    }

    /// write to data state
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
