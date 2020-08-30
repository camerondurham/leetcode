# @param {Integer} x
# @return {Integer}
def my_sqrt(x)
    lo = 0
    hi = x + 1
    mid = lo

    while lo < hi
      mid = lo + (hi - mid) / 2
      if mid * mid > x
        hi = mid
      else
        lo = mid + 1
      end
    end
    lo - 1
end

puts "Sqrt(#{8}) = #{my_sqrt(8)}"
puts "Sqrt(#{9}) = #{my_sqrt(9)}"
puts "Sqrt(#{100}) = #{my_sqrt(100)}"
