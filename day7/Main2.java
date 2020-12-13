import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;

public class Main2 {
    static HashMap<String, ArrayList<Bag> > bags = new HashMap<>();
    static HashMap<String, Boolean> expanded = new HashMap<>();

    public static void main(String[] args) throws IOException {
        BufferedReader rd = new BufferedReader(new InputStreamReader(System.in));
        String buff;

        while ((buff = rd.readLine()) != null) {
            String[] inputs = buff.split("contain");
            String[] keyRaw = inputs[0].split(" ");
            String[] valueRaw = inputs[1].split(",");

            String key = keyRaw[0] + " " + keyRaw[1];
            ArrayList<Bag> list = new ArrayList<>();

            for (int i = 0; i < valueRaw.length; i++) {
                String[] value = valueRaw[i].trim().split(" ");

                if (value[0].compareTo("no") == 0) {
                    break;
                }

                int sum = Integer.parseInt(value[0]);
                String id = value[1] + " " + value[2];

                list.add(new Bag(id, sum));
            }

            bags.put(key, list);
            expanded.put(key, false);
        }

        int sum = 0;

        ArrayList<Bag> goldBag = expand("shiny gold");

        for (Bag bag: goldBag) {
            sum += bag.amount;
        }

        System.out.println(sum);
    }

    public static ArrayList<Bag> expand(String key) {
        if (!expanded.get(key)) {
            ArrayList<Bag> res = new ArrayList<>();
            ArrayList<Bag> seed = bags.get(key);

            res.addAll(0, seed);

            for (int i = 0; i < seed.size(); i++) {
                int amount = seed.get(i).amount;

                ArrayList<Bag> insides = expand(seed.get(i).name);

                for (int j = 0; j < insides.size(); j++) {
                    Bag current = insides.get(j);

                    res.add(new Bag(current.name, current.amount * amount));
                }
            }

            bags.put(key, res);
            expanded.put(key, true);
        }

        return bags.get(key);
    }
}

class Bag {
    String name;
    int amount;

    public Bag(String name, int amount) {
        this.name = name;
        this.amount = amount;
    }
}
