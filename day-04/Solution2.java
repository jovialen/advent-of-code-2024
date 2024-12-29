import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Solution2 {
    private String data;
    private int width;

    public Solution2(String data, int width) {
        this.data = data;
        this.width = width;
    }

    public int xyToI(int x, int y) {
        return y * width + x;
    }

    public String getAt(int x, int y) {
        if (x < 0 || x >= width || y < 0 || y >= width) {
            return "";
        }
        return "" + data.charAt(xyToI(x, y));
    }

    public int solve() {
        int count = 0;
        
        int nextStart = data.indexOf("A");
        while (nextStart >= 0) {
            int x = nextStart % width;
            int y = nextStart / width;

            String a = getAt(x - 1, y - 1) + getAt(x + 1, y + 1);
            String b = getAt(x - 1, y + 1) + getAt(x + 1, y - 1);

            if ((a.equals("MS") || a.equals("SM")) && (b.equals("MS") || b.equals("SM"))) {
                count += 1;
            }

            data = data.replaceFirst("A", ".");
            nextStart = data.indexOf("A");
        }

        return count;
    }

    public static void main(String[] args) throws IOException {
        if (args.length != 1) {
            System.out.println("Usage: java Solution2.java <input file>");
            return;
        }

        String inputFilePath = args[0];
        String inputFile = Files.readString(Path.of(inputFilePath));
        int width = inputFile.indexOf("\n");
        inputFile = inputFile.replace("\n", "");

        Solution2 solution = new Solution2(inputFile, width);
        System.out.println(solution.solve());
    }
}