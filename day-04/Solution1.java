import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Solution1 {
    private String data;
    private int width;

    public Solution1(String data, int width) {
        this.data = data;
        this.width = width;
    }

    public int xyToI(int x, int y) {
        return y * width + x;
    }

    public int solve() {
        int count = 0;
        
        int nextStart = data.indexOf("X");
        while (nextStart >= 0) {
            int offsetX[] = { -1, -1, 0, 1, 1, 1, 0, -1 };
            int offsetY[] = { 0, -1, -1, -1, 0, 1, 1, 1 };

            for (int directionIndex = 0; directionIndex < 8; directionIndex++) {
                int x = nextStart % width;
                int y = nextStart / width;

                String s = "X";
                for (int k = 0; k < 3; k++) {
                    x += offsetX[directionIndex];
                    y += offsetY[directionIndex];

                    if (x < 0 || y < 0 || x >= width || y >= width) {
                        break;
                    }

                    s += data.charAt(xyToI(x, y));
                }

                if (s.equals("XMAS")) {
                    count += 1;
                }
            }

            data = data.replaceFirst("X", ".");
            nextStart = data.indexOf("X");
        }

        return count;
    }

    public static void main(String[] args) throws IOException {
        if (args.length != 1) {
            System.out.println("Usage: java Solution1.java <input file>");
            return;
        }

        String inputFilePath = args[0];
        String inputFile = Files.readString(Path.of(inputFilePath));
        int width = inputFile.indexOf("\n");
        inputFile = inputFile.replace("\n", "");

        Solution1 solution = new Solution1(inputFile, width);
        System.out.println(solution.solve());
    }
}