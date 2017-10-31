use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovmskpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 80, 237], OperandSize::Dword)
}

#[test]
fn vmovmskpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 80, 218], OperandSize::Qword)
}

#[test]
fn vmovmskpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(ESP)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 80, 231], OperandSize::Dword)
}

#[test]
fn vmovmskpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(RDX)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 80, 214], OperandSize::Qword)
}

