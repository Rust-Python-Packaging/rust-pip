
# POSSIBLE SETUPS

## There are mainly 3 setups in typical python projects

1) **setup.py**
  setup.py is a python file, the presence of which is an indication that the module/package you are about to install has likely been packaged and distributed with Distutils, which is the standard for distributing Python Modules.
2) **pyproject.toml**
  pyproject.toml is a standard replacement for setup.py according to <a href="https://peps.python.org/pep-0518/">PEP 518</a>.
3) **setup.cfg**
  it provides some basic configuration details inspite of setup details, like versionfile_source, Version Control System like git.