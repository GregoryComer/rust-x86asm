use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 237], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDI, 2046569824, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 159, 96, 45, 252, 121], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 233], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 19], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 227], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 856764648, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 137, 232, 48, 17, 51], OperandSize::Qword)
}

