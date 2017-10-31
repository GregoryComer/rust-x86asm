use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprold_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 114, 200, 118], OperandSize::Dword)
}

#[test]
fn vprold_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 206785689, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 114, 140, 222, 153, 76, 83, 12, 2], OperandSize::Dword)
}

#[test]
fn vprold_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1531170785, Some(OperandSize::Dword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 158, 114, 12, 181, 225, 207, 67, 91, 1], OperandSize::Dword)
}

#[test]
fn vprold_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 131, 114, 205, 62], OperandSize::Qword)
}

#[test]
fn vprold_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM29)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 21, 132, 114, 10, 4], OperandSize::Qword)
}

#[test]
fn vprold_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 53, 149, 114, 12, 198, 70], OperandSize::Qword)
}

#[test]
fn vprold_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 114, 201, 93], OperandSize::Dword)
}

#[test]
fn vprold_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 1258266649, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 171, 114, 142, 25, 160, 255, 74, 80], OperandSize::Dword)
}

#[test]
fn vprold_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 784313649, Some(OperandSize::Dword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 190, 114, 140, 74, 49, 173, 191, 46, 13], OperandSize::Dword)
}

#[test]
fn vprold_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 5, 175, 114, 201, 125], OperandSize::Qword)
}

#[test]
fn vprold_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 161, 114, 12, 121, 26], OperandSize::Qword)
}

#[test]
fn vprold_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1861053948, Some(OperandSize::Dword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 189, 114, 12, 181, 252, 109, 237, 110, 101], OperandSize::Qword)
}

#[test]
fn vprold_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 114, 206, 61], OperandSize::Dword)
}

#[test]
fn vprold_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 41053943, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 114, 140, 119, 247, 110, 114, 2, 30], OperandSize::Dword)
}

#[test]
fn vprold_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1927075945, Some(OperandSize::Dword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 222, 114, 12, 189, 105, 216, 220, 114, 46], OperandSize::Dword)
}

#[test]
fn vprold_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 77, 201, 114, 202, 7], OperandSize::Qword)
}

#[test]
fn vprold_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM31)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 194, 114, 9, 9], OperandSize::Qword)
}

#[test]
fn vprold_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 481738522, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 210, 114, 140, 135, 26, 191, 182, 28, 106], OperandSize::Qword)
}

