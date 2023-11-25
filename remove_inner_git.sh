#!/bin/bash

source ~/.scripts/colours.sh

for it in ./*; do
	if [ -d $it ]; then
		cd $it
		if [ -f ".git" ] || [ -d ".git" ]; then
			echo -e "${Green} Removing .git for $it ${NC}"
			rm -rf .git
		fi
		cd ../
	fi
done
