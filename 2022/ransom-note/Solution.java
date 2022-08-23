class Solution {
    public boolean canConstruct(String ransomNote, String magazine) {
        Map<Character,Integer> countMap = new HashMap();
        for(char c : magazine.toCharArray()) {
            int v = countMap.getOrDefault(c, 0);
            countMap.put(c, v+1);
        }
        for(char c : ransomNote.toCharArray()) {
            if (!countMap.containsKey(c)) {
                return false;
            }
            int v = countMap.get(c);
            if (v-1 < 0) {
                return false;
            } else {
                countMap.put(c,v-1);
            }
        }
        return true;
    }
}
