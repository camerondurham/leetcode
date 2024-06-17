class Solution {
    public int carFleet(int target, int[] position, int[] speed) {
        int n = position.length;
        double[][] pairs = new double[n][2];
        for (int i = 0; i < n; i++) {
            pairs[i][0] = position[i];
            pairs[i][1] = speed[i];
        }
        Arrays.sort(pairs, (a, b) -> Double.compare(b[0], a[0]));
        int fleetCount = 0;
        double[] timeToReach = new double[n];
        for (int i = 0; i < n; i++) {
            timeToReach[i] = (target - pairs[i][0] /* distance to target*/) / pairs[i][1];
            if (i >= 1 && timeToReach[i] <= timeToReach[i-1]) {
                timeToReach[i] = timeToReach[i - 1];
            } else {
                fleetCount++;
            }
        }
        return fleetCount;
    } 
}

