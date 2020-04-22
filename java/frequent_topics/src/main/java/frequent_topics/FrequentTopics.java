package frequent_topics;

import java.util.*;
import java.util.stream.Collectors;

import static java.util.Collections.emptyList;
import static java.util.Collections.frequency;

public class FrequentTopics {
    public static List<String> frequentTopics(List<String> topics, List<String> phrases, int takeMostCommon) {
        if (takeMostCommon < 1 || topics.size() < 1 || phrases.size() < 1) {
            return emptyList();
        }

        List<String> uniqueWordFromPhrases = phrases.stream()
                .map(s -> Arrays.asList(s.split(" ")).stream().collect(Collectors.toSet()))
                .flatMap(Set::stream)
                .collect(Collectors.toList());


        List<Pair> frequentPairs = topics.stream().map(f ->
                new Pair(frequency(uniqueWordFromPhrases, f), f)
        )
                .filter(p -> p.getFrequency()> 0)
                .collect(Collectors.toList());

        Collections.sort(frequentPairs, Comparator.comparing(Pair::getFrequency).reversed()
                .thenComparing(Pair::getWord));

        int topMostCommonPossible = takeMostCommon > frequentPairs.size() ? frequentPairs.size() : takeMostCommon;

        return frequentPairs.subList(0, topMostCommonPossible).stream().map(w -> w.getWord()).collect(Collectors.toList());
    }
}
