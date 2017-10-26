use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 95, 252], OperandSize::Dword)
}

#[test]
fn vmaxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 276964749, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 95, 140, 115, 141, 37, 130, 16], OperandSize::Dword)
}

#[test]
fn vmaxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 95, 228], OperandSize::Qword)
}

#[test]
fn vmaxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1292306111, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 95, 144, 191, 6, 7, 77], OperandSize::Qword)
}

#[test]
fn vmaxss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 118, 153, 95, 236], OperandSize::Dword)
}

#[test]
fn vmaxss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1918600594, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 118, 141, 95, 146, 146, 133, 91, 114], OperandSize::Dword)
}

#[test]
fn vmaxss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 102, 145, 95, 233], OperandSize::Qword)
}

#[test]
fn vmaxss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 102, 139, 95, 47], OperandSize::Qword)
}

