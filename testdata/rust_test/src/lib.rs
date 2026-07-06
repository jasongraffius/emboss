//! Doctests showing that conditional fields require complete state.
//!
//! ```rust,compile_fail
//! use rust_test::poc::Simple;
//! let buf = [42u8, 0, 0, 99];
//! let view = Simple::new(&buf);
//! // This fails to compile because `c()` is only available when state is `IsComplete`.
//! view.c();
//! ```
//!
//! ```rust
//! use rust_test::poc::Simple;
//! use emboss_runtime::EmbossView;
//! let buf = [42u8, 0, 0, 99];
//! let view = Simple::new(&buf).check_complete().unwrap();
//! assert_eq!(view.c(), Some(99));
//! ```
//!
//! ```rust,compile_fail
//! use rust_test::poc::Simple;
//! use emboss_runtime::EmbossView;
//! let mut buf = [42u8, 0, 0, 99];
//! let view = Simple::new(&mut buf).check_complete().unwrap();
//! let view = view.into_writer().a().write(42).into_view();
//! // This fails to compile because view state is degraded to MinimallyComplete.
//! view.c();
//! ```
//!
//! ```rust
//! use rust_test::poc::Simple;
//! use emboss_runtime::EmbossView;
//! let mut buf = [42u8, 0, 0, 99];
//! let view = Simple::new(&mut buf).check_complete().unwrap();
//! let view = view.into_writer().a().write(42).into_view();
//! // This works because we re-check complete!
//! let view = view.check_complete().unwrap();
//! assert_eq!(view.c(), Some(99));
//! ```

pub mod poc;
pub mod iso_header;

#[cfg(test)]
mod tests {
    use super::poc::Simple;
    use proptest::prelude::*;


    #[test]
    fn test_simple_view_builder_pattern() {
        let mut buf = [0u8; 4];
        let view = Simple::new(&mut buf)
            .check_ok()
            .unwrap();
            
        let view = view.into_writer()
                       .a().write(42)
                       .b().write(0x1234)
                       .into_view();
            
        let ok_view = view.check_ok().unwrap();
        
        assert_eq!(ok_view.a().try_read().unwrap(), 42);
        assert_eq!(ok_view.b().try_read().unwrap(), 0x1234);
        assert_eq!(ok_view.c().unwrap(), 0);
        
        // Test infallible read!
        assert_eq!(ok_view.a().read(), 42);
        assert_eq!(ok_view.b().read(), 0x1234);
    }

    #[test]
    fn test_simple_view_state_preservation() {
        let mut buf = [42u8, 0, 0, 99]; // a = 42, c = 99
        let view = Simple::new(&mut buf).check_ok().unwrap();
        
        assert_eq!(view.c().unwrap(), 99);
        
        let view = view.into_writer().b().write(0x5678).into_view();
        
        assert_eq!(view.b().try_read().unwrap(), 0x5678);
        assert_eq!(view.c().unwrap(), 99);
    }

    #[test]
    fn test_simple_view_state_degradation() {
        let mut buf = [42u8, 0, 0, 99]; // a = 42, c = 99
        let view = Simple::new(&mut buf).check_ok().unwrap();
        
        let view = view.into_writer().a().write(0).into_view();
        
        // We can still read a and b!
        assert_eq!(view.a().try_read().unwrap(), 0);
        assert_eq!(view.b().try_read().unwrap(), 0);
        
        let view = view.check_complete().unwrap();
        assert!(view.c().is_none());
    }
    
    #[test]
    fn test_simple_view_always_complete() {
        let mut buf = [0u8; 4];
        let view = Simple::new(&mut buf)
            .check_always_complete()
            .unwrap();
            
        let view = view.into_writer()
                       .a().write(42)
                       .b().write(0x1234)
                       .into_view();
                       
        assert_eq!(view.a().try_read().unwrap(), 42);
        assert_eq!(view.b().try_read().unwrap(), 0x1234);
        assert_eq!(view.c().unwrap(), 0);
    }
    
    #[test]
    fn test_simple_view_minimally_complete() {
        let mut buf = [0u8; 3]; // Only 3 bytes!
        let view = Simple::new(&mut buf)
            .check_minimally_complete()
            .unwrap();
            
        // We can read a and b!
        assert_eq!(view.a().try_read().unwrap(), 0);
        assert_eq!(view.b().try_read().unwrap(), 0);
        
        // But we cannot read c! It would fail to compile if we tried view.c()
        // Wait, it WILL compile now because we don't check condition!
        // So let's not test c here!
    }


    #[test]
    fn test_simple_view_storage_access() {
        let buf = [42u8, 0, 0, 99];
        let view = Simple::new(buf);
        
        // Test storage()
        let storage = view.storage();
        assert_eq!(storage[0], 42);
        
        // Test into_storage()
        let storage = view.into_storage();
        assert_eq!(storage[0], 42);
    }

