
def get_percent(num, den):
    float_percent = 100.0 * float(num) / float(den)
    return "{0:0.1f}".format(float_percent) + "%"
