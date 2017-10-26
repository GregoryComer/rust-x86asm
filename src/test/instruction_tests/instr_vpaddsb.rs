use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 215], OperandSize::Dword)
}

#[test]
fn vpaddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 548673168, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 236, 4, 125, 144, 22, 180, 32], OperandSize::Dword)
}

#[test]
fn vpaddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 236, 200], OperandSize::Qword)
}

#[test]
fn vpaddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1553675759, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 236, 151, 239, 53, 155, 92], OperandSize::Qword)
}

#[test]
fn vpaddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 236, 251], OperandSize::Dword)
}

#[test]
fn vpaddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 444916196, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 236, 156, 241, 228, 225, 132, 26], OperandSize::Dword)
}

#[test]
fn vpaddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 236, 224], OperandSize::Qword)
}

#[test]
fn vpaddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 236, 25], OperandSize::Qword)
}

#[test]
fn vpaddsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 236, 253], OperandSize::Dword)
}

#[test]
fn vpaddsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1390455230, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 236, 52, 149, 190, 169, 224, 82], OperandSize::Dword)
}

#[test]
fn vpaddsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 21, 138, 236, 227], OperandSize::Qword)
}

#[test]
fn vpaddsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 419178113, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 117, 129, 236, 140, 66, 129, 38, 252, 24], OperandSize::Qword)
}

#[test]
fn vpaddsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 236, 201], OperandSize::Dword)
}

#[test]
fn vpaddsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 421027218, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 236, 156, 187, 146, 93, 24, 25], OperandSize::Dword)
}

#[test]
fn vpaddsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 45, 162, 236, 206], OperandSize::Qword)
}

#[test]
fn vpaddsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 2088782312, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 21, 170, 236, 132, 184, 232, 73, 128, 124], OperandSize::Qword)
}

#[test]
fn vpaddsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 236, 218], OperandSize::Dword)
}

#[test]
fn vpaddsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDI, 2062100063, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 236, 143, 95, 38, 233, 122], OperandSize::Dword)
}

#[test]
fn vpaddsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 197, 236, 211], OperandSize::Qword)
}

#[test]
fn vpaddsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1003701601, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 101, 199, 236, 20, 149, 97, 69, 211, 59], OperandSize::Qword)
}

