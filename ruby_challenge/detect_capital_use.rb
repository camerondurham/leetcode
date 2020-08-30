# @param {String} word
# @return {Boolean}
def detect_capital_use(word)
    up = true
    lo = true
    cap = true
    count = 0
    word.split('').each do |c|
        upper = c == c.upcase
        lower = c != c.upcase
        up = upper && up
        lo = lower && lo
        cap = cap && ((count == 0 && upper) || (count > 0 && lower))
        count += 1
    end
    return up || lo || cap
end
