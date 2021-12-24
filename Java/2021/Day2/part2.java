import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

class Part2 {
  public static void main(String[] args) throws FileNotFoundException {
    ArrayList<String[]> data = readFile("input.txt");

    int horizontal = 0;
    int depth = 0;
    int aim = 0;

    for (String[] action : data) {
      String command = action[0];
      int steps = Integer.parseInt(action[1]);

      switch (command) {
        case "forward":
          horizontal += steps;
          depth += aim * steps;
          break;

        case "down":
          aim += steps;
          break;

        case "up":
        aim -= steps;
          break;
      }
    }

    System.out.println(horizontal*depth);
  }

  private static ArrayList<String[]> readFile(String path) throws FileNotFoundException {
    File myFile = new File(path);
    Scanner fileScanner = new Scanner(myFile);

    ArrayList<String[]> data = new ArrayList<String[]>();

    while (fileScanner.hasNextLine()) {
      String[] line = fileScanner.nextLine().trim().split(" ");

      data.add(line);
    }

    return data;
  }
}
