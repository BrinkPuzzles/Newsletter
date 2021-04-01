from enum import Enum
import pprint
pp = pprint.PrettyPrinter(indent=4)

class FaveColor(Enum):
    Blue = 1
    Red = 2
    Green = 3
    Orange = 4

class Side(Enum):
    Top = 1
    Right = 2
    Left = 3
    Bottom = 4


class Profile:
    def __init__(self, fave_color, num, side):
        self.fave_color = fave_color
        self.num = num
        self.side = side
    def __str__(self):
        return "Profile { fave_color: %s, num: %s, side: %s }" % (self.fave_color, self.num, self.side)

class Answer:
    def __init__(self, fave_color, num, side):
        self.fave_color = fave_color
        self.num = num
        self.side = side
    def __str__(self):
        return "Answer { fave_color: %s, num: %s, side: %s }" % (self.fave_color, self.num, self.side)

# In order for a table of caller-to-profile matches to be valid,
# at least one caller has to match every profile.
def is_valid(match_table):
    for answer in range(0, len(match_table[0])):
        all_false = True
        for profile in range(0, len(match_table)):
            if match_table[profile][answer]:
                all_false = False
        if all_false:
            return False
    return True

# Take some profiles and some callers and return which callers
# could match the profiles.
def match_table(profiles, answers):
    patterns = []

    for a in answers:
        pattern = []
        for p in profiles:
            pattern.append(could_match(p, a))
        patterns.append(pattern)
    return patterns

# A caller could match a profile if all answered questions match.
# Unanswered questions are ignored.
def could_match(profile, answer):
    return (answer.fave_color is None or profile.fave_color == answer.fave_color) and \
    (answer.num is None or profile.num == answer.num) and \
    (answer.side is None or profile.side == answer.side)


# There are four profiles and four callers...
profiles = []

profiles.append(Profile(FaveColor.Blue, 3, Side.Top))
profiles.append(Profile(FaveColor.Orange, 2, Side.Left))
profiles.append(Profile(FaveColor.Orange, 1, Side.Top))
profiles.append(Profile(FaveColor.Red, 3, Side.Right))

answers = []

answers.append(Answer(FaveColor.Red, None, Side.Right))
answers.append(Answer(FaveColor.Orange, 2, None))
answers.append(Answer(None, 1, Side.Top))
answers.append(Answer(None, 3, None))

# TODO: How do I iterate over the callers, assuming each caller is the hacker one at a time...
# I can make a truth table that says whether each caller could match each profile. Then I can
# test to see if the table makes sense with the is_valid function. is_valid will check to see
# if there's one caller per profile. If assuming one of the callers is lying yields a valid
# truth table, they could be the hacker. If only one assumptions makes sense, then only one
# profile could be the hacker...
#
# match_table makes truth tables and is_valid validates them.

