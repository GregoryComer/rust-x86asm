use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 224, 214], OperandSize::Dword)
}

#[test]
fn vpavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 224, 12, 190], OperandSize::Dword)
}

#[test]
fn vpavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 224, 199], OperandSize::Qword)
}

#[test]
fn vpavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 224, 20, 143], OperandSize::Qword)
}

#[test]
fn vpavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 224, 217], OperandSize::Dword)
}

#[test]
fn vpavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 224, 51], OperandSize::Dword)
}

#[test]
fn vpavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 224, 213], OperandSize::Qword)
}

#[test]
fn vpavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 227625109, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 224, 132, 158, 149, 72, 145, 13], OperandSize::Qword)
}

#[test]
fn vpavgb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 224, 211], OperandSize::Dword)
}

#[test]
fn vpavgb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 133674365, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 224, 164, 215, 125, 181, 247, 7], OperandSize::Dword)
}

#[test]
fn vpavgb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 85, 134, 224, 193], OperandSize::Qword)
}

#[test]
fn vpavgb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1103745521, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 13, 140, 224, 60, 253, 241, 209, 201, 65], OperandSize::Qword)
}

#[test]
fn vpavgb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 224, 200], OperandSize::Dword)
}

#[test]
fn vpavgb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 224, 16], OperandSize::Dword)
}

#[test]
fn vpavgb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 37, 169, 224, 226], OperandSize::Qword)
}

#[test]
fn vpavgb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 163, 224, 55], OperandSize::Qword)
}

#[test]
fn vpavgb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 224, 218], OperandSize::Dword)
}

#[test]
fn vpavgb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 224, 4, 78], OperandSize::Dword)
}

#[test]
fn vpavgb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 193, 224, 214], OperandSize::Qword)
}

#[test]
fn vpavgb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 93, 207, 224, 41], OperandSize::Qword)
}

