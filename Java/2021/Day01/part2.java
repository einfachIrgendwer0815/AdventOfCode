import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.List;
import java.util.ArrayList;

class Part2 {
  public static void main(String[] args) throws FileNotFoundException {
    List<Integer> data = readFile("input.txt");

    ArrayList<ArrayList<Integer>> activeGroups = new ArrayList<ArrayList<Integer>>();

    List<Integer> sums = new ArrayList<Integer>();

    for (int i = 0; i < data.size(); i++) {
      if (activeGroups.size() < 3) {
        activeGroups.add(new ArrayList<Integer>());
      }

      for (ArrayList<Integer> group : activeGroups) {
        group.add(data.get(i));
      }

      ArrayList<ArrayList<Integer>> groupsCopy = new ArrayList<ArrayList<Integer>>(activeGroups);
      for (int j = 0; j < activeGroups.size(); j++) {
        if (activeGroups.get(j).size() >= 3) {
          ArrayList<Integer> group = activeGroups.get(j);
          int sum = group.get(0) + group.get(1) + group.get(2);

          sums.add(sum);

          groupsCopy.remove(groupsCopy.indexOf(group));
        }
      }

      activeGroups = groupsCopy;
    }


    int increases = 0;

    for (int i = 1; i < sums.size(); i++) {
      if (sums.get(i) > sums.get(i-1)) {
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
