use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 219, 221], OperandSize::Dword)
}

#[test]
fn vpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1515072087, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 219, 60, 197, 87, 42, 78, 90], OperandSize::Dword)
}

#[test]
fn vpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 914661121, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 154, 219, 151, 1, 159, 132, 54], OperandSize::Dword)
}

#[test]
fn vpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 21, 137, 219, 223], OperandSize::Qword)
}

#[test]
fn vpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1099341517, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 117, 132, 219, 36, 245, 205, 158, 134, 65], OperandSize::Qword)
}

#[test]
fn vpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RAX, 1162989081, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 5, 155, 219, 160, 25, 206, 81, 69], OperandSize::Qword)
}

#[test]
fn vpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 219, 229], OperandSize::Dword)
}

#[test]
fn vpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 1861221124, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 219, 160, 4, 251, 239, 110], OperandSize::Dword)
}

#[test]
fn vpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1261860259, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 191, 219, 164, 201, 163, 117, 54, 75], OperandSize::Dword)
}

#[test]
fn vpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 13, 161, 219, 201], OperandSize::Qword)
}

#[test]
fn vpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 117, 161, 219, 12, 134], OperandSize::Qword)
}

#[test]
fn vpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 117, 190, 219, 52, 158], OperandSize::Qword)
}

#[test]
fn vpandd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 219, 244], OperandSize::Dword)
}

#[test]
fn vpandd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 142833928, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 219, 4, 157, 8, 121, 131, 8], OperandSize::Dword)
}

#[test]
fn vpandd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 220, 219, 30], OperandSize::Dword)
}

#[test]
fn vpandd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 117, 207, 219, 198], OperandSize::Qword)
}

#[test]
fn vpandd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RAX, 94798269, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 37, 194, 219, 152, 189, 129, 166, 5], OperandSize::Qword)
}

#[test]
fn vpandd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 405453806, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 125, 215, 219, 36, 221, 238, 187, 42, 24], OperandSize::Qword)
}

