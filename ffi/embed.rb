require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib './embed/target/release/libembed.so'
  attach_function :process, [], :void
end

start_time = Time.now
Hello.process
end_time = Time.now

puts start_time
puts end_time
