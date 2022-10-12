import pandas as pd

if __name__ == '__main__':
    original = pd.read_csv('../data/iris.csv')
    boosted = pd.concat([original]*10000)
    boosted.to_csv("irisx10000.csv", index=False)