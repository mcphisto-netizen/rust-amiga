//! Generic sequence implementation (Nim seq<T> equivalent)

use crate::amiga_mem::{alloc, dealloc};
use core::{ptr, slice};

pub struct Seq<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Seq<T> {
    pub fn new() -> Self {
        Self {  ptr::null_mut(), len: 0, capacity: 0 }
    }
    
    pub fn with_capacity(cap: usize) -> Self {
        let mut s = Self::new();
        s.reserve(cap);
        s
    }
    
    pub fn from_elem(elem: T, n: usize) -> Self where T: Clone {
        let mut s = Self::with_capacity(n);
        for _ in 0..n { s.push(elem.clone()); }
        s
    }
    
    #[inline] pub fn len(&self) -> usize { self.len }
    #[inline] pub fn is_empty(&self) -> bool { self.len == 0 }
    #[inline] pub fn capacity(&self) -> usize { self.capacity }
    
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            if self.data.is_null() { &[] }
            else { slice::from_raw_parts(self.data, self.len) }
        }
    }
    
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            if self.data.is_null() { &mut [] }
            else { slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
    
    pub fn reserve(&mut self, additional: usize) {
        let new_cap = self.len + additional;
        if new_cap <= self.capacity { return; }
        
        let new_capacity = core::cmp::max(new_cap, self.capacity * 2);
        let new_size = new_capacity * core::mem::size_of::<T>();
        
        let new_ Option<*mut u8> = if self.data.is_null() {
            alloc(new_size)
        } else {
            let old_size = self.capacity * core::mem::size_of::<T>();
            alloc(new_size).map(|new_ptr| {
                unsafe {
                    ptr::copy_nonoverlapping(
                        self.data as *mut u8,
                        new_ptr,
                        self.len * core::mem::size_of::<T>()
                    );
                    dealloc(self.data as *mut u8, old_size);
                }
                new_ptr
            })
        };
        
        if let Some(ptr) = new_data {
            self.data = ptr as *mut T;
            self.capacity = new_capacity;
        }
    }
    
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity { self.reserve(4); }
        if self.len < self.capacity {
            unsafe {
                ptr::write(self.data.add(self.len), value);
            }
            self.len += 1;
        }
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { None }
        else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.data.add(self.len)))
            }
        }
    }
    
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.data.add(i));
            }
        }
        self.len = 0;
    }
}

impl<T> Default for Seq<T> { fn default() -> Self { Self::new() } }

impl<T> Drop for Seq<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len { ptr::drop_in_place(self.data.add(i)); }
                let size = self.capacity * core::mem::size_of::<T>();
                dealloc(self.data as *mut u8, size);
            }
        }
    }
}

impl<T: Clone> Clone for Seq<T> {
    fn clone(&self) -> Self {
        let mut new = Self::with_capacity(self.len);
        for item in self.as_slice() { new.push(item.clone()); }
        new
    }
}

impl<T> core::ops::Index<usize> for Seq<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.as_slice()[index] }
}

impl<T> core::ops::IndexMut<usize> for Seq<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}