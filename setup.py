# setup.py
from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="synthdb",
    version="0.1.0",
    author="Abhinav Raj",
    author_email="your.email@example.com",
    description="Generate production-quality synthetic databases with one command",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/abhinavraj2004/synthdb",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
    ],
    python_requires=">=3.8",
    install_requires=[
        "sqlalchemy>=2.0.0",
        "psycopg2-binary>=2.9.0",
        "pymysql>=1.1.0",
        "faker>=20.0.0",
        "click>=8.1.0",
        "rich>=13.0.0",
        "tqdm>=4.66.0",
        "pydantic>=2.0.0",
        "python-dotenv>=1.0.0",
    ],
    entry_points={
        "console_scripts": [
            "synthdb=synthdb.cli:cli",
        ],
    },
)