#!/bin/sh
#
# Last Modification: 2024-04-27 19:32:40
#

BINARY_NAME="termint"
SIZES="32x32 48x48 64x64 128x128"
ICONS_DIRECTORY="$HOME/.local/share/icons"

if [ ! -d $ICONS_DIRECTORY ]; then
    echo "The directory $ICONS_DIRECTORY does not exist."
    exit
fi

echo "The icon files will be copied to the following location: $HOME/.local/share/icons/hicolor/"
echo "Do you want to proceed? (yes/no)"
read CHOICE

if [ "$CHOICE" = "yes" ]; then
    echo "You chose yes. Proceeding..."

    for SIZE in $SIZES
    do
        cp icons/hicolor/$SIZE/apps/$BINARY_NAME.png $HOME/.local/share/icons/hicolor/$SIZE/apps/
    done

    echo "Update resources..."
    export XDG_DATA_DIRS="$HOME/.local/share:$XDG_DATA_DIRS"
    xdg-icon-resource forceupdate
    gtk-update-icon-cache -f $HOME/.local/share/icons/hicolor
    update-mime-database $HOME/.local/share/mime/

elif [ "$CHOICE" = "no" ]; then
    echo "Canceled!"
else
    echo "Invalid choice. Please choose either yes or no."
fi