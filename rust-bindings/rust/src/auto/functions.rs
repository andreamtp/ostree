// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use ObjectType;
use ffi;
use gio;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;


#[cfg(any(feature = "v2017_15", feature = "dox"))]
pub fn break_hardlink<'a, P: Into<Option<&'a gio::Cancellable>>>(dfd: i32, path: &str, skip_xattrs: bool, cancellable: P) -> Result<(), Error> {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_break_hardlink(dfd, path.to_glib_none().0, skip_xattrs.to_glib(), cancellable.0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn check_version(required_year: u32, required_release: u32) -> bool {
    unsafe {
        from_glib(ffi::ostree_check_version(required_year, required_release))
    }
}

//pub fn checksum_b64_from_bytes(csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32) -> Option<String> {
//    unsafe { TODO: call ffi::ostree_checksum_b64_from_bytes() }
//}

//pub fn checksum_b64_inplace_from_bytes(csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32, buf: &str) {
//    unsafe { TODO: call ffi::ostree_checksum_b64_inplace_from_bytes() }
//}

//pub fn checksum_b64_inplace_to_bytes(checksum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 28 }; 32, buf: u8) {
//    unsafe { TODO: call ffi::ostree_checksum_b64_inplace_to_bytes() }
//}

//pub fn checksum_b64_to_bytes(checksum: &str) -> /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32 {
//    unsafe { TODO: call ffi::ostree_checksum_b64_to_bytes() }
//}

//pub fn checksum_bytes_peek(bytes: &glib::Variant) -> /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32 {
//    unsafe { TODO: call ffi::ostree_checksum_bytes_peek() }
//}

//pub fn checksum_bytes_peek_validate(bytes: &glib::Variant) -> Result</*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32, Error> {
//    unsafe { TODO: call ffi::ostree_checksum_bytes_peek_validate() }
//}

//pub fn checksum_file<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(f: &P, objtype: ObjectType, out_csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32, cancellable: Q) -> Result<(), Error> {
//    unsafe { TODO: call ffi::ostree_checksum_file() }
//}

//pub fn checksum_file_async<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>, R: /*Ignored*/gio::AsyncReadyCallback>(f: &P, objtype: ObjectType, io_priority: i32, cancellable: Q, callback: R) {
//    unsafe { TODO: call ffi::ostree_checksum_file_async() }
//}

//#[cfg(any(feature = "v2017_13", feature = "dox"))]
//pub fn checksum_file_at<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a gio::Cancellable>>>(dfd: i32, path: &str, stbuf: P, objtype: ObjectType, flags: /*Ignored*/ChecksumFlags, out_checksum: &str, cancellable: Q) -> Result<(), Error> {
//    unsafe { TODO: call ffi::ostree_checksum_file_at() }
//}

//pub fn checksum_file_from_input<'a, 'b, 'c, P: Into<Option<&'a glib::Variant>>, Q: IsA<gio::InputStream> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c gio::Cancellable>>>(file_info: &gio::FileInfo, xattrs: P, in_: R, objtype: ObjectType, out_csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32, cancellable: S) -> Result<(), Error> {
//    unsafe { TODO: call ffi::ostree_checksum_file_from_input() }
//}

//pub fn checksum_from_bytes(csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32) -> Option<String> {
//    unsafe { TODO: call ffi::ostree_checksum_from_bytes() }
//}

pub fn checksum_from_bytes_v(csum_v: &glib::Variant) -> Option<String> {
    unsafe {
        from_glib_full(ffi::ostree_checksum_from_bytes_v(csum_v.to_glib_none().0))
    }
}

//pub fn checksum_inplace_from_bytes(csum: /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32, buf: &str) {
//    unsafe { TODO: call ffi::ostree_checksum_inplace_from_bytes() }
//}

//pub fn checksum_to_bytes(checksum: &str) -> /*Unknown conversion*//*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 32 {
//    unsafe { TODO: call ffi::ostree_checksum_to_bytes() }
//}

pub fn checksum_to_bytes_v(checksum: &str) -> Option<glib::Variant> {
    unsafe {
        from_glib_full(ffi::ostree_checksum_to_bytes_v(checksum.to_glib_none().0))
    }
}

//pub fn cmd__private__() -> /*Ignored*/Option<CmdPrivateVTable> {
//    unsafe { TODO: call ffi::ostree_cmd__private__() }
//}

pub fn commit_get_content_checksum(commit_variant: &glib::Variant) -> Option<String> {
    unsafe {
        from_glib_full(ffi::ostree_commit_get_content_checksum(commit_variant.to_glib_none().0))
    }
}

pub fn commit_get_parent(commit_variant: &glib::Variant) -> Option<String> {
    unsafe {
        from_glib_full(ffi::ostree_commit_get_parent(commit_variant.to_glib_none().0))
    }
}

pub fn commit_get_timestamp(commit_variant: &glib::Variant) -> u64 {
    unsafe {
        ffi::ostree_commit_get_timestamp(commit_variant.to_glib_none().0)
    }
}

pub fn content_file_parse<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(compressed: bool, content_path: &P, trusted: bool, cancellable: Q) -> Result<(gio::InputStream, gio::FileInfo, glib::Variant), Error> {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut out_file_info = ptr::null_mut();
        let mut out_xattrs = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_content_file_parse(compressed.to_glib(), content_path.to_glib_none().0, trusted.to_glib(), &mut out_input, &mut out_file_info, &mut out_xattrs, cancellable.0, &mut error);
        if error.is_null() { Ok((from_glib_full(out_input), from_glib_full(out_file_info), from_glib_full(out_xattrs))) } else { Err(from_glib_full(error)) }
    }
}

pub fn content_file_parse_at<'a, P: Into<Option<&'a gio::Cancellable>>>(compressed: bool, parent_dfd: i32, path: &str, trusted: bool, cancellable: P) -> Result<(gio::InputStream, gio::FileInfo, glib::Variant), Error> {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut out_file_info = ptr::null_mut();
        let mut out_xattrs = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_content_file_parse_at(compressed.to_glib(), parent_dfd, path.to_glib_none().0, trusted.to_glib(), &mut out_input, &mut out_file_info, &mut out_xattrs, cancellable.0, &mut error);
        if error.is_null() { Ok((from_glib_full(out_input), from_glib_full(out_file_info), from_glib_full(out_xattrs))) } else { Err(from_glib_full(error)) }
    }
}

