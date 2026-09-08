# Synthetic rule fixture, not a real-project showcase.
def choose(enabled, valid):
    if not enabled:
        return "disabled"
    else:
        if valid:
            if enabled:
                return "ready"
    return "invalid"


if __name__ == "__main__":
    for enabled in (False, True):
        for valid in (False, True):
            print(enabled, valid, choose(enabled, valid))
