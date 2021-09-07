from os import path
import subprocess
import shutil
import os


def main():
    if path.exists("release"):
        shutil.rmtree("release")

    os.mkdir("release")

    os.chdir("frontend")
    subprocess.run(["npm", "run", "build"])
    shutil.copytree("public", "../release/public")

    os.chdir("../backend")
    subprocess.run(["cargo", "build", "--release"])
    shutil.copyfile("target/release/backend", "../release/oasis")
    shutil.copytree("migrations", "../release/migrations")
    shutil.copyfile(".env-release", "../release/.env")

    os.chdir("../release")
    os.chmod("oasis", 0o755)


if __name__ == "__main__":
    main()
