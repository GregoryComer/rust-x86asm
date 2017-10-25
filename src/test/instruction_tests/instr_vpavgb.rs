use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 224, 233], OperandSize::Dword)
}

#[test]
fn vpavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 224, 20, 219], OperandSize::Dword)
}

#[test]
fn vpavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 224, 252], OperandSize::Qword)
}

#[test]
fn vpavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1592707261, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 224, 156, 135, 189, 200, 238, 94], OperandSize::Qword)
}

#[test]
fn vpavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 224, 209], OperandSize::Dword)
}

#[test]
fn vpavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 601494920, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 224, 156, 206, 136, 21, 218, 35], OperandSize::Dword)
}

#[test]
fn vpavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 224, 232], OperandSize::Qword)
}

#[test]
fn vpavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 1056273278, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 224, 138, 126, 115, 245, 62], OperandSize::Qword)
}

#[test]
fn vpavgb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 224, 246], OperandSize::Dword)
}

#[test]
fn vpavgb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 224, 46], OperandSize::Dword)
}

#[test]
fn vpavgb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 132, 224, 223], OperandSize::Qword)
}

#[test]
fn vpavgb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 37, 139, 224, 41], OperandSize::Qword)
}

#[test]
fn vpavgb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 224, 220], OperandSize::Dword)
}

#[test]
fn vpavgb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 266927038, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 224, 44, 181, 190, 251, 232, 15], OperandSize::Dword)
}

#[test]
fn vpavgb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 21, 165, 224, 255], OperandSize::Qword)
}

#[test]
fn vpavgb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 109, 161, 224, 35], OperandSize::Qword)
}

#[test]
fn vpavgb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 224, 199], OperandSize::Dword)
}

#[test]
fn vpavgb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 224, 60, 192], OperandSize::Dword)
}

#[test]
fn vpavgb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 61, 203, 224, 239], OperandSize::Qword)
}

#[test]
fn vpavgb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 198, 224, 20, 131], OperandSize::Qword)
}

