use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 94, 235], OperandSize::Dword)
}

#[test]
fn vdivsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 2077723787, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 94, 143, 139, 140, 215, 123], OperandSize::Dword)
}

#[test]
fn vdivsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 94, 230], OperandSize::Qword)
}

#[test]
fn vdivsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 94, 49], OperandSize::Qword)
}

#[test]
fn vdivsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 158, 94, 212], OperandSize::Dword)
}

#[test]
fn vdivsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1551482291, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 239, 143, 94, 147, 179, 189, 121, 92], OperandSize::Dword)
}

#[test]
fn vdivsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 135, 177, 94, 249], OperandSize::Qword)
}

#[test]
fn vdivsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 167, 135, 94, 31], OperandSize::Qword)
}

