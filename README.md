# icontool
Tool for working with BYOND DreamMaker Icon (.dmi) files

## Usage
Convert a DreamMaker Icon (dmi) file to YAML (yml) file:

    icontool decompile icon.dmi
    icontool decompile --output icon.dmi.yml icon.dmi

Convert a YAML (yml) file to a DreamMaker Icon (dmi) file:

    icontool compile icon.dmi.yml
    icontool compile --output icon.dmi icon.dmi.yml

Flatten metadata from a file for use in a YAML (yml) file:

    icontool flat icon.dmi.metadata

Output the metadata contained in a DreamMaker Icon (dmi) file:

    icontool metadata icon.dmi
    icontool metadata --output icon.dmi.metadata icon.dmi

## License
icontool  
Copyright 2024 Patrick Meade

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
