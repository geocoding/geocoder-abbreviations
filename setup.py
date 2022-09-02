import setuptools

setuptools.setup(
    name="geocoder_abbreviations_py",
    version="4.6.16",
    description="Language/Country Specific Street Abbreviations",
    url="https://github.com/mapbox/geocoder-abbreviations",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: Other/Proprietary License",
        "Operating System :: OS Independent",
    ],
    package_data={"tokens": ["*.json"]},
    license='MIT'
)