from os import path
import subprocess
import shutil
import os


def main():
    if path.exists("release"):
        shutil.rmtree("release")

    os.mkdir("release")
    filename = "oasis.exe" if os.name == "nt" else "oasis"
    shellEnable = True if os.name == "nt" else False

    os.chdir("frontend")
    subprocess.run(["npm", "i"], shell=shellEnable)
    subprocess.run(["npm", "run", "build"], shell=shellEnable)
    shutil.copytree("public", "../release/public")

    os.chdir("../backend")
    subprocess.run(["cargo", "build", "--release"], shell=shellEnable)
    shutil.copyfile("target/release/" + filename, "../release/" + filename)
    shutil.copyfile(".env-release", "../release/.env")

    os.chdir("../release")
    os.chmod(filename, 0o755)


if __name__ == "__main__":
    main()
