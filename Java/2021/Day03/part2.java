import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

class Part2 {
  public static void main(String[] args) throws FileNotFoundException {
    ArrayList<String> data = readFile("input.txt");

    if (data.size() == 0) {
      return;
    }

    int oxygen = extractData(data, "oxygen");
    int co2 = extractData(data, "co2");

    System.out.println(oxygen*co2);
  }

  private static int extractData(ArrayList<String> data, String type) {
    for(int i = 0; i < data.get(0).length(); i++) {
      int ones = 0;
      int zeros = 0;

      for (int j = 0; j < data.size(); j++) {
        if(data.get(j).charAt(i) == '1') {
          ones++;
        } else {
          zeros++;
        }
      }

      char toKeep;

      if((type == "oxygen" && (ones >= zeros)) || (type == "co2" && (ones < zeros))) {
        toKeep = '1';
      } else {
        toKeep = '0';
      }

      ArrayList<String> dataCopy = new ArrayList<String>();

      for (int j = 0; j < data.size(); j++) {
        if(data.get(j).charAt(i) == toKeep) {
          dataCopy.add(data.get(j));
        }
      }

      data = dataCopy;

      if (data.size() == 1) {
        break;
      }
    }

    return Integer.parseInt(data.get(0), 2);
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
