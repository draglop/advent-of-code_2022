// https://adventofcode.com/2022/day/7
// (part 1)

fn number_parse(number: &str) -> (usize, usize) {
    let mut digit_count: usize = 0;
    for c in number.chars() {
        if c < '0' || c > '9' {
            break;
        }
        digit_count += 1;
    }
    let n = number
        .get(0..digit_count)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    return (n, digit_count);
}

fn directory_size(max_size: usize, summed_size: &mut usize, lines: &mut Vec<&str>) -> usize {
    let mut this_size: usize = 0;

    let mut going_up: bool = false;
    while !going_up && !(lines.len() == 0) {
        let entry: &str = lines.pop().unwrap();
        if entry == "$ cd .." {
            going_up = true;
        } else if entry == "$ ls" {
            // do nothing, we don't care about listing commands
        } else if entry.starts_with("$ cd ") {
            // sub dir, add its size to this dir size
            this_size += directory_size(max_size, summed_size, lines);
        } else {
            // listing entry, get its size if its a file
            assert!(!entry.starts_with("$")); // make sure it's not a command
            if !entry.starts_with("dir ") {
                // don't care about directory entries we need a 'cd' command to know its size
                let (entry_size, digit_count) = number_parse(entry);
                assert!(digit_count > 0);
                this_size += entry_size;
            }
        }
    }

    // add if it matches the requirement
    if this_size <= max_size {
        *summed_size += this_size;
    }

    return this_size;
}

fn sum_directories(max_size: usize, terminal: &str) -> usize {
    let mut summed_size: usize = 0;

    let mut terminal_lines: Vec<&str> = terminal.split('\n').collect();
    terminal_lines.reverse(); // so we can pop at the end

    // cd can only move one level up or down, we don't need to keep track of paths
    // passing 'summed_size' as argument instead of keeping track of sizes in a container
    directory_size(max_size, &mut summed_size, &mut terminal_lines);

    return summed_size;
}

