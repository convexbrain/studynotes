
#echo $*
#echo ${@:2}

p=`echo $1 | sed 's|/*$||'`
#echo $p

mkdir $p
(
    cd $p
    for i in ${@:2}
    do
        pp=`echo "$p"_"$i"`
        #echo $pp
        cargo new $pp && cp -v ../template/src/main.rs $pp/src/
    done
)

