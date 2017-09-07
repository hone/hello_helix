require "helix_runtime"

begin
  require "hello_helix/native"
rescue LoadError
  warn "Unable to load hello_helix/native. Please run `rake build`"
end