pub fn content_stream_parse<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>>(compressed: bool, input: &P, input_length: u64, trusted: bool, cancellable: Q) -> Result<(gio::InputStream, gio::FileInfo, glib::Variant), Error> {
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut out_file_info = ptr::null_mut();
        let mut out_xattrs = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_content_stream_parse(compressed.to_glib(), input.to_glib_none().0, input_length, trusted.to_glib(), &mut out_input, &mut out_file_info, &mut out_xattrs, cancellable.0, &mut error);
        if error.is_null() { Ok((from_glib_full(out_input), from_glib_full(out_file_info), from_glib_full(out_xattrs))) } else { Err(from_glib_full(error)) }
    }
}

pub fn create_directory_metadata<'a, P: Into<Option<&'a glib::Variant>>>(dir_info: &gio::FileInfo, xattrs: P) -> Option<glib::Variant> {
    let xattrs = xattrs.into();
    let xattrs = xattrs.to_glib_none();
    unsafe {
        from_glib_full(ffi::ostree_create_directory_metadata(dir_info.to_glib_none().0, xattrs.0))
    }
}

//pub fn diff_dirs<'a, P: IsA<gio::File>, Q: IsA<gio::File>, R: Into<Option<&'a gio::Cancellable>>>(flags: /*Ignored*/DiffFlags, a: &P, b: &Q, modified: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 24 }, removed: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }, added: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }, cancellable: R) -> Result<(), Error> {
//    unsafe { TODO: call ffi::ostree_diff_dirs() }
//}

//pub fn diff_dirs_with_options<'a, 'b, P: IsA<gio::File>, Q: IsA<gio::File>, R: Into<Option<&'a /*Ignored*/DiffDirsOptions>>, S: Into<Option<&'b gio::Cancellable>>>(flags: /*Ignored*/DiffFlags, a: &P, b: &Q, modified: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 24 }, removed: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }, added: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }, options: R, cancellable: S) -> Result<(), Error> {
//    unsafe { TODO: call ffi::ostree_diff_dirs_with_options() }
//}

//pub fn diff_print<P: IsA<gio::File>, Q: IsA<gio::File>>(a: &P, b: &Q, modified: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 24 }, removed: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }, added: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 4, id: 15 }) {
//    unsafe { TODO: call ffi::ostree_diff_print() }
//}

//pub fn gpg_error_quark() -> /*Ignored*/glib::Quark {
//    unsafe { TODO: call ffi::ostree_gpg_error_quark() }
//}

//pub fn hash_object_name<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(a: P) -> u32 {
//    unsafe { TODO: call ffi::ostree_hash_object_name() }
//}

//pub fn metadata_variant_type(objtype: ObjectType) -> /*Ignored*/Option<glib::VariantType> {
//    unsafe { TODO: call ffi::ostree_metadata_variant_type() }
//}

pub fn object_from_string(str: &str) -> (String, ObjectType) {
    unsafe {
        let mut out_checksum = ptr::null_mut();
        let mut out_objtype = mem::uninitialized();
        ffi::ostree_object_from_string(str.to_glib_none().0, &mut out_checksum, &mut out_objtype);
        (from_glib_full(out_checksum), from_glib(out_objtype))
    }
}

pub fn object_name_deserialize(variant: &glib::Variant) -> (String, ObjectType) {
    unsafe {
        let mut out_checksum = ptr::null();
        let mut out_objtype = mem::uninitialized();
        ffi::ostree_object_name_deserialize(variant.to_glib_none().0, &mut out_checksum, &mut out_objtype);
        (from_glib_none(out_checksum), from_glib(out_objtype))
    }
}