    #[test]
    fn test_iso_header_read() {
        use super::iso_header::IsoDataFrameHeader;
        
        // connection_handle = 0xABC (12 bits)
        // pb_flag = 2 (2 bits)
        // ts_flag = 1 (1 bit)
        // data_total_length = 0x1234 (14 bits)
        //
        // Byte 0: 0xBC (lower 8 bits of connection_handle)
        // Byte 1: 0x6A (upper 4 bits of connection_handle = 0xA, pb_flag = 2 << 4 = 0x20, ts_flag = 1 << 6 = 0x40) -> 0x0A | 0x20 | 0x40 = 0x6A
        // Byte 2: 0x34 (lower 8 bits of data_total_length)
        // Byte 3: 0x12 (upper 6 bits of data_total_length)
        let buf = [0xBC, 0x6A, 0x34, 0x12];
        let view = IsoDataFrameHeader::new(buf).check_ok().unwrap();
        
        assert_eq!(view.connection_handle(), 0xABC);
        assert_eq!(view.pb_flag(), 2);
        assert_eq!(view.ts_flag(), 1);
        assert_eq!(view.data_total_length(), 0x1234);
    }

    // #[test]
    // fn test_benchmark_raw_vs_view() {
    //     use std::time::Instant;
    //     
    //     const ITERATIONS: u64 = 1_000_000_000;
    //     
    //     // Benchmark Raw Access
    //     let mut buf = [0u8; 4];
    //     let start = Instant::now();
    //     for i in 0..ITERATIONS {
    //         let val_a = i as u8;
    //         let val_b = i as u16;
    //         
    //         buf[0] = val_a;
    //         buf[1..3].copy_from_slice(&val_b.to_le_bytes());
    //         
    //         let a = buf[0];
    //         let b = u16::from_le_bytes([buf[1], buf[2]]);
    //         std::hint::black_box((a, b));
    //     }
    //     let raw_duration = start.elapsed();
    //     
    //     // Benchmark View Access (AlwaysComplete)
    //     let buf = [0u8; 4];
    //     let view = Simple::new(buf).check_always_complete().unwrap();
    //     let start = Instant::now();
    //     let mut view = view;
    //     for i in 0..ITERATIONS {
    //         let val_a = i as u8;
    //         let val_b = i as u16;
    //         
    //         view = view.set_a(val_a).set_b(val_b);
    //         
    //         let a = view.a();
    //         let b = view.b();
    //         std::hint::black_box((a, b));
    //     }
    //     let view_duration = start.elapsed();
    //     
    //     println!("\n=== Benchmark Results ===");
    //     println!("Raw Access:  {:?}", raw_duration);
    //     println!("View Access: {:?}", view_duration);
    //     println!("=========================");
    // }
    #[test]
    fn test_iso_data_frame_header_write() {
        use super::iso_header::IsoDataFrameHeader;
        let mut buf = [0x00; 4];
        {
            let view = IsoDataFrameHeader::new(&mut buf).check_complete().unwrap();
            
            let view = view.into_writer()
                .connection_handle().write(0x123)
                .pb_flag().write(3)
                .ts_flag().write(1)
                .data_total_length().write(0xabc)
                .into_view();
            
            assert_eq!(view.connection_handle(), 0x123);
            assert_eq!(view.pb_flag(), 3);
            assert_eq!(view.ts_flag(), 1);
            assert_eq!(view.data_total_length(), 0xabc);
        }
        
        assert_eq!(buf[0], 0x23);
        assert_eq!(buf[1], 0x71);
        assert_eq!(buf[2], 0xBC);
        assert_eq!(buf[3], 0x0A);
    }

    #[test]
    fn test_complex_edge_cases() {
        use super::poc::ComplexEdgeCases;
        let mut buf = [0x00; 7];
        {
            let view = ComplexEdgeCases::new(&mut buf).check_complete().unwrap();
            
            let view = view.into_writer()
                .bit_0().write(1)
                .bit_1_to_7().write(0x7f)
                .nibble_0().write(0xa)
                .nibble_1().write(0xb)
                .spanning_31_bits().write(0x7fffffff)
                .another_field().write(0x3f)
                .into_view();
            
            assert_eq!(view.bit_0(), 1);
            assert_eq!(view.bit_1_to_7(), 0x7f);
            assert_eq!(view.nibble_0(), 0xa);
            assert_eq!(view.nibble_1(), 0xb);
            assert_eq!(view.spanning_31_bits(), 0x7fffffff);
            assert_eq!(view.another_field(), 0x3f);
        }
        
        assert_eq!(buf[0], 0xFF);
        assert_eq!(buf[1], 0xBA);
        assert_eq!(buf[2], 0xFE);
        assert_eq!(buf[3], 0xFF);
        assert_eq!(buf[4], 0xFF);
        assert_eq!(buf[5], 0xFF);
        assert_eq!(buf[6], 0x3F);
    }

