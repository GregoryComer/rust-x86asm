use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 93, 238], OperandSize::Dword)
}

#[test]
fn vminss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 438888582, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 93, 132, 121, 134, 232, 40, 26], OperandSize::Dword)
}

#[test]
fn vminss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 93, 237], OperandSize::Qword)
}

#[test]
fn vminss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 467605619, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 93, 172, 198, 115, 24, 223, 27], OperandSize::Qword)
}

#[test]
fn vminss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 70, 159, 93, 202], OperandSize::Dword)
}

#[test]
fn vminss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 70, 142, 93, 28, 222], OperandSize::Dword)
}

#[test]
fn vminss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 30, 151, 93, 212], OperandSize::Qword)
}

#[test]
fn vminss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 780260846, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 70, 138, 93, 146, 238, 213, 129, 46], OperandSize::Qword)
}

