use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 241], OperandSize::Dword)
}

#[test]
fn cvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 699362767, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 20, 157, 207, 109, 175, 41], OperandSize::Dword)
}

#[test]
fn cvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 231], OperandSize::Qword)
}

#[test]
fn cvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 55], OperandSize::Qword)
}