fn main() {
    // example
    assert_eq!(95437, sum_directories(100000, "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"));
    assert_eq!(1077191, sum_directories(100000, "$ cd /\n$ ls\ndir cmjgvh\ndir czrzl\ndir fcbt\ndir hdh\n259661 hjsbd.mzp\ndir jgrdd\ndir lqblqtng\ndir pgvmpmn\ndir pqqcvcm\ndir zglbptq\n$ cd cmjgvh\n$ ls\ndir hdh\n134565 hdh.sjv\ndir hgrpfmt\n282147 mjtq.ffd\n42343 rvmzv.rtb\ndir sjgvbd\n31468 wgtjmb.thf\n$ cd hdh\n$ ls\n267125 htplc.gdw\n$ cd ..\n$ cd hgrpfmt\n$ ls\n39132 lndwz\n280595 rffmsvdw\n$ cd ..\n$ cd sjgvbd\n$ ls\n26464 ghg.zmq\n1533 zsgdbd.dmm\n$ cd ..\n$ cd ..\n$ cd czrzl\n$ ls\ndir cmh\n242795 hpgnd.mmt\n157748 hps.ptg\n129797 qjrhbjql.zdc\n18290 sfhrzzcd.hwm\n63141 sngg.vdw\n$ cd cmh\n$ ls\ndir nglsj\ndir szs\n76775 vdpqhvrm.mcz\n$ cd nglsj\n$ ls\n307082 cjvph.fvc\n286825 szs.vpj\n$ cd ..\n$ cd szs\n$ ls\n6233 fgpnnvm\n94070 jjc.szq\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd fcbt\n$ ls\n150417 grncq.brq\n$ cd ..\n$ cd hdh\n$ ls\ndir cljdmh\ndir fptsr\ndir schjz\ndir vrrcrhzs\n$ cd cljdmh\n$ ls\ndir blsrsqz\n21876 blthtcl\n287047 cjvph.fvc\ndir grncq\ndir hdh\ndir jdjps\ndir mgzlprt\ndir mrwc\n33008 pcwnlp.czm\ndir rdtdjb\n135650 smf\n173978 zsgdbd.dmm\n$ cd blsrsqz\n$ ls\n275130 dvqsffcn\n$ cd ..\n$ cd grncq\n$ ls\n213418 bhdpmv.zzt\n153483 grncq.jsf\n$ cd ..\n$ cd hdh\n$ ls\ndir dzrrh\ndir gjjqhfq\n231302 grncq\ndir hdh\ndir jnfczjjt\ndir ljl\ndir pwv\n154659 rqvgf\ndir vdvg\ndir vpdqnv\n$ cd dzrrh\n$ ls\ndir grncq\n$ cd grncq\n$ ls\n288753 grd\n$ cd ..\n$ cd ..\n$ cd gjjqhfq\n$ ls\ndir hdh\ndir mnj\n$ cd hdh\n$ ls\n241795 szs.mhq\n$ cd ..\n$ cd mnj\n$ ls\n200772 sfhrzzcd.hwm\n237342 zsgdbd.dmm\n$ cd ..\n$ cd ..\n$ cd hdh\n$ ls\n30588 qptfqt.ggc\n$ cd ..\n$ cd jnfczjjt\n$ ls\n93796 rqbsqhp.grv\n30301 sfhrzzcd.hwm\n$ cd ..\n$ cd ljl\n$ ls\ndir lvpvmlnb\ndir tsb\n227393 twnj\n$ cd lvpvmlnb\n$ ls\n16586 gnlmdb\n340921 grncq\ndir lqblqtng\ndir nvp\n$ cd lqblqtng\n$ ls\n190888 dfjrnbwq\n79149 dvqsffcn\ndir grncq\n267567 lqblqtng\n169475 vzwm.pnd\n305249 zsgdbd.dmm\n$ cd grncq\n$ ls\n323438 zjpg\n$ cd ..\n$ cd ..\n$ cd nvp\n$ ls\n239588 mffsww.qzc\ndir qqhwjn\n208451 qrjv.lns\n34563 sfhrzzcd.hwm\n$ cd qqhwjn\n$ ls\n47343 wcpqgvh.cfl\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd tsb\n$ ls\ndir lqblqtng\n40229 szs\n214138 whfw\n$ cd lqblqtng\n$ ls\n16005 fgpnnvm\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd pwv\n$ ls\ndir hzwljqm\n51565 pjvpm\n$ cd hzwljqm\n$ ls\n106132 fgpnnvm\n$ cd ..\n$ cd ..\n$ cd vdvg\n$ ls\n267870 zgm.wbw\n$ cd ..\n$ cd vpdqnv\n$ ls\ndir cmmc\n239093 dvqsffcn\ndir rhgb\n142265 sfhrzzcd.hwm\ndir szs\n$ cd cmmc\n$ ls\ndir pjhhbggb\n$ cd pjhhbggb\n$ ls\n195797 psqml.cjl\n$ cd ..\n$ cd ..\n$ cd rhgb\n$ ls\n102805 qrnntf\n$ cd ..\n$ cd szs\n$ ls\n314078 tcslpbc\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd jdjps\n$ ls\n12330 twnj\n$ cd ..\n$ cd mgzlprt\n$ ls\n15552 gzthzjsr\ndir hdh\ndir qrfvn\ndir twnj\n331113 zsgdbd.dmm\n$ cd hdh\n$ ls\ndir cfdrnjsg\n343470 zsgdbd.dmm\n$ cd cfdrnjsg\n$ ls\ndir gddbd\n309726 grncq.gmr\ndir vzdfj\n$ cd gddbd\n$ ls\n226040 cjvph.fvc\n47672 twnj\n67109 zsgdbd.dmm\n$ cd ..\n$ cd vzdfj\n$ ls\n29653 grncq\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd qrfvn\n$ ls\n74136 lqblqtng\ndir mjjrz\ndir zrsz\n$ cd mjjrz\n$ ls\n27688 fgpnnvm\n$ cd ..\n$ cd zrsz\n$ ls\n9100 wbrgdtv\n$ cd ..\n$ cd ..\n$ cd twnj\n$ ls\n41216 cjvph.fvc\n250320 wnhjfm\n$ cd ..\n$ cd ..\n$ cd mrwc\n$ ls\ndir bjwnmw\n116091 grncq.phw\n24868 hdh.lwn\n29567 qsrtrvr.jbw\n100251 szs.dvg\n$ cd bjwnmw\n$ ls\n128769 rdpsnm\n$ cd ..\n$ cd ..\n$ cd rdtdjb\n$ ls\n132588 gsgjr\n149600 lqblqtng.nnr\n179302 wfbqblml.tgc\n76170 wfrcm.fvp\n$ cd ..\n$ cd ..\n$ cd fptsr\n$ ls\n169594 lzlcml.mgm\ndir zrc\ndir ztd\n$ cd zrc\n$ ls\n110339 gjpgwrcm.lhg\n$ cd ..\n$ cd ztd\n$ ls\n203770 zsgdbd.dmm\n$ cd ..\n$ cd ..\n$ cd schjz\n$ ls\n3212 cjvph.fvc\n288619 jcltshwj\n295116 qmbp.mpd\n$ cd ..\n$ cd vrrcrhzs\n$ ls\ndir twnj\n$ cd twnj\n$ ls\n70492 dvqsffcn\n44411 mhgspcgz\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd jgrdd\n$ ls\n325202 dvqsffcn\n$ cd ..\n$ cd lqblqtng\n$ ls\ndir bbzv\ndir gzqqp\ndir vgt\n$ cd bbzv\n$ ls\n326190 fgpnnvm\n$ cd ..\n$ cd gzqqp\n$ ls\n179929 crn.vpf\n263365 dvqsffcn\n251298 hdh.rsh\ndir jgrtqpv\n62514 lqblqtng.spm\ndir njdhqsvj\n32894 sfhrzzcd.hwm\n$ cd jgrtqpv\n$ ls\ndir bmvfszlz\n286549 zsgdbd.dmm\n$ cd bmvfszlz\n$ ls\n169838 fgpnnvm\n$ cd ..\n$ cd ..\n$ cd njdhqsvj\n$ ls\n2729 bgd\n294119 fgcfbrdz.bll\n$ cd ..\n$ cd ..\n$ cd vgt\n$ ls\ndir btf\ndir csgvtvsq\ndir czbhw\ndir jqnvscr\ndir nqp\ndir rlvnnmh\ndir tmf\ndir twjtnhll\n$ cd btf\n$ ls\n333891 cjvph.fvc\n$ cd ..\n$ cd csgvtvsq\n$ ls\ndir ccrvn\ndir szs\ndir tsmqm\ndir tssfbq\n$ cd ccrvn\n$ ls\n14614 qbl.rjg\n$ cd ..\n$ cd szs\n$ ls\n92863 vswznzs\n$ cd ..\n$ cd tsmqm\n$ ls\n206954 dncrjt.sch\n252714 zsgdbd.dmm\n64306 zzr.snv\n$ cd ..\n$ cd tssfbq\n$ ls\n326310 chffstg.qcr\n51533 cjvph.fvc\n$ cd ..\n$ cd ..\n$ cd czbhw\n$ ls\n290187 bdn.vjp\n185832 cjvph.fvc\ndir lqblqtng\n198843 twnj.flf\n64179 zjwg.jwc\n256134 zrtmfn\n$ cd lqblqtng\n$ ls\n82035 cjvph.fvc\n$ cd ..\n$ cd ..\n$ cd jqnvscr\n$ ls\ndir bgqf\n255437 dvqsffcn\n147384 fzwzqb\ndir gbf\ndir gqzf\ndir grncq\n287135 grncq.pcg\n333854 hqw.hgc\ndir nzrsswd\ndir svmqtq\n$ cd bgqf\n$ ls\n325037 jsv\n$ cd ..\n$ cd gbf\n$ ls\n274259 fgpnnvm\n$ cd ..\n$ cd gqzf\n$ ls\n224386 hwwqrq.hcz\n$ cd ..\n$ cd grncq\n$ ls\ndir bvsfn\ndir gcbzghtz\n100460 njbgfrg\n210677 phnslpqc.rbf\ndir tgz\n95034 vdpbwsn\ndir vsrnbb\n$ cd bvsfn\n$ ls\n184581 lwj.gvm\ndir pszwl\n129220 sfhrzzcd.hwm\n162987 smdvbnrq.zjd\n71207 tspgccr.gdf\n323030 zsgdbd.dmm\n$ cd pszwl\n$ ls\ndir szs\ndir twnj\n$ cd szs\n$ ls\n341367 cbcvvtg.hcg\n112908 rggrhm\n$ cd ..\n$ cd twnj\n$ ls\n29609 sfhrzzcd.hwm\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd gcbzghtz\n$ ls\n139868 scggzqr.bzw\n268881 smlhjltf.rwr\n247122 wbwzdpg.djs\n$ cd ..\n$ cd tgz\n$ ls\n214842 grncq\ndir njffnbp\ndir sfvsdzs\ndir sslvmwt\n212789 szqph.gmw\n$ cd njffnbp\n$ ls\n243579 lmwd.mgz\n$ cd ..\n$ cd sfvsdzs\n$ ls\n7846 nthbtv.zdp\n$ cd ..\n$ cd sslvmwt\n$ ls\n241215 szs.rlb\n$ cd ..\n$ cd ..\n$ cd vsrnbb\n$ ls\n69152 cjvph.fvc\ndir dvjmd\n142844 dvqsffcn\n242145 msm.nlf\ndir nttvm\ndir rjfjwbsm\n47597 twnj.lft\n123883 zsgdbd.dmm\n$ cd dvjmd\n$ ls\ndir lnhr\n70772 qvs.gmv\n80548 twnj.jdf\n$ cd lnhr\n$ ls\n202959 hdh\n$ cd ..\n$ cd ..\n$ cd nttvm\n$ ls\ndir hdh\n$ cd hdh\n$ ls\n178915 fzgzdpbv.jdj\n$ cd ..\n$ cd ..\n$ cd rjfjwbsm\n$ ls\n114145 grncq\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd nzrsswd\n$ ls\ndir bnrj\n17967 cjvph.fvc\n237712 fgpnnvm\ndir spqqqd\ndir szs\n96651 twnj\n6447 twnj.dft\n$ cd bnrj\n$ ls\n40924 cbhq.wpc\n175930 cjvph.fvc\ndir grncq\ndir lfrtwv\n267323 mmq.zwz\ndir qsbmrs\ndir wpsj\n$ cd grncq\n$ ls\n151440 ctmwl\n83350 lqblqtng\n145599 tpqfd.zfv\n$ cd ..\n$ cd lfrtwv\n$ ls\ndir cdzvst\n144562 fgpnnvm\ndir pllhlhr\ndir qvdlrsw\n$ cd cdzvst\n$ ls\n211524 dvsnph.hrf\ndir wchrhl\n$ cd wchrhl\n$ ls\n73032 lntqbfl.nbz\n$ cd ..\n$ cd ..\n$ cd pllhlhr\n$ ls\n70197 dtccz\n$ cd ..\n$ cd qvdlrsw\n$ ls\ndir gdmppfzv\n$ cd gdmppfzv\n$ ls\n244356 nhnsd\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd qsbmrs\n$ ls\n155216 cvt.bdn\ndir gcvlpg\ndir hdh\n$ cd gcvlpg\n$ ls\n306542 bwnfl.rsl\n67067 cjvph.fvc\n341913 zsgdbd.dmm\n$ cd ..\n$ cd hdh\n$ ls\n138120 cjvph.fvc\n$ cd ..\n$ cd ..\n$ cd wpsj\n$ ls\n301391 sfhrzzcd.hwm\n$ cd ..\n$ cd ..\n$ cd spqqqd\n$ ls\n40806 dvqsffcn\ndir gqsbsmfm\n41886 grncq.jqr\ndir pdfsb\n44167 pnq.nfr\n127242 twnj.chn\n125868 vvgzv.rmv\n$ cd gqsbsmfm\n$ ls\n99936 dvqsffcn\n$ cd ..\n$ cd pdfsb\n$ ls\n253190 fgpnnvm\n$ cd ..\n$ cd ..\n$ cd szs\n$ ls\n164955 vswnnw\n$ cd ..\n$ cd ..\n$ cd svmqtq\n$ ls\ndir grncq\n$ cd grncq\n$ ls\n274192 hjdfj.qnw\ndir tcrhb\ndir twnj\n$ cd tcrhb\n$ ls\n64293 grncq.jvh\n$ cd ..\n$ cd twnj\n$ ls\n149048 cjvph.fvc\n192053 rdgv\n101473 zct.rmf\n280169 zsgdbd.dmm\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd nqp\n$ ls\n107620 ndgz.gwb\n$ cd ..\n$ cd rlvnnmh\n$ ls\n27340 gcpcffp.fqg\n$ cd ..\n$ cd tmf\n$ ls\n193196 fgpnnvm\n22126 gwftf.wcr\n92461 hdh\n50807 qbdmzjd.jvg\ndir qrrmhwn\ndir rdcsmpfm\ndir rgl\n36742 zsgdbd.dmm\n$ cd qrrmhwn\n$ ls\ndir jgqzqhdc\n109318 vtrtz.zvh\ndir zmbtd\n$ cd jgqzqhdc\n$ ls\ndir zsdbppb\n$ cd zsdbppb\n$ ls\n325921 fgpnnvm\n156452 tlcs.vzz\n$ cd ..\n$ cd ..\n$ cd zmbtd\n$ ls\n93641 bpzttjt\ndir grncq\ndir hdh\n7832 shl.gbz\n95398 twnj.fsd\n$ cd grncq\n$ ls\ndir qshl\n$ cd qshl\n$ ls\n312490 stqg.gwj\n$ cd ..\n$ cd ..\n$ cd hdh\n$ ls\n225454 sfhrzzcd.hwm\n265069 twnj.hfs\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd rdcsmpfm\n$ ls\n72657 dvqsffcn\n308572 fgpnnvm\ndir hdh\ndir lqblqtng\n180474 lsgjgsrb.jfm\n67774 ptpjjwc.bqn\n$ cd hdh\n$ ls\n19858 sfhrzzcd.hwm\n$ cd ..\n$ cd lqblqtng\n$ ls\n142164 cjvph.fvc\ndir jpbh\ndir prwnwvp\ndir qbnthms\ndir sthfhjf\n$ cd jpbh\n$ ls\n167969 fgpnnvm\n182107 jfzrww.tgb\n$ cd ..\n$ cd prwnwvp\n$ ls\ndir cczhv\n70167 dvqsffcn\ndir grncq\ndir mbjwpdb\ndir nll\ndir qjjgjzbl\n101178 qtqgcj\n113714 sfhrzzcd.hwm\n$ cd cczhv\n$ ls\n158391 fmwzpjdm.nwz\n$ cd ..\n$ cd grncq\n$ ls\n238901 hdh\n188528 mjwcd.bqt\n61037 wpfg.shs\n$ cd ..\n$ cd mbjwpdb\n$ ls\ndir bphnpft\n72033 fgpnnvm\ndir hdh\ndir qdmjnqc\ndir rplsvs\n85983 tpfjp\n303863 wzwvbnw.blb\n$ cd bphnpft\n$ ls\n274331 gmmmlsj.pwp\n$ cd ..\n$ cd hdh\n$ ls\n154778 szs.dtw\n$ cd ..\n$ cd qdmjnqc\n$ ls\n133639 grncq.wnj\n$ cd ..\n$ cd rplsvs\n$ ls\n332906 zpstcbj.zvt\n$ cd ..\n$ cd ..\n$ cd nll\n$ ls\n82368 cjvph.fvc\n278280 ctmnzd\ndir hdh\ndir lqblqtng\n211682 pshd.fds\n319207 sfhrzzcd.hwm\n46873 vrf.fgg\n$ cd hdh\n$ ls\ndir hdtg\n$ cd hdtg\n$ ls\n19779 cjvph.fvc\n$ cd ..\n$ cd ..\n$ cd lqblqtng\n$ ls\n301792 dmhslp.hvz\n305639 dvqsffcn\n321833 strqtwrw\n112471 twnj.fbn\n20731 zsgdbd.dmm\n$ cd ..\n$ cd ..\n$ cd qjjgjzbl\n$ ls\n78166 sztm.zrz\n$ cd ..\n$ cd ..\n$ cd qbnthms\n$ ls\n158365 bvwjvrvr.bvb\n$ cd ..\n$ cd sthfhjf\n$ ls\n239051 dvqsffcn\ndir ghgcfwg\n327346 glfcswq.cqc\ndir tjqhqs\n124072 twnj.cbn\n261890 zsgdbd.dmm\n28917 zvbn.rbz\n$ cd ghgcfwg\n$ ls\ndir szs\n$ cd szs\n$ ls\n29912 fgpnnvm\n$ cd ..\n$ cd ..\n$ cd tjqhqs\n$ ls\n125555 twnj.dnv\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd rgl\n$ ls\ndir grncq\n59380 szs.jvh\ndir wjjzprnz\n$ cd grncq\n$ ls\n283850 lqblqtng.cdf\n$ cd ..\n$ cd wjjzprnz\n$ ls\n339632 hbg.vpp\n15887 zglntj.qtt\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd twjtnhll\n$ ls\n207430 cjvph.fvc\ndir dnv\ndir hdh\n232881 wwp\n$ cd dnv\n$ ls\n215966 lqblqtng.tdf\n39850 szb.fqn\n45436 twnj\n$ cd ..\n$ cd hdh\n$ ls\ndir grrs\n288906 hvjv\n219092 mnr.qhg\n336470 rwh\n145474 sfhrzzcd.hwm\n24384 szs.fwn\ndir twnj\n$ cd grrs\n$ ls\n94496 dvqsffcn\n$ cd ..\n$ cd twnj\n$ ls\ndir sbhbnsw\n$ cd sbhbnsw\n$ ls\n162705 sfhrzzcd.hwm\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd pgvmpmn\n$ ls\n18511 sfhrzzcd.hwm\n$ cd ..\n$ cd pqqcvcm\n$ ls\n201925 lqblqtng.mwb\n300811 scjclwwz.bng\n$ cd ..\n$ cd zglbptq\n$ ls\ndir brhmv\n237267 grncq\ndir hdh\ndir mhnfrn\n131601 qtjwj\n69185 tdtgmgw.qwc\n96764 vppqqpf.znc\n$ cd brhmv\n$ ls\ndir ddd\ndir dpnghvpp\n102457 grncq\n239654 jtvzhvd.jvm\n276919 sfhrzzcd.hwm\n235776 zsgdbd.dmm\n$ cd ddd\n$ ls\ndir grncq\ndir mbrcgcd\n$ cd grncq\n$ ls\n110323 fgpnnvm\n$ cd ..\n$ cd mbrcgcd\n$ ls\n100317 pbb.hcb\n$ cd ..\n$ cd ..\n$ cd dpnghvpp\n$ ls\n259305 bscmwtd.mlw\ndir hdh\n$ cd hdh\n$ ls\n49708 grncq.zdl\ndir szs\n$ cd szs\n$ ls\n225139 fgpnnvm\n258801 twnj.nrj\ndir wwzpvb\n$ cd wwzpvb\n$ ls\n4309 qlddd.cps\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd ..\n$ cd hdh\n$ ls\n24775 hdh.htz\n238695 jzpvhl\n$ cd ..\n$ cd mhnfrn\n$ ls\n30250 sfhrzzcd.hwm\ndir twnj\n$ cd twnj\n$ ls\n244061 qddtlr.mpf"));
}

/*
--- Day 7: No Space Left On Device ---

You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device

Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

    cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
        cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
        cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
        cd / switches the current directory to the outermost directory, /.
    ls means list. It prints out all of the files and directories immediately contained by the current directory:
        123 abc means that the current directory contains a file named abc with size 123.
        dir xyz means that the current directory contains a directory named xyz.

Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

    The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
    The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
    Directory d has total size 24933642.
    As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.

To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?
*/
