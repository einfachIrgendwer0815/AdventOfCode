import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

class Part1 {
  public static void main(String[] args) throws FileNotFoundException {
    ArrayList<String> data = readFile("input.txt");

    String mostCommonBits = "";
    String leastCommonBits = "";

    if (data.size() == 0) {
      return;
    }

    int[] ones = new int[data.get(0).length()];
    int[] zeros = new int[data.get(0).length()];

    for (int i = 0; i < data.get(0).length(); i++) {
      for (int j = 0; j < data.size(); j++) {
        if (data.get(j).charAt(i) == '1') {
          ones[i]++;
        } else {
          zeros[i]++;
        }
      }
    }

    for (int i = 0; i < data.get(0).length(); i++) {
      if (ones[i] > zeros[i]) {
        mostCommonBits += '1';
        leastCommonBits += '0';
      } else {
        mostCommonBits += '0';
        leastCommonBits += '1';
      }
    }

    int gammaRate = Integer.parseInt(mostCommonBits, 2);
    int epsilonRate = Integer.parseInt(leastCommonBits, 2);

    System.out.println(gammaRate*epsilonRate);
  }

  private static ArrayList<String> readFile(String path) throws FileNotFoundException {
    File myFile = new File(path);
    Scanner fileScanner = new Scanner(myFile);

    ArrayList<String> data = new ArrayList<String>();

    while (fileScanner.hasNextLine()) {
      String line = fileScanner.nextLine().trim();

      data.add(line);
    }

    return data;
  }
}
