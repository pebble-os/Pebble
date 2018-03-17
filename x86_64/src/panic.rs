/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt     : ::core::fmt::Arguments,
                        file    : &'static str,
                        line    : u32) -> !
{
    error!("PANIC in {} at line {}: \n    {}", file, line, fmt);
    loop {}
}