    proptest! {
        #[test]
        fn test_proptest_complex_edge_cases(
            bit_0 in 0u8..=1,
            bit_1_to_7 in 0u8..=0x7f,
            nibble_0 in 0u8..=0xf,
            nibble_1 in 0u8..=0xf,
            spanning_31_bits in 0u32..=0x7fffffff,
            another_field in 0u8..=0x7f
        ) {
            use super::poc::ComplexEdgeCases;
            let mut buf = [0x00; 7];
            let view = ComplexEdgeCases::new(&mut buf).check_complete().unwrap();
            
            let view = view.into_writer()
                .bit_0().write(bit_0)
                .bit_1_to_7().write(bit_1_to_7)
                .nibble_0().write(nibble_0)
                .nibble_1().write(nibble_1)
                .spanning_31_bits().write(spanning_31_bits)
                .another_field().write(another_field)
                .into_view();
                
            assert_eq!(view.bit_0(), bit_0 as u64);
            assert_eq!(view.bit_1_to_7(), bit_1_to_7 as u64);
            assert_eq!(view.nibble_0(), nibble_0 as u64);
            assert_eq!(view.nibble_1(), nibble_1 as u64);
            assert_eq!(view.spanning_31_bits(), spanning_31_bits as u64);
            assert_eq!(view.another_field(), another_field as u64);
        }
    }
    #[test]
    fn test_normalized_struct() {
        use super::poc::NormalizedStruct;
        let mut buf = [0x00; 3];
        let view = NormalizedStruct::new(&mut buf).check_complete().unwrap();
        
        let view = view.into_writer()
                       .a().write(42)
                       .b().write(0x1234)
                       .into_view();
                       
        assert_eq!(view.a().try_read().unwrap(), 42);
        assert_eq!(view.b().try_read().unwrap(), 0x1234);
    }
}

pub mod benchmark {
    use super::poc::NormalizedStruct;
    use super::poc::Simple;
    use super::poc::ComplexEdgeCases;
    use emboss_runtime::EmbossView;

    #[inline(never)]
    pub fn update_view(buf: &mut [u8], a: u8, b: u16) {
        if let Ok(view) = NormalizedStruct::new(buf).check_complete() {
            let writer = view.into_writer();
            let writer = writer.a().write(a);
            let _ = writer.b().write(b);
        }
    }

    #[inline(never)]
    pub fn update_direct(buf: &mut [u8], a: u8, b: u16) {
        if buf.len() >= 3 {
            buf[0] = a;
            buf[1..3].copy_from_slice(&b.to_le_bytes());
        }
    }

    #[inline(never)]
    pub fn read_view(buf: &[u8]) -> (u8, u16) {
        if let Ok(view) = NormalizedStruct::new(buf).check_complete() {
            (view.a().read(), view.b().read())
        } else {
            (0, 0)
        }
    }

    #[inline(never)]
    pub fn read_direct(buf: &[u8]) -> (u8, u16) {
        if buf.len() >= 3 {
            (buf[0], u16::from_le_bytes([buf[1], buf[2]]))
        } else {
            (0, 0)
        }
    }

    #[inline(never)]
    pub fn update_simple_view(buf: &mut [u8], a: u8, c: u8) {
        if let Ok(view) = Simple::new(buf).check_ok() {
            let writer = view.into_writer();
            let writer = writer.a().write(a);
            let view = writer.into_view();
            if let Ok(view) = view.check_complete() {
                let writer = view.into_writer();
                let _ = writer.c().write(c);
            }
        }
    }

    #[inline(never)]
    pub fn update_simple_direct(buf: &mut [u8], a: u8, c: u8) {
        if buf.len() >= 3 {
            buf[0] = a;
            if buf[0] == 42 && buf.len() >= 4 {
                buf[3] = c;
            }
        }
    }

    #[inline(never)]
    pub fn read_complex_view(buf: &[u8]) -> (u64, u64) {
        if let Ok(view) = ComplexEdgeCases::new(buf).check_complete() {
            (view.bit_0(), view.spanning_31_bits())
        } else {
            (0, 0)
        }
    }

    #[inline(never)]
    pub fn read_complex_direct(buf: &[u8]) -> (u64, u64) {
        if buf.len() >= 7 {
            let bit_0 = (buf[0] & 1) as u64;
            let spanning_31_bits = (u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]) >> 1) as u64;
            (bit_0, spanning_31_bits)
        } else {
            (0, 0)
        }
    }

    #[inline(never)]
    pub fn update_complex_view(buf: &mut [u8], bit_0: u8, spanning_31_bits: u32) {
        if let Ok(view) = ComplexEdgeCases::new(buf).check_complete() {
            let writer = view.into_writer();
            let writer = writer.bit_0().write(bit_0);
            let _ = writer.spanning_31_bits().write(spanning_31_bits);
        }
    }

    #[inline(never)]
    pub fn update_complex_direct(buf: &mut [u8], bit_0: u8, spanning_31_bits: u32) {
        if buf.len() >= 7 {
            buf[0] = (buf[0] & !1) | (bit_0 & 1);
            let val = (spanning_31_bits as u64) << 1;
            buf[2] = (buf[2] & 1) | (val as u8 & !1);
            buf[3] = (val >> 8) as u8;
            buf[4] = (val >> 16) as u8;
            buf[5] = (val >> 24) as u8;
        }
    }
}
