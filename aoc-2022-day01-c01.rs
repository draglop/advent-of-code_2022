// https://adventofcode.com/2022/day/1
// (part 1)

fn find_most_calories(list: &str) -> i32 {
    let mut most: i32 = 0;

    let lines: Vec<&str> = list.split('\n').collect();
    let mut current: i32 = 0;
    for line in lines {
        if line.len() == 0 {
            // elf separator
            if current > most {
                most = current;
            }
            current = 0;
        } else {
            let number = line.parse::<i32>();
            if !number.is_ok() {
                panic!("{} is not a number", line);
            }
            current += number.unwrap();
        }
    }
    if current > most {
        most = current;
    }

    return most;
}

fn main() {
    // example
    assert_eq!(
        24000,
        find_most_calories("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")
    );
    // user puzzle input
    assert_eq!(
        66719,
        find_most_calories("5686\n2211\n1513\n7036\n5196\n10274\n2967\n2551\n\n5942\n5827\n2514\n4024\n\n9857\n13173\n13071\n17540\n\n8264\n2725\n6163\n3589\n4223\n8568\n3009\n8662\n1376\n\n1270\n5911\n6619\n4174\n1153\n7989\n2435\n3577\n1086\n3233\n\n16008\n16955\n13004\n\n5135\n2622\n7433\n2508\n6498\n6702\n4321\n3999\n5778\n2692\n1523\n\n7310\n1841\n2040\n4938\n6186\n1555\n6107\n2880\n4305\n1270\n8060\n\n8727\n5727\n12263\n14610\n9171\n\n42938\n\n1860\n5190\n3635\n1963\n4026\n4287\n3410\n1670\n6451\n3981\n1281\n1225\n6461\n3709\n\n5058\n5947\n1528\n10692\n11369\n12969\n\n7290\n4303\n6729\n3143\n4367\n2374\n2881\n1956\n3864\n6972\n3263\n6477\n\n1507\n5380\n5788\n4267\n2937\n1139\n1529\n3569\n2081\n3857\n4758\n2987\n2080\n2219\n1794\n\n2735\n8620\n3851\n6929\n3448\n6822\n5281\n7563\n4385\n7865\n\n2160\n3457\n2468\n6635\n3777\n6423\n3603\n7088\n3747\n4105\n3059\n3236\n\n14116\n4368\n18640\n1213\n\n11151\n11231\n10021\n12658\n\n1899\n4539\n4194\n6465\n6112\n5642\n4383\n1999\n1089\n6234\n5598\n2817\n1435\n4993\n\n16336\n\n6654\n6290\n2606\n1222\n4484\n4007\n5560\n4120\n2672\n1716\n3431\n6629\n3534\n\n1428\n1117\n4014\n4237\n3441\n1564\n2492\n3999\n1975\n1689\n5245\n2862\n4494\n2527\n\n4460\n2987\n4546\n2783\n6449\n4539\n5181\n6599\n5812\n6772\n5509\n2650\n3553\n\n1375\n7067\n8702\n4222\n1146\n2016\n7478\n5190\n5963\n4371\n\n6564\n1322\n4502\n1932\n1589\n3294\n7798\n7951\n1151\n\n9588\n3857\n6452\n5841\n4617\n7876\n\n3290\n3008\n8186\n15610\n11186\n\n2275\n4886\n7045\n1983\n6616\n7320\n6840\n1071\n5123\n6501\n4227\n5072\n\n1553\n3815\n3787\n3013\n2284\n4355\n1161\n4593\n4336\n2256\n2382\n5055\n3923\n5132\n\n5379\n1987\n4347\n5061\n5045\n6672\n1153\n5484\n6456\n6824\n1588\n\n14163\n16215\n1954\n9164\n\n7350\n5067\n2170\n7769\n5656\n1661\n7576\n7416\n6151\n5020\n5729\n\n32348\n20553\n\n15878\n7366\n13034\n3482\n8740\n\n16102\n8408\n16886\n\n8592\n9925\n9337\n4966\n5435\n6582\n9328\n6201\n\n3962\n6432\n6527\n5883\n5532\n4407\n2796\n5365\n1840\n7334\n2920\n4086\n\n10664\n5976\n8604\n2827\n10060\n10229\n11492\n\n5451\n3545\n5641\n5779\n7277\n2628\n1250\n1811\n5818\n6112\n3898\n3523\n\n1371\n1946\n5127\n4787\n3784\n1134\n2292\n5031\n5291\n5038\n1637\n1178\n1697\n2475\n4239\n\n5933\n9062\n2975\n5058\n1127\n1918\n5812\n3932\n3434\n\n4523\n10216\n10764\n9355\n1272\n3639\n2747\n2548\n\n5359\n5827\n3677\n7954\n6695\n7177\n7101\n3889\n4736\n5698\n3803\n\n2079\n1641\n1348\n3200\n4035\n1547\n1347\n5528\n3003\n1209\n3457\n3948\n4284\n3396\n2369\n\n1015\n2463\n5926\n4967\n1398\n4356\n2397\n4613\n2909\n3431\n3482\n\n2449\n3480\n2076\n3984\n3030\n2249\n2718\n4658\n4959\n3047\n5571\n3218\n\n6923\n7052\n3144\n6109\n7223\n5610\n2834\n5771\n1290\n2216\n7407\n1853\n\n16073\n5598\n2369\n3785\n15890\n\n5797\n1682\n6510\n8054\n\n6160\n6406\n3893\n3531\n3712\n2649\n3254\n2373\n6053\n6616\n4503\n3573\n5476\n\n11525\n7276\n12639\n2181\n3772\n\n15209\n18108\n14012\n13754\n\n5564\n9463\n10638\n9542\n2412\n4357\n10507\n\n4852\n3004\n5131\n4503\n6019\n5520\n1506\n1493\n2572\n2354\n4924\n4807\n4789\n4351\n3845\n\n40493\n\n39589\n\n4519\n6704\n4962\n9477\n1208\n10288\n3798\n5526\n\n2682\n13451\n10034\n2545\n4452\n7412\n\n32413\n5857\n\n6163\n11407\n8780\n5351\n2741\n9916\n10314\n\n6667\n35063\n\n12333\n22183\n23309\n\n2949\n5861\n4380\n3457\n1019\n6456\n4615\n4039\n6861\n2787\n6200\n4583\n3176\n\n4526\n7517\n8417\n7109\n8327\n6758\n3958\n\n12106\n13851\n16017\n7920\n13186\n\n1444\n5154\n10869\n6868\n5040\n11545\n9097\n\n1244\n4683\n8043\n9237\n4766\n12954\n\n7405\n2364\n7117\n6204\n1116\n2605\n4528\n1003\n4004\n7295\n6348\n\n7835\n6402\n8314\n1188\n6044\n7310\n4614\n7415\n1987\n\n2847\n5827\n5559\n4660\n3528\n1034\n5672\n5868\n4208\n2761\n4184\n4177\n\n51072\n\n4750\n8432\n4449\n4830\n2616\n1373\n9126\n9834\n\n36394\n26194\n\n1227\n13357\n16812\n9012\n\n18457\n20244\n10274\n\n3070\n4738\n5567\n7328\n7028\n4186\n1472\n7041\n4009\n4126\n6411\n1744\n\n2004\n10907\n7451\n4526\n8140\n6890\n\n4716\n3610\n2470\n1736\n2892\n5414\n2949\n5628\n1411\n2775\n2604\n4958\n5322\n1891\n4458\n\n3784\n11731\n8898\n12113\n13296\n10644\n\n3783\n1713\n1379\n7704\n5959\n3955\n9411\n7517\n1514\n\n2962\n2602\n1501\n1045\n1479\n5280\n4134\n5198\n4167\n5033\n5241\n1822\n1567\n3668\n2178\n\n5060\n6325\n2962\n1971\n5843\n4140\n6175\n3161\n1466\n6243\n2931\n3443\n4895\n4249\n\n66339\n\n6137\n4851\n3798\n9698\n9988\n5932\n10712\n7545\n\n5195\n3263\n1797\n2538\n1837\n2693\n5952\n5333\n3238\n3717\n3950\n4183\n3355\n1280\n5517\n\n5045\n4841\n2418\n4492\n3604\n4101\n2854\n5791\n2241\n4027\n1901\n3826\n5477\n5254\n3898\n\n53971\n\n1540\n6936\n1328\n5334\n2123\n4618\n6537\n2609\n5653\n7098\n3316\n\n8590\n5386\n8241\n6987\n3924\n6265\n1818\n9420\n\n5772\n4715\n1295\n2652\n4765\n7480\n7577\n5010\n7227\n6538\n4707\n\n1952\n8437\n25310\n\n15936\n11883\n8696\n10347\n\n8198\n1014\n1004\n10270\n6566\n9284\n10468\n4297\n\n15267\n17919\n16656\n2900\n\n3359\n5649\n1962\n5618\n1020\n5969\n7258\n7309\n2926\n3786\n2299\n6614\n\n31552\n33941\n\n5132\n6793\n3625\n5910\n7575\n2603\n8697\n5588\n3027\n3054\n\n2876\n4464\n2819\n7178\n5485\n6972\n6319\n1102\n5341\n3281\n6218\n6124\n\n9471\n5155\n1390\n9056\n1916\n3727\n3844\n6099\n\n6751\n4444\n10612\n3560\n6783\n7374\n3158\n\n11589\n2594\n7521\n8873\n3482\n9678\n\n2023\n2438\n1459\n5165\n5927\n4658\n3113\n1489\n2826\n5113\n3540\n4479\n5627\n1006\n4791\n\n6965\n9145\n5658\n13566\n4225\n3005\n\n8233\n9556\n6895\n7522\n1053\n4909\n4475\n8203\n\n6371\n6845\n4501\n8168\n8605\n7805\n4562\n1825\n7172\n8205\n\n5165\n1183\n2962\n6412\n3125\n1423\n5257\n1541\n2680\n1459\n1834\n1652\n4339\n\n2552\n7801\n15625\n9736\n\n4617\n8744\n4576\n13632\n\n8073\n7400\n8054\n6318\n5631\n6028\n2021\n2856\n1557\n6371\n7764\n\n4259\n2112\n4290\n2650\n6900\n6061\n6765\n2745\n3157\n5283\n5755\n3457\n3872\n\n62600\n\n32544\n11804\n\n1788\n7140\n5592\n12124\n6868\n8209\n2575\n\n2533\n2662\n5275\n1751\n5218\n2712\n4346\n2166\n3709\n5848\n5855\n4637\n1644\n3088\n1907\n\n8172\n1759\n7682\n6871\n3318\n9522\n7511\n6831\n4015\n\n52441\n\n6537\n2908\n1451\n6115\n1954\n1099\n5712\n8426\n\n1236\n7381\n5167\n6563\n7318\n2436\n1325\n2948\n2710\n6319\n2608\n3591\n\n3734\n4626\n1460\n5719\n1715\n1842\n4747\n1875\n2922\n3464\n5489\n5568\n5174\n3365\n\n3662\n2200\n4326\n4968\n4482\n5444\n6657\n5091\n2117\n5027\n5595\n3765\n\n6974\n2450\n7465\n7285\n6168\n7462\n3116\n4750\n4413\n6386\n\n6612\n4050\n11379\n18968\n\n63747\n\n12307\n7764\n11390\n1859\n9217\n5600\n\n7166\n3973\n6159\n6484\n6661\n4646\n5470\n1719\n4798\n2951\n3190\n\n2213\n5373\n2129\n1122\n5100\n6373\n5480\n1418\n4490\n3008\n4265\n2939\n6175\n5050\n\n3364\n2910\n2761\n4320\n3238\n1077\n2253\n4776\n5965\n3933\n1826\n3258\n2282\n2310\n6098\n\n8459\n7811\n11796\n5612\n5306\n1946\n11206\n\n1864\n8864\n3044\n3377\n5829\n1790\n9450\n2676\n7701\n\n3103\n4985\n6899\n1125\n5296\n4143\n1526\n1579\n6668\n1724\n1255\n5107\n3720\n\n13512\n4587\n7594\n14548\n6246\n\n54371\n\n24950\n7799\n\n2613\n1020\n6330\n5597\n5295\n5496\n4732\n1885\n1815\n5758\n4727\n4220\n6374\n6162\n\n9551\n13121\n1684\n2595\n8505\n\n16528\n1799\n11308\n\n1418\n14711\n2147\n4801\n10105\n\n1782\n1685\n1395\n2044\n5382\n5480\n3573\n2435\n4070\n1733\n5930\n6195\n1692\n2888\n\n1688\n1365\n3200\n7047\n7839\n10228\n5983\n9591\n\n5619\n3939\n2610\n4845\n3442\n2821\n2711\n2356\n2747\n1590\n5593\n4981\n5711\n2920\n3485\n\n1987\n8420\n8357\n9771\n1106\n2037\n8409\n5252\n\n4473\n2256\n4295\n3253\n5912\n5230\n5528\n1421\n2026\n1223\n4933\n5041\n5405\n5195\n\n5986\n3142\n4773\n2566\n3557\n2614\n5763\n1462\n1942\n3376\n3863\n1121\n1001\n2506\n1328\n\n8899\n32401\n\n3826\n4732\n9256\n4515\n1866\n6861\n4562\n7148\n\n9787\n4012\n3233\n2360\n1353\n9267\n4474\n\n3933\n6957\n3359\n2793\n2137\n1946\n1787\n3257\n1387\n6363\n3830\n7331\n\n6606\n3538\n1473\n4664\n3248\n2199\n3458\n2771\n3712\n5024\n7635\n\n4009\n2360\n1715\n3068\n5032\n7249\n8362\n1018\n\n2848\n6090\n1763\n4889\n2423\n5758\n2886\n2869\n3108\n6094\n5110\n2166\n2701\n5737\n2866\n\n3125\n7121\n4234\n5931\n3149\n8701\n6860\n6051\n1847\n\n4292\n2447\n5733\n1676\n1638\n4310\n5501\n4375\n4814\n5728\n5735\n2035\n4964\n1828\n\n11862\n5538\n10841\n6768\n5855\n2538\n2995\n\n12807\n11304\n8668\n11295\n2680\n\n7589\n3729\n1859\n5542\n13730\n\n2782\n2929\n7461\n5840\n3916\n3574\n5958\n7601\n6122\n\n3173\n3044\n4904\n1544\n6463\n5239\n1532\n6951\n5903\n3948\n4742\n5825\n6288\n\n6793\n6722\n2365\n8678\n8568\n6098\n4378\n10526\n\n7743\n2658\n8311\n9915\n9120\n6152\n7100\n2698\n\n3476\n5111\n1201\n4971\n3830\n4158\n4172\n2841\n6041\n1082\n3207\n3050\n4469\n1108\n4274\n\n36944\n\n34023\n\n1962\n8656\n6074\n5546\n1960\n5754\n2000\n5672\n2729\n6064\n\n8110\n2537\n4370\n8336\n8927\n3813\n10038\n\n5609\n3904\n4523\n6963\n5864\n6166\n3660\n4891\n6953\n2136\n3276\n1712\n\n5198\n5254\n2456\n2133\n5835\n6961\n4780\n4041\n3036\n7408\n1156\n4275\n\n1475\n2273\n1772\n5900\n5851\n1855\n3375\n5359\n3649\n3862\n6099\n1670\n5600\n4647\n4341\n\n46294\n\n2287\n2354\n13619\n12330\n\n6849\n6447\n2673\n4925\n3479\n2903\n6599\n2637\n1192\n2638\n3227\n2511\n\n4905\n2874\n2714\n5883\n1294\n4703\n1253\n1953\n2612\n3925\n5052\n5528\n5792\n1995\n\n4126\n3937\n4979\n2042\n6663\n4358\n3326\n2671\n4920\n6420\n1173\n6682\n\n6146\n10792\n\n5228\n9530\n2288\n6322\n6413\n8780\n2075\n8491\n8592\n\n6969\n11615\n4852\n13647\n2478\n2086\n\n1406\n6041\n7324\n5281\n1048\n10324\n2467\n9719\n\n1130\n6482\n4859\n6020\n1310\n1177\n5693\n6083\n3293\n2918\n4021\n6944\n\n40367\n\n15216\n23154\n19153\n\n2732\n5987\n3554\n5038\n4885\n3758\n3484\n5554\n6351\n5914\n6207\n6271\n1024\n5960\n\n4697\n5988\n6690\n2995\n6827\n4316\n3337\n7094\n2862\n6290\n\n63498\n\n5589\n3523\n1863\n1700\n4449\n7025\n7054\n3637\n2383\n4719\n5384\n1387\n\n4577\n3062\n6850\n2126\n6193\n2972\n4998\n3929\n5273\n3607\n7216\n\n1426\n6363\n12553\n7710\n7427\n7299\n\n12100\n8643\n6472\n12582\n10330\n12994\n\n1106\n1185\n1573\n6559\n1967\n1086\n4571\n6671\n2747\n2082\n6384\n1095\n6899\n\n22318\n10321\n20543\n\n5025\n10184\n4425\n8082\n5629\n5123\n7509\n3100\n\n11271\n\n5133\n10929\n10907\n3629\n\n39333\n\n15442\n4322\n5391\n10882\n\n1252\n5624\n2407\n1285\n2655\n1530\n5705\n1976\n5795\n5008\n3813\n6850\n2362\n\n9416\n3180\n4462\n9918\n8511\n4608\n7612\n\n3916\n1591\n1388\n1359\n4867\n3931\n1067\n5182\n2090\n2947\n1294\n2085\n3805\n1590\n\n7226\n19778\n11590\n12208\n\n3548\n1990\n2859\n4534\n2179\n1744\n1306\n5906\n3215\n3481\n2609\n2419\n4632\n1157\n2905\n\n18058\n4750\n\n49356\n\n16871\n8564\n9745\n\n1053\n7954\n7528\n6434\n6002\n3767\n4369\n4096\n6194\n2337\n\n6370\n3509\n2154\n6608\n3095\n2018\n4408\n2043\n5681\n4497\n3804\n6079\n3573\n\n4693\n7043\n2251\n3734\n5938\n4208\n1597\n4259\n2465\n4080\n3073\n\n7024\n1532\n7929\n5973\n6399\n6470\n1448\n1294\n4885\n6496\n7414\n\n6637\n6833\n7369\n2115\n7831\n1481\n2643\n4148\n6127\n2478\n3002\n\n1639\n5157\n2462\n5910\n2454\n4438\n2088\n3383\n5588\n2774\n3770\n2140\n2121\n3549\n1125\n\n14689\n1193\n7130\n14422\n\n5902\n8740\n11007\n2637\n4399\n13932\n\n8542\n\n8006\n3383\n6661\n\n3629\n5891\n4089\n4036\n1894\n3724\n4280\n4668\n7766\n7213\n4984\n\n2121\n4136\n4122\n4981\n3366\n3487\n5660\n6185\n5341\n3040\n1184\n3292\n3104\n2783\n\n4434\n2764\n5501\n2961\n5751\n6443\n7688\n3503\n4029\n3115\n1031\n\n6883\n3437\n8649\n3473\n1330\n1610\n2567\n2166\n\n24926\n18747\n\n5563\n1884\n6674\n5340\n2876\n3261\n5075\n3746\n4940\n3418\n6437\n5463\n\n3014\n6235\n7541\n2502\n7472\n8412\n9054\n3331\n\n8768\n15514\n12115\n8092\n\n3537\n3246\n6697\n1753\n6707\n7686\n4786\n6161\n5616\n\n2253\n1839\n3053\n4429\n2569\n4310\n4188\n5145\n4144\n4740\n3299\n4502\n1495\n1925\n2112\n\n46591\n\n46938\n\n3368\n6572\n1033\n3438\n1798\n6177\n4166\n1909\n4290\n4280\n\n4748\n12999\n13505\n10698\n\n4707\n1446\n2259\n2201\n3459\n1993\n1617\n6531\n3460\n2272\n1754\n6588\n2898\n\n8242\n6533\n8501\n9404\n2286\n1011\n1940\n4199\n4995\n\n11510\n7982\n10005\n15579\n\n3578\n12406\n7940\n11947\n1380\n12643\n\n4574\n2465\n2184\n4976\n3793\n1405\n3976\n5843\n4954\n2814\n1596\n5310\n1758\n4990\n5705\n\n7945\n5108\n9589\n9098\n3039\n8847\n3776\n8315\n5749\n\n26315\n\n5415\n1420\n4067\n5821\n7466\n7027\n7916\n6201\n4556\n2711\n\n25290\n1682\n5357\n\n29197\n35285\n\n1112\n8038\n5132\n8695\n7350\n6903\n1253\n5873\n5274\n3940\n\n6671\n3196\n9273\n2164\n5533\n7340\n5761\n8737\n5184\n\n1281\n2505\n6171\n5617\n1200\n5848\n6105\n4476\n3495\n1808\n5065\n2231\n\n7002\n1749\n13548\n\n3303\n6583\n3171\n3051\n1036\n7790\n7159\n4326\n4447\n7013\n\n5551\n3972\n3022\n5275\n2300\n5675\n2422\n2813\n3501\n3537\n2440\n3393\n5644\n3351\n2454\n\n5692\n4309\n4409\n1967\n2068\n6467\n6315\n8051\n6237\n8069\n2213\n\n42975\n\n11367\n14938\n7848\n15849\n1867\n\n2803\n3757\n4045\n1854\n5027\n3637\n5425\n3113\n4754\n1822\n1086\n1024\n1890\n3692\n\n4391\n13299\n9709\n4887\n8221\n7477\n\n1104\n3085\n1590\n4909\n1787\n4197\n3948\n4187\n1126\n3158\n1919\n4529\n1791\n1510\n5279\n")
    );
}

/*
--- Day 1: Calorie Counting ---

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

This list represents the Calories of the food carried by five Elves:

    The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    The second Elf is carrying one food item with 4000 Calories.
    The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
*/
