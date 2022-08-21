/*
Runtime: 1 ms, faster than 90.78% of Java online submissions for Flood Fill.
Memory Usage: 48.4 MB, less than 23.29% of Java online submissions for Flood Fill.
*/
class Solution {
    public int[][] floodFill(int[][] image, int sr, int sc, int color) {
        int startColor = image[sr][sc];
        helper(image, sr, sc, startColor, color);
        return image;
    }
    private void helper(int[][] image, int r, int c, int startColor, int targetColor) {
        if (image[r][c] != startColor || image[r][c] == targetColor) {
            return;
        } else {
            image[r][c] = targetColor;
            if (r+1 < image.length && image[r+1][c] == startColor) {
                helper(image, r+1, c, startColor, targetColor);
            }
            if (r-1 >= 0 && image[r-1][c] == startColor) {
                helper(image, r-1, c, startColor, targetColor);
            }
            if (c+1 < image[0].length && image[r][c+1] == startColor) {
                helper(image, r, c+1, startColor, targetColor);
            }
            if (c-1 >= 0 && image[r][c-1] == startColor) {
                helper(image, r, c-1, startColor, targetColor);
            }
            return;
        }
    }
}
