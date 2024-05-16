class Solution(object):
    def strongPasswordChecker(self, password):
        n = len(password)
        has_lower = any(c.islower() for c in password)
        has_upper = any(c.isupper() for c in password)
        has_digit = any(c.isdigit() for c in password)
        
        required_types = 3 - sum([has_lower, has_upper, has_digit])  # Each missing type adds a required change

        # Count sequences of three or more repeated characters
        repeats = []
        i = 0
        while i < n:
            j = i
            while j < n and password[i] == password[j]:
                j += 1
            if j - i >= 3:
                repeats.append(j - i)
            i = j
        
        deletes = max(0, n - 20)
        min_changes = 0
        i = 0

        # We prioritize removing characters from larger groups first
        while i < len(repeats) and deletes > 0:
            while repeats[i] >= 3 and deletes > 0:
                repeats[i] -= 1
                deletes -= 1
                if repeats[i] < 3:
                    break

        # After deletions, we check how many replacements are needed
        for length in repeats:
            if length >= 3:
                min_changes += length // 3

        return max(required_types, min_changes, deletes)