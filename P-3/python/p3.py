import unittest

def swap_beginning_end(l):
    print("sport", end="")
    beginning = l[:int(len(l)/2)]
    end = l[int(len(l)/2):]
    return end + beginning

def value_to_index(l):
    print("hawk", end=" ")
    return [l[i] for i in l]

def swap_first_last(l):
    print("glass", end=" ")
    first = l.pop(0)
    last = l.pop()
    return [last] + l + [first]

def do_si_do(l):
    print("snap", end=" ")
    mixed = []
    it = iter(l)
    for e in it:
        nxt = next(it, None)
        if nxt is not None:
            mixed.append(nxt)
        mixed.append(e)
    return mixed

def rotate(l):
    print("gossip", end=" ")
    last = l.pop()
    return [last] + l

def shuffle(l):
    print("alter", end=" ")
    mixed = []
    beginning = l[:int(len(l)/2)]
    end = l[int(len(l)/2):]
    for (i, e) in enumerate(end):
        mixed.append(e)
        if i < len(beginning):
            mixed.append(beginning[i])
    return mixed

def substitution_table(l):
    print("avocado", end=" ")
    return [l[4], l[0], l[2], l[3], l[1], l[5]]

def substitution_table2(l):
    print("alcohol", end=" ")
    return [l[5], l[1], l[2], l[0], l[4], l[3]]

def twin_propeller(l):
    print("dolphin", end=" ")
    beginning = l[:int(len(l)/2)]
    end = l[int(len(l)/2):]
    beginning.reverse()
    end.reverse()
    return beginning + end

def fold_shuffle(l):
    print("material", end=" ")
    mixed = []
    beginning = l[:int(len(l)/2)]
    end = l[int(len(l)/2):]
    beginning.reverse()
    end.reverse()
    for (i, e) in enumerate(end):
        mixed.append(e)
        if i < len(beginning):
            mixed.append(beginning[i])
    return mixed

def propeller_shuffle(l):
    print("scene", end=" ")
    mixed = []
    beginning = l[:int(len(l)/2)]
    beginning.reverse()
    end = l[int(len(l)/2):]
    for (i, e) in enumerate(end):
        mixed.append(e)
        if i < len(beginning):
            mixed.append(beginning[i])
    return mixed

def reverse(l):
    print("floor", end=" ")
    l.reverse()
    return l


def run_first_6(l):
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    return l

def run_second_6(l):
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    l = xxxxxxxx(l)
    return l

class Tests(unittest.TestCase):
    def test_a(self):
        self.assertEqual(run_first_6([0, 1, 2, 3, 4, 5]), [3, 0, 1, 4, 5, 2])

    # clw answerwallet --passphrase "X Y Z"
    # address 131ykwoBx3uPiWwUEWw63KYLLjX93d3L1E
    def test_b(self):
        self.assertEqual(run_second_6(run_first_6([0, 1, 2, 3, 4, 5])), [5, 4, 3, 2, 1, 0])

unittest.main()
