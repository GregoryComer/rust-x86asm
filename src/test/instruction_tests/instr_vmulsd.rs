use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 89, 251], OperandSize::Dword)
}

#[test]
fn vmulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1894072399, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 89, 132, 222, 79, 64, 229, 112], OperandSize::Dword)
}

#[test]
fn vmulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 89, 241], OperandSize::Qword)
}

#[test]
fn vmulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 498704300, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 89, 164, 121, 172, 159, 185, 29], OperandSize::Qword)
}

#[test]
fn vmulsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 207, 250, 89, 193], OperandSize::Dword)
}

#[test]
fn vmulsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 622253272, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 215, 143, 89, 28, 245, 216, 212, 22, 37], OperandSize::Dword)
}

#[test]
fn vmulsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 175, 146, 89, 229], OperandSize::Qword)
}

#[test]
fn vmulsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RAX, 700169623, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 215, 142, 89, 144, 151, 189, 187, 41], OperandSize::Qword)
}

