// https://adventofcode.com/2022/day/3
// (part 2)

fn item_priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        return c as i32 - 'a' as i32 + 1;
    } else if c >= 'A' && c <= 'Z' {
        return c as i32 - 'A' as i32 + 27;
    } else {
        panic!();
    }
}

fn badge_find(list: &[&str; 3]) -> char {
    let mut elf1: std::collections::HashSet<char> = std::collections::HashSet::new();
    for i in 0..list[0].len() {
        elf1.insert(list[0].chars().nth(i).unwrap());
    }
    let mut elf2_matches: std::collections::HashSet<char> = std::collections::HashSet::new();
    for i in 0..list[1].len() {
        let c: char = list[1].chars().nth(i).unwrap();
        if elf1.contains(&c) {
            elf2_matches.insert(c);
        }
    }
    let mut elf3_matches: std::collections::HashSet<char> = std::collections::HashSet::new();
    for i in 0..list[2].len() {
        let c: char = list[2].chars().nth(i).unwrap();
        if elf2_matches.contains(&c) {
            elf3_matches.insert(c);
        }
    }
    assert_eq!(1, elf3_matches.len());
    return *elf3_matches.iter().nth(0).unwrap();
}

fn badge_priority(list: &[&str; 3]) -> i32 {
    let badge: char = badge_find(list);
    return item_priority(badge);
}

fn badges_sum(list: &str) -> i32 {
    let mut sum: i32 = 0;

    let lines: Vec<&str> = list.split('\n').collect();
    assert_eq!(0, lines.len() % 3);
    for i in (0..lines.len()).step_by(3) {
        sum += badge_priority(&[lines[i], lines[i + 1], lines[i + 2]]);
    }

    return sum;
}

