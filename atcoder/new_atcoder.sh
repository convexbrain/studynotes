
#echo $*
#echo ${@:2}

mkdir $1
(
    cd $1
    for i in ${@:2}
    do
        #echo $1_$i
        cargo new $1_$i && cp -v ../template/src/main.rs $1_$i/src/
    done
)

