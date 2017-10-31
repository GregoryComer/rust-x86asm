use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 224, 224], OperandSize::Dword)
}

#[test]
fn vpavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 224, 60, 199], OperandSize::Dword)
}

#[test]
fn vpavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 224, 250], OperandSize::Qword)
}

#[test]
fn vpavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1266528110, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 224, 44, 205, 110, 175, 125, 75], OperandSize::Qword)
}

#[test]
fn vpavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 224, 215], OperandSize::Dword)
}

#[test]
fn vpavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 106444470, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 224, 148, 153, 182, 54, 88, 6], OperandSize::Dword)
}

#[test]
fn vpavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 224, 195], OperandSize::Qword)
}

#[test]
fn vpavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDX, 1735249760, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 224, 138, 96, 207, 109, 103], OperandSize::Qword)
}

#[test]
fn vpavgb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 224, 251], OperandSize::Dword)
}

#[test]
fn vpavgb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1756424942, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 224, 159, 238, 234, 176, 104], OperandSize::Dword)
}

#[test]
fn vpavgb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 77, 131, 224, 235], OperandSize::Qword)
}

#[test]
fn vpavgb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 421540655, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 53, 137, 224, 12, 245, 47, 51, 32, 25], OperandSize::Qword)
}

#[test]
fn vpavgb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 174, 224, 242], OperandSize::Dword)
}

#[test]
fn vpavgb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1272175953, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 224, 140, 241, 81, 221, 211, 75], OperandSize::Dword)
}

#[test]
fn vpavgb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 53, 164, 224, 195], OperandSize::Qword)
}

#[test]
fn vpavgb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 169351668, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 174, 224, 44, 221, 244, 25, 24, 10], OperandSize::Qword)
}

#[test]
fn vpavgb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 224, 252], OperandSize::Dword)
}

#[test]
fn vpavgb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 620219003, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 224, 140, 146, 123, 202, 247, 36], OperandSize::Dword)
}

#[test]
fn vpavgb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 53, 199, 224, 241], OperandSize::Qword)
}

#[test]
fn vpavgb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1202178403, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 117, 204, 224, 172, 177, 99, 201, 167, 71], OperandSize::Qword)
}

