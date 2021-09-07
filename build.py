from os import path
import subprocess
import shutil
import os


def main():
    if path.exists("release"):
        shutil.rmtree("release")

    os.mkdir("release")
    filename = "oasis.exe" if os.name == "nt" else "oasis"
    
    os.chdir("frontend")
    subprocess.run(["npm", "run", "build"], shell=True)
    shutil.copytree("public", "../release/public")

    os.chdir("../backend")
    subprocess.run(["cargo", "build", "--release"], shell=True)
    shutil.copyfile("target/release/" + filename, "../release/" + filename)
    shutil.copytree("migrations", "../release/migrations")
    shutil.copyfile(".env-release", "../release/.env")

    os.chdir("../release")
    os.chmod(filename, 0o755)


if __name__ == "__main__":
    main()
