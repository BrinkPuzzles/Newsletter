
# Menu of soaps. first[i] goes with second[i] for all indices i. Example: "Carnival Calliope".
firsts = ["Dazzling", "Chickadee's", "Pumpkin", "Daily", "Deepest", "Carnival"]
seconds = ["Daylight", "Song", "Spice", "Lailac", "Purple", "Calliope"]

# List of possible soaps.
possible_soaps = []


def digest(s):
    s_lower = s.lower()
    sv = [char for char in s_lower]
    hash_code = 0

    for c in sv:
        # make letter a = code 0
        hash_code += ord(c) - ord('a')

    return hash_code


for f in firsts:
    for s in seconds:
        possible_soaps.append(((digest(f), digest(s)), f + " " + s))

# Each soap has a code: "(D1, D2)". Where D1 and D2 are integers given by
# the digest function below.

# Initialize a container to hold the digests.
first_digests = [0] * len(firsts)
second_digests = [0] * len(seconds)

for i in range(len(firsts)):
    first_digests[i] = digest(firsts[i])

# Remove extra elements (zeros).
first_digests = [x for x in first_digests if x != 0]

# Repeat for second words.
for i in range(len(seconds)):
    second_digests[i] = digest(seconds[i])

second_digests = [x for x in second_digests if x != 0]

# Loop through the digests and match them up with soap names within possible soaps.
for i, d in enumerate(first_digests):
    matches = [x for x in possible_soaps if x[0] == (first_digests[i], second_digests[i])]
    if len(matches) > 0:
        print(matches[0][1])
