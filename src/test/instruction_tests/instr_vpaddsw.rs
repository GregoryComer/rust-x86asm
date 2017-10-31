use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 237, 206], OperandSize::Dword)
}

#[test]
fn vpaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 486015809, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 237, 156, 191, 65, 3, 248, 28], OperandSize::Dword)
}

#[test]
fn vpaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 237, 220], OperandSize::Qword)
}

#[test]
fn vpaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 237, 55], OperandSize::Qword)
}

#[test]
fn vpaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 237, 227], OperandSize::Dword)
}

#[test]
fn vpaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 237, 4, 75], OperandSize::Dword)
}

#[test]
fn vpaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 237, 232], OperandSize::Qword)
}

#[test]
fn vpaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 237, 52, 184], OperandSize::Qword)
}

#[test]
fn vpaddsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 237, 217], OperandSize::Dword)
}

#[test]
fn vpaddsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1729975222, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 237, 148, 123, 182, 83, 29, 103], OperandSize::Dword)
}

#[test]
fn vpaddsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 13, 141, 237, 242], OperandSize::Qword)
}

#[test]
fn vpaddsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 5, 140, 237, 52, 154], OperandSize::Qword)
}

#[test]
fn vpaddsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 237, 253], OperandSize::Dword)
}

#[test]
fn vpaddsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 804774634, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 237, 44, 253, 234, 226, 247, 47], OperandSize::Dword)
}

#[test]
fn vpaddsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 61, 175, 237, 218], OperandSize::Qword)
}

#[test]
fn vpaddsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 109, 164, 237, 8], OperandSize::Qword)
}

#[test]
fn vpaddsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 201, 237, 215], OperandSize::Dword)
}

#[test]
fn vpaddsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2118230906, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 237, 28, 69, 122, 163, 65, 126], OperandSize::Dword)
}

#[test]
fn vpaddsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 13, 201, 237, 199], OperandSize::Qword)
}

#[test]
fn vpaddsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 698122515, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 69, 207, 237, 12, 205, 19, 129, 156, 41], OperandSize::Qword)
}

