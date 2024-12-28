def maxScoreSightSeeingPair(values: List[int]) -> int:
    max_score = float("-inf")

    best_i_value = values[0] + 0
    for j in range(1, len(values)):
        current_score = best_i_value + values[j] - j
        max_score = max(max_score, current_score)
        best_i_value = max(best_i_value, values[j] + j)

    return max_score