pub fn object_name_serialize(checksum: &str, objtype: ObjectType) -> Option<glib::Variant> {
    unsafe {
        from_glib_none(ffi::ostree_object_name_serialize(checksum.to_glib_none().0, objtype.to_glib()))
    }
}

pub fn object_to_string(checksum: &str, objtype: ObjectType) -> Option<String> {
    unsafe {
        from_glib_full(ffi::ostree_object_to_string(checksum.to_glib_none().0, objtype.to_glib()))
    }
}

pub fn object_type_from_string(str: &str) -> ObjectType {
    unsafe {
        from_glib(ffi::ostree_object_type_from_string(str.to_glib_none().0))
    }
}

pub fn object_type_to_string(objtype: ObjectType) -> Option<String> {
    unsafe {
        from_glib_none(ffi::ostree_object_type_to_string(objtype.to_glib()))
    }
}

pub fn parse_refspec(refspec: &str) -> Result<(Option<String>, String), Error> {
    unsafe {
        let mut out_remote = ptr::null_mut();
        let mut out_ref = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_parse_refspec(refspec.to_glib_none().0, &mut out_remote, &mut out_ref, &mut error);
        if error.is_null() { Ok((from_glib_full(out_remote), from_glib_full(out_ref))) } else { Err(from_glib_full(error)) }
    }
}

pub fn raw_file_to_archive_z2_stream<'a, 'b, P: IsA<gio::InputStream>, Q: Into<Option<&'a glib::Variant>>, R: Into<Option<&'b gio::Cancellable>>>(input: &P, file_info: &gio::FileInfo, xattrs: Q, cancellable: R) -> Result<gio::InputStream, Error> {
    let xattrs = xattrs.into();
    let xattrs = xattrs.to_glib_none();
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_raw_file_to_archive_z2_stream(input.to_glib_none().0, file_info.to_glib_none().0, xattrs.0, &mut out_input, cancellable.0, &mut error);
        if error.is_null() { Ok(from_glib_full(out_input)) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v2017_3", feature = "dox"))]
pub fn raw_file_to_archive_z2_stream_with_options<'a, 'b, 'c, P: IsA<gio::InputStream>, Q: Into<Option<&'a glib::Variant>>, R: Into<Option<&'b glib::Variant>>, S: Into<Option<&'c gio::Cancellable>>>(input: &P, file_info: &gio::FileInfo, xattrs: Q, options: R, cancellable: S) -> Result<gio::InputStream, Error> {
    let xattrs = xattrs.into();
    let xattrs = xattrs.to_glib_none();
    let options = options.into();
    let options = options.to_glib_none();
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_raw_file_to_archive_z2_stream_with_options(input.to_glib_none().0, file_info.to_glib_none().0, xattrs.0, options.0, &mut out_input, cancellable.0, &mut error);
        if error.is_null() { Ok(from_glib_full(out_input)) } else { Err(from_glib_full(error)) }
    }
}

pub fn raw_file_to_content_stream<'a, 'b, P: IsA<gio::InputStream>, Q: Into<Option<&'a glib::Variant>>, R: Into<Option<&'b gio::Cancellable>>>(input: &P, file_info: &gio::FileInfo, xattrs: Q, cancellable: R) -> Result<(gio::InputStream, u64), Error> {
    let xattrs = xattrs.into();
    let xattrs = xattrs.to_glib_none();
    let cancellable = cancellable.into();
    let cancellable = cancellable.to_glib_none();
    unsafe {
        let mut out_input = ptr::null_mut();
        let mut out_length = mem::uninitialized();
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_raw_file_to_content_stream(input.to_glib_none().0, file_info.to_glib_none().0, xattrs.0, &mut out_input, &mut out_length, cancellable.0, &mut error);
        if error.is_null() { Ok((from_glib_full(out_input), out_length)) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_checksum_string(sha256: &str) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_checksum_string(sha256.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub fn validate_collection_id<'a, P: Into<Option<&'a str>>>(collection_id: P) -> Result<(), Error> {
    let collection_id = collection_id.into();
    let collection_id = collection_id.to_glib_none();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_collection_id(collection_id.0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v2017_8", feature = "dox"))]
pub fn validate_remote_name(remote_name: &str) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_remote_name(remote_name.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_rev(rev: &str) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_rev(rev.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_checksum_string(checksum: &str) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_checksum_string(checksum.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_commit(commit: &glib::Variant) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_commit(commit.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_csum_v(checksum: &glib::Variant) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_csum_v(checksum.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_dirmeta(dirmeta: &glib::Variant) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_dirmeta(dirmeta.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_dirtree(dirtree: &glib::Variant) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_dirtree(dirtree.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_file_mode(mode: u32) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_file_mode(mode, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

pub fn validate_structureof_objtype(objtype: u8) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::ostree_validate_structureof_objtype(objtype, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}
