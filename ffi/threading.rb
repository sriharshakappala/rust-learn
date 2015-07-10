threads = []

start_time = Time.now
10.times do
  threads << Thread.new do
    count = 0
    50000000.times do
      count += 1
    end
  end
end

threads.each {|t| t.join }

end_time = Time.now
puts "Done!"

puts start_time
puts end_time
