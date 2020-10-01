from setuptools import setup, find_packages

setup(
    author_email="foobar@example.com",
    author="Matthew Planchard",
    classifiers=[
        "Intended Audience :: Developers",
        'License :: OSI Approved :: MIT License',
        "Operating System :: POSIX :: Linux",
        "Operating System :: MacOS :: MacOS X",
        "Operating System :: Microsoft :: Windows",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3 :: Only",
    ],
    description="A simple test package",
    entry_points={},
    extras_require={},
    keywords=["foo", "bar"],
    long_description="A really simple test package",
    name="foo_pkg",
    package_data={},
    packages=find_packages(),
    python_requires=">=3.5",
    setup_requires=[],
    tests_require=[],
    url="https://github.com/mplanchard/serval",
    version="1.0.0",
)
