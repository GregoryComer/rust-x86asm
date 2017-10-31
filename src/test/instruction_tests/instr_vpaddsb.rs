use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 236, 234], OperandSize::Dword)
}

#[test]
fn vpaddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 476871584, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 130, 160, 123, 108, 28], OperandSize::Dword)
}

#[test]
fn vpaddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 236, 206], OperandSize::Qword)
}

#[test]
fn vpaddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 236, 25], OperandSize::Qword)
}

#[test]
fn vpaddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 236, 222], OperandSize::Dword)
}

#[test]
fn vpaddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 660768528, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 236, 171, 16, 135, 98, 39], OperandSize::Dword)
}

#[test]
fn vpaddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 236, 245], OperandSize::Qword)
}

#[test]
fn vpaddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 236, 28, 66], OperandSize::Qword)
}

#[test]
fn vpaddsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 236, 197], OperandSize::Dword)
}

#[test]
fn vpaddsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 799400599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 236, 12, 205, 151, 226, 165, 47], OperandSize::Dword)
}

#[test]
fn vpaddsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 77, 135, 236, 223], OperandSize::Qword)
}

#[test]
fn vpaddsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RSI, 1681940849, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 61, 135, 236, 142, 113, 97, 64, 100], OperandSize::Qword)
}

#[test]
fn vpaddsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 236, 248], OperandSize::Dword)
}

#[test]
fn vpaddsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 236, 52, 112], OperandSize::Dword)
}

#[test]
fn vpaddsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 69, 173, 236, 243], OperandSize::Qword)
}

#[test]
fn vpaddsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RBX, 960409113, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 45, 166, 236, 187, 25, 174, 62, 57], OperandSize::Qword)
}

#[test]
fn vpaddsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 236, 226], OperandSize::Dword)
}

#[test]
fn vpaddsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 236, 20, 114], OperandSize::Dword)
}

#[test]
fn vpaddsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 5, 195, 236, 234], OperandSize::Qword)
}

#[test]
fn vpaddsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 93, 203, 236, 20, 190], OperandSize::Qword)
}