fn main() {
    // example
    assert_eq!(
        'r',
        badge_find(&[
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ])
    );
    assert_eq!(
        'Z',
        badge_find(&[
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ])
    );
    assert_eq!(70, badges_sum("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    //// user puzzle input
    assert_eq!(2631, badges_sum("FqdWDFppHWhmwwzdjvjTRTznjdMv\nZBJrDVfQcfSRMLjZnjjM\ncBffPfbrbQcgQJggfVQJBPbCwlPtWFDWHFHhpmmGlGmlqmDG\nPNbMLgmPgRDgRtMPDdmdbmdmQrTBVCZnVnpCnNHHVZBNVZHc\nljvvqhlvshhnrcpBZqpTcr\nzGhWzFTJvsFttddWbMRdmP\nhhGdDhfdDhmdnHwtzzRtdzbzQQQg\nWTTZJLsvLRJCcWJWScWWLtzjwBzBttbBzSwVQjQBtt\nvLJcLLTRsvsTZPqHGhFHGhhPhfqD\ndJszLvzvCZZsJmszCrrdFmpppMpDMQPMgmDcDgSS\nnRjRbnnjlNhblnjtVtQlWttMFPfMfPgDMpBgSBPgBS\nGWVQTVTnTNhjblQNRsdJLLdTCdzdZLrdrJ\nRnLJBfmJfmNBHlQvvbdQ\ngrhgrtqgjJhhggNHqvwWqvbNlbHw\nGVTTsFFjJjVVFVGCFTJDDjhFcZmRMZfnZcncSpMSGcRPZpLp\nbwSNRNSRzSWnPnJRldMBMQmMlsCcPPCP\nFGpDqTTVFFprpjLVQMMGtMclcmHGtBdc\nTjhhgTLpVZhpLDZqrTqZVpBSwJwzNnWNWJvzzNSggwwN\nPHTMsmwrJMwLJvJddvdHwvcWnnWfccqGnhhfGcDqsnGc\ntZVlzQZblBgcSqqRhRmzqf\nCmlgmZlQtFtZNlVZdHLMrMvvCrrvvTTC\nPrDGBBddprmzddrSqccRgSTpqbsMRR\nhvLtfFNvvZNfGGfRgbqsRNlTSSgsbM\nQQQjZHHQjvththFffCHPPzmGrmDzDWrWBw\nGrjGrpjjCsnwhsGGPwlPTPLPVttPqLVl\nHzSHHhczRlLTHqqq\nDbhvFSSzQcZbcFbcQjrJrMJmmZnGJmJnjn\nmvTDsJLwzlWNDDnZngZNdd\nBVtPqFMqtvQFqPqjFBMVtRZGNGhfNcfQdpfgnQgNcNgp\nHBHtPbHCLCzsLJvT\nnVHVFfggbQVmFFfhLpBpBTrfLBCB\ncjjRwJdqtwwwNNjcwRMwnLNTpNsGCnLsLGhBhpNn\ntJqStcRMMMjPwlnvzgQWzmHzlmQl\nnNSRcDHRmHhhDZZZdBDfWJdfrJ\nLGrGVtjCPCbbQQQQLvQpbVQZzsqsBMdBqMvZMMJZqJvdBW\nGCPLlLPPPbTpbCbpLPpVlmrmmrHhHcHnHrHHgTFFnm\nrQbnBrDTQcdpHttt\nqNsNpfjLpNLRNqLMtdVsddcmVzdzVh\nWJfLCfvpWpNLbwwwBwbZFvlZ\nZslGPhBWBJbNjmbTmSWN\nqtDCZcfZtDjbjCQvmmSj\nttpRzqtqRLDzpRtDdtPlhhZZGBHGPsGZPhwd\nWjtcNnMtztGFrsNjcgRHvdwHhhjRwlhwlg\npPSpPBPVqJqBCCPvZdHlVGddwdhvgH\nqpDBpBqPTDLqGLmpTPqbDmWMNrFrrFQnFMNnMNQtMm\ndNwfsjFLQLFNBhTCTPPTBJhhTP\nVmVMgqgRVHtztmgqgzgqRzgMPWCRWTpPCCPWPThDdZPCRZJp\nVMqtMgGHmgVMvmqMMtGMSmbslnQFcsndfjNNsnfscnbddF\nqbWcqrFNCJGSChvLGv\nMnslRSpSVsGgGhDDwLvl\nRjMtfnpmmmjSWSbjrNPN\nrsCsqTVgfCnQFdCznQ\nZRsBvMvmZMGQQmFnDmQzzQ\nMlGLlBjRGjjWWGRGMlNrLLrssTPNfLcTgrVr\nvpSBBcJnWnSmcqmcChdcgf\nNRNPFDwwDbThqTJffZQHhd\nDzPDNVNbLwPzMLRbNMDjBlvpWjrvrVJjjSsBjr\ndndGpnWdVnBPFFHTBgGH\nqCcCmjqNNJDtcJQjqJqMCQJcFTBRZRwBZBHDPRHBHZDbgBBR\nszJcCtNQqjqCNjjNhdgWfLLdhVWsSVnr\nNjJLgSLGGCLWvqNqNBvwFb\nhtmVmtHnlZmDVhtmhblmDMHddBRvwwZQQqWPFvFwdvRWdR\nHnlnHfDsbSTbJzsz\nJvZfsPsPhRfZZnCdtnmjHVRLCH\nTzcczTtgqpgGSTlHHVHCGjCdHdnQQQ\nwDBDSDlWzwwzDqqMtrMrfsfbNvJNJB\nThfQTdQzQbgdhdNbJFJSlbBLlmqqHBSHCRHsHm\nrWBWWPcvpGGwjPPpvpPGplRSnLLnCrLRsRlLsLmLmR\ncGWPPMtpZDwpMpJQBQQQDBBVNfdQ\nWcWDRLSzFrRFFccPmnssMrGtmMnnGm\njvvgCCTvNqTtJqsnqwPBmspZMB\ngbVgbvlChJVbCdbtTgjdLWSHWcHSDcQzFzRzDF\nzTrHgrFWRrWvMpPNBVZZHVfN\nhdnlltGLtGSQPVLSNBQN\nCqGGtGwlhlGdtGmbtjtmmvDRFvVFTTRDRbRgFJRbDr\nDTFpQFrZDFBDFrTNFjSWJsWlWjsRJcslsp\nzzPqvdqNzvqzfzMfzqmCzzfJsclWjSSRWglgjclWSmcjlJ\nhfPGwhhvMGCVCdddhQQZQnNrTDQnFrtn\nDDMFjwVTgVmMWgVpdqtlJnpvHHnslw\nfFfZzLSfzBfZBZtqJJHsnSsldlld\nGLcNhRGLZBhmDWTjDTWF\nFbVcQRVRBFfNFfccVfZcWddnGrrHncWtdHsZ\nhwGwjgTSGrssZHHTdn\nLvPvCJgJLwjlSJmSPLvvgGfFBpBVfLLMBQMRRbzMVb\nrfMCJPBMMCrSCSBGZZqRlRLzqhqh\nLDvdNmbgHjHgnmnvnHjgDjqlZhZzszhqzWsRWRRNcRNZ\ndmbvHDdnjDjVmjTmHjJVrfprPCCVtLSMrSpJ\nfdfTBfNVZffMmdfdhGhcJLJrvnlJvrBJ\nFFWFWTzWSWtFgPHgRPWTzggpJcvvSCGSGJnrvhhrrJhlGlGn\npztFjzRTqWzgHqHWtPtPFgmsNfqbDdwDNVwbmfwbdNsb\nzCHvDWwvCwgpNRCWWHttCwvNPVTqrRrVbbsnbqQPbVsbPrqG\njBBcmZBGmBSSJdmhBVbnnQrVbVqqVPbhbP\nZfJBmLjfJZMcdZmJffGMtHtwNCvWwwwztMzg\nrgFgllfdpFlTHfTnfnNPNtPBBVtpzmVVPmmP\nbLSSbGhGWSWmLzztcQPCQC\njvwhhwRwbwshjGhWwsRRTFflFJFzfddFrHzTrJ\nGSwgSdwfvdfvwgGwBLdJbjjpmFjSTRpqHmRrjptrqt\nNDMVMCsWQVCsQFjFTHjVLqjFrp\nPzWDDCNNlCWfbhZZLfBGwl\nclFLFpFJcVmmWWgWWcWTdwZqCZCZqDqDDSwvwrdHCZ\nGGLfGtbfMNbbLGtzjBNnsMjjZCwDvQZHrHQvDHHHHHHrrNHq\nBthbnfPGMstmJgRPpFRTLL\nVRcdVRPTgVTLVMwmggJBwblJFlmb\nCqDDQsjCCrnnnQQtDcrnGCssJZNvZbvNQmZwmZlbNbwFNwmF\npstjcqrprsHrfDpnrCnHCzRhSMhMMMRVPLMhddfhzT\nQNDQThccDghdcLLgVsrVLVlMGM\nBbbWpppFFpnfnFbBBPRMTJMRsGlJGlJVLf\nwpnbmvbnSwSnmzHHzwwhQqTctNCcqdNqZvhQjj\nlwCLwLjzLhLHCvwjGCZJbQSHdBQdDdbDtdSQ\npnzcTVsszpncgFdJpbtDBDttDSJS\nzTPcRrfzgzCCvGRhLLqW\nBgDbztvdDzLZZwMbDDcFHQQJPLWWPJQJHTHF\njfpNqSrpCqNfNSpjCqSqshNFRQJcJWRRFssgWRHWWFWHQJ\nmrnNNgNfMZwmDtZw\nZrrFPQsQPRLcvPJvhg\nHpmTjnBmDDDwqtVcHNvJhNVRhgvh\ntTpTGfnttqwnqQJzFdzfSdzSzr\nqVVZqfVNdnBZMNzNnPzfMqbzJvFSjSllvjBwrvrvFrlsrjJs\nLhHtDTmWmWmGDhGLWHghHLCwrwRjjsPJrSjFjFrFClvrww\nmDDgtcLcmtTWTTHhpWLDHhDNMqVnqfdqbZdQpqVbzZVPVb\nWGpVMtGZplgHVWMtZpZFHJjndvFdjddJhnjLHF\nDTrRcSSccfzcCPDCTnvJvjbrrhjNvqjqJL\nzczwPDTmfZZtgLwWlp\nsTtTpvMjpBdmTlhlBTdvsvdncmVcVVVVHzFqVnFfqzHzWV\nGGwQRZNCRgDgsDDbSbSgfGFFrHfVHVVWrHWWcVFF\nRZRPgbNSJDsTsplPLsvd\nGrGNGhpnPFFBfCQCMwrVlwTC\nPPWHmmSvSvdTfJvJVVfCfJ\nRjWDmDdDbjcSHmRRLRRHjdDBsFFbBngBzhqbBhpqBPgFFg\nTfdNjJjmShGcWvQNQqcNFN\ngsRZRMqbDpsHHnZnngMZFPlzQRPWvRtPwtRWzlPv\nbrgMMgZCLsDpHGGjfGJVSCqddT\nBdSRjHScGMVjGdcScwLgqQqphNqNDqBBQW\nzzQClvtttrwqrwgCwp\nPQJTzvJJTtJQlvQftmfdmdmGRcSdcMSGdS\nPjPwvwlfGlGCGwppWBsWQVVQnpCQ\nFRSHzMJdrfRnpmpWmp\nTzJTMNNrHLJfTJccvjqwwgGLZLLc\ndCpjsGvcsLvszTrRRlRrDJ\nhPNqqpbhFlrDnPrRnz\nSBbphFNtLvvSfLfc\nzTFnnZzqrjFVnZTrtwMDptbpMwMMBDzb\nJGPCjWGgJjPWGJjNchmbBRpNNDsRsNBbbRBppD\nJWCggWSCmhCmvWHTQrTQfHjdVlQZll\nWWbrmZjbmjpbWSmcWHSbLddwvDggLFDhFrRlFFDw\nqPMVzTPQVfzvVzBQTMtRswwlDdhDghhsfLwdhs\nMMMPQBPJPzCVzvzQVtBJJMzCbmSZNWbSccHHmmZpWSZGZS\nBgmMgjlBMjHPssBnwphtFwhSVVmGGwFS\nrfCqQQrfvCQNRqCnCthFVSGSGtVS\nTrzLNvWfQvrWQZNWsnHsssBBcZBMsDdZ\nCtJCddDHDDPGHCdNVLPBdLDbbGFbwZSsSSZrfFFrGbGlsb\ngvjpnvhRphjmpmSfsTSnPZZrZfrZ\nghRRRhQgWvmPWtHHtLdMtH\nqNgQgNgNQFVbqVQDMRZMDRBHHJHRFh\njnTTPzWZCzrWzGRHMrhmBvJBSBBM\njtCGWnCWPTsTzlctWPtqLbwqZggNwgLbwdVq\nDJgJDgFqCGlhFDGDCWhqCwRfpbcpbsgsVscRpwbwws\nSmLZQmMVvLbsbssNQsQN\ntHZMLZZZMVtmnMHWDHjWlGhFDqCl\nPZRmjlDBWRBWBQMdQQBDPRhfzSSTTnfFnfzFpVFjfSVFgS\nwwHCHLtLwnzNHVTZZV\nrJLtZtLZcCrvJwcbrGLvrcDQDhPldhGdmmPRQhhRBlPM\nzShzVhbqlbpbRNRscBNwCc\nDfmFWnfmnMFDdngdngvJFHgQQNwNsHHCQCwNTTzRcN\nFFFZMWWZZJMmJJMFFpztlPtSllLVphttGZ\ngWhGwmwQGhVwGzBMnDFmdmDDLbCnLn\ntRRHNHHlPHPfPltllNNNRsVPFrTrTbrMMTTDMdLFCrFdPP\nHvRvvHNpNvfqSshwqwVQZWhggwhw\njDDRDVqNsRMMVFjFbtzpBlpllCBlhSLHSStH\nJJWZZTwWcmZCzQShHhzhhm\nwTrgvTwccvdcfvJvWJrvJTNzNMfGNNGjFRsjNDbfVzDs\ncVWPPThWctcFRdQpzLTzBL\nNNCqwNSrsDqNSSgLgffDCNpBdRvvdpmHRQvBdBqvdzBB\nCgbffsDJsSsNgbgJrlcGhGtjnWcJPncjWL\nRwwTGRjGlwWNgjgfQVNmjj\nHJPdLhLzhbtbdLPLbHHJLdcfBffBgrFQVZgzvgBfvrvvmr\ncmchnhtPqLSJJbdPLntlsDGCTWWDsDRwWWWTwS\nDDDBsPGPbwhDcDcj\nrgfNgCmHMvrrttvtfmNLgrLcQQJTJhcnjTQHcZlwlQHnwj\nwgfLmgFgWdBFzSVR\nRwsbssJjnbJwwsGPPdDLfTDLLLWvWNDGDpWD\nzVtHqzHHVcBQTdpSgvQDSpTS\nHHhFCdrrHchmrhcmwsjZwRwMJlshJJJZ\nBdQgtJMLBqshLfNhbccfFhffNc\nDnVWvvpvWvzrpwRWDbTvbTjTFGlfFSfNSFHGmFGNFmnPlmFS\nwDRDZzVWVjrRVjzrQgQdbgCZLCsCQtbd\njQSgWjQmFFvHmjHWVVpZbGlbGlfGpbfGGWpf\nzBPtTqzPBcdwrzPJwqCztTNvCGpbGfDnfhZpNGGZGZbG\nMTzPtcrcwzTttdBJwPvFVHjMLFHQMgsSLHvs\nBHtjmmTtmDtHZjMMdNzCzCWcWZsZdsZs\nJRRJVPLwQJrVMPJLVVwChpRRhcdzcNzhzChNdW\nSrFwfGwGSnnGPlBMDtjbllggBn\nrsMDTrgsBNBgMgDBhfhDghrtcRWJttcmGRWLGQQLJRrqRL\nlnVVjvPbwpndvVwlVCjVwtLcsmLLqWWtttlsmGGcmm\nSsjVSCZbgTBTfNZH\nwlddvlldlBzqSmStdqmmngwgDCgrpMpgCngrMCfn\njVLTHGRHjjvPGcDrfNMbnpngVbpf\nRJRTTRZJLRJQZcGGHLhHvTdmBFWFtWzqdWWQlFzqBFWt\nHDgZHpZSDpBQdRpHHRsDBNNzTvfTQqcqNNTqTqPQvl\nrnMFFMFJwmNgvmfTzcfq\nCWMtjCWMCCLWrWVWJwBbpgtRZSHbddZHBDDb\nhjLWPZJpZptwJghSfgHTMTgRMR\nlCcrblcnlzqDsvbfffTRSMnffjHTgj\nsqscDcblGDDpWPwFGwjLLp\nplQtRqRlGpPPPLZtmtpttRtJjHJvddLTHrLHJJjbdHvrrN\ngcMgTBWBCTczjnvNznnbgH\nFBhSwsWFWDhQZQqTTZllsT\nWsVttWDbvbtRjDVtDbDbJjrjFpCCcBrSCwrrjwww\nnzHqGNMfHqMMwJLLsNBrNcBS\nqgnHflgzHlqlZzhnzssfzbZtmVvmTQvQdTtbbDbTvv\nRtQCBbJDFhJtQtZtCbMnVnPVfGPpJVJWWmwJPG\nNsczRsrdrNrjlcSTGGnwmGmnmVPWwmVT\nSHcljRHzNHHjSHcrgNslcczgZtDgMDBCvQQbhDBCbFBCFCCF\ndRTFRJTRTgJzSSJmzJfN\njLbQllLvvvrQlLQBrvQmFBGzCFtPSMmSGCtPPF\nQjsvDvqlvrQQvnsLjDWghTwZWcccFdRVpc\nbWnDbMJMFbhZSfngpfpd\nHjcrlvjRjrjlLqTqpwQgpfSQgghZgvgf\nTHNTjClLHDCCpWmbtC\nNpBNsnFMLBcPMZccbQGblWRgGZmbRWdW\nCqrJTHCvDfrfwwJHRBvRGvgQmvmlWtlR\nrHzJJJDDwJCqjHTwBLccMpFhshnSSsMz\nbqVqqrDMpLFbLpJJQDMjbpZndwsvwHHswvnvnnZslG\nhgfRghhBWgfzBgZvncvcGvWrnZsG\nfTSSSBtTzhPththRrrCBzVLqMMJQjVJJCLjFjjqjJq\npQlCJQjVvVGGnjNqFbFP\nwTqtmSmTBhstmstTmWRSdLdfPdNGnhGhdPNnNnrN\nmzTZBwWTcSTsTmZRRmzTsVMMVMQMDJgVqqMgcJlVDl\nvVwCqdCDvMrlDJCqrDMrPdTFWwZNbRcbmZWQbRQZWZcQWm\nfGSfhfjHGBlhSpRmpZFZNbQWjj\nntGlgSSGgStVqdMCDCJnqd\nTlTRCrhCpmnCRtMRRWbvMBfwBB\nDVdQcsccdczbtQBWvmQBBW\nPgHPNcmcqqTphlHTCGrT\nGVLCBmdLVtlrmqGCqrTCGnjGSfwzNfPzfNNGzSSw\nrMDcWMJsDWRvRcsZPwpJpSzjwwfFjfjJ\nhcsscQQsDcWbHbmLgTbVgTmlrbgq\nVwJndPThQQjdvbrb\nBHlzFFjlZGBBlZBDFSmllfGggLNbNvggtCgNrLQLbfvbfg\nSHSHHlZSGZSDHGzHssGmnjhjphJPPpPwnsJRwPMM\nQFFGpfGtwgtjwvpwpGGjjBHgqzNNllHRlRllNdBRJl\nSWVDWVrDhnWhnqVnWPrhcSldBJPsRMMdPHsRdJHRMMBP\nSWZZcmnnZnSCLZDZftFFCvbfTTqTCTQF\nTvZBTFZpshPggBNN\nbbctWQDjcnwtwDDDllQzLfgsPzqgrsTPLshWfP\nbwRbQRnmDwtbTjmwRJQRQVZHpCpHVdvpMHZZMHZJvd\nCfqzMCGvGqNrCFFNwcMBbnnbbtBbDStw\nsJjsTjvVjLVlLLbtHSnSSwQsDBSH\ndJlvLVgJTRVPWWjjggCzhNGzzGZdhqrdmqrq\nzGSHWGjzpRsWVfsNwZ\nBmPCLzPlJBBvQmLFQrwwrVtfqtqZtsvwfR\nhLLzlmmBPFFLJMlmgDTMgdSGbDgGHbSp\nppngVjZwNZwwVJjjnnVVJJJpLrsgmsmrbctsLcLmscDggDsL\nvHvRzRPvQPPRqPTlffRTrssbtqmmtbcMcNrDrrtD\nCHWdWzWQzdChNVZjZZwB\nLjVFhwjbFggMdVggTVMNRWRGWqQWtRNWGlQW\nDCzCpJppzJJDScFBzvBGnRWGWrNRQNNWGtNGqB\nvFJSzDSszzzccSZHzDDJmvSJhVwLjfVTPZLfwLgjThLZwhVh\nFMSSNScRlSGzfqWbqqcpWBhpmW\nTrPrnQNnJwtQpmmpmbbL\nvwvCsJZZnnwVnVHsfNjDRgSjRzgRjZjN\nCRhsghlqlvjhPslQdrMndMTWdPnTMJ\ntcDNFDpDSDwDtHrSDwDtFmtpJzWTnzQJpzQGnGWQMWnnTGBd\nZfFSmSfwcwcScZHtVRhRCqLqCfrVrrfg\nvHPsBzvRvhCzbwbnjHLVmVbW\nggdlpZNdZdflWTGpVMjgVbqjVmngnVbr\nlNpfllFGpZTtclDZzSWQFCWWzChPzQhS\nddbJQGsgJcQccCjjTC\nDDflHqNNNjCCPNbT\nLFpLzfzqHzLbLvtndJhs\nmHtbGdwzmtHZrtrHtHGwrmtcFgBFRsBFcdglRsfsdgJBNN\nWvpSVjCCTCSPVDqTVjLqJwWRcWsNlgBcBlfWBRfl\nqDwjvqjVTDDLSPpnjqVnzbhbZbrGMmHzbMHHhh\ncMrrVsdtCdVtwqdHgLjPLFLfcfZZJB\nzGSlGGvpDWWvbSSNTggZPPTTJLZPjPzJ\nbNQDWbjlGSGNnWGblbQDQCmdqhsMtrsVnCrtnwdnCt\nWZSnCCMMdMMMSJMSVZmmFqVjqjVwVjjc\nzvzTlQQQQTNGbmQTjqBjcwwwGgBHGwqB\nmthbtmlPhTrNzNhhPLzlPzWfWPJdRCWspPWSSRsWSnJp\nnmqsjNFqNLcVlPNvRdvPPv\nMrggHrMpbtTpgpDptlQRJJldVRPBsDQvvV\ngrgrTzfTGSFGsGSCLc\nhjdjCfQCLdQcWMfDDQhLsLCjqvVqzzZZprBFbzVdvVnqnBbd\nNTglmJGJRnRZVrVvFr\ngPGgPJSTPJtHncnjWDhDMDhP\nZWWqBqqmgFFDMTTWDMwwcW\nddGPpJQPprRSCrQzJPJGjHDvssMTDZDwcHhMsHsMHS\nzNRGrCRRCjPpdGQJNglqntnbtgNBZtFqtt\npslQSspQrqHfgMRl\ntJBTjsTLGMBGMFMg\ntDwnhZdnLdZDwczzcPvsVvVW\nnZnMSDnGtnzmzWZZcMmgMcHwFFVHhHwbHsVrwQhVhF\ndlTqNqfTjdJflCppCQQRHVwFRJrvJsFVQs\nLNTqBLPprpLLzWtSnMZBMDGm\nPBjlPvvcJlJzwqjnnjLnwm\ndMfdpVfVhGVfVpVTtWWbzwngLMLwMRCmgmLMmsww\nSWppdTrprThhrGVztcrllrQJZNlBZlrNZB\nTTrNcjGNWDdDPDpPjDPNrbmFPfQSFmfSMCmCfSmSSR\nsshhqshzBVnzHgwCMbCwcfmRFmRMbm\ngHVVnBzJcBgVhtZZtgJhpGrGNpvDpddvprNGrlTJ"));
}

/*
--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/
