// https://adventofcode.com/2022/day/3
// (part 1)

fn item_priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        return c as i32 - 'a' as i32 + 1;
    } else if c >= 'A' && c <= 'Z' {
        return c as i32 - 'A' as i32 + 27;
    } else {
        panic!();
    }
}

fn rucksack_sum(list: &str) -> i32 {
    assert_eq!(0, list.len() % 2);

    let mut sum: i32 = 0;

    let mid: usize = list.len() / 2;
    let mut part1: std::collections::HashSet<char> = std::collections::HashSet::new();
    for i in 0..mid {
        part1.insert(list.chars().nth(i).unwrap());
    }
    let mut matches: std::collections::HashSet<char> = std::collections::HashSet::new();
    for i in mid..list.len() {
        let c: char = list.chars().nth(i).unwrap();
        if part1.contains(&c) && !matches.contains(&c) {
            sum += item_priority(c);
            matches.insert(c);
        }
    }

    return sum;
}

fn rucksacks_sum(list: &str) -> i32 {
    let mut sum: i32 = 0;

    let lines: Vec<&str> = list.split('\n').collect();
    for line in lines {
        sum += rucksack_sum(line);
    }

    return sum;
}

fn main() {
    // example
    assert_eq!(157, rucksacks_sum("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    //// user puzzle input
    assert_eq!(8243, rucksacks_sum("FqdWDFppHWhmwwzdjvjTRTznjdMv\nZBJrDVfQcfSRMLjZnjjM\ncBffPfbrbQcgQJggfVQJBPbCwlPtWFDWHFHhpmmGlGmlqmDG\nPNbMLgmPgRDgRtMPDdmdbmdmQrTBVCZnVnpCnNHHVZBNVZHc\nljvvqhlvshhnrcpBZqpTcr\nzGhWzFTJvsFttddWbMRdmP\nhhGdDhfdDhmdnHwtzzRtdzbzQQQg\nWTTZJLsvLRJCcWJWScWWLtzjwBzBttbBzSwVQjQBtt\nvLJcLLTRsvsTZPqHGhFHGhhPhfqD\ndJszLvzvCZZsJmszCrrdFmpppMpDMQPMgmDcDgSS\nnRjRbnnjlNhblnjtVtQlWttMFPfMfPgDMpBgSBPgBS\nGWVQTVTnTNhjblQNRsdJLLdTCdzdZLrdrJ\nRnLJBfmJfmNBHlQvvbdQ\ngrhgrtqgjJhhggNHqvwWqvbNlbHw\nGVTTsFFjJjVVFVGCFTJDDjhFcZmRMZfnZcncSpMSGcRPZpLp\nbwSNRNSRzSWnPnJRldMBMQmMlsCcPPCP\nFGpDqTTVFFprpjLVQMMGtMclcmHGtBdc\nTjhhgTLpVZhpLDZqrTqZVpBSwJwzNnWNWJvzzNSggwwN\nPHTMsmwrJMwLJvJddvdHwvcWnnWfccqGnhhfGcDqsnGc\ntZVlzQZblBgcSqqRhRmzqf\nCmlgmZlQtFtZNlVZdHLMrMvvCrrvvTTC\nPrDGBBddprmzddrSqccRgSTpqbsMRR\nhvLtfFNvvZNfGGfRgbqsRNlTSSgsbM\nQQQjZHHQjvththFffCHPPzmGrmDzDWrWBw\nGrjGrpjjCsnwhsGGPwlPTPLPVttPqLVl\nHzSHHhczRlLTHqqq\nDbhvFSSzQcZbcFbcQjrJrMJmmZnGJmJnjn\nmvTDsJLwzlWNDDnZngZNdd\nBVtPqFMqtvQFqPqjFBMVtRZGNGhfNcfQdpfgnQgNcNgp\nHBHtPbHCLCzsLJvT\nnVHVFfggbQVmFFfhLpBpBTrfLBCB\ncjjRwJdqtwwwNNjcwRMwnLNTpNsGCnLsLGhBhpNn\ntJqStcRMMMjPwlnvzgQWzmHzlmQl\nnNSRcDHRmHhhDZZZdBDfWJdfrJ\nLGrGVtjCPCbbQQQQLvQpbVQZzsqsBMdBqMvZMMJZqJvdBW\nGCPLlLPPPbTpbCbpLPpVlmrmmrHhHcHnHrHHgTFFnm\nrQbnBrDTQcdpHttt\nqNsNpfjLpNLRNqLMtdVsddcmVzdzVh\nWJfLCfvpWpNLbwwwBwbZFvlZ\nZslGPhBWBJbNjmbTmSWN\nqtDCZcfZtDjbjCQvmmSj\nttpRzqtqRLDzpRtDdtPlhhZZGBHGPsGZPhwd\nWjtcNnMtztGFrsNjcgRHvdwHhhjRwlhwlg\npPSpPBPVqJqBCCPvZdHlVGddwdhvgH\nqpDBpBqPTDLqGLmpTPqbDmWMNrFrrFQnFMNnMNQtMm\ndNwfsjFLQLFNBhTCTPPTBJhhTP\nVmVMgqgRVHtztmgqgzgqRzgMPWCRWTpPCCPWPThDdZPCRZJp\nVMqtMgGHmgVMvmqMMtGMSmbslnQFcsndfjNNsnfscnbddF\nqbWcqrFNCJGSChvLGv\nMnslRSpSVsGgGhDDwLvl\nRjMtfnpmmmjSWSbjrNPN\nrsCsqTVgfCnQFdCznQ\nZRsBvMvmZMGQQmFnDmQzzQ\nMlGLlBjRGjjWWGRGMlNrLLrssTPNfLcTgrVr\nvpSBBcJnWnSmcqmcChdcgf\nNRNPFDwwDbThqTJffZQHhd\nDzPDNVNbLwPzMLRbNMDjBlvpWjrvrVJjjSsBjr\ndndGpnWdVnBPFFHTBgGH\nqCcCmjqNNJDtcJQjqJqMCQJcFTBRZRwBZBHDPRHBHZDbgBBR\nszJcCtNQqjqCNjjNhdgWfLLdhVWsSVnr\nNjJLgSLGGCLWvqNqNBvwFb\nhtmVmtHnlZmDVhtmhblmDMHddBRvwwZQQqWPFvFwdvRWdR\nHnlnHfDsbSTbJzsz\nJvZfsPsPhRfZZnCdtnmjHVRLCH\nTzcczTtgqpgGSTlHHVHCGjCdHdnQQQ\nwDBDSDlWzwwzDqqMtrMrfsfbNvJNJB\nThfQTdQzQbgdhdNbJFJSlbBLlmqqHBSHCRHsHm\nrWBWWPcvpGGwjPPpvpPGplRSnLLnCrLRsRlLsLmLmR\ncGWPPMtpZDwpMpJQBQQQDBBVNfdQ\nWcWDRLSzFrRFFccPmnssMrGtmMnnGm\njvvgCCTvNqTtJqsnqwPBmspZMB\ngbVgbvlChJVbCdbtTgjdLWSHWcHSDcQzFzRzDF\nzTrHgrFWRrWvMpPNBVZZHVfN\nhdnlltGLtGSQPVLSNBQN\nCqGGtGwlhlGdtGmbtjtmmvDRFvVFTTRDRbRgFJRbDr\nDTFpQFrZDFBDFrTNFjSWJsWlWjsRJcslsp\nzzPqvdqNzvqzfzMfzqmCzzfJsclWjSSRWglgjclWSmcjlJ\nhfPGwhhvMGCVCdddhQQZQnNrTDQnFrtn\nDDMFjwVTgVmMWgVpdqtlJnpvHHnslw\nfFfZzLSfzBfZBZtqJJHsnSsldlld\nGLcNhRGLZBhmDWTjDTWF\nFbVcQRVRBFfNFfccVfZcWddnGrrHncWtdHsZ\nhwGwjgTSGrssZHHTdn\nLvPvCJgJLwjlSJmSPLvvgGfFBpBVfLLMBQMRRbzMVb\nrfMCJPBMMCrSCSBGZZqRlRLzqhqh\nLDvdNmbgHjHgnmnvnHjgDjqlZhZzszhqzWsRWRRNcRNZ\ndmbvHDdnjDjVmjTmHjJVrfprPCCVtLSMrSpJ\nfdfTBfNVZffMmdfdhGhcJLJrvnlJvrBJ\nFFWFWTzWSWtFgPHgRPWTzggpJcvvSCGSGJnrvhhrrJhlGlGn\npztFjzRTqWzgHqHWtPtPFgmsNfqbDdwDNVwbmfwbdNsb\nzCHvDWwvCwgpNRCWWHttCwvNPVTqrRrVbbsnbqQPbVsbPrqG\njBBcmZBGmBSSJdmhBVbnnQrVbVqqVPbhbP\nZfJBmLjfJZMcdZmJffGMtHtwNCvWwwwztMzg\nrgFgllfdpFlTHfTnfnNPNtPBBVtpzmVVPmmP\nbLSSbGhGWSWmLzztcQPCQC\njvwhhwRwbwshjGhWwsRRTFflFJFzfddFrHzTrJ\nGSwgSdwfvdfvwgGwBLdJbjjpmFjSTRpqHmRrjptrqt\nNDMVMCsWQVCsQFjFTHjVLqjFrp\nPzWDDCNNlCWfbhZZLfBGwl\nclFLFpFJcVmmWWgWWcWTdwZqCZCZqDqDDSwvwrdHCZ\nGGLfGtbfMNbbLGtzjBNnsMjjZCwDvQZHrHQvDHHHHHHrrNHq\nBthbnfPGMstmJgRPpFRTLL\nVRcdVRPTgVTLVMwmggJBwblJFlmb\nCqDDQsjCCrnnnQQtDcrnGCssJZNvZbvNQmZwmZlbNbwFNwmF\npstjcqrprsHrfDpnrCnHCzRhSMhMMMRVPLMhddfhzT\nQNDQThccDghdcLLgVsrVLVlMGM\nBbbWpppFFpnfnFbBBPRMTJMRsGlJGlJVLf\nwpnbmvbnSwSnmzHHzwwhQqTctNCcqdNqZvhQjj\nlwCLwLjzLhLHCvwjGCZJbQSHdBQdDdbDtdSQ\npnzcTVsszpncgFdJpbtDBDttDSJS\nzTPcRrfzgzCCvGRhLLqW\nBgDbztvdDzLZZwMbDDcFHQQJPLWWPJQJHTHF\njfpNqSrpCqNfNSpjCqSqshNFRQJcJWRRFssgWRHWWFWHQJ\nmrnNNgNfMZwmDtZw\nZrrFPQsQPRLcvPJvhg\nHpmTjnBmDDDwqtVcHNvJhNVRhgvh\ntTpTGfnttqwnqQJzFdzfSdzSzr\nqVVZqfVNdnBZMNzNnPzfMqbzJvFSjSllvjBwrvrvFrlsrjJs\nLhHtDTmWmWmGDhGLWHghHLCwrwRjjsPJrSjFjFrFClvrww\nmDDgtcLcmtTWTTHhpWLDHhDNMqVnqfdqbZdQpqVbzZVPVb\nWGpVMtGZplgHVWMtZpZFHJjndvFdjddJhnjLHF\nDTrRcSSccfzcCPDCTnvJvjbrrhjNvqjqJL\nzczwPDTmfZZtgLwWlp\nsTtTpvMjpBdmTlhlBTdvsvdncmVcVVVVHzFqVnFfqzHzWV\nGGwQRZNCRgDgsDDbSbSgfGFFrHfVHVVWrHWWcVFF\nRZRPgbNSJDsTsplPLsvd\nGrGNGhpnPFFBfCQCMwrVlwTC\nPPWHmmSvSvdTfJvJVVfCfJ\nRjWDmDdDbjcSHmRRLRRHjdDBsFFbBngBzhqbBhpqBPgFFg\nTfdNjJjmShGcWvQNQqcNFN\ngsRZRMqbDpsHHnZnngMZFPlzQRPWvRtPwtRWzlPv\nbrgMMgZCLsDpHGGjfGJVSCqddT\nBdSRjHScGMVjGdcScwLgqQqphNqNDqBBQW\nzzQClvtttrwqrwgCwp\nPQJTzvJJTtJQlvQftmfdmdmGRcSdcMSGdS\nPjPwvwlfGlGCGwppWBsWQVVQnpCQ\nFRSHzMJdrfRnpmpWmp\nTzJTMNNrHLJfTJccvjqwwgGLZLLc\ndCpjsGvcsLvszTrRRlRrDJ\nhPNqqpbhFlrDnPrRnz\nSBbphFNtLvvSfLfc\nzTFnnZzqrjFVnZTrtwMDptbpMwMMBDzb\nJGPCjWGgJjPWGJjNchmbBRpNNDsRsNBbbRBppD\nJWCggWSCmhCmvWHTQrTQfHjdVlQZll\nWWbrmZjbmjpbWSmcWHSbLddwvDggLFDhFrRlFFDw\nqPMVzTPQVfzvVzBQTMtRswwlDdhDghhsfLwdhs\nMMMPQBPJPzCVzvzQVtBJJMzCbmSZNWbSccHHmmZpWSZGZS\nBgmMgjlBMjHPssBnwphtFwhSVVmGGwFS\nrfCqQQrfvCQNRqCnCthFVSGSGtVS\nTrzLNvWfQvrWQZNWsnHsssBBcZBMsDdZ\nCtJCddDHDDPGHCdNVLPBdLDbbGFbwZSsSSZrfFFrGbGlsb\ngvjpnvhRphjmpmSfsTSnPZZrZfrZ\nghRRRhQgWvmPWtHHtLdMtH\nqNgQgNgNQFVbqVQDMRZMDRBHHJHRFh\njnTTPzWZCzrWzGRHMrhmBvJBSBBM\njtCGWnCWPTsTzlctWPtqLbwqZggNwgLbwdVq\nDJgJDgFqCGlhFDGDCWhqCwRfpbcpbsgsVscRpwbwws\nSmLZQmMVvLbsbssNQsQN\ntHZMLZZZMVtmnMHWDHjWlGhFDqCl\nPZRmjlDBWRBWBQMdQQBDPRhfzSSTTnfFnfzFpVFjfSVFgS\nwwHCHLtLwnzNHVTZZV\nrJLtZtLZcCrvJwcbrGLvrcDQDhPldhGdmmPRQhhRBlPM\nzShzVhbqlbpbRNRscBNwCc\nDfmFWnfmnMFDdngdngvJFHgQQNwNsHHCQCwNTTzRcN\nFFFZMWWZZJMmJJMFFpztlPtSllLVphttGZ\ngWhGwmwQGhVwGzBMnDFmdmDDLbCnLn\ntRRHNHHlPHPfPltllNNNRsVPFrTrTbrMMTTDMdLFCrFdPP\nHvRvvHNpNvfqSshwqwVQZWhggwhw\njDDRDVqNsRMMVFjFbtzpBlpllCBlhSLHSStH\nJJWZZTwWcmZCzQShHhzhhm\nwTrgvTwccvdcfvJvWJrvJTNzNMfGNNGjFRsjNDbfVzDs\ncVWPPThWctcFRdQpzLTzBL\nNNCqwNSrsDqNSSgLgffDCNpBdRvvdpmHRQvBdBqvdzBB\nCgbffsDJsSsNgbgJrlcGhGtjnWcJPncjWL\nRwwTGRjGlwWNgjgfQVNmjj\nHJPdLhLzhbtbdLPLbHHJLdcfBffBgrFQVZgzvgBfvrvvmr\ncmchnhtPqLSJJbdPLntlsDGCTWWDsDRwWWWTwS\nDDDBsPGPbwhDcDcj\nrgfNgCmHMvrrttvtfmNLgrLcQQJTJhcnjTQHcZlwlQHnwj\nwgfLmgFgWdBFzSVR\nRwsbssJjnbJwwsGPPdDLfTDLLLWvWNDGDpWD\nzVtHqzHHVcBQTdpSgvQDSpTS\nHHhFCdrrHchmrhcmwsjZwRwMJlshJJJZ\nBdQgtJMLBqshLfNhbccfFhffNc\nDnVWvvpvWvzrpwRWDbTvbTjTFGlfFSfNSFHGmFGNFmnPlmFS\nwDRDZzVWVjrRVjzrQgQdbgCZLCsCQtbd\njQSgWjQmFFvHmjHWVVpZbGlbGlfGpbfGGWpf\nzBPtTqzPBcdwrzPJwqCztTNvCGpbGfDnfhZpNGGZGZbG\nMTzPtcrcwzTttdBJwPvFVHjMLFHQMgsSLHvs\nBHtjmmTtmDtHZjMMdNzCzCWcWZsZdsZs\nJRRJVPLwQJrVMPJLVVwChpRRhcdzcNzhzChNdW\nSrFwfGwGSnnGPlBMDtjbllggBn\nrsMDTrgsBNBgMgDBhfhDghrtcRWJttcmGRWLGQQLJRrqRL\nlnVVjvPbwpndvVwlVCjVwtLcsmLLqWWtttlsmGGcmm\nSsjVSCZbgTBTfNZH\nwlddvlldlBzqSmStdqmmngwgDCgrpMpgCngrMCfn\njVLTHGRHjjvPGcDrfNMbnpngVbpf\nRJRTTRZJLRJQZcGGHLhHvTdmBFWFtWzqdWWQlFzqBFWt\nHDgZHpZSDpBQdRpHHRsDBNNzTvfTQqcqNNTqTqPQvl\nrnMFFMFJwmNgvmfTzcfq\nCWMtjCWMCCLWrWVWJwBbpgtRZSHbddZHBDDb\nhjLWPZJpZptwJghSfgHTMTgRMR\nlCcrblcnlzqDsvbfffTRSMnffjHTgj\nsqscDcblGDDpWPwFGwjLLp\nplQtRqRlGpPPPLZtmtpttRtJjHJvddLTHrLHJJjbdHvrrN\ngcMgTBWBCTczjnvNznnbgH\nFBhSwsWFWDhQZQqTTZllsT\nWsVttWDbvbtRjDVtDbDbJjrjFpCCcBrSCwrrjwww\nnzHqGNMfHqMMwJLLsNBrNcBS\nqgnHflgzHlqlZzhnzssfzbZtmVvmTQvQdTtbbDbTvv\nRtQCBbJDFhJtQtZtCbMnVnPVfGPpJVJWWmwJPG\nNsczRsrdrNrjlcSTGGnwmGmnmVPWwmVT\nSHcljRHzNHHjSHcrgNslcczgZtDgMDBCvQQbhDBCbFBCFCCF\ndRTFRJTRTgJzSSJmzJfN\njLbQllLvvvrQlLQBrvQmFBGzCFtPSMmSGCtPPF\nQjsvDvqlvrQQvnsLjDWghTwZWcccFdRVpc\nbWnDbMJMFbhZSfngpfpd\nHjcrlvjRjrjlLqTqpwQgpfSQgghZgvgf\nTHNTjClLHDCCpWmbtC\nNpBNsnFMLBcPMZccbQGblWRgGZmbRWdW\nCqrJTHCvDfrfwwJHRBvRGvgQmvmlWtlR\nrHzJJJDDwJCqjHTwBLccMpFhshnSSsMz\nbqVqqrDMpLFbLpJJQDMjbpZndwsvwHHswvnvnnZslG\nhgfRghhBWgfzBgZvncvcGvWrnZsG\nfTSSSBtTzhPththRrrCBzVLqMMJQjVJJCLjFjjqjJq\npQlCJQjVvVGGnjNqFbFP\nwTqtmSmTBhstmstTmWRSdLdfPdNGnhGhdPNnNnrN\nmzTZBwWTcSTsTmZRRmzTsVMMVMQMDJgVqqMgcJlVDl\nvVwCqdCDvMrlDJCqrDMrPdTFWwZNbRcbmZWQbRQZWZcQWm\nfGSfhfjHGBlhSpRmpZFZNbQWjj\nntGlgSSGgStVqdMCDCJnqd\nTlTRCrhCpmnCRtMRRWbvMBfwBB\nDVdQcsccdczbtQBWvmQBBW\nPgHPNcmcqqTphlHTCGrT\nGVLCBmdLVtlrmqGCqrTCGnjGSfwzNfPzfNNGzSSw\nrMDcWMJsDWRvRcsZPwpJpSzjwwfFjfjJ\nhcsscQQsDcWbHbmLgTbVgTmlrbgq\nVwJndPThQQjdvbrb\nBHlzFFjlZGBBlZBDFSmllfGggLNbNvggtCgNrLQLbfvbfg\nSHSHHlZSGZSDHGzHssGmnjhjphJPPpPwnsJRwPMM\nQFFGpfGtwgtjwvpwpGGjjBHgqzNNllHRlRllNdBRJl\nSWVDWVrDhnWhnqVnWPrhcSldBJPsRMMdPHsRdJHRMMBP\nSWZZcmnnZnSCLZDZftFFCvbfTTqTCTQF\nTvZBTFZpshPggBNN\nbbctWQDjcnwtwDDDllQzLfgsPzqgrsTPLshWfP\nbwRbQRnmDwtbTjmwRJQRQVZHpCpHVdvpMHZZMHZJvd\nCfqzMCGvGqNrCFFNwcMBbnnbbtBbDStw\nsJjsTjvVjLVlLLbtHSnSSwQsDBSH\ndJlvLVgJTRVPWWjjggCzhNGzzGZdhqrdmqrq\nzGSHWGjzpRsWVfsNwZ\nBmPCLzPlJBBvQmLFQrwwrVtfqtqZtsvwfR\nhLLzlmmBPFFLJMlmgDTMgdSGbDgGHbSp\nppngVjZwNZwwVJjjnnVVJJJpLrsgmsmrbctsLcLmscDggDsL\nvHvRzRPvQPPRqPTlffRTrssbtqmmtbcMcNrDrrtD\nCHWdWzWQzdChNVZjZZwB\nLjVFhwjbFggMdVggTVMNRWRGWqQWtRNWGlQW\nDCzCpJppzJJDScFBzvBGnRWGWrNRQNNWGtNGqB\nvFJSzDSszzzccSZHzDDJmvSJhVwLjfVTPZLfwLgjThLZwhVh\nFMSSNScRlSGzfqWbqqcpWBhpmW\nTrPrnQNnJwtQpmmpmbbL\nvwvCsJZZnnwVnVHsfNjDRgSjRzgRjZjN\nCRhsghlqlvjhPslQdrMndMTWdPnTMJ\ntcDNFDpDSDwDtHrSDwDtFmtpJzWTnzQJpzQGnGWQMWnnTGBd\nZfFSmSfwcwcScZHtVRhRCqLqCfrVrrfg\nvHPsBzvRvhCzbwbnjHLVmVbW\nggdlpZNdZdflWTGpVMjgVbqjVmngnVbr\nlNpfllFGpZTtclDZzSWQFCWWzChPzQhS\nddbJQGsgJcQccCjjTC\nDDflHqNNNjCCPNbT\nLFpLzfzqHzLbLvtndJhs\nmHtbGdwzmtHZrtrHtHGwrmtcFgBFRsBFcdglRsfsdgJBNN\nWvpSVjCCTCSPVDqTVjLqJwWRcWsNlgBcBlfWBRfl\nqDwjvqjVTDDLSPpnjqVnzbhbZbrGMmHzbMHHhh\ncMrrVsdtCdVtwqdHgLjPLFLfcfZZJB\nzGSlGGvpDWWvbSSNTggZPPTTJLZPjPzJ\nbNQDWbjlGSGNnWGblbQDQCmdqhsMtrsVnCrtnwdnCt\nWZSnCCMMdMMMSJMSVZmmFqVjqjVwVjjc\nzvzTlQQQQTNGbmQTjqBjcwwwGgBHGwqB\nmthbtmlPhTrNzNhhPLzlPzWfWPJdRCWspPWSSRsWSnJp\nnmqsjNFqNLcVlPNvRdvPPv\nMrggHrMpbtTpgpDptlQRJJldVRPBsDQvvV\ngrgrTzfTGSFGsGSCLc\nhjdjCfQCLdQcWMfDDQhLsLCjqvVqzzZZprBFbzVdvVnqnBbd\nNTglmJGJRnRZVrVvFr\ngPGgPJSTPJtHncnjWDhDMDhP\nZWWqBqqmgFFDMTTWDMwwcW\nddGPpJQPprRSCrQzJPJGjHDvssMTDZDwcHhMsHsMHS\nzNRGrCRRCjPpdGQJNglqntnbtgNBZtFqtt\npslQSspQrqHfgMRl\ntJBTjsTLGMBGMFMg\ntDwnhZdnLdZDwczzcPvsVvVW\nnZnMSDnGtnzmzWZZcMmgMcHwFFVHhHwbHsVrwQhVhF\ndlTqNqfTjdJflCppCQQRHVwFRJrvJsFVQs\nLNTqBLPprpLLzWtSnMZBMDGm\nPBjlPvvcJlJzwqjnnjLnwm\ndMfdpVfVhGVfVpVTtWWbzwngLMLwMRCmgmLMmsww\nSWppdTrprThhrGVztcrllrQJZNlBZlrNZB\nTTrNcjGNWDdDPDpPjDPNrbmFPfQSFmfSMCmCfSmSSR\nsshhqshzBVnzHgwCMbCwcfmRFmRMbm\ngHVVnBzJcBgVhtZZtgJhpGrGNpvDpddvprNGrlTJ"));
}

/*
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
*/
