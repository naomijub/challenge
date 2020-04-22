package frequent_topics;

public class Pair {
    private int frequency;
    private String word;

    public Pair(int frequency, String word) {
        this.frequency = frequency;
        this.word = word;
    }

    public int getFrequency() {
        return frequency;
    }

    public String getWord() {
        return word;
    }

}