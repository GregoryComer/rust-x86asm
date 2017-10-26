use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 247], OperandSize::Dword)
}

#[test]
fn vpabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 52, 70], OperandSize::Dword)
}

#[test]
fn vpabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 209], OperandSize::Qword)
}

#[test]
fn vpabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 47], OperandSize::Qword)
}

#[test]
fn vpabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 236], OperandSize::Dword)
}

#[test]
fn vpabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 4, 67], OperandSize::Dword)
}

#[test]
fn vpabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 221], OperandSize::Qword)
}

#[test]
fn vpabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 2024381866, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 140, 147, 170, 157, 169, 120], OperandSize::Qword)
}

#[test]
fn vpabsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 29, 226], OperandSize::Dword)
}

#[test]
fn vpabsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 907244333, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 29, 132, 146, 45, 115, 19, 54], OperandSize::Dword)
}

#[test]
fn vpabsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 138, 29, 215], OperandSize::Qword)
}

#[test]
fn vpabsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 324421814, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 29, 156, 208, 182, 72, 86, 19], OperandSize::Qword)
}

