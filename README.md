# LasGrep (lg)

lg is a command line tool to filter asprs's las and laz files thanks to their header.

The main use of this tool is to be combined with other command line tools (cp, mv, rm, wc, etc.)

Type ```lg --help``` for the help.

## Some examples

Get tiles with type 1 points :

```shell
lg --point-format 1 ./las_folder/
```

Get tile with less 100 000 points :
```shell
lg --point-number "<100000" ./las_folder/
```

Get laz files that intersects a wkt :
```shell
# The CRS of wkt must be the same of the tile
lg --extensions=laz --wkt="POINT(10 10)" --intersects ./las_folder/
```

Copy the files selected in a folder :

```shell
# Linux version
lg --point-format 1 ./las_folder/ | xargs cp -t /destination/folder/
```

```shell
# Windows version (Power shell)
lg .\las_folder | ForEach {cp $_.Replace("'","")  \destination\folder\}
```