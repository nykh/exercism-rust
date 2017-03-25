import os, sys
import argparse

parser = argparse.ArgumentParser(description='Remove #[ignore] in test file')
parser.add_argument('test')
args = parser.parse_args()

if not os.path.exists(args.test):
    print("The file does not exists!")
    sys.exit(1)

with open(args.test, 'r') as testfile:
    text = testfile.readlines()

new_text = filter(lambda line: not line.startswith('#[ignore'), text)

with open(args.test, 'w') as testfile:
    testfile.writelines(new_text)
