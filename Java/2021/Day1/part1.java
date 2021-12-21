import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.List;
import java.util.ArrayList;

class Part1 {
  public static void main(String[] args) throws FileNotFoundException {
    List<Integer> data = readFile("input.txt");

    int increases = 0;
    for (int i = 1; i < data.size(); i++) {
      if (data.get(i) > data.get(i-1)) {
        increases++;
      }
    }

    System.out.println(increases);
  }

  private static List<Integer> readFile(String path) throws FileNotFoundException {
    File myFile = new File(path);
    Scanner fileScanner = new Scanner(myFile);

    List<Integer> data = new ArrayList<Integer>();

    while (fileScanner.hasNextLine()) {
      String line = fileScanner.nextLine().trim();
      data.add(Integer.parseInt(line));
    }

    return data;
  }
}
