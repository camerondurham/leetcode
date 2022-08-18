import java.util.HashMap;
import java.util.Map;

class Scratch {
    public int longestPalindrome(String s) {
        Map<Character,Integer> m = new HashMap<>();
        int even = 0;
        int odd = 0;
        for(char c : s.toCharArray()) {
            int prev = m.getOrDefault(c,0);
            m.put(c,prev+1);
        }
        for(Map.Entry<Character,Integer> e : m.entrySet()) {
            if (e.getValue() % 2 == 1) {
                odd = e.getValue() % 2;
                even += (e.getValue() - 1);
            } else {
                even += e.getValue();
            }

        }
        return even + odd;
    }
    public static void main(String[] args) {
        Scratch s = new Scratch();
        System.out.println(s.longestPalindrome("abccccdd"));
    }
}
