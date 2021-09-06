from os import path
import subprocess
import shutil
import os


def main():
    if path.exists("release"):
        shutil.rmtree("release")

    os.mkdir("release")
    os.mkdir("release/bin")
    os.mkdir("release/frontend")

    os.chdir("frontend")
    subprocess.run(["npm", "run", "build"])
    shutil.copytree("public", "../release/frontend/public")

    os.chdir("../backend")
    subprocess.run(["cargo", "build", "--release"])
    shutil.copyfile("target/release/backend", "../release/bin/oasis")
    shutil.copyfile("Rocket.toml", "../release/bin/Rocket.toml")
    shutil.copytree("migrations", "../release/bin/migrations")

    os.chdir("../release/bin")
    os.chmod("oasis", 0o755)


if __name__ == "__main__":
    main()